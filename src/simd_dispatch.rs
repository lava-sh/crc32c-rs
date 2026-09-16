#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm64ec",
))]
#[macro_export]
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
impl CpuVendor {
    fn detect() -> Self {
        let arch::CpuidResult { ebx, edx, ecx, .. } = arch::__cpuid(0);

        match (ebx, edx, ecx) {
            // Genu       ineI         ntel
            (0x756e_6547, 0x4965_6e69, 0x6c65_746e) => Self::Intel,
            // Auth       enti         cAMD
            (0x6874_7541, 0x6974_6e65, 0x444d_4163) => Self::Amd,
            _ => Self::Unknown,
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum CpuModel {
    Unknown = 0,
    // Intel
    CascadeLake,
    IceLake,
    SapphireRapids,
    // Amd
    Zen1, // Naples, Summit Ridge, Whitehaven, Raven Ridge
    Zen2, // Rome, Matisse, Renoir, Lucienne, Castle Peak
    Zen3, // Milan, Milan-X, Vermeer, Cezanne, Rembrandt
    Zen4, // Genoa, Genoa-X, Bergamo, Raphael, Phoenix
    Zen5, // Turin, Turin Dense, Granite Ridge, Strix Point, Strix Halo, Krackan Point
    Zen6, // Future architectures (reserved model ranges)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl CpuModel {
    /// Decodes `(family, model)` from CPUID leaf 1 EAX register.
    ///
    /// See: https://www.thomas-krenn.com/en/wiki/CPUID#Processor_Signature
    const fn decode_family_model(eax1: u32) -> (u32, u32) {
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

        (family, model)
    }

    const fn detect(vendor: CpuVendor, eax1: u32) -> Self {
        let (family, model) = Self::decode_family_model(eax1);

        match vendor {
            // See: https://codebrowser.dev/linux/linux/arch/x86/include/asm/intel-family.h.html
            CpuVendor::Intel if family == 0x6 => match model {
                // `Skylake-SP`, `Cascade Lake` and `Cooper Lake` all of
                // them natively support both `AVX-512 VL` and `PCLMULQDQ`.
                0x55 => Self::CascadeLake,
                0x6A | 0x6C | 0x7D | 0x7E | 0x9D => Self::IceLake,
                0x8F => Self::SapphireRapids,
                _ => Self::Unknown,
            },
            // See: https://codebrowser.dev/linux/linux/arch/x86/kernel/cpu/amd.c.html
            CpuVendor::Amd => match family {
                0x17 => match model {
                    0x00..=0x2F | 0x50..=0x5F => Self::Zen1,
                    0x30..=0x4F | 0x60..=0x7F | 0x90..=0x91 | 0xA0..=0xAF => Self::Zen2,
                    _ => Self::Unknown,
                },
                0x19 => match model {
                    0x00..=0x0F | 0x20..=0x5F => Self::Zen3,
                    0x10..=0x1F | 0x60..=0xAF => Self::Zen4,
                    _ => Self::Unknown,
                },
                0x1A => match model {
                    0x00..=0x2F | 0x40..=0x4F | 0x70..=0x7F => Self::Zen5,
                    0x50..=0x5F | 0x80..=0xAF | 0xC0..=0xCF => Self::Zen6,
                    _ => Self::Unknown,
                },
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
    Avx512vlVpclmulqdq_v4s5x3, // Ice Lake (1)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx512vlVpclmulqdq_v3s2x4, // Genoa (1)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx512vlVpclmulqdq_v3s1_s3, // Sapphire Rapids (1)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Avx512vlPclmulqdq_v9s3x4e, // Cascade Lake (1)

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42,

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42Pclmulqdq_v8s3x3, // Cascade Lake (2), Sapphire Rapids (3)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42Pclmulqdq_v7s3x3, // Ice Lake (3)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42Pclmulqdq_v1s4x2, // Milan (1)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42Pclmulqdq_v1s3x3, // Rome (1)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    Sse42Pclmulqdq_v1s3x2, // Genoa (3)

    #[allow(dead_code)]
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    AesSha3_v9s3x2e_s3, // Apple M1 (1)
    #[allow(dead_code)]
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    AesCrc_v3s4x2e_v2, // Ampere Altra (1)
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    AesCrc_v12e_v1, // Apple M1 (2), Ampere Altra (2)
}

impl SimdIsa {
    #[cold]
    fn detect() -> Self {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            let vendor = CpuVendor::detect();
            let eax1 = arch::__cpuid(1).eax;
            let model = CpuModel::detect(vendor, eax1);

            match model {
                CpuModel::SapphireRapids if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) => {
                    return Self::Avx512vlVpclmulqdq_v3s1_s3;
                }

                CpuModel::Zen4 | CpuModel::Zen5 | CpuModel::Zen6 => {
                    if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) {
                        return Self::Avx512vlVpclmulqdq_v3s2x4;
                    }
                    if detect_features!(x86, ["sse4.2", "pclmulqdq"]) {
                        return Self::Sse42Pclmulqdq_v1s3x2;
                    }
                }

                CpuModel::IceLake => {
                    if detect_features!(x86, ["avx512vl", "vpclmulqdq"]) {
                        return Self::Avx512vlVpclmulqdq_v4s5x3;
                    }
                    if detect_features!(x86, ["sse4.2", "pclmulqdq"]) {
                        return Self::Sse42Pclmulqdq_v7s3x3;
                    }
                }

                CpuModel::CascadeLake if detect_features!(x86, ["avx512vl", "pclmulqdq"]) => {
                    return Self::Avx512vlPclmulqdq_v9s3x4e;
                }
                CpuModel::SapphireRapids | CpuModel::CascadeLake
                    if detect_features!(x86, ["sse4.2", "pclmulqdq"]) =>
                {
                    return Self::Sse42Pclmulqdq_v8s3x3;
                }

                CpuModel::Zen3 if detect_features!(x86, ["sse4.2", "pclmulqdq"]) => {
                    return Self::Sse42Pclmulqdq_v1s4x2;
                }

                CpuModel::Zen1 | CpuModel::Zen2
                    if detect_features!(x86, ["sse4.2", "pclmulqdq"]) =>
                {
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
                return Self::Sse42;
            }
        }
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        {
            cfg_select! {
                target_vendor = "apple" => {
                    if detect_features!(aarch64, ["crc", "aes", "sha3"]) {
                        return Self::AesSha3_v9s3x2e_s3;
                    }
                }
                _ => {
                    if detect_features!(aarch64, ["crc", "aes"]) {
                        return Self::AesCrc_v3s4x2e_v2;
                    }
                }
            }
            if detect_features!(aarch64, ["crc", "aes"]) {
                return Self::AesCrc_v12e_v1;
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
