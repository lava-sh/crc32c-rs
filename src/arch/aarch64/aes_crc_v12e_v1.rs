#![allow(clippy::wildcard_imports, clippy::cast_ptr_alignment)]

use core::arch::aarch64::*;

#[inline]
#[target_feature(enable = "aes")]
fn clmul_lo_e(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
    let a = vgetq_lane_u64::<0>(a);
    let b = vgetq_lane_u64::<0>(b);
    let clmul = vmull_p64(a, b);
    veorq_u64(vreinterpretq_u64_p128(clmul), c)
}

#[inline]
#[target_feature(enable = "aes")]
fn clmul_hi_e(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
    let a = vreinterpretq_p64_u64(a);
    let b = vreinterpretq_p64_u64(b);
    let clmul = vmull_high_p64(a, b);
    veorq_u64(vreinterpretq_u64_p128(clmul), c)
}

#[inline]
#[target_feature(enable = "aes")]
fn clmul_scalar(a: u32, b: u32) -> uint64x2_t {
    vreinterpretq_u64_p128(vmull_p64(u64::from(a), u64::from(b)))
}

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
#[target_feature(enable = "crc,aes")]
fn crc_shift(crc: u32, nbytes: usize) -> uint64x2_t {
    clmul_scalar(crc, xnmodp((nbytes * 8 - 33) as u64))
}

