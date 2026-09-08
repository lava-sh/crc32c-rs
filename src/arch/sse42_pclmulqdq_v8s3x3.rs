#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "pclmulqdq")]
fn clmul_lo(a: __m128i, b: __m128i) -> __m128i {
    _mm_clmulepi64_si128::<0>(a, b)
}

#[inline]
#[target_feature(enable = "pclmulqdq")]
fn clmul_hi(a: __m128i, b: __m128i) -> __m128i {
    _mm_clmulepi64_si128::<17>(a, b)
}

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn clmul_scalar(a: u32, b: u32) -> __m128i {
    _mm_clmulepi64_si128::<0>(
        _mm_cvtsi32_si128(a.cast_signed()),
        _mm_cvtsi32_si128(b.cast_signed()),
    )
}

#[inline]
#[target_feature(enable = "sse4.2")]
fn crc32_u64(crc: u32, value: u64) -> u32 {
    #[cfg(target_arch = "x86")]
    {
        _mm_crc32_u32(_mm_crc32_u32(crc, value as u32), (value >> 32) as u32)
    }
    #[cfg(target_arch = "x86_64")]
    {
        _mm_crc32_u64(u64::from(crc), value) as u32
    }
}

#[inline]
#[target_feature(enable = "sse4.2")]
fn extract_u64(value: __m128i, index: i32) -> u64 {
    #[cfg(target_arch = "x86")]
    {
        let value = if index == 0 {
            value
        } else {
            _mm_srli_si128(value, 8)
        };
        u64::from(_mm_cvtsi128_si32(value).cast_unsigned())
            | (u64::from(_mm_cvtsi128_si32(_mm_srli_si128(value, 4)).cast_unsigned()) << 32)
    }
    #[cfg(target_arch = "x86_64")]
    {
        match index {
            0 => _mm_extract_epi64::<0>(value).cast_unsigned(),
            1 => _mm_extract_epi64::<1>(value).cast_unsigned(),
            _ => unreachable!(),
        }
    }
}

