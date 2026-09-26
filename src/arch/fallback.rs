// Based on:
// * https://github.com/MuntasirSZN/crc-fast-rust/tree/e3f3c613c3e158b2b82d576347ffc6e5e07ac5ba
// * https://create.stephan-brumme.com/crc32/#slicing-by-16-overview

use core::hint::{Locality, prefetch_read};

use super::table::CRC32C_TABLE;

#[inline]
pub fn crc32c(crc0: u32, buf: &[u8], len: usize) -> u32 {
    const UNROLL: usize = 4;
    const BYTES_AT_ONCE: usize = 16 * UNROLL;
    const PREFETCH_AHEAD: usize = 256;

    let bytes = &buf[..len];
    let mut crc = !crc0;
    #[allow(clippy::cast_ptr_alignment)]
    let mut current = bytes.as_ptr().cast::<u32>();
    let mut length = bytes.len();

    while length >= BYTES_AT_ONCE + PREFETCH_AHEAD {
        // SAFETY: length >= 320, so the prefetch address 256 bytes
        // ahead remains within the input buffer.
        prefetch_read(
            unsafe { current.cast::<u8>().add(PREFETCH_AHEAD) },
            Locality::L1,
        );

        for _ in 0..UNROLL {
            crc = unsafe { block(&mut current, crc) };
        }

        length -= BYTES_AT_ONCE;
    }

    while length >= 16 {
        crc = unsafe { block(&mut current, crc) };
        length -= 16;
    }

    let mut current_char = current.cast::<u8>();

    while length != 0 {
        if length >= 8 {
            // SAFETY: `length` in `8..16`, so `current_char` is valid for `length` bytes.
            return !unsafe { tail(crc, current_char, length) };
        }

        crc =
            (crc >> 8) ^ CRC32C_TABLE[0][((crc & 0xFF) as u8 ^ unsafe { *current_char }) as usize];

        current_char = unsafe { current_char.add(1) };
        length -= 1;
    }

    !crc
}

/// # Safety
///
/// `cur` must be valid for reads of 16 bytes, and `cur.add(4)`
/// must stay within the same allocation or one past its end.
#[inline(always)]
unsafe fn block(cur: &mut *const u32, crc: u32) -> u32 {
    // SAFETY: caller guarantees 16 readable bytes at `cur`.
    let [mut a, b, c, d] = unsafe { cur.cast::<[u32; 4]>().read_unaligned() };
    a ^= crc;
    // SAFETY: 16 bytes were readable, so `cur + 4` stays in bounds.
    *cur = unsafe { cur.add(4) };

    let mut crc = 0;
    for (i, w) in [a, b, c, d].iter().enumerate() {
        for (j, byte) in w.to_be_bytes().iter().enumerate() {
            crc ^= CRC32C_TABLE[15 - i * 4 - j][*byte as usize];
        }
    }
    crc
}

/// # Safety
///
/// `ptr` must be valid for reads of `len` bytes.
unsafe fn tail(crc: u32, ptr: *const u8, len: usize) -> u32 {
    debug_assert!((8..16).contains(&len));

    // SAFETY: `len >= 8`.
    let lo = u64::from_le(unsafe { ptr.cast::<u64>().read_unaligned() }) ^ u64::from(crc);
    // SAFETY: `len - 8 + 8 = len`, so the last 8 bytes are in bounds.
    let hi = u64::from_le(unsafe { ptr.add(len - 8).cast::<u64>().read_unaligned() });

    let [l0, l1, l2, l3, l4, l5, l6, l7] = lo.to_le_bytes();

    let mut crc = CRC32C_TABLE[len - 1][l0 as usize]
        ^ CRC32C_TABLE[len - 2][l1 as usize]
        ^ CRC32C_TABLE[len - 3][l2 as usize]
        ^ CRC32C_TABLE[len - 4][l3 as usize]
        ^ CRC32C_TABLE[len - 5][l4 as usize]
        ^ CRC32C_TABLE[len - 6][l5 as usize]
        ^ CRC32C_TABLE[len - 7][l6 as usize]
        ^ CRC32C_TABLE[len - 8][l7 as usize];

    let skip = 16 - len; // 1..8
    let h = hi >> (skip * 8);
    for k in 0..(len - 8) {
        let b = (h >> (k * 8)) as u8;
        crc ^= CRC32C_TABLE[len - 9 - k][b as usize];
    }

    crc
}
