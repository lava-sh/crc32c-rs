//! Harness shared by the CodSpeed benchmarks.
//!
//! `crc32c-rs` is a PyO3 extension module, so its only library target is a
//! `cdylib` that links against `libpython`: it cannot be used as a dependency
//! of a benchmark target. The CRC32C kernels are therefore compiled straight
//! into this crate through `#[path]`, which keeps the benchmarks on exactly
//! the code that ships in the wheel while keeping the benchmark build free of
//! any Python toolchain.

#![feature(hint_prefetch)]

use core::fmt;

#[path = "../../../src/arch/mod.rs"]
pub mod arch;
pub mod payloads;
#[path = "../../../src/simd_dispatch.rs"]
pub mod simd_dispatch;

use crate::arch::fallback;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
use crate::arch::{aes_crc_v12e_v1, aes_sha3_v9s3x2e_s3, aes_v3s4x2e_v2};
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::arch::{
    avx512vl_pclmulqdq_v9s3x4e, avx512vl_vpclmulqdq_v3s1_s3_hybrid, avx512vl_vpclmulqdq_v3s2x4,
    avx512vl_vpclmulqdq_v4s5x3, sse42_pclmulqdq_v1s3x2_hybrid, sse42_pclmulqdq_v1s3x3,
    sse42_pclmulqdq_v1s4x2, sse42_pclmulqdq_v7s3x3, sse42_pclmulqdq_v8s3x3,
};

pub type Fn = unsafe fn(u32, *const u8, usize) -> u32;

fn fallback_kernel(value: u32, ptr: *const u8, len: usize) -> u32 {
    // SAFETY: `ptr` and `len` always come from a live slice.
    unsafe { fallback::crc32c(value, core::slice::from_raw_parts(ptr, len), len) }
}

#[derive(Clone, Copy)]
pub struct Kernel {
    pub name: &'static str,
    pub func: Fn,
}

impl fmt::Display for Kernel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl Kernel {
    /// Runs the kernel on `data`, chaining `value` as the previous CRC.
    #[must_use]
    pub fn run(self, data: &[u8], value: u32) -> u32 {
        // SAFETY: the kernel is only ever built by `available()`, which checks
        // the CPU features it needs, and `data` stays alive for the whole call.
        unsafe { (self.func)(value, data.as_ptr(), data.len()) }
    }
}

const fn kernel_entry(name: &'static str, func: Fn) -> Kernel {
    Kernel { name, func }
}

/// Portable table-based kernel, the only one available on every CPU.
pub const FALLBACK: Kernel = kernel_entry("fallback", fallback_kernel);

#[must_use]
pub fn available() -> Vec<Kernel> {
    let mut kernels = vec![FALLBACK];

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        // The two hybrids keep the ids of the kernels they extend (the same code
        // with MariaDB's small/mid-size branch spliced in), so CodSpeed reports
        // them as a diff against the past measurements of those ids instead of as
        // new benchmarks — and those kernels are benchmarked through their
        // successor rather than twice.
        if crate::detect_features!(x86, ["sse4.2", "pclmulqdq"]) {
            kernels.extend_from_slice(&[
                kernel_entry(
                    "sse42_pclmulqdq_v1s3x2",
                    sse42_pclmulqdq_v1s3x2_hybrid::crc32c,
                ),
                kernel_entry("sse42_pclmulqdq_v1s3x3", sse42_pclmulqdq_v1s3x3::crc32c),
                kernel_entry("sse42_pclmulqdq_v1s4x2", sse42_pclmulqdq_v1s4x2::crc32c),
                kernel_entry("sse42_pclmulqdq_v7s3x3", sse42_pclmulqdq_v7s3x3::crc32c),
                kernel_entry("sse42_pclmulqdq_v8s3x3", sse42_pclmulqdq_v8s3x3::crc32c),
            ]);
        }
        if crate::detect_features!(x86, ["avx512vl", "pclmulqdq"]) {
            kernels.push(kernel_entry(
                "avx512vl_pclmulqdq_v9s3x4e",
                avx512vl_pclmulqdq_v9s3x4e::crc32c,
            ));
        }
        if crate::detect_features!(x86, ["avx512vl", "vpclmulqdq"]) {
            kernels.extend_from_slice(&[
                kernel_entry(
                    "avx512vl_vpclmulqdq_v3s2x4",
                    avx512vl_vpclmulqdq_v3s2x4::crc32c,
                ),
                kernel_entry(
                    "avx512vl_vpclmulqdq_v4s5x3",
                    avx512vl_vpclmulqdq_v4s5x3::crc32c,
                ),
            ]);
        }
        if crate::detect_features!(x86, ["avx512bw", "avx512dq", "avx512vl", "vpclmulqdq"]) {
            kernels.push(kernel_entry(
                "avx512vl_vpclmulqdq_v3s1_s3",
                avx512vl_vpclmulqdq_v3s1_s3_hybrid::crc32c,
            ));
        }
    }
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    {
        if crate::detect_features!(aarch64, ["crc", "aes"]) {
            kernels.extend_from_slice(&[
                kernel_entry("aes_crc_v12e_v1", aes_crc_v12e_v1::crc32c),
                kernel_entry("aes_v3s4x2e_v2", aes_v3s4x2e_v2::crc32c),
            ]);
        }
        if crate::detect_features!(aarch64, ["crc", "aes", "sha3"]) {
            kernels.push(kernel_entry(
                "aes_sha3_v9s3x2e_s3",
                aes_sha3_v9s3x2e_s3::crc32c,
            ));
        }
    }

    kernels
}

#[cfg(test)]
mod tests {
    use crate::available;

    /// Check value of CRC32C, as defined by RFC 3720.
    const CHECK: u32 = 0xE306_9283;

    #[test]
    fn kernels_agree_on_the_check_value() {
        let data = b"123456789";

        for kernel in available() {
            assert_eq!(kernel.run(data, 0), CHECK, "{kernel} is not CRC32C");
        }
    }
}
