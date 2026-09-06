mod arch;
mod exceptions;
mod py_buffer;
mod simd_dispatch;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[pyo3::pymodule(name = "_crc32c_rs")]
mod crc32c_rs {
    use pyo3::prelude::*;

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    use crate::arch::{avx512_pclmulqdq, avx512_vpclmulqdq, see42_pclmulqdq};
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    use crate::arch::{neon64, neon64_sha3};
    use crate::{arch::fallback, py_buffer::PyBuffer, simd_dispatch::SimdIsa};

    // releasing / reacquiring the GIL has  overhead that outweighs the benefit
    // for small buffers, so only detach the GIL for inputs at or above this size.
    const GIL_MINSIZE: usize = 32 * 1024; // 32 KB

    #[pymodule_export]
    #[allow(non_upper_case_globals)]
    const __version__: &str = env!("CARGO_PKG_VERSION");

    #[pymodule_export]
    use crate::exceptions::UnsupportedCPUFeatureError;

    #[inline]
    fn crc32c_dispatch(
        py: Python<'_>,
        buffer: &PyBuffer,
        value: u32,
        impl_fn: unsafe fn(u32, *const u8, usize) -> u32,
    ) -> u32 {
        let (ptr, len) = buffer.ptr_len();

        if len < GIL_MINSIZE {
            // SAFETY: the caller selected the implementation after its runtime
            // feature check & the `Py_buffer` remains alive for this call.
            unsafe { impl_fn(value, ptr, len) }
        } else {
            let ptr = ptr as usize;
            py.detach(|| {
                // SAFETY: `ptr` and `len` came from a live `Py_buffer` & the
                // selected CRC implementation was checked before this call.
                unsafe { impl_fn(value, ptr as *const u8, len) }
            })
        }
    }

    #[inline]
    fn fallback(value: u32, ptr: *const u8, len: usize) -> u32 {
        unsafe { fallback::crc32c(value, core::slice::from_raw_parts(ptr, len), len) }
    }

    #[pyfunction(name = "_crc32c", signature = (data, value = 0, /))]
    fn crc32c(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        let buffer = PyBuffer::get(py, data)?;

        let impl_fn = match SimdIsa::detected() {
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdIsa::Avx512Vpclmulqdq => avx512_vpclmulqdq::crc32c,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdIsa::Avx512Pclmulqdq => avx512_pclmulqdq::crc32c,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdIsa::Sse42Pclmulqdq => see42_pclmulqdq::crc32c,
            #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
            SimdIsa::Neon64Sha3 => neon64_sha3::crc32c,
            #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
            SimdIsa::Neon64 => neon64::crc32c,
            _ => fallback,
        };

        Ok(crc32c_dispatch(py, &buffer, value, impl_fn))
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[pyfunction(name = "_crc32c_avx512_vpclmulqdq", signature = (data, value = 0, /))]
    fn crc32c_avx512_vpclmulqdq(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if SimdIsa::detected() != SimdIsa::Avx512Vpclmulqdq {
            return Err(UnsupportedCPUFeatureError::new_err(
                "AVX512F, AVX512VL, and VPCLMULQDQ are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(
            py,
            &buffer,
            value,
            avx512_vpclmulqdq::crc32c,
        ))
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[pyfunction(name = "_crc32c_avx512_pclmulqdq", signature = (data, value = 0, /))]
    fn crc32c_avx512_pclmulqdq(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::Avx512Pclmulqdq | SimdIsa::Avx512Vpclmulqdq
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "AVX512F, AVX512VL, and PCLMULQDQ are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(
            py,
            &buffer,
            value,
            avx512_pclmulqdq::crc32c,
        ))
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[pyfunction(name = "_crc32c_see42_pclmulqdq", signature = (data, value = 0, /))]
    fn crc32c_see42_pclmulqdq(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::Sse42Pclmulqdq | SimdIsa::Avx512Vpclmulqdq | SimdIsa::Avx512Pclmulqdq
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "SSE4.2 and PCLMULQDQ are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(py, &buffer, value, see42_pclmulqdq::crc32c))
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    #[pyfunction(name = "_crc32c_neon64_sha3", signature = (data, value = 0, /))]
    fn crc32c_neon64_sha3(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        if SimdIsa::detected() != SimdIsa::Neon64Sha3 {
            return Err(UnsupportedCPUFeatureError::new_err(
                "CRC, AES, and SHA3 are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(py, &buffer, value, neon64_sha3::crc32c))
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    #[pyfunction(name = "_crc32c_neon64", signature = (data, value = 0, /))]
    fn crc32c_neon64(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        if !matches!(SimdIsa::detected(), SimdIsa::Neon64 | SimdIsa::Neon64Sha3) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "CRC and AES are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(py, &buffer, value, neon64::crc32c))
    }

    #[pyfunction(name = "_crc32c_fallback", signature = (data, value = 0, /))]
    fn crc32c_fallback(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(py, &buffer, value, fallback))
    }
}
