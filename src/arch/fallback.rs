// Based on https://github.com/MuntasirSZN/crc-fast-rust/tree/e3f3c613c3e158b2b82d576347ffc6e5e07ac5ba
use super::table::CRC32C_TABLE;

#[inline]
pub fn crc32c(crc0: u32, buf: &[u8], len: usize) -> u32 {
    let bytes = &buf[..len];
    let mut crc = !crc0;
    let mut ptr = bytes.as_ptr();
    let mut remaining = bytes.len();

    while remaining >= 16 {
        // SAFETY: `remaining >= 16`, so offsets 0..15 are within `bytes`;
        // `ptr` advances only after all 16 bytes have been consumed.
        crc = CRC32C_TABLE[0][unsafe { *ptr.add(15) } as usize]
            ^ CRC32C_TABLE[1][unsafe { *ptr.add(14) } as usize]
            ^ CRC32C_TABLE[2][unsafe { *ptr.add(13) } as usize]
            ^ CRC32C_TABLE[3][unsafe { *ptr.add(12) } as usize]
            ^ CRC32C_TABLE[4][unsafe { *ptr.add(11) } as usize]
            ^ CRC32C_TABLE[5][unsafe { *ptr.add(10) } as usize]
            ^ CRC32C_TABLE[6][unsafe { *ptr.add(9) } as usize]
            ^ CRC32C_TABLE[7][unsafe { *ptr.add(8) } as usize]
            ^ CRC32C_TABLE[8][unsafe { *ptr.add(7) } as usize]
            ^ CRC32C_TABLE[9][unsafe { *ptr.add(6) } as usize]
            ^ CRC32C_TABLE[10][unsafe { *ptr.add(5) } as usize]
            ^ CRC32C_TABLE[11][unsafe { *ptr.add(4) } as usize]
            ^ CRC32C_TABLE[12][(unsafe { *ptr.add(3) } ^ (crc >> 24) as u8) as usize]
            ^ CRC32C_TABLE[13][(unsafe { *ptr.add(2) } ^ (crc >> 16) as u8) as usize]
            ^ CRC32C_TABLE[14][(unsafe { *ptr.add(1) } ^ (crc >> 8) as u8) as usize]
            ^ CRC32C_TABLE[15][(unsafe { *ptr } ^ crc as u8) as usize];
        ptr = unsafe { ptr.add(16) };
        remaining -= 16;
    }

    while remaining != 0 {
        // SAFETY: `remaining != 0` and `ptr` points to the next unprocessed
        // byte of `bytes`.
        let value = unsafe { *ptr };
        crc = CRC32C_TABLE[0][((crc as u8) ^ value) as usize] ^ (crc >> 8);
        ptr = unsafe { ptr.add(1) };
        remaining -= 1;
    }

    !crc
}
