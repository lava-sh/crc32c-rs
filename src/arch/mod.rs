#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx512_vpclmulqdq;
pub mod fallback;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod see42_pclmulqdq;
pub mod table;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx512_pclmulqdq;
