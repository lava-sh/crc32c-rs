use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm64ec",
))]
macro_rules! detect_features {
    (x86, [$($feat:tt),+ $(,)?]) => {{
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        { true $(&& std::arch::is_x86_feature_detected!($feat))+ }
    }};
    (aarch64, [$($feat:tt),+ $(,)?]) => {{
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        { true $(&& std::arch::is_aarch64_feature_detected!($feat))+ }
    }};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum CpuVendor {
    Unknown,
    Intel,
    Amd,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
struct VendorInfo {
    ebx: u32,
    edx: u32,
    ecx: u32,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl CpuVendor {
    fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        let leaf0 = core::arch::x86_64::__cpuid(0);
        #[cfg(target_arch = "x86")]
        let leaf0 = core::arch::x86::__cpuid(0);

        let vi = VendorInfo {
            ebx: leaf0.ebx,
            edx: leaf0.edx,
            ecx: leaf0.ecx,
        };
        let s = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                (&raw const vi).cast::<u8>(),
                size_of::<VendorInfo>(),
            ))
        };

        match s {
            "GenuineIntel" => Self::Intel,
            "AuthenticAMD" => Self::Amd,
            _ => Self::Unknown,
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const ICE_LAKE_MODELS: &[u32] = &[0x6A, 0x6C, 0x7D, 0x7E, 0x8C, 0x8D, 0xA5];

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum CpuModel {
    Unknown = 0,
    CascadeLake,    // 0x55
    IceLake,        // 0x6A/0x6C/0x7D/0x7E/0x8C/0x8D/0xA5
    SapphireRapids, // 0x8F
    Rome,           // 0x17
    Milan,          // 0x19
    Genoa,          // 0x1A
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl CpuModel {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn detect(vendor: CpuVendor, eax1: u32) -> Self {
        let base_family = (eax1 >> 8) & 0xF;
        let base_model = (eax1 >> 4) & 0xF;
        let ext_family = (eax1 >> 20) & 0xFF;
        let ext_model = (eax1 >> 16) & 0xF;

        let family = if base_family == 0xF {
            base_family + ext_family
        } else {
            base_family
        };
        let model = if base_family == 0xF || base_family == 0x6 {
            (ext_model << 4) | base_model
        } else {
            base_model
        };

        match vendor {
            CpuVendor::Intel if family == 0x6 => match model {
                0x55 => Self::CascadeLake,
                m if ICE_LAKE_MODELS.contains(&m) => Self::IceLake,
                0x8F => Self::SapphireRapids,
                _ => Self::Unknown,
            },
            CpuVendor::Amd => match family {
                0x17 => Self::Rome,
                0x19 => Self::Milan,
                0x1A => Self::Genoa,
                _ => Self::Unknown,
            },
            _ => Self::Unknown,
        }
    }
}

// SIMD Instruction Set Architecture
#[repr(usize)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SimdIsa {
    Fallback = 0,

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Genoa (1)
    Avx512vlVpclmulqdq_v3s2x4,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Ice Lake (1)
    Avx512vlVpclmulqdq_v4s5x3,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Sapphire Rapids (1)
    Avx512vlVpclmulqdq_v3s1_s3,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Cascade Lake (1)
    Avx512vlPclmulqdq_v9s3x4e,

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Cascade Lake (2), Sapphire Rapids (3)
    Sse42Pclmulqdq_v8s3x3,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Ice Lake (3)
    Sse42Pclmulqdq_v7s3x3,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Genoa (3)
    Sse42Pclmulqdq_v1s3x2,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Milan (1)
    Sse42Pclmulqdq_v1s4x2,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Rome (1)
    Sse42Pclmulqdq_v1s3x3,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    // Rome (2), Milan (2)
    Sse42_s3k4096e,

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    // Apple M1 (1)
    AesSha3_v9s3x2e_s3,
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    // Ampere Altra (1)
    AesCrc_v3s4x2e_v2,
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    // Apple M1 (2)
    AesCrc_v12e_v1,
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    // Ampere Altra (2)
    CrcNeon_s3k95760_s3,
}

impl SimdIsa {
    #[cold]
    fn detect() -> Self {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86 as arch;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64 as arch;

            let vendor = CpuVendor::detect();
            let eax1 = arch::__cpuid(1).eax;
            let model = CpuModel::detect(vendor, eax1);

            match model {
                CpuModel::SapphireRapids if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) => {
                    return Self::Avx512vlVpclmulqdq_v3s1_s3;
                }

                CpuModel::Genoa if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) => {
                    return Self::Avx512vlVpclmulqdq_v3s2x4;
                }
                CpuModel::Genoa if detect_features!(x86, ["sse4.2", "pclmulqdq"]) => {
                    return Self::Sse42Pclmulqdq_v1s3x2;
                }

                CpuModel::IceLake if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) => {
                    return Self::Avx512vlVpclmulqdq_v4s5x3;
                }
                CpuModel::IceLake if detect_features!(x86, ["sse4.2", "pclmulqdq"]) => {
                    return Self::Sse42Pclmulqdq_v7s3x3;
                }

                CpuModel::CascadeLake if detect_features!(x86, ["avx512vl", "pclmulqdq"]) => {
                    return Self::Avx512vlPclmulqdq_v9s3x4e;
                }
                CpuModel::SapphireRapids | CpuModel::CascadeLake
                    if detect_features!(x86, ["sse4.2", "pclmulqdq"]) =>
                {
                    return Self::Sse42Pclmulqdq_v8s3x3;
                }

                CpuModel::Milan if detect_features!(x86, ["sse4.2", "pclmulqdq"]) => {
                    return Self::Sse42Pclmulqdq_v1s4x2;
                }
                CpuModel::Milan | CpuModel::Milan if detect_features!(x86, ["sse4.2"]) => {
                    return Self::Sse42_s3k4096e;
                }

                CpuModel::Rome if detect_features!(x86, ["sse4.2", "pclmulqdq"]) => {
                    return Self::Sse42Pclmulqdq_v1s3x3;
                }

                _ => {}
            }

            if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) {
                return Self::Avx512vlVpclmulqdq_v4s5x3;
            }
            if detect_features!(x86, ["avx512vl", "pclmulqdq"]) {
                return Self::Avx512vlPclmulqdq_v9s3x4e;
            }
            if detect_features!(x86, ["sse4.2", "pclmulqdq"]) {
                return Self::Sse42Pclmulqdq_v8s3x3;
            }
            if detect_features!(x86, ["sse4.2"]) {
                return Self::Sse42_s3k4096e;
            }
        }
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        {
            if detect_features!(aarch64, ["crc", "aes", "sha3"]) {
                return Self::AesSha3_v9s3x2e_s3;
            }
            if detect_features!(aarch64, ["crc", "aes"]) {
                cfg_select! {
                    target_vendor = "apple" => return Self::AesCrc_v12e_v1,
                    _ => return Self::AesCrc_v3s4x2e_v2,
                }
            }
            if detect_features!(aarch64, ["crc"]) {
                return Self::CrcNeon_s3k95760_s3;
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
