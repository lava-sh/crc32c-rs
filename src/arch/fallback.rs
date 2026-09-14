// Based on:
// * https://github.com/MuntasirSZN/crc-fast-rust/tree/e3f3c613c3e158b2b82d576347ffc6e5e07ac5ba
// * https://create.stephan-brumme.com/crc32/#slicing-by-16-overview

use core::hint::{Locality, prefetch_read};

use super::table::CRC32C_TABLE;

#[rustfmt::skip]
macro_rules! block {
    ($current:expr, $crc:expr) => {{
        let a = unsafe { $current.add(0).read_unaligned() } ^ $crc;
        let b = unsafe { $current.add(1).read_unaligned() };
        let c = unsafe { $current.add(2).read_unaligned() };
        let d = unsafe { $current.add(3).read_unaligned() };

        $current = unsafe { $current.add(4) };

        let [a0, a1, a2, a3] = a.to_be_bytes();
        let [b0, b1, b2, b3] = b.to_be_bytes();
        let [c0, c1, c2, c3] = c.to_be_bytes();
        let [d0, d1, d2, d3] = d.to_be_bytes();

        $crc = CRC32C_TABLE[ 0][d0 as usize]
            ^  CRC32C_TABLE[ 1][d1 as usize]
            ^  CRC32C_TABLE[ 2][d2 as usize]
            ^  CRC32C_TABLE[ 3][d3 as usize]
            ^  CRC32C_TABLE[ 4][c0 as usize]
            ^  CRC32C_TABLE[ 5][c1 as usize]
            ^  CRC32C_TABLE[ 6][c2 as usize]
            ^  CRC32C_TABLE[ 7][c3 as usize]
            ^  CRC32C_TABLE[ 8][b0 as usize]
            ^  CRC32C_TABLE[ 9][b1 as usize]
            ^  CRC32C_TABLE[10][b2 as usize]
            ^  CRC32C_TABLE[11][b3 as usize]
            ^  CRC32C_TABLE[12][a0 as usize]
            ^  CRC32C_TABLE[13][a1 as usize]
            ^  CRC32C_TABLE[14][a2 as usize]
            ^  CRC32C_TABLE[15][a3 as usize];
    }};
}

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
            block!(current, crc);
        }

        length -= BYTES_AT_ONCE;
    }

    while length >= 16 {
        block!(current, crc);

        length -= 16;
    }

    let mut current_char = current.cast::<u8>();

    while length != 0 {
        crc =
            (crc >> 8) ^ CRC32C_TABLE[0][((crc & 0xFF) as u8 ^ unsafe { *current_char }) as usize];

        current_char = unsafe { current_char.add(1) };
        length -= 1;
    }

    !crc
}
