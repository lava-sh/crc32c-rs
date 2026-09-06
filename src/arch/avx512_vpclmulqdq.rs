#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
#[target_feature(enable = "avx512f,vpclmulqdq")]
fn clmul_lo(a: __m512i, b: __m512i) -> __m512i {
    _mm512_clmulepi64_epi128::<0>(a, b)
}

#[inline]
#[target_feature(enable = "avx512f,vpclmulqdq")]
fn clmul_hi(a: __m512i, b: __m512i) -> __m512i {
    _mm512_clmulepi64_epi128::<17>(a, b)
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
        let crc = _mm_crc32_u32(crc, value as u32);
        _mm_crc32_u32(crc, (value >> 32) as u32)
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
        (_mm_cvtsi128_si32(value) as u32) as u64
            | ((_mm_cvtsi128_si32(_mm_srli_si128(value, 4)) as u32) as u64) << 32
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
#[target_feature(enable = "avx512vl,vpclmulqdq")]
pub unsafe fn crc32c(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    crc0 = !crc0;
    while len != 0 && (buf as usize & 7) != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    while (buf as usize & 56) != 0 && len >= 8 {
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    if len >= 208 {
        let blk = (len - 8) / 200;
        let klen = blk * 8;
        let mut buf2 = buf;
        let mut vc;
        let mut z0;
        let mut x0 = unsafe { _mm512_loadu_si512(buf2.cast()) };
        let mut x1 = unsafe { _mm512_loadu_si512(buf2.add(64).cast()) };
        let mut x2 = unsafe { _mm512_loadu_si512(buf2.add(128).cast()) };
        let mut y0;
        let mut y1;
        let mut y2;
        let mut k;

        k = _mm512_broadcast_i32x4(_mm_setr_epi32(
            0xa87a_b8a8_u32.cast_signed(),
            0,
            0xab7a_ff2a_u32.cast_signed(),
            0,
        ));
        x0 = _mm512_xor_si512(
            _mm512_zextsi128_si512(_mm_cvtsi32_si128(crc0.cast_signed())),
            x0,
        );
        crc0 = 0;
        buf2 = unsafe { buf2.add(192) };
        len -= 200;
        buf = unsafe { buf.add(blk * 192) };
        while len >= 208 {
            y0 = clmul_lo(x0, k);
            x0 = clmul_hi(x0, k);
            y1 = clmul_lo(x1, k);
            x1 = clmul_hi(x1, k);
            y2 = clmul_lo(x2, k);
            x2 = clmul_hi(x2, k);
            x0 = _mm512_ternarylogic_epi64::<0x96>(x0, y0, unsafe {
                _mm512_loadu_si512(buf2.cast())
            });
            x1 = _mm512_ternarylogic_epi64::<0x96>(x1, y1, unsafe {
                _mm512_loadu_si512(buf2.add(64).cast())
            });
            x2 = _mm512_ternarylogic_epi64::<0x96>(x2, y2, unsafe {
                _mm512_loadu_si512(buf2.add(128).cast())
            });
            crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
            buf = unsafe { buf.add(8) };
            buf2 = unsafe { buf2.add(192) };
            len -= 200;
        }
        k = _mm512_broadcast_i32x4(_mm_setr_epi32(
            0x740e_ef02_u32.cast_signed(),
            0,
            0x9e4a_ddf8_u32.cast_signed(),
            0,
        ));
        y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = _mm512_ternarylogic_epi64::<0x96>(x0, y0, x1);
        x1 = x2;
        y0 = clmul_lo(x0, k);
        x0 = clmul_hi(x0, k);
        x0 = _mm512_ternarylogic_epi64::<0x96>(x0, y0, x1);
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        vc = 0;
        k = _mm512_setr_epi32(
            0x1c29_1d04_u32.cast_signed(),
            0,
            0xddc0_152b_u32.cast_signed(),
            0,
            0x3da6_d0cb_u32.cast_signed(),
            0,
            0xba4f_c28e_u32.cast_signed(),
            0,
            0xf20c_0dfe_u32.cast_signed(),
            0,
            0x493c_7d27_u32.cast_signed(),
            0,
            0,
            0,
            0,
            0,
        );
        y0 = clmul_lo(x0, k);
        k = clmul_hi(x0, k);
        y0 = _mm512_xor_si512(y0, k);
        z0 = _mm_ternarylogic_epi64::<0x96>(
            _mm512_castsi512_si128(y0),
            _mm512_extracti32x4_epi32::<1>(y0),
            _mm512_extracti32x4_epi32::<2>(y0),
        );
        z0 = _mm_xor_si128(z0, _mm512_extracti32x4_epi32::<3>(x0));
        vc ^= extract_u64(
            crc_shift(
                crc32_u64(crc32_u64(0, extract_u64(z0, 0)), extract_u64(z0, 1)),
                klen + 8,
            ),
            0,
        );
        crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() } ^ vc);
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    if len >= 32 {
        let klen = ((len - 8) / 24) * 8;
        let mut crc1 = 0u32;
        let mut crc2 = 0u32;
        loop {
            crc0 = crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                buf.add(klen).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                buf.add(klen * 2).cast::<u64>().read_unaligned()
            });
            buf = unsafe { buf.add(8) };
            len -= 24;
            if len < 32 {
                break;
            }
        }
        let vc0 = crc_shift(crc0, klen * 2 + 8);
        let vc1 = crc_shift(crc1, klen + 8);
        let vc = extract_u64(_mm_xor_si128(vc0, vc1), 0);
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
