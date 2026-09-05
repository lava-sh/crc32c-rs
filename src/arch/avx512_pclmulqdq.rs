#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

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
    _mm_clmulepi64_si128::<0>(_mm_cvtsi32_si128(a as i32), _mm_cvtsi32_si128(b as i32))
}

#[inline]
#[target_feature(enable = "sse4.2")]
fn crc32_u64(crc: u32, value: u64) -> u32 {
    #[cfg(target_arch = "x86")]
    {
        let crc = _mm_crc32_u32(crc, value as u32);
        _mm_crc32_u32(crc, (value >> 32) as u32)
    }

    #[cfg(target_arch = "x86_64")]
    {
        _mm_crc32_u64(crc as u64, value) as u32
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
        (_mm_cvtsi128_si32(value) as u32) as u64
            | ((_mm_cvtsi128_si32(_mm_srli_si128(value, 4)) as u32) as u64) << 32
    }

    #[cfg(target_arch = "x86_64")]
    {
        match index {
            0 => _mm_extract_epi64::<0>(value) as u64,
            1 => _mm_extract_epi64::<1>(value) as u64,
            _ => unreachable!(),
        }
    }
}

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn xnmodp(mut n: u64) -> u32 {
    let mut stack = !1u64;
    for _ in 0.. {
        if n <= 191 {
            break;
        }
        stack = (stack << 1) + (n & 1);
        n = (n >> 1) - 16;
    }
    stack = !stack;
    let mut acc = 0x80000000u32 >> (n & 31);
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
        let x = _mm_cvtsi32_si128(acc as i32);
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
#[target_feature(enable = "avx512vl,pclmulqdq")]
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
    if len >= 240 {
        let blk = len / 240;
        let klen = blk * 32;
        let end = unsafe { buf.add(len) };
        let mut buf2 = unsafe { buf.add(klen * 3) };
        let mut crc1 = 0u32;
        let mut crc2 = 0u32;
        let vc0;
        let vc1;
        let vc2;
        let vc;
        let mut x0 = unsafe { _mm_loadu_si128(buf2.cast()) };
        let mut x1 = unsafe { _mm_loadu_si128(buf2.add(16).cast()) };
        let mut x2 = unsafe { _mm_loadu_si128(buf2.add(32).cast()) };
        let mut x3 = unsafe { _mm_loadu_si128(buf2.add(48).cast()) };
        let mut x4 = unsafe { _mm_loadu_si128(buf2.add(64).cast()) };
        let mut x5 = unsafe { _mm_loadu_si128(buf2.add(80).cast()) };
        let mut x6 = unsafe { _mm_loadu_si128(buf2.add(96).cast()) };
        let mut x7 = unsafe { _mm_loadu_si128(buf2.add(112).cast()) };
        let mut x8 = unsafe { _mm_loadu_si128(buf2.add(128).cast()) };
        let mut y0;
        let mut y1;
        let mut y2;
        let mut y3;
        let mut y4;
        let mut y5;
        let mut y6;
        let mut y7;
        let mut y8;
        let mut k;

        k = _mm_setr_epi32(0x7e908048u32 as i32, 0, 0xc96cfdc0u32 as i32, 0);
        buf2 = unsafe { buf2.add(144) };
        let mut blocks = blk - 1;
        while blocks != 0 {
            y0 = clmul_lo(x0, k);
            x0 = clmul_hi(x0, k);
            y1 = clmul_lo(x1, k);
            x1 = clmul_hi(x1, k);
            y2 = clmul_lo(x2, k);
            x2 = clmul_hi(x2, k);
            y3 = clmul_lo(x3, k);
            x3 = clmul_hi(x3, k);
            y4 = clmul_lo(x4, k);
            x4 = clmul_hi(x4, k);
            y5 = clmul_lo(x5, k);
            x5 = clmul_hi(x5, k);
            y6 = clmul_lo(x6, k);
            x6 = clmul_hi(x6, k);
            y7 = clmul_lo(x7, k);
            x7 = clmul_hi(x7, k);
            y8 = clmul_lo(x8, k);
            x8 = clmul_hi(x8, k);
            x0 = _mm_ternarylogic_epi64::<0x96>(x0, y0, unsafe { _mm_loadu_si128(buf2.cast()) });
            x1 = _mm_ternarylogic_epi64::<0x96>(x1, y1, unsafe {
                _mm_loadu_si128(buf2.add(16).cast())
            });
            x2 = _mm_ternarylogic_epi64::<0x96>(x2, y2, unsafe {
                _mm_loadu_si128(buf2.add(32).cast())
            });
            x3 = _mm_ternarylogic_epi64::<0x96>(x3, y3, unsafe {
                _mm_loadu_si128(buf2.add(48).cast())
            });
            x4 = _mm_ternarylogic_epi64::<0x96>(x4, y4, unsafe {
                _mm_loadu_si128(buf2.add(64).cast())
            });
            x5 = _mm_ternarylogic_epi64::<0x96>(x5, y5, unsafe {
                _mm_loadu_si128(buf2.add(80).cast())
            });
            x6 = _mm_ternarylogic_epi64::<0x96>(x6, y6, unsafe {
                _mm_loadu_si128(buf2.add(96).cast())
            });
            x7 = _mm_ternarylogic_epi64::<0x96>(x7, y7, unsafe {
                _mm_loadu_si128(buf2.add(112).cast())
            });
            x8 = _mm_ternarylogic_epi64::<0x96>(x8, y8, unsafe {
                _mm_loadu_si128(buf2.add(128).cast())
            });
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
            crc0 = crc32_u64(crc0, unsafe { buf.add(24).cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                buf.add(klen + 24).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                buf.add(klen * 2 + 24).cast::<u64>().read_unaligned()
            });
            buf = unsafe { buf.add(32) };
            buf2 = unsafe { buf2.add(144) };
            blocks -= 1;
        }
        k = _mm_setr_epi32(0xf20c0dfeu32 as i32, 0, 0x493c7d27u32 as i32, 0);
        y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = _mm_ternarylogic_epi64::<0x96>(x0, y0, x1);
        x1 = x2;
        x2 = x3;
        x3 = x4;
        x4 = x5;
        x5 = x6;
        x6 = x7;
        x7 = x8;
        y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        y2 = clmul_lo(x2, k);
        x2 = clmul_hi(x2, k);
        y4 = clmul_lo(x4, k);
        x4 = clmul_hi(x4, k);
        y6 = clmul_lo(x6, k);
        x6 = clmul_hi(x6, k);
        x0 = _mm_ternarylogic_epi64::<0x96>(x0, y0, x1);
        x2 = _mm_ternarylogic_epi64::<0x96>(x2, y2, x3);
        x4 = _mm_ternarylogic_epi64::<0x96>(x4, y4, x5);
        x6 = _mm_ternarylogic_epi64::<0x96>(x6, y6, x7);
        k = _mm_setr_epi32(0x3da6d0cbu32 as i32, 0, 0xba4fc28eu32 as i32, 0);
        y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        y4 = clmul_lo(x4, k);
        x4 = clmul_hi(x4, k);
        x0 = _mm_ternarylogic_epi64::<0x96>(x0, y0, x2);
        x4 = _mm_ternarylogic_epi64::<0x96>(x4, y4, x6);
        k = _mm_setr_epi32(0x740eef02u32 as i32, 0, 0x9e4addf8u32 as i32, 0);
        y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = _mm_ternarylogic_epi64::<0x96>(x0, y0, x4);
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
        crc0 = crc32_u64(crc0, unsafe { buf.add(24).cast::<u64>().read_unaligned() });
        crc1 = crc32_u64(crc1, unsafe {
            buf.add(klen + 24).cast::<u64>().read_unaligned()
        });
        crc2 = crc32_u64(crc2, unsafe {
            buf.add(klen * 2 + 24).cast::<u64>().read_unaligned()
        });
        vc0 = crc_shift(crc0, klen * 2 + blk * 144);
        vc1 = crc_shift(crc1, klen + blk * 144);
        vc2 = crc_shift(crc2, blk * 144);
        vc = extract_u64(_mm_ternarylogic_epi64::<0x96>(vc0, vc1, vc2), 0);
        crc0 = crc32_u64(0, extract_u64(x0, 0));
        crc0 = crc32_u64(crc0, vc ^ extract_u64(x0, 1));
        buf = buf2;
        len = unsafe { end.offset_from(buf) as usize };
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