#[inline]
#[target_feature(enable = "crc,aes")]
unsafe fn crc32c_small(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    if len >= 112 {
        let blk = len / 112;
        let klen = blk * 16;
        let mut buf2 = unsafe { buf.add(klen * 4) };
        let mut crc1 = 0_u32;
        let mut crc2 = 0_u32;
        let mut crc3 = 0_u32;

        let mut x0 = unsafe { vld1q_u64(buf2.cast::<u64>()) };
        let mut x1 = unsafe { vld1q_u64(buf2.add(16).cast::<u64>()) };
        let mut x2 = unsafe { vld1q_u64(buf2.add(32).cast::<u64>()) };
        let k_values = [0x1c29_1d04_u64, 0xddc0_152b_u64];
        let mut k = unsafe { vld1q_u64(k_values.as_ptr()) };
        buf2 = unsafe { buf2.add(48) };

        if klen >= 32 {
            let limit = unsafe { buf.add(klen - 32) };
            while buf <= limit {
                let y0 = clmul_lo_e(x0, k, unsafe { vld1q_u64(buf2.cast::<u64>()) });
                x0 = clmul_hi_e(x0, k, y0);
                let y1 = clmul_lo_e(x1, k, unsafe { vld1q_u64(buf2.add(16).cast::<u64>()) });
                x1 = clmul_hi_e(x1, k, y1);
                let y2 = clmul_lo_e(x2, k, unsafe { vld1q_u64(buf2.add(32).cast::<u64>()) });
                x2 = clmul_hi_e(x2, k, y2);

                unsafe {
                    crc0 = __crc32cd(crc0, buf.cast::<u64>().read_unaligned());
                    crc1 = __crc32cd(crc1, buf.add(klen).cast::<u64>().read_unaligned());
                    crc2 = __crc32cd(crc2, buf.add(klen * 2).cast::<u64>().read_unaligned());
                    crc3 = __crc32cd(crc3, buf.add(klen * 3).cast::<u64>().read_unaligned());
                    crc0 = __crc32cd(crc0, buf.add(8).cast::<u64>().read_unaligned());
                    crc1 = __crc32cd(crc1, buf.add(klen + 8).cast::<u64>().read_unaligned());
                    crc2 = __crc32cd(crc2, buf.add(klen * 2 + 8).cast::<u64>().read_unaligned());
                    crc3 = __crc32cd(crc3, buf.add(klen * 3 + 8).cast::<u64>().read_unaligned());
                }

                buf = unsafe { buf.add(16) };
                buf2 = unsafe { buf2.add(48) };
            }
        }

        let k_values = [0xf20c_0dfe_u64, 0x493c_7d27_u64];
        k = unsafe { vld1q_u64(k_values.as_ptr()) };
        let y0 = clmul_lo_e(x0, k, x1);
        x0 = clmul_hi_e(x0, k, y0);
        let x1 = x2;
        let y0 = clmul_lo_e(x0, k, x1);
        x0 = clmul_hi_e(x0, k, y0);

        unsafe {
            crc0 = __crc32cd(crc0, buf.cast::<u64>().read_unaligned());
            crc1 = __crc32cd(crc1, buf.add(klen).cast::<u64>().read_unaligned());
            crc2 = __crc32cd(crc2, buf.add(klen * 2).cast::<u64>().read_unaligned());
            crc3 = __crc32cd(crc3, buf.add(klen * 3).cast::<u64>().read_unaligned());
            crc0 = __crc32cd(crc0, buf.add(8).cast::<u64>().read_unaligned());
            crc1 = __crc32cd(crc1, buf.add(klen + 8).cast::<u64>().read_unaligned());
            crc2 = __crc32cd(crc2, buf.add(klen * 2 + 8).cast::<u64>().read_unaligned());
            crc3 = __crc32cd(crc3, buf.add(klen * 3 + 8).cast::<u64>().read_unaligned());
        }

        let vc0 = crc_shift(crc0, klen * 3 + blk * 48);
        let vc1 = crc_shift(crc1, klen * 2 + blk * 48);
        let vc2 = crc_shift(crc2, klen + blk * 48);
        let vc3 = crc_shift(crc3, blk * 48);
        let vc = vgetq_lane_u64(veorq_u64(veorq_u64(vc0, vc1), veorq_u64(vc2, vc3)), 0);

        crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
        crc0 = __crc32cd(crc0, vc ^ vgetq_lane_u64(x0, 1));

        buf = buf2;
        len -= blk * 112;
    } else if len >= 16 {
        let mut x0 = unsafe { vld1q_u64(buf.cast::<u64>()) };
        let k_values = [0xf20c_0dfe_u64, 0x493c_7d27_u64];
        let k = unsafe { vld1q_u64(k_values.as_ptr()) };
        x0 = veorq_u64(vsetq_lane_u64(u64::from(crc0), vmovq_n_u64(0), 0), x0);
        buf = unsafe { buf.add(16) };
        len -= 16;

        while len >= 16 {
            let y0 = clmul_lo_e(x0, k, unsafe { vld1q_u64(buf.cast::<u64>()) });
            x0 = clmul_hi_e(x0, k, y0);
            buf = unsafe { buf.add(16) };
            len -= 16;
        }

        crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
        crc0 = __crc32cd(crc0, vgetq_lane_u64(x0, 1));
    }

    while len >= 8 {
        crc0 = __crc32cd(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len != 0 {
        crc0 = __crc32cb(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    crc0
}

#[inline]
#[target_feature(enable = "crc,aes")]
pub unsafe fn crc32c(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    crc0 = !crc0;

    if len <= 16384 {
        return !unsafe { crc32c_small(crc0, buf, len) };
    }

    let align = buf as usize & 7;
    if align != 0 {
        let n = (8 - align).min(len);
        for _ in 0..n {
            crc0 = __crc32cb(crc0, unsafe { *buf });
            buf = unsafe { buf.add(1) };
        }
        len -= n;
    }
    if (buf as usize & 8) != 0 && len >= 8 {
        crc0 = __crc32cd(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    if len >= 192 {
        let end = unsafe { buf.add(len) };
        let limit = unsafe { buf.add(len - 192) };
        // First vector chunk.
        let mut x0 = unsafe { vld1q_u64(buf.cast::<u64>()) };
        let mut x1 = unsafe { vld1q_u64(buf.add(16).cast::<u64>()) };
        let mut x2 = unsafe { vld1q_u64(buf.add(32).cast::<u64>()) };
        let mut x3 = unsafe { vld1q_u64(buf.add(48).cast::<u64>()) };
        let mut x4 = unsafe { vld1q_u64(buf.add(64).cast::<u64>()) };
        let mut x5 = unsafe { vld1q_u64(buf.add(80).cast::<u64>()) };
        let mut x6 = unsafe { vld1q_u64(buf.add(96).cast::<u64>()) };
        let mut x7 = unsafe { vld1q_u64(buf.add(112).cast::<u64>()) };
        let mut x8 = unsafe { vld1q_u64(buf.add(128).cast::<u64>()) };
        let mut x9 = unsafe { vld1q_u64(buf.add(144).cast::<u64>()) };
        let mut x10 = unsafe { vld1q_u64(buf.add(160).cast::<u64>()) };
        let mut x11 = unsafe { vld1q_u64(buf.add(176).cast::<u64>()) };
        let mut k = unsafe { vld1q_u64([0xa87a_b8a8_u64, 0xab7a_ff2a_u64].as_ptr()) };
        x0 = veorq_u64(vsetq_lane_u64(u64::from(crc0), vmovq_n_u64(0), 0), x0);
        buf = unsafe { buf.add(192) };
        // Main loop.
        while buf <= limit {
            let y0 = clmul_lo_e(x0, k, unsafe { vld1q_u64(buf.cast::<u64>()) });
            x0 = clmul_hi_e(x0, k, y0);
            let y1 = clmul_lo_e(x1, k, unsafe { vld1q_u64(buf.add(16).cast::<u64>()) });
            x1 = clmul_hi_e(x1, k, y1);
            let y2 = clmul_lo_e(x2, k, unsafe { vld1q_u64(buf.add(32).cast::<u64>()) });
            x2 = clmul_hi_e(x2, k, y2);
            let y3 = clmul_lo_e(x3, k, unsafe { vld1q_u64(buf.add(48).cast::<u64>()) });
            x3 = clmul_hi_e(x3, k, y3);
            let y4 = clmul_lo_e(x4, k, unsafe { vld1q_u64(buf.add(64).cast::<u64>()) });
            x4 = clmul_hi_e(x4, k, y4);
            let y5 = clmul_lo_e(x5, k, unsafe { vld1q_u64(buf.add(80).cast::<u64>()) });
            x5 = clmul_hi_e(x5, k, y5);
            let y6 = clmul_lo_e(x6, k, unsafe { vld1q_u64(buf.add(96).cast::<u64>()) });
            x6 = clmul_hi_e(x6, k, y6);
            let y7 = clmul_lo_e(x7, k, unsafe { vld1q_u64(buf.add(112).cast::<u64>()) });
            x7 = clmul_hi_e(x7, k, y7);
            let y8 = clmul_lo_e(x8, k, unsafe { vld1q_u64(buf.add(128).cast::<u64>()) });
            x8 = clmul_hi_e(x8, k, y8);
            let y9 = clmul_lo_e(x9, k, unsafe { vld1q_u64(buf.add(144).cast::<u64>()) });
            x9 = clmul_hi_e(x9, k, y9);
            let y10 = clmul_lo_e(x10, k, unsafe { vld1q_u64(buf.add(160).cast::<u64>()) });
            x10 = clmul_hi_e(x10, k, y10);
            let y11 = clmul_lo_e(x11, k, unsafe { vld1q_u64(buf.add(176).cast::<u64>()) });
            x11 = clmul_hi_e(x11, k, y11);
            buf = unsafe { buf.add(192) };
        }
        // Reduce x0 ... x11 to just x0.
        k = unsafe { vld1q_u64([0xf20c_0dfe_u64, 0x493c_7d27_u64].as_ptr()) };
        let y0 = clmul_lo_e(x0, k, x1);
        x0 = clmul_hi_e(x0, k, y0);
        let y2 = clmul_lo_e(x2, k, x3);
        x2 = clmul_hi_e(x2, k, y2);
        let y4 = clmul_lo_e(x4, k, x5);
        x4 = clmul_hi_e(x4, k, y4);
        let y6 = clmul_lo_e(x6, k, x7);
        x6 = clmul_hi_e(x6, k, y6);
        let y8 = clmul_lo_e(x8, k, x9);
        x8 = clmul_hi_e(x8, k, y8);
        let y10 = clmul_lo_e(x10, k, x11);
        x10 = clmul_hi_e(x10, k, y10);
        k = unsafe { vld1q_u64([0x3da6_d0cb_u64, 0xba4f_c28e_u64].as_ptr()) };
        let y0 = clmul_lo_e(x0, k, x2);
        x0 = clmul_hi_e(x0, k, y0);
        let y4 = clmul_lo_e(x4, k, x6);
        x4 = clmul_hi_e(x4, k, y4);
        let y8 = clmul_lo_e(x8, k, x10);
        x8 = clmul_hi_e(x8, k, y8);
        k = unsafe { vld1q_u64([0x740e_ef02_u64, 0x9e4a_ddf8_u64].as_ptr()) };
        let y0 = clmul_lo_e(x0, k, x4);
        x0 = clmul_hi_e(x0, k, y0);
        x4 = x8;
        let y0 = clmul_lo_e(x0, k, x4);
        x0 = clmul_hi_e(x0, k, y0);
        // Reduce 128 bits to 32 bits, and multiply by x^32.
        crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
        crc0 = __crc32cd(crc0, vgetq_lane_u64(x0, 1));
        len = unsafe { end.offset_from(buf).cast_unsigned() };
    }
    if len >= 16 {
        // First vector chunk.
        let mut x0 = unsafe { vld1q_u64(buf.cast::<u64>()) };
        let k = unsafe { vld1q_u64([0xf20c_0dfe_u64, 0x493c_7d27_u64].as_ptr()) };
        x0 = veorq_u64(vsetq_lane_u64(u64::from(crc0), vmovq_n_u64(0), 0), x0);
        buf = unsafe { buf.add(16) };
        len -= 16;
        // Main loop.
        while len >= 16 {
            let y0 = clmul_lo_e(x0, k, unsafe { vld1q_u64(buf.cast::<u64>()) });
            x0 = clmul_hi_e(x0, k, y0);
            buf = unsafe { buf.add(16) };
            len -= 16;
        }
        // Reduce 128 bits to 32 bits, and multiply by x^32.
        crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
        crc0 = __crc32cd(crc0, vgetq_lane_u64(x0, 1));
    }

    while len >= 8 {
        crc0 = __crc32cd(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len != 0 {
        crc0 = __crc32cb(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    !crc0
}
