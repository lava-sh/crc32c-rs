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

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    use crate::arch::{aes_sha3_v9s3x2e_s3, aes_v3s4x2e_v2};
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    use crate::arch::{
        avx512vl_pclmulqdq_v9s3x4e, avx512vl_vpclmulqdq_v3s1_s3, see42_pclmulqdq_v7s3x3,
    };
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
            SimdIsa::Avx512vlVpclmulqdq_v3s1_s3 => avx512vl_vpclmulqdq_v3s1_s3::crc32c,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdIsa::Avx512vlPclmulqdq_v9s3x4e => avx512vl_pclmulqdq_v9s3x4e::crc32c,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdIsa::Sse42Pclmulqdq_v7s3x3 => see42_pclmulqdq_v7s3x3::crc32c,
            #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
            SimdIsa::AesSha3_v9s3x2e_s3 => aes_sha3_v9s3x2e_s3::crc32c,
            #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
            SimdIsa::AesCrc_v3s4x2e_v2 => aes_v3s4x2e_v2::crc32c,
            _ => fallback,
        };

        Ok(crc32c_dispatch(py, &buffer, value, impl_fn))
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[pyfunction(name = "_crc32c_avx512vl_vpclmulqdq_v3s1_s3", signature = (data, value = 0, /))]
    fn crc32c_avx512vl_vpclmulqdq_v3s1_s3(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if SimdIsa::detected() != SimdIsa::Avx512vlVpclmulqdq_v3s1_s3 {
            return Err(UnsupportedCPUFeatureError::new_err(
                "AVX512VL and VPCLMULQDQ are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(
            py,
            &buffer,
            value,
            avx512vl_vpclmulqdq_v3s1_s3::crc32c,
        ))
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[pyfunction(name = "_crc32c_avx512vl_pclmulqdq_v9s3x4e", signature = (data, value = 0, /))]
    fn crc32c_avx512vl_pclmulqdq_v9s3x4e(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::Avx512vlPclmulqdq_v9s3x4e | SimdIsa::Avx512vlVpclmulqdq_v3s1_s3
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "AVX512VL and PCLMULQDQ are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(
            py,
            &buffer,
            value,
            avx512vl_pclmulqdq_v9s3x4e::crc32c,
        ))
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[pyfunction(name = "_crc32c_see42_pclmulqdq_v7s3x3", signature = (data, value = 0, /))]
    fn crc32c_see42_pclmulqdq_v7s3x3(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::Sse42Pclmulqdq_v7s3x3
                | SimdIsa::Avx512vlVpclmulqdq_v3s1_s3
                | SimdIsa::Avx512vlPclmulqdq_v9s3x4e
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "SSE4.2 and PCLMULQDQ are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(
            py,
            &buffer,
            value,
            see42_pclmulqdq_v7s3x3::crc32c,
        ))
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    #[pyfunction(name = "_crc32c_aes_sha3_v9s3x2e_s3", signature = (data, value = 0, /))]
    fn crc32c_aes_sha3_v9s3x2e_s3(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
    ) -> PyResult<u32> {
        if SimdIsa::detected() != SimdIsa::AesSha3_v9s3x2e_s3 {
            return Err(UnsupportedCPUFeatureError::new_err(
                "CRC, AES, and SHA3 are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(
            py,
            &buffer,
            value,
            aes_sha3_v9s3x2e_s3::crc32c,
        ))
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    #[pyfunction(name = "_crc32c_aes_v3s4x2e_v2", signature = (data, value = 0, /))]
    fn crc32c_aes_v3s4x2e_v2(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::AesCrc_v3s4x2e_v2 | SimdIsa::AesSha3_v9s3x2e_s3
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "CRC and AES are not supported by this CPU",
            ));
        }
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(py, &buffer, value, aes_v3s4x2e_v2::crc32c))
    }

    #[pyfunction(name = "_crc32c_fallback", signature = (data, value = 0, /))]
    fn crc32c_fallback(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        let buffer = PyBuffer::get(py, data)?;
        Ok(crc32c_dispatch(py, &buffer, value, fallback))
    }
}
