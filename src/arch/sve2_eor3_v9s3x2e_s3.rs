// SVE2 port of `aes_sha3_v9s3x2e_s3.rs` (itself a port of corsix/fast-crc32's
// `neon_eor3` `v9s3x2e_s3` schedule): the same three scalar CRC streams over
// `buf` plus nine PMULL-folded 16-byte accumulators over `buf2`, but with the
// vector part expressed through SVE2 intrinsics instead of inline asm.
//
// What actually changes:
//
// * `pmull` / `pmull2` inline asm becomes `svpmullb_pair_u64` /
//   `svpmullt_pair_u64`. Rust has had stable NEON polynomial-multiply wrappers
//   (`vmull_p64`, `vmull_high_p64`) for a while, so leaving `asm!` is possible
//   without SVE at all; the SVE2 instructions are worth it only where the core
//   implements the SVE2-AES extension (FEAT_SVE_AES + FEAT_SVE_PMULL128), i.e.
//   `is_aarch64_feature_detected!("sve2-aes")`. That is a *stricter* feature
//   than plain SVE2, and it is what this module is gated on.
// * `veor3q_u64` (SHA3 extension) becomes `sveor3_u64` (SVE2 proper).
// * The vectors are `svuint64_t`, so the kernel is vector-length agnostic: the
//   same binary runs on 128-bit (Neoverse N2/V2, Cortex-X2/A710) and 256/512-bit
//   (Graviton3, A64FX-class) cores.
//
// What does *not* change, contrary to the usual SVE2 pitch:
//
// * The extra lanes are not used. `svld1rq_u64` replicates one 16-byte quadword
//   into every 128-bit lane and the same constants are broadcast, so every lane
//   computes the code below and only lane 0 carries the state. Using the width
//   would need the nine accumulators packed into lanes, which changes the final
//   reduction (the nine streams are folded with *different* powers of x, so the
//   pairwise reduction chain would have to become a shuffle-based tree).
// * The tail loops cannot go away. There is no SVE instruction that computes
//   CRC-32C: `CRC32B/H/W/X` are scalar, and no vector form exists in SVE2. The
//   `< 8` and `< 16` remainders still have to go through `__crc32cb`, one byte
//   per instruction, exactly as in the NEON kernel. Predication cannot help,
//   because a partial vector still has to be folded byte by byte afterwards.
//
// So this is a faithful mechanical port, not a new algorithm: identical
// instruction mix per byte, no asm barriers around the multiply, and the option
// to grow with the hardware vector length later.
//
// Because it does the same work as the NEON kernel, `simd_dispatch.rs` does not
// prefer it: picking it by default would change nothing on a 128-bit-VL core
// while making the default path depend on the SVE2 crypto extension. It is
// reachable explicitly, as `crc32c_rs.crc32c_sve2_eor3_v9s3x2e_s3`.

#![allow(clippy::wildcard_imports, clippy::cast_ptr_alignment)]

use core::arch::aarch64::*;

// Every function below carries the full feature set it needs: `sve` and `sve2`
// for the vector code, `sve2-aes` for PMULL, `crc` for `__crc32c*` and `aes`
// for the NEON `vmull_p8` still used by `xnmodp`.

/// Polynomial multiply, bottom halves, on every 128-bit lane: the SVE2 spelling
/// of NEON `pmull v.1q, v.1d, v.1d`.
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
fn clmul_lo(a: svuint64_t, b: svuint64_t) -> svuint64_t {
    svpmullb_pair_u64(a, b)
}

/// Polynomial multiply, top halves: the SVE2 spelling of `pmull2`.
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
fn clmul_hi(a: svuint64_t, b: svuint64_t) -> svuint64_t {
    svpmullt_pair_u64(a, b)
}

/// Low 64 bits of lane 0.
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
fn lane0(v: svuint64_t) -> u64 {
    svlastb_u64(svwhilelt_b64_u64(0, 1), v)
}

