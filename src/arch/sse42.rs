#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

// CRC-32C (iSCSI) polynomial in reversed bit order.
const POLY: u64 = 0x82f6_3b78;

// Multiply a matrix times a vector over the Galois field of two elements,
// GF(2). Each element is a bit in an unsigned integer. mat must have at
// least as many entries as the power of two for most significant one bit in
// vec.
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

// Multiply a matrix by itself over GF(2). Both mat and square must have 32
// rows.
const fn gf2_matrix_square(mat: &[u32; 32]) -> [u32; 32] {
    let mut square = [0_u32; 32];
    let mut n = 0;
    while n < 32 {
        square[n] = gf2_matrix_times(mat, mat[n]);
        n += 1;
    }
    square
}

// Construct an operator to apply len zeros to a crc. len must be a power of
// two. If len is not a power of two, then the result is the same as for the
// largest power of two less than len. The result for len == 0 is the same as
// for len == 1. A version of this routine could be easily written for any
// len, but that is not needed for this application.
const fn crc32c_zeros_op(mut len: u64) -> [u32; 32] {
    let mut odd = [0_u32; 32]; // odd-power-of-two zeros operator
    // put operator for one zero bit in odd
    odd[0] = POLY as u32; // CRC-32C polynomial
    let mut row: u32 = 1;
    let mut n = 1;
    while n < 32 {
        odd[n] = row;
        row <<= 1;
        n += 1;
    }
    // put operator for two zero bits in even
    let mut even = gf2_matrix_square(&odd);
    // put operator for four zero bits in odd
    odd = gf2_matrix_square(&even);
    // first square will put the operator for one zero byte (eight zero bits),
    // in even -- next square puts operator for two zero bytes in odd, and so
    // on, until len has been rotated down to zero
    loop {
        even = gf2_matrix_square(&odd);
        len >>= 1;
        if len == 0 {
            return even;
        }
        odd = gf2_matrix_square(&even);
        len >>= 1;
        if len == 0 {
            // answer ended up in odd -- copy to even
            even = odd;
            break;
        }
    }
    even
}

// Take a length and build four lookup tables for applying the zeros operator
// for that length, byte-by-byte on the operand.
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

// Block sizes for three-way parallel crc computation. LONG and SHORT must
// both be powers of two.
const LONG: usize = 8192;
const SHORT: usize = 256;

// Tables for hardware crc that shift a crc by LONG and SHORT zeros.
static CRC32C_LONG: [[u32; 256]; 4] = crc32c_zeros(LONG as u64);
static CRC32C_SHORT: [[u32; 256]; 4] = crc32c_zeros(SHORT as u64);

// Apply the zeros operator table to crc.
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

// Compute CRC-32C using the Intel hardware instruction.
#[target_feature(enable = "sse4.2")]
pub unsafe fn crc32c(crc: u32, buf: *const u8, mut len: usize) -> u32 {
    // pre-process the crc
    let mut crc0: u32 = !crc;
    let mut next = buf;
    // compute the crc for up to seven leading bytes to bring the data pointer
    // to an eight-byte boundary
    while len != 0 && (next as usize & 7) != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *next });
        next = unsafe { next.add(1) };
        len -= 1;
    }
    // compute the crc on sets of LONG*3 bytes, executing three independent crc
    // instructions, each on LONG bytes -- this is optimized for the Nehalem,
    // Westmere, Sandy Bridge, and Ivy Bridge architectures, which have a
    // throughput of one crc per cycle, but a latency of three cycles
    while len >= LONG * 3 {
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = unsafe { next.add(LONG) };
        loop {
            crc0 = crc32_u64(crc0, unsafe { next.cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                next.add(LONG).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                next.add(LONG * 2).cast::<u64>().read_unaligned()
            });
            next = unsafe { next.add(8) };
            if next >= end {
                break;
            }
        }
        crc0 = crc32c_shift(&CRC32C_LONG, crc0) ^ crc1;
        crc0 = crc32c_shift(&CRC32C_LONG, crc0) ^ crc2;
        next = unsafe { next.add(LONG * 2) };
        len -= LONG * 3;
    }
    // do the same thing, but now on SHORT*3 blocks for the remaining data less
    // than a LONG*3 block
    while len >= SHORT * 3 {
        let mut crc1: u32 = 0;
        let mut crc2: u32 = 0;
        let end = unsafe { next.add(SHORT) };
        loop {
            crc0 = crc32_u64(crc0, unsafe { next.cast::<u64>().read_unaligned() });
            crc1 = crc32_u64(crc1, unsafe {
                next.add(SHORT).cast::<u64>().read_unaligned()
            });
            crc2 = crc32_u64(crc2, unsafe {
                next.add(SHORT * 2).cast::<u64>().read_unaligned()
            });
            next = unsafe { next.add(8) };
            if next >= end {
                break;
            }
        }
        crc0 = crc32c_shift(&CRC32C_SHORT, crc0) ^ crc1;
        crc0 = crc32c_shift(&CRC32C_SHORT, crc0) ^ crc2;
        next = unsafe { next.add(SHORT * 2) };
        len -= SHORT * 3;
    }
    // compute the crc on the remaining eight-byte units less than a SHORT*3
    // block
    {
        let end = unsafe { next.add(len - (len & 7)) };
        while next < end {
            crc0 = crc32_u64(crc0, unsafe { next.cast::<u64>().read_unaligned() });
            next = unsafe { next.add(8) };
        }
        len &= 7;
    }
    // compute the crc for up to seven trailing bytes
    while len != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *next });
        next = unsafe { next.add(1) };
        len -= 1;
    }
    // return a post-processed crc
    !crc0
}
