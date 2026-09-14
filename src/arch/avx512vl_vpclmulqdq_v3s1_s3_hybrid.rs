use super::{avx512vl_vpclmulqdq_v3s1_s3, avx512vl_vpclmulqdq_v3s2x4, maria_avx512};

// `v3s1_s3` with MariaDB's mid-size branch injected:
//
// - `len < 256`: `v3s1_s3` (its tail is faster than MariaDB's `< 256` branch);
// - `256..=8192`: MariaDB's `size >= 256` branch — 2-way/4-way 128/256-byte
//   block folds plus the `b896`/`b384` vectorized reduction;
// - `8192 < len <= 24 MiB`: `v3s2x4`;
// - `len > 24 MiB`: `v3s1_s3`.
#[inline]
#[target_feature(enable = "avx512bw,avx512dq,avx512vl,vpclmulqdq")]
pub unsafe fn crc32c(crc: u32, buf: *const u8, len: usize) -> u32 {
    const V3S2X4_LIMIT: usize = 24 * 1024 * 1024;

    if (256..=8192).contains(&len) {
        unsafe { maria_avx512::crc32c(crc, buf, len) }
    } else if (8192..=V3S2X4_LIMIT).contains(&len) {
        unsafe { avx512vl_vpclmulqdq_v3s2x4::crc32c(crc, buf, len) }
    } else {
        unsafe { avx512vl_vpclmulqdq_v3s1_s3::crc32c(crc, buf, len) }
    }
}
