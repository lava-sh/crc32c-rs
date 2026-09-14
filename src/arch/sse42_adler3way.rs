#![allow(clippy::wildcard_imports)]
#![allow(clippy::missing_const_for_fn)]
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const POLY: u64 = 0x82f6_3b78;

fn gf2_matrix_times(mat: &[u32; 32], vec: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut v = vec;
    for &m in mat {
        if v & 1 != 0 {
            sum ^= u64::from(m);
        }
        v >>= 1;
    }
    sum
}

fn gf2_matrix_square(square: &mut [u32; 32], mat: &[u32; 32]) {
    for n in 0..32 {
        square[n] = gf2_matrix_times(mat, u64::from(mat[n])) as u32;
    }
}

fn crc32c_zeros_op(even: &mut [u32; 32], mut len: u64) {
    let mut odd = [0_u32; 32];
    odd[0] = POLY as u32;
    let mut row: u32 = 1;
    for slot in &mut odd[1..] {
        *slot = row;
        row <<= 1;
    }
    gf2_matrix_square(even, &odd);
    gf2_matrix_square(&mut odd, even);
    loop {
        gf2_matrix_square(even, &odd);
        len >>= 1;
        if len == 0 {
            return;
        }
        gf2_matrix_square(&mut odd, even);
        len >>= 1;
        if len == 0 {
            *even = odd;
            return;
        }
    }
}

fn crc32c_zeros(zeros: &mut [[u32; 256]; 4], len: u64) {
    let mut op = [0_u32; 32];
    crc32c_zeros_op(&mut op, len);
    for n in 0u32..256 {
        zeros[0][n as usize] = gf2_matrix_times(&op, u64::from(n)) as u32;
        zeros[1][n as usize] = gf2_matrix_times(&op, u64::from(n) << 8) as u32;
        zeros[2][n as usize] = gf2_matrix_times(&op, u64::from(n) << 16) as u32;
        zeros[3][n as usize] = gf2_matrix_times(&op, u64::from(n) << 24) as u32;
    }
}

#[inline]
fn crc32c_shift(zeros: &[[u32; 256]; 4], crc: u32) -> u32 {
    zeros[0][(crc & 0xff) as usize]
        ^ zeros[1][((crc >> 8) & 0xff) as usize]
        ^ zeros[2][((crc >> 16) & 0xff) as usize]
        ^ zeros[3][((crc >> 24) & 0xff) as usize]
}

const LONG: usize = 8192;
const SHORT: usize = 256;

use std::sync::OnceLock;

fn long_shift() -> &'static [[u32; 256]; 4] {
    static T: OnceLock<[[u32; 256]; 4]> = OnceLock::new();
    T.get_or_init(|| {
        let mut t = [[0_u32; 256]; 4];
        crc32c_zeros(&mut t, LONG as u64);
        t
    })
}

fn short_shift() -> &'static [[u32; 256]; 4] {
    static T: OnceLock<[[u32; 256]; 4]> = OnceLock::new();
    T.get_or_init(|| {
        let mut t = [[0_u32; 256]; 4];
        crc32c_zeros(&mut t, SHORT as u64);
        t
    })
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
unsafe fn read_u64(p: *const u8) -> u64 {
    p.cast::<u64>().read_unaligned()
}

#[target_feature(enable = "sse4.2")]
pub unsafe fn crc32c(crc: u32, mut buf: *const u8, mut len: usize) -> u32 {
    let mut crc0: u32 = crc ^ 0xffff_ffff;
    while len != 0 && (buf as usize & 7) != 0 {
        crc0 = _mm_crc32_u8(crc0, *buf);
        buf = buf.add(1);
        len -= 1;
    }
    while len >= LONG * 3 {
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = buf.add(LONG);
        loop {
            crc0 = crc32_u64(crc0, read_u64(buf));
            crc1 = crc32_u64(crc1, read_u64(buf.add(LONG)));
            crc2 = crc32_u64(crc2, read_u64(buf.add(LONG * 2)));
            buf = buf.add(8);
            if buf >= end {
                break;
            }
        }
        crc0 = crc32c_shift(long_shift(), crc0) ^ crc1;
        crc0 = crc32c_shift(long_shift(), crc0) ^ crc2;
        buf = buf.add(LONG * 2);
        len -= LONG * 3;
    }
    while len >= SHORT * 3 {
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = buf.add(SHORT);
        loop {
            crc0 = crc32_u64(crc0, read_u64(buf));
            crc1 = crc32_u64(crc1, read_u64(buf.add(SHORT)));
            crc2 = crc32_u64(crc2, read_u64(buf.add(SHORT * 2)));
            buf = buf.add(8);
            if buf >= end {
                break;
            }
        }
        crc0 = crc32c_shift(short_shift(), crc0) ^ crc1;
        crc0 = crc32c_shift(short_shift(), crc0) ^ crc2;
        buf = buf.add(SHORT * 2);
        len -= SHORT * 3;
    }
    let end8 = buf.add(len & !7);
    while buf < end8 {
        crc0 = crc32_u64(crc0, read_u64(buf));
        buf = buf.add(8);
    }
    len &= 7;
    while len != 0 {
        crc0 = _mm_crc32_u8(crc0, *buf);
        buf = buf.add(1);
        len -= 1;
    }
    !crc0
}