/// Low 64 bits of lane 0's high element.
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
fn lane1(v: svuint64_t) -> u64 {
    svlastb_u64(svwhilelt_b64_u64(0, 2), v)
}

/// One 16-byte quadword, replicated into every 128-bit lane. Reading exactly the
/// quadword (rather than `svld1_u64` with a full predicate, which would consume
/// `VL` bytes) keeps the loop from over-reading the buffer on wide vectors.
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
unsafe fn load_lane0(ptr: *const u8) -> svuint64_t {
    // SAFETY: the caller guarantees 16 readable bytes at `ptr`.
    unsafe { svld1rq_u64(svptrue_b64(), ptr.cast::<u64>()) }
}

/// The two `u64` constants of a schedule step, replicated per lane.
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
fn k_pair(lo: u64, hi: u64) -> svuint64_t {
    svdupq_n_u64(lo, hi)
}

/// `a * b` over GF(2) in lane 0, low 64 bits only (the callers only ever use
/// the low half, so there is no need to carry the full 128-bit result around).
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
fn clmul_scalar_lo(a: u32, b: u32) -> u64 {
    lane0(svpmullb_pair_n_u64(svdup_n_u64(u64::from(a)), u64::from(b)))
}

// The rest of the schedule is scalar or NEON-only and is kept as-is: `vmull_p8`
// (the 8x8 polynomial multiply used by `xnmodp`) has no SVE2 equivalent.

// x^n mod P, in log(n) time
#[inline]
#[target_feature(enable = "crc,aes")]
fn xnmodp(mut n: u64) -> u32 {
    let mut stack = !1_u64;
    while n > 191 {
        stack = (stack << 1) + (n & 1);
        n = (n >> 1) - 16;
    }
    stack = !stack;

    let mut acc = 0x8000_0000_u32 >> (n & 31);
    n >>= 5;
    while n != 0 {
        acc = __crc32cw(acc, 0);
        n -= 1;
    }

    loop {
        let low = stack & 1;
        stack >>= 1;
        if stack == 0 {
            break;
        }

        let x = vreinterpret_p8_u64(vmov_n_u64(u64::from(acc)));
        let y = vgetq_lane_u64(vreinterpretq_u64_p16(vmull_p8(x, x)), 0);
        acc = __crc32cd(0, y << low);
    }
    acc
}

#[inline]
#[target_feature(enable = "crc,aes,sve,sve2,sve2-aes")]
fn crc_shift(crc: u32, nbytes: usize) -> u64 {
    clmul_scalar_lo(crc, xnmodp((nbytes * 8 - 33) as u64))
}

