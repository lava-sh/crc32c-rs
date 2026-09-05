use super::table::CRC32C_TABLE;

#[inline(always)]
const fn crc_u8(crc: u32, val: u8) -> u32 {
    (crc >> 8) ^ CRC32C_TABLE[0][((crc & 0xFF) as u8 ^ val) as usize]
}

#[inline(always)]
const fn crc_u32(mut crc: u32, val: u32) -> u32 {
    crc ^= val;
    CRC32C_TABLE[0][(crc >> 24) as usize]
        ^ CRC32C_TABLE[1][((crc >> 16) & 0xFF) as usize]
        ^ CRC32C_TABLE[3][(crc & 0xFF) as usize]
        ^ CRC32C_TABLE[2][((crc >> 8) & 0xFF) as usize]
}

#[inline]
pub fn crc32c(mut crc0: u32, buf: &[u8], mut len: usize) -> u32 {
    crc0 = !crc0;
    let mut offset = 0;

    while len > 0 && ((buf.as_ptr() as usize + offset) & 3) != 0 {
        crc0 = crc_u8(crc0, buf[offset]);
        offset += 1;
        len -= 1;
    }

    while len >= 4 {
        let val = unsafe {
            let ptr = buf.as_ptr().add(offset);
            ptr.cast::<u32>().read_unaligned()
        };
        crc0 = crc_u32(crc0, val);
        offset += 4;
        len -= 4;
    }

    while len > 0 {
        crc0 = crc_u8(crc0, buf[offset]);
        offset += 1;
        len -= 1;
    }

    !crc0
}
