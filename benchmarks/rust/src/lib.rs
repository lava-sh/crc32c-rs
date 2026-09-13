//! Harness shared by the CodSpeed benchmarks.
//!
//! `crc32c-rs` is a PyO3 extension module, so its only library target is a
//! `cdylib` that links against `libpython`: it cannot be used as a dependency
//! of a benchmark target. The CRC32C kernels are therefore compiled straight
//! into this crate through `#[path]`, which keeps the benchmarks on exactly
//! the code that ships in the wheel while keeping the benchmark build free of
//! any Python toolchain.

#![feature(hint_prefetch)]

use core::{fmt, ops};

#[path = "../../../src/arch/mod.rs"]
pub mod arch;
pub mod payloads;
#[path = "../../../src/simd_dispatch.rs"]
pub mod simd_dispatch;

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
use crate::arch::{aes_crc_v12e_v1, aes_sha3_v9s3x2e_s3, aes_v3s4x2e_v2};
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::arch::{
    avx512vl_pclmulqdq_v9s3x4e, avx512vl_vpclmulqdq_v3s1_s3, avx512vl_vpclmulqdq_v3s2x4,
    avx512vl_vpclmulqdq_v4s5x3, sse42_pclmulqdq_v1s3x2, sse42_pclmulqdq_v1s3x3,
    sse42_pclmulqdq_v1s4x2, sse42_pclmulqdq_v7s3x3, sse42_pclmulqdq_v8s3x3,
};
use crate::{arch::fallback, simd_dispatch::SimdIsa};

pub type Fn = unsafe fn(u32, *const u8, usize) -> u32;

fn fallback_kernel(value: u32, ptr: *const u8, len: usize) -> u32 {
    // SAFETY: `ptr` and `len` always come from a live slice.
    unsafe { fallback::crc32c(value, core::slice::from_raw_parts(ptr, len), len) }
}

/// A CRC32C kernel, named after the module it is implemented in.
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
        if crate::detect_features!(x86, ["sse4.2", "pclmulqdq"]) {
            kernels.extend_from_slice(&[
                kernel_entry("sse42_pclmulqdq_v1s3x2", sse42_pclmulqdq_v1s3x2::crc32c),
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
                    "avx512vl_vpclmulqdq_v3s1_s3",
                    avx512vl_vpclmulqdq_v3s1_s3::crc32c,
                ),
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

#[must_use]
pub fn kernel() -> Fn {
    match SimdIsa::detected() {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Avx512vlVpclmulqdq_v3s1_s3 => avx512vl_vpclmulqdq_v3s1_s3::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Avx512vlVpclmulqdq_v3s2x4 => avx512vl_vpclmulqdq_v3s2x4::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Avx512vlVpclmulqdq_v4s5x3 => avx512vl_vpclmulqdq_v4s5x3::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Avx512vlPclmulqdq_v9s3x4e => avx512vl_pclmulqdq_v9s3x4e::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Sse42Pclmulqdq_v7s3x3 => sse42_pclmulqdq_v7s3x3::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Sse42Pclmulqdq_v8s3x3 => sse42_pclmulqdq_v8s3x3::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Sse42Pclmulqdq_v1s3x2 => sse42_pclmulqdq_v1s3x2::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Sse42Pclmulqdq_v1s3x3 => sse42_pclmulqdq_v1s3x3::crc32c,
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        SimdIsa::Sse42Pclmulqdq_v1s4x2 => sse42_pclmulqdq_v1s4x2::crc32c,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        SimdIsa::AesSha3_v9s3x2e_s3 => aes_sha3_v9s3x2e_s3::crc32c,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        SimdIsa::AesCrc_v3s4x2e_v2 => aes_v3s4x2e_v2::crc32c,
        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        SimdIsa::AesCrc_v12e_v1 => aes_crc_v12e_v1::crc32c,
        _ => fallback_kernel,
    }
}

pub fn dispatched() -> impl ops::Fn(&[u8], u32) -> u32 + Copy {
    let kernel = kernel();

    move |data, value| {
        // SAFETY: `kernel` was selected from the ISA detected at runtime, and
        // `data` stays alive for the whole call.
        unsafe { kernel(value, data.as_ptr(), data.len()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FALLBACK, available, dispatched};

    /// Check value of CRC32C, as defined by RFC 3720.
    const CHECK: u32 = 0xE306_9283;

    /// Guards against the harness benchmarking something that is not CRC32C,
    /// e.g. after the kernels have moved around in `src/`.
    #[test]
    fn kernels_agree_on_the_check_value() {
        let data = b"123456789";

        assert_eq!(FALLBACK.run(data, 0), CHECK);
        assert_eq!(dispatched()(data, 0), CHECK);

        for kernel in available() {
            assert_eq!(kernel.run(data, 0), CHECK, "{kernel} is not CRC32C");
        }
    }
}