#[inline]
#[target_feature(enable = "crc,aes,sve,sve2,sve2-aes")]
unsafe fn crc32c_small(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    unsafe {
        core::hint::assert_unchecked((128..=1024).contains(&len));
    }

    let klen = ((len - 8) / 24) * 8;
    let mut crc1 = 0_u32;
    let mut crc2 = 0_u32;
    loop {
        crc0 = unsafe { __crc32cd(crc0, buf.cast::<u64>().read_unaligned()) };
        crc1 = unsafe { __crc32cd(crc1, buf.add(klen).cast::<u64>().read_unaligned()) };
        crc2 = unsafe { __crc32cd(crc2, buf.add(klen * 2).cast::<u64>().read_unaligned()) };
        buf = unsafe { buf.add(8) };
        len -= 24;
        if len < 32 {
            break;
        }
    }

    let vc = crc_shift(crc0, klen * 2 + 8) ^ crc_shift(crc1, klen + 8);

    buf = unsafe { buf.add(klen * 2) };
    crc0 = crc2;
    crc0 = unsafe { __crc32cd(crc0, buf.cast::<u64>().read_unaligned() ^ vc) };
    buf = unsafe { buf.add(8) };
    len -= 8;

    while len >= 8 {
        crc0 = unsafe { __crc32cd(crc0, buf.cast::<u64>().read_unaligned()) };
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len != 0 {
        crc0 = unsafe { __crc32cb(crc0, *buf) };
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    crc0
}

#[inline]
#[target_feature(enable = "crc,aes,sve,sve2,sve2-aes")]
pub unsafe fn crc32c(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    crc0 = !crc0;

    if (128..=1024).contains(&len) {
        return !unsafe { crc32c_small(crc0, buf, len) };
    }

    while len != 0 && (buf as usize & 7) != 0 {
        crc0 = unsafe { __crc32cb(crc0, *buf) };
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    if (buf as usize & 8) != 0 && len >= 8 {
        crc0 = unsafe { __crc32cd(crc0, *(buf.cast::<u64>())) };
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    if len >= 192 {
        let end = unsafe { buf.add(len) };
        let blk = len / 192;
        let klen = blk * 16;
        let mut buf2 = unsafe { buf.add(klen * 3) };
        let limit = unsafe { buf.add(klen).sub(32) };
        let mut crc1 = 0_u32;
        let mut crc2 = 0_u32;

        // First vector chunk.
        let mut x0 = unsafe { load_lane0(buf2) };
        let mut x1 = unsafe { load_lane0(buf2.add(16)) };
        let mut x2 = unsafe { load_lane0(buf2.add(32)) };
        let mut x3 = unsafe { load_lane0(buf2.add(48)) };
        let mut x4 = unsafe { load_lane0(buf2.add(64)) };
        let mut x5 = unsafe { load_lane0(buf2.add(80)) };
        let mut x6 = unsafe { load_lane0(buf2.add(96)) };
        let mut x7 = unsafe { load_lane0(buf2.add(112)) };
        let mut x8 = unsafe { load_lane0(buf2.add(128)) };

        let mut k = k_pair(0x7e90_8048, 0xc96c_fdc0);
        buf2 = unsafe { buf2.add(144) };

        // Main loop.
        while buf <= limit {
            let y0 = clmul_lo(x0, k);
            x0 = clmul_hi(x0, k);
            let y1 = clmul_lo(x1, k);
            x1 = clmul_hi(x1, k);
            let y2 = clmul_lo(x2, k);
            x2 = clmul_hi(x2, k);
            let y3 = clmul_lo(x3, k);
            x3 = clmul_hi(x3, k);
            let y4 = clmul_lo(x4, k);
            x4 = clmul_hi(x4, k);
            let y5 = clmul_lo(x5, k);
            x5 = clmul_hi(x5, k);
            let y6 = clmul_lo(x6, k);
            x6 = clmul_hi(x6, k);
            let y7 = clmul_lo(x7, k);
            x7 = clmul_hi(x7, k);
            let y8 = clmul_lo(x8, k);
            x8 = clmul_hi(x8, k);

            x0 = sveor3_u64(x0, y0, unsafe { load_lane0(buf2) });
            x1 = sveor3_u64(x1, y1, unsafe { load_lane0(buf2.add(16)) });
            x2 = sveor3_u64(x2, y2, unsafe { load_lane0(buf2.add(32)) });
            x3 = sveor3_u64(x3, y3, unsafe { load_lane0(buf2.add(48)) });
            x4 = sveor3_u64(x4, y4, unsafe { load_lane0(buf2.add(64)) });
            x5 = sveor3_u64(x5, y5, unsafe { load_lane0(buf2.add(80)) });
            x6 = sveor3_u64(x6, y6, unsafe { load_lane0(buf2.add(96)) });
            x7 = sveor3_u64(x7, y7, unsafe { load_lane0(buf2.add(112)) });
            x8 = sveor3_u64(x8, y8, unsafe { load_lane0(buf2.add(128)) });

            unsafe {
                crc0 = __crc32cd(crc0, *(buf.cast::<u64>()));
                crc1 = __crc32cd(crc1, *(buf.add(klen).cast::<u64>()));
                crc2 = __crc32cd(crc2, *(buf.add(klen * 2).cast::<u64>()));
                crc0 = __crc32cd(crc0, *(buf.add(8).cast::<u64>()));
                crc1 = __crc32cd(crc1, *(buf.add(klen + 8).cast::<u64>()));
                crc2 = __crc32cd(crc2, *(buf.add(klen * 2 + 8).cast::<u64>()));
            }

            buf = unsafe { buf.add(16) };
            buf2 = unsafe { buf2.add(144) };
        }

        // Reduce x0 ... x8 to just x0.
        k = k_pair(0xf20c_0dfe, 0x493c_7d27);

        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = sveor3_u64(x0, y0, x1);
        x1 = x2;
        x2 = x3;
        x3 = x4;
        x4 = x5;
        x5 = x6;
        x6 = x7;
        x7 = x8;

        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        let y2 = clmul_lo(x2, k);
        x2 = clmul_hi(x2, k);
        let y4 = clmul_lo(x4, k);
        x4 = clmul_hi(x4, k);
        let y6 = clmul_lo(x6, k);
        x6 = clmul_hi(x6, k);

        x0 = sveor3_u64(x0, y0, x1);
        x2 = sveor3_u64(x2, y2, x3);
        x4 = sveor3_u64(x4, y4, x5);
        x6 = sveor3_u64(x6, y6, x7);

        k = k_pair(0x3da6_d0cb, 0xba4f_c28e);

        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        let y4 = clmul_lo(x4, k);
        x4 = clmul_hi(x4, k);
        x0 = sveor3_u64(x0, y0, x2);
        x4 = sveor3_u64(x4, y4, x6);

        k = k_pair(0x740e_ef02, 0x9e4a_ddf8);
        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = sveor3_u64(x0, y0, x4);

        // Final scalar chunk.
        unsafe {
            crc0 = __crc32cd(crc0, *(buf.cast::<u64>()));
            crc1 = __crc32cd(crc1, *(buf.add(klen).cast::<u64>()));
            crc2 = __crc32cd(crc2, *(buf.add(klen * 2).cast::<u64>()));
            crc0 = __crc32cd(crc0, *(buf.add(8).cast::<u64>()));
            crc1 = __crc32cd(crc1, *(buf.add(klen + 8).cast::<u64>()));
            crc2 = __crc32cd(crc2, *(buf.add(klen * 2 + 8).cast::<u64>()));
        }

        let vc0 = crc_shift(crc0, klen * 2 + blk * 144);
        let vc1 = crc_shift(crc1, klen + blk * 144);
        let vc2 = crc_shift(crc2, blk * 144);
        let vc = vc0 ^ vc1 ^ vc2;

        // Reduce 128 bits to 32 bits, and multiply by x^32.
        crc0 = __crc32cd(0, lane0(x0));
        crc0 = __crc32cd(crc0, vc ^ lane1(x0));

        buf = buf2;
        len = unsafe { end.offset_from(buf).cast_unsigned() };
    }

    if len >= 32 {
        let klen = ((len - 8) / 24) * 8;
        let mut crc1 = 0_u32;
        let mut crc2 = 0_u32;

        // Main loop.
        loop {
            unsafe {
                crc0 = __crc32cd(crc0, *(buf.cast::<u64>()));
                crc1 = __crc32cd(crc1, *(buf.add(klen).cast::<u64>()));
                crc2 = __crc32cd(crc2, *(buf.add(klen * 2).cast::<u64>()));
            }
            buf = unsafe { buf.add(8) };
            len -= 24;
            if len < 32 {
                break;
            }
        }

        let vc = crc_shift(crc0, klen * 2 + 8) ^ crc_shift(crc1, klen + 8);

        // Final 8 bytes.
        buf = unsafe { buf.add(klen * 2) };
        crc0 = crc2;
        crc0 = unsafe { __crc32cd(crc0, *(buf.cast::<u64>()) ^ vc) };
        buf = unsafe { buf.add(8) };
        len -= 8;
    }

    while len >= 8 {
        crc0 = unsafe { __crc32cd(crc0, *(buf.cast::<u64>())) };
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len != 0 {
        crc0 = unsafe { __crc32cb(crc0, *buf) };
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    !crc0
}
