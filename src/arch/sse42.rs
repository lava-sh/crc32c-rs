#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const POLY: u64 = 0x82f6_3b78;

const fn gf2_matrix_times(mat: &[u32; 32], mut vec: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut n = 0;
    while n < 32 {
        if vec & 1 != 0 {
            sum ^= mat[n];
        }
        vec >>= 1;
        n += 1;
    }
    sum
}

const fn gf2_matrix_square(mat: &[u32; 32]) -> [u32; 32] {
    let mut square = [0_u32; 32];
    let mut n = 0;
    while n < 32 {
        square[n] = gf2_matrix_times(mat, mat[n]);
        n += 1;
    }
    square
}

const fn crc32c_zeros_op(mut len: u64) -> [u32; 32] {
    let mut odd = [0_u32; 32];
    odd[0] = POLY as u32;
    let mut row: u32 = 1;
    let mut n = 1;
    while n < 32 {
        odd[n] = row;
        row <<= 1;
        n += 1;
    }
    let mut even = gf2_matrix_square(&odd);
    odd = gf2_matrix_square(&even);
    loop {
        even = gf2_matrix_square(&odd);
        len >>= 1;
        if len == 0 {
            return even;
        }
        odd = gf2_matrix_square(&even);
        len >>= 1;
        if len == 0 {
            even = odd;
            break;
        }
    }
    even
}

const fn crc32c_zeros(len: u64) -> [[u32; 256]; 4] {
    let op = crc32c_zeros_op(len);
    let mut zeros = [[0_u32; 256]; 4];
    let mut n = 0_u32;
    while n < 256 {
        zeros[0][n as usize] = gf2_matrix_times(&op, n);
        zeros[1][n as usize] = gf2_matrix_times(&op, n << 8);
        zeros[2][n as usize] = gf2_matrix_times(&op, n << 16);
        zeros[3][n as usize] = gf2_matrix_times(&op, n << 24);
        n += 1;
    }
    zeros
}

const LONG: usize = 8192;
const SHORT: usize = 256;
const SHORT2: usize = 128;
const SHORT3: usize = 64;

static CRC32C_LONG: [[u32; 256]; 4] = crc32c_zeros(LONG as u64);
static CRC32C_SHORT: [[u32; 256]; 4] = crc32c_zeros(SHORT as u64);
static CRC32C_SHORT2: [[u32; 256]; 4] = crc32c_zeros(SHORT2 as u64);
static CRC32C_SHORT3: [[u32; 256]; 4] = crc32c_zeros(SHORT3 as u64);

#[inline]
const fn crc32c_shift(zeros: &[[u32; 256]; 4], crc: u32) -> u32 {
    zeros[0][(crc & 0xff) as usize]
        ^ zeros[1][((crc >> 8) & 0xff) as usize]
        ^ zeros[2][((crc >> 16) & 0xff) as usize]
        ^ zeros[3][(crc >> 24) as usize]
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

macro_rules! round3 {
    ($next:ident, $len:ident, $crc0:ident, $chunk:expr, $zeros:expr) => {{
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = unsafe { $next.add($chunk) };
        loop {
            $crc0 = crc32_u64($crc0, unsafe { $next.cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                $next.add($chunk).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                $next.add($chunk * 2).cast::<u64>().read_unaligned()
            });
            $next = unsafe { $next.add(8) };
            if $next >= end {
                break;
            }
        }
        $crc0 = crc32c_shift($zeros, $crc0) ^ crc1;
        $crc0 = crc32c_shift($zeros, $crc0) ^ crc2;
        $next = unsafe { $next.add($chunk * 2) };
        $len -= $chunk * 3;
    }};
}

#[target_feature(enable = "sse4.2")]
pub unsafe fn crc32c(crc: u32, buf: *const u8, mut len: usize) -> u32 {
    let mut crc0: u32 = !crc;
    let mut next = buf;
    while len != 0 && (next as usize & 7) != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *next });
        next = unsafe { next.add(1) };
        len -= 1;
    }
    while len >= LONG * 3 {
        round3!(next, len, crc0, LONG, &CRC32C_LONG);
    }
    while len >= SHORT * 3 {
        round3!(next, len, crc0, SHORT, &CRC32C_SHORT);
    }
    while len >= SHORT2 * 3 {
        round3!(next, len, crc0, SHORT2, &CRC32C_SHORT2);
    }
    while len >= SHORT3 * 3 {
        round3!(next, len, crc0, SHORT3, &CRC32C_SHORT3);
    }
    {
        let end = unsafe { next.add(len - (len & 7)) };
        while next < end {
            crc0 = crc32_u64(crc0, unsafe { next.cast::<u64>().read_unaligned() });
            next = unsafe { next.add(8) };
        }
        len &= 7;
    }
    while len != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *next });
        next = unsafe { next.add(1) };
        len -= 1;
    }
    !crc0
}
