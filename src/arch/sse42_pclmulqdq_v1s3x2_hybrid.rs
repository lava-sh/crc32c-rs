use super::{maria_sse42, sse42_pclmulqdq_v1s3x2};

// `v1s3x2` with MariaDB's 3-way window injected: below 128 bytes every SSE
// path is call-bound, while `crc32c_3way` wins from 128 B up to 12 KiB.
#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
pub unsafe fn crc32c(crc: u32, buf: *const u8, len: usize) -> u32 {
    if (128..12288).contains(&len) {
        unsafe { maria_sse42::crc32c(crc, buf, len) }
    } else {
        unsafe { sse42_pclmulqdq_v1s3x2::crc32c(crc, buf, len) }
    }
}
