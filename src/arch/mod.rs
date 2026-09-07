#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
pub mod aes_sha3_v9s3x2e_s3;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
pub mod aes_v3s4x2e_v2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx512vl_pclmulqdq_v9s3x4e;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx512vl_vpclmulqdq_v3s1_s3;
pub mod fallback;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod see42_pclmulqdq_v7s3x3;
pub mod table;
