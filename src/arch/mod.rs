#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx512_pclmulqdq;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx512_vpclmulqdq;
pub mod fallback;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
pub mod neon64;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
pub mod neon64_sha3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod see42_pclmulqdq;
pub mod table;