// x^n mod P, in log(n) time
#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn xnmodp(mut n: u64) -> u32 {
    let mut stack = !1u64;
    while n > 191 {
        stack = (stack << 1) + (n & 1);
        n = (n >> 1) - 16;
    }
    stack = !stack;
    let mut acc = 0x8000_0000_u32 >> (n & 31);
    n >>= 5;
    while n != 0 {
        acc = _mm_crc32_u32(acc, 0);
        n -= 1;
    }
    loop {
        let low = stack & 1;
        stack >>= 1;
        if stack == 0 {
            break;
        }
        let x = _mm_cvtsi32_si128(acc.cast_signed());
        let y = extract_u64(_mm_clmulepi64_si128::<0>(x, x), 0);
        acc = crc32_u64(0, y << low);
    }
    acc
}

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn crc_shift(crc: u32, nbytes: usize) -> __m128i {
    clmul_scalar(crc, xnmodp((nbytes * 8 - 33) as u64))
}

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
pub unsafe fn crc32c(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    crc0 = !crc0;
    while len != 0 && (buf as usize & 7) != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    if (buf as usize & 8) != 0 && len >= 8 {
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    if len >= 208 {
        let blk = (len - 8) / 200;
        let klen = blk * 24;
        let mut buf2 = buf;
        let mut crc1 = 0u32;
        let mut crc2 = 0u32;
        // First vector chunk.
        let mut x0 = unsafe { _mm_loadu_si128(buf2.cast()) };
        let mut x1 = unsafe { _mm_loadu_si128(buf2.add(16).cast()) };
        let mut x2 = unsafe { _mm_loadu_si128(buf2.add(32).cast()) };
        let mut x3 = unsafe { _mm_loadu_si128(buf2.add(48).cast()) };
        let mut x4 = unsafe { _mm_loadu_si128(buf2.add(64).cast()) };
        let mut x5 = unsafe { _mm_loadu_si128(buf2.add(80).cast()) };
        let mut x6 = unsafe { _mm_loadu_si128(buf2.add(96).cast()) };
        let mut x7 = unsafe { _mm_loadu_si128(buf2.add(112).cast()) };
        let k = _mm_setr_epi32(
            0x6992_cea2_u32.cast_signed(),
            0,
            0x0d3b_6092_u32.cast_signed(),
            0,
        );
        x0 = _mm_xor_si128(_mm_cvtsi32_si128(crc0.cast_signed()), x0);
        crc0 = 0;
        buf2 = unsafe { buf2.add(128) };
        len -= 200;
        buf = unsafe { buf.add(blk * 128) };
        // Main loop.
        while len >= 208 {
            let y0 = clmul_lo(x0, k);
            x0 = _mm_xor_si128(
                clmul_hi(x0, k),
                _mm_xor_si128(y0, unsafe { _mm_loadu_si128(buf2.cast()) }),
            );
            let y1 = clmul_lo(x1, k);
            x1 = _mm_xor_si128(
                clmul_hi(x1, k),
                _mm_xor_si128(y1, unsafe { _mm_loadu_si128(buf2.add(16).cast()) }),
            );
            let y2 = clmul_lo(x2, k);
            x2 = _mm_xor_si128(
                clmul_hi(x2, k),
                _mm_xor_si128(y2, unsafe { _mm_loadu_si128(buf2.add(32).cast()) }),
            );
            let y3 = clmul_lo(x3, k);
            x3 = _mm_xor_si128(
                clmul_hi(x3, k),
                _mm_xor_si128(y3, unsafe { _mm_loadu_si128(buf2.add(48).cast()) }),
            );
            let y4 = clmul_lo(x4, k);
            x4 = _mm_xor_si128(
                clmul_hi(x4, k),
                _mm_xor_si128(y4, unsafe { _mm_loadu_si128(buf2.add(64).cast()) }),
            );
            let y5 = clmul_lo(x5, k);
            x5 = _mm_xor_si128(
                clmul_hi(x5, k),
                _mm_xor_si128(y5, unsafe { _mm_loadu_si128(buf2.add(80).cast()) }),
            );
            let y6 = clmul_lo(x6, k);
            x6 = _mm_xor_si128(
                clmul_hi(x6, k),
                _mm_xor_si128(y6, unsafe { _mm_loadu_si128(buf2.add(96).cast()) }),
            );
            let y7 = clmul_lo(x7, k);
            x7 = _mm_xor_si128(
                clmul_hi(x7, k),
                _mm_xor_si128(y7, unsafe { _mm_loadu_si128(buf2.add(112).cast()) }),
            );
            // Final scalar chunk.
            crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                buf.add(klen).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                buf.add(klen * 2).cast::<u64>().read_unaligned()
            });
            crc0 = crc32_u64(crc0, unsafe { buf.add(8).cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                buf.add(klen + 8).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                buf.add(klen * 2 + 8).cast::<u64>().read_unaligned()
            });
            crc0 = crc32_u64(crc0, unsafe { buf.add(16).cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                buf.add(klen + 16).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                buf.add(klen * 2 + 16).cast::<u64>().read_unaligned()
            });
            buf = unsafe { buf.add(24) };
            buf2 = unsafe { buf2.add(128) };
            len -= 200;
        }
        // Reduce x0 ... x7 to just x0.
        let k = _mm_setr_epi32(
            0xf20c_0dfe_u32.cast_signed(),
            0,
            0x493c_7d27_u32.cast_signed(),
            0,
        );
        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        let y2 = clmul_lo(x2, k);
        x2 = clmul_hi(x2, k);
        let y4 = clmul_lo(x4, k);
        x4 = clmul_hi(x4, k);
        let y6 = clmul_lo(x6, k);
        x6 = clmul_hi(x6, k);
        x0 = _mm_xor_si128(x0, _mm_xor_si128(y0, x1));
        x2 = _mm_xor_si128(x2, _mm_xor_si128(y2, x3));
        x4 = _mm_xor_si128(x4, _mm_xor_si128(y4, x5));
        x6 = _mm_xor_si128(x6, _mm_xor_si128(y6, x7));
        let k = _mm_setr_epi32(
            0x3da6_d0cb_u32.cast_signed(),
            0,
            0xba4f_c28e_u32.cast_signed(),
            0,
        );
        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        let y4 = clmul_lo(x4, k);
        x4 = clmul_hi(x4, k);
        x0 = _mm_xor_si128(x0, _mm_xor_si128(y0, x2));
        x4 = _mm_xor_si128(x4, _mm_xor_si128(y4, x6));
        let k = _mm_setr_epi32(
            0x740e_ef02_u32.cast_signed(),
            0,
            0x9e4a_ddf8_u32.cast_signed(),
            0,
        );
        let y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = _mm_xor_si128(x0, _mm_xor_si128(y0, x4));
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        crc1 = crc32_u64(crc1, unsafe {
            buf.add(klen).cast::<u64>().read_unaligned()
        });
        crc2 = crc32_u64(crc2, unsafe {
            buf.add(klen * 2).cast::<u64>().read_unaligned()
        });
        crc0 = crc32_u64(crc0, unsafe { buf.add(8).cast::<u64>().read_unaligned() });
        crc1 = crc32_u64(crc1, unsafe {
            buf.add(klen + 8).cast::<u64>().read_unaligned()
        });
        crc2 = crc32_u64(crc2, unsafe {
            buf.add(klen * 2 + 8).cast::<u64>().read_unaligned()
        });
        crc0 = crc32_u64(crc0, unsafe { buf.add(16).cast::<u64>().read_unaligned() });
        crc1 = crc32_u64(crc1, unsafe {
            buf.add(klen + 16).cast::<u64>().read_unaligned()
        });
        crc2 = crc32_u64(crc2, unsafe {
            buf.add(klen * 2 + 16).cast::<u64>().read_unaligned()
        });
        buf = unsafe { buf.add(24) };
        let vc0 = crc_shift(crc0, klen * 2 + 8);
        let vc1 = crc_shift(crc1, klen + 8);
        let mut vc = extract_u64(_mm_xor_si128(vc0, vc1), 0);
        // Reduce 128 bits to 32 bits, and multiply by x^32.
        vc ^= extract_u64(
            crc_shift(
                crc32_u64(crc32_u64(0, extract_u64(x0, 0)), extract_u64(x0, 1)),
                klen * 3 + 8,
            ),
            0,
        );
        // Final 8 bytes.
        buf = unsafe { buf.add(klen * 2) };
        crc0 = crc2;
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() } ^ vc);
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len >= 8 {
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    !crc0
}
