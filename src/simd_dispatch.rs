#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

macro_rules! detect_features {
    (x86, [$($feat:tt),+ $(,)?]) => {{
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        { true $(&& is_x86_feature_detected!($feat))+ }
    }};
    (aarch64, [$($feat:tt),+ $(,)?]) => {{
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        { true $(&& is_aarch64_feature_detected!($feat))+ }
    }};
    (arm, [$($feat:tt),+ $(,)?]) => {{
        #[cfg(target_arch = "arm")]
        { true $(&& is_arm_feature_detected!($feat))+ }
    }};
}

// SIMD Instruction Set Architecture
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum SimdIsa {
    Fallback = 0,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42Pclmulqdq = 1,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx512Vpclmulqdq = 2,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx512Pclmulqdq = 3,
    #[cfg(target_arch = "arm")]
    Neon32 = 4,
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    Neon64 = 5,
}

impl SimdIsa {
    #[cold]
    fn detect() -> Self {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            if detect_features!(x86, ["avx512f", "avx512vl", "vpclmulqdq"]) {
                return Self::Avx512Vpclmulqdq;
            }
            if detect_features!(x86, ["avx512vl", "pclmulqdq"]) {
                return Self::Avx512Pclmulqdq;
            }
            if detect_features!(x86, ["sse4.2", "pclmulqdq"]) {
                return Self::Sse42Pclmulqdq;
            }
        }
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        {
            if is_aarch64_feature_detected!("neon") {
                return Self::Neon64;
            }
        }
        #[cfg(target_arch = "arm")]
        {
            if is_arm_feature_detected!("neon") {
                return Self::Neon32;
            }
        }

        Self::Fallback
    }

    #[inline(always)]
    pub fn detected() -> Self {
        static CACHED: AtomicUsize = AtomicUsize::new(usize::MAX);

        let raw = CACHED.load(Ordering::Relaxed);
        let raw = if raw == usize::MAX {
            let detect = Self::detect() as usize;
            CACHED.store(detect, Ordering::Relaxed);
            detect
        } else {
            raw
        };

        // SAFETY: `SimdIsa` is `#[repr(usize)]`, any valid discriminant is
        // a valid bit pattern for the enum, so transmute cannot produce UB.
        unsafe { core::mem::transmute::<usize, Self>(raw) }
    }
}
