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
        // SAFETY: `length >= BYTES_AT_ONCE + PREFETCH_AHEAD` (>= 320), so
        // `current + PREFETCH_AHEAD` (256 bytes ahead) is still inside `bytes`.
        prefetch_read(
            unsafe { current.cast::<u8>().add(PREFETCH_AHEAD) },
            Locality::L1,
        );

        for _ in 0..UNROLL {
            // SAFETY: each `UNROLL` iteration consumes 16 bytes; the `while`
            // guard guarantees `BYTES_AT_ONCE` bytes remain, so 4 such
            // iterations worth of `read_unaligned`/`add(1)` stay in-bounds.
            let one = unsafe { current.read_unaligned() } ^ crc;
            current = unsafe { current.add(1) };

            let two = unsafe { current.read_unaligned() };
            current = unsafe { current.add(1) };

            let three = unsafe { current.read_unaligned() };
            current = unsafe { current.add(1) };

            let four = unsafe { current.read_unaligned() };
            current = unsafe { current.add(1) };

            crc = CRC32C_TABLE[0][((four >> 24) & 0xFF) as usize]
                ^ CRC32C_TABLE[1][((four >> 16) & 0xFF) as usize]
                ^ CRC32C_TABLE[2][((four >> 8) & 0xFF) as usize]
                ^ CRC32C_TABLE[3][(four & 0xFF) as usize]
                ^ CRC32C_TABLE[4][((three >> 24) & 0xFF) as usize]
                ^ CRC32C_TABLE[5][((three >> 16) & 0xFF) as usize]
                ^ CRC32C_TABLE[6][((three >> 8) & 0xFF) as usize]
                ^ CRC32C_TABLE[7][(three & 0xFF) as usize]
                ^ CRC32C_TABLE[8][((two >> 24) & 0xFF) as usize]
                ^ CRC32C_TABLE[9][((two >> 16) & 0xFF) as usize]
                ^ CRC32C_TABLE[10][((two >> 8) & 0xFF) as usize]
                ^ CRC32C_TABLE[11][(two & 0xFF) as usize]
                ^ CRC32C_TABLE[12][((one >> 24) & 0xFF) as usize]
                ^ CRC32C_TABLE[13][((one >> 16) & 0xFF) as usize]
                ^ CRC32C_TABLE[14][((one >> 8) & 0xFF) as usize]
                ^ CRC32C_TABLE[15][(one & 0xFF) as usize];
        }

        length -= BYTES_AT_ONCE;
    }

    while length >= 16 {
        // SAFETY: `length >= 16` guarantees exactly 4 u32 reads (16 bytes)
        let one = unsafe { current.read_unaligned() } ^ crc;
        current = unsafe { current.add(1) };

        let two = unsafe { current.read_unaligned() };
        current = unsafe { current.add(1) };

        let three = unsafe { current.read_unaligned() };
        current = unsafe { current.add(1) };

        let four = unsafe { current.read_unaligned() };
        current = unsafe { current.add(1) };

        crc = CRC32C_TABLE[0][((four >> 24) & 0xFF) as usize]
            ^ CRC32C_TABLE[1][((four >> 16) & 0xFF) as usize]
            ^ CRC32C_TABLE[2][((four >> 8) & 0xFF) as usize]
            ^ CRC32C_TABLE[3][(four & 0xFF) as usize]
            ^ CRC32C_TABLE[4][((three >> 24) & 0xFF) as usize]
            ^ CRC32C_TABLE[5][((three >> 16) & 0xFF) as usize]
            ^ CRC32C_TABLE[6][((three >> 8) & 0xFF) as usize]
            ^ CRC32C_TABLE[7][(three & 0xFF) as usize]
            ^ CRC32C_TABLE[8][((two >> 24) & 0xFF) as usize]
            ^ CRC32C_TABLE[9][((two >> 16) & 0xFF) as usize]
            ^ CRC32C_TABLE[10][((two >> 8) & 0xFF) as usize]
            ^ CRC32C_TABLE[11][(two & 0xFF) as usize]
            ^ CRC32C_TABLE[12][((one >> 24) & 0xFF) as usize]
            ^ CRC32C_TABLE[13][((one >> 16) & 0xFF) as usize]
            ^ CRC32C_TABLE[14][((one >> 8) & 0xFF) as usize]
            ^ CRC32C_TABLE[15][(one & 0xFF) as usize];

        length -= 16;
    }

    let mut current_char = current.cast::<u8>();

    while length != 0 {
        // SAFETY: `current_char` was advanced by exactly the number of bytes
        // consumed above, so it points to the first unread byte of `bytes`;
        // `length != 0` guarantees this byte is still within bounds.
        let value = unsafe { *current_char };

        crc = (crc >> 8) ^ CRC32C_TABLE[0][((crc & 0xFF) as u8 ^ value) as usize];

        current_char = unsafe { current_char.add(1) };
        length -= 1;
    }

    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32c_16_bytes_matches() {
        let data = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        let expected = crc32c(0, &data, data.len());

        let mut actual = 0;
        for byte in data {
            actual = crc32c(actual, &[byte], 1);
        }

        assert_eq!(expected, actual);
    }
}
