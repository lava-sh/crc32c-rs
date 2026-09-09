// Based on:
// * https://github.com/MuntasirSZN/crc-fast-rust/tree/e3f3c613c3e158b2b82d576347ffc6e5e07ac5ba
// * https://create.stephan-brumme.com/crc32/#slicing-by-16-overview
use super::table::CRC32C_TABLE;

#[inline(always)]
const fn prefetch(ptr: *const u8) {
    // _MM_HINT_T0 = 3
    core::intrinsics::prefetch_read_instruction::<_, 3>(ptr);
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
        // SAFETY: length >= 320, so PREFETCH_AHEAD bytes are
        // within the original buffer.

        prefetch(unsafe { current.cast::<u8>().add(PREFETCH_AHEAD) });

        for _ in 0..UNROLL {
            // SAFETY: length >= BYTES_AT_ONCE, therefore all four
            // 4-byte reads are within the remaining buffer.
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
        // SAFETY: length >= 16, so all four reads are within bytes.
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
        // SAFETY: length != 0 and current_char points to the next
        // unprocessed byte within bytes.
        let value = unsafe { *current_char };

        crc = (crc >> 8) ^ CRC32C_TABLE[0][((crc & 0xFF) as u8 ^ value) as usize];

        current_char = unsafe { current_char.add(1) };
        length -= 1;
    }

    !crc
}
