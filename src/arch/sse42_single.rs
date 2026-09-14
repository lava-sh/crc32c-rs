#![allow(clippy::wildcard_imports)]
#![allow(clippy::missing_const_for_fn)]
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

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
pub unsafe fn crc32c(crc: u32, mut buf: *const u8, len: usize) -> u32 {
    let mut crc: u32 = crc ^ 0xffff_ffff;
    let end8 = buf.add(len & !7);
    while buf < end8 {
        crc = crc32_u64(crc, read_u64(buf));
        buf = buf.add(8);
    }
    let end = buf.add(len & 7);
    while buf < end {
        crc = _mm_crc32_u8(crc, *buf);
        buf = buf.add(1);
    }
    !crc
}
