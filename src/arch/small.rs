#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

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

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn clmul_scalar(a: u32, b: u32) -> __m128i {
    _mm_clmulepi64_si128::<0>(
        _mm_cvtsi32_si128(a.cast_signed()),
        _mm_cvtsi32_si128(b.cast_signed()),
    )
}

// x^n mod P, in log(n) time
#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
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
pub unsafe fn crc32_small(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    if len >= 32 {
        let klen = ((len - 8) / 24) * 8;
        let mut crc1 = 0_u32;
        let mut crc2 = 0_u32;
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
    crc0
}
