#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::table::{LONG_TABLE, SHORT_TABLE};

const LONG: usize = 8192;
const SHORT: usize = 256;

#[inline]
const fn crc32c_shift(zeros: &[[u32; 256]; 4], crc: u32) -> u32 {
    let a = (crc & 0xff) as usize;
    let b = ((crc >> 8) & 0xff) as usize;
    let c = ((crc >> 16) & 0xff) as usize;
    let d = (crc >> 24) as usize;
    zeros[0][a] ^ zeros[1][b] ^ zeros[2][c] ^ zeros[3][d]
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

#[allow(clippy::missing_const_for_fn)]
#[inline]
#[target_feature(enable = "sse4.2")]
unsafe fn read_u64(p: *const u8) -> u64 {
    unsafe { p.cast::<u64>().read_unaligned() }
}

#[target_feature(enable = "sse4.2")]
pub unsafe fn crc32c(crc: u32, mut buf: *const u8, mut len: usize) -> u32 {
    let mut crc0: u32 = crc ^ 0xffff_ffff;
    while len != 0 && (buf as usize & 7) != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    while len >= LONG * 3 {
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = unsafe { buf.add(LONG) };
        loop {
            crc0 = crc32_u64(crc0, unsafe { read_u64(buf) });
            crc1 = crc32_u64(crc1, unsafe { read_u64(buf.add(LONG)) });
            crc2 = crc32_u64(crc2, unsafe { read_u64(buf.add(LONG * 2)) });
            buf = unsafe { buf.add(8) };
            if buf >= end {
                break;
            }
        }
        crc0 = crc32c_shift(&LONG_TABLE, crc0) ^ crc1;
        crc0 = crc32c_shift(&LONG_TABLE, crc0) ^ crc2;
        buf = unsafe { buf.add(LONG * 2) };
        len -= LONG * 3;
    }
    while len >= SHORT * 3 {
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = unsafe { buf.add(SHORT) };
        loop {
            crc0 = crc32_u64(crc0, unsafe { read_u64(buf) });
            crc1 = crc32_u64(crc1, unsafe { read_u64(buf.add(SHORT)) });
            crc2 = crc32_u64(crc2, unsafe { read_u64(buf.add(SHORT * 2)) });
            buf = unsafe { buf.add(8) };
            if buf >= end {
                break;
            }
        }
        crc0 = crc32c_shift(&SHORT_TABLE, crc0) ^ crc1;
        crc0 = crc32c_shift(&SHORT_TABLE, crc0) ^ crc2;
        buf = unsafe { buf.add(SHORT * 2) };
        len -= SHORT * 3;
    }
    let end8 = unsafe { buf.add(len & !7) };
    while buf < end8 {
        crc0 = crc32_u64(crc0, unsafe { read_u64(buf) });
        buf = unsafe { buf.add(8) };
    }
    len &= 7;
    while len != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    !crc0
}
