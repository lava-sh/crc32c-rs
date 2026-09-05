mod arch;
mod exceptions;
mod simd_dispatch;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[pyo3::pymodule(name = "_crc32c_rs")]
mod crc32c_rs {
    use core::mem::MaybeUninit;

    use pyo3::{ffi, prelude::*};

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    use crate::arch::{avx512_vpclmulqdq, see42_pclmulqdq, avx512_pclmulqdq};
    use crate::{arch::fallback, simd_dispatch::SimdIsa};

    #[pymodule_export]
    #[allow(non_upper_case_globals, clippy::allow_attributes)]
    const __version__: &str = env!("CARGO_PKG_VERSION");

    #[pymodule_export]
    use crate::exceptions::UnsupportedCPUFeatureError;

    #[pyfunction(name = "_crc32c", signature = (data, value = 0, /))]
    fn crc32c(data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        let buf: &[u8] = data.extract()?;

        match SimdIsa::detected() {
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            SimdIsa::Sse42Pclmulqdq => {
                Ok(unsafe { see42_pclmulqdq::crc32c(value, buf.as_ptr(), buf.len()) })
            }
            _ => Ok(fallback::crc32c(value, buf, buf.len())),
        }
    }

    #[pyfunction(name = "_crc32c_avx512_vpclmulqdq", signature = (data, value = 0, /))]
    fn crc32c_avx512_vpclmulqdq(data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        if SimdIsa::detected() != SimdIsa::Avx512Vpclmulqdq {
            return Err(UnsupportedCPUFeatureError::new_err(
                "AVX512F, AVX512VL, and VPCLMULQDQ are not supported by this CPU",
            ));
        }
        let buf: &[u8] = data.extract()?;
        // SAFETY: the cached runtime ISA check above.
        Ok(unsafe { avx512_vpclmulqdq::crc32c(value, buf.as_ptr(), buf.len()) })
    }

    #[pyfunction(name = "_crc32c_avx512_pclmulqdq", signature = (data, value = 0, /))]
    fn crc32c_avx512_pclmulqdq(data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::Avx512Pclmulqdq | SimdIsa::Avx512Vpclmulqdq
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "AVX512F, AVX512VL, and PCLMULQDQ are not supported by this CPU",
            ));
        }
        let buf: &[u8] = data.extract()?;
        // SAFETY: the cached runtime ISA check above.
        Ok(unsafe { avx512_pclmulqdq::crc32c(value, buf.as_ptr(), buf.len()) })
    }

    #[pyfunction(name = "_crc32c_see42_pclmulqdq", signature = (data, value = 0, /))]
    fn crc32c_see42_pclmulqdq(data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        if !matches!(
            SimdIsa::detected(),
            SimdIsa::Sse42Pclmulqdq | SimdIsa::Avx512Vpclmulqdq | SimdIsa::Avx512Pclmulqdq
        ) {
            return Err(UnsupportedCPUFeatureError::new_err(
                "SSE4.2 and PCLMULQDQ are not supported by this CPU",
            ));
        }
        let buf: &[u8] = data.extract()?;
        // SAFETY: the cached runtime ISA check above.
        Ok(unsafe { see42_pclmulqdq::crc32c(value, buf.as_ptr(), buf.len()) })
    }

    #[pyfunction(name = "_crc32c_fallback", signature = (data, value = 0, /))]
    fn crc32c_fallback(data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        let buf: &[u8] = data.extract()?;
        Ok(fallback::crc32c(value, buf, buf.len()))
    }

    #[pyfunction(name = "_crc32c_2", signature = (data, value = 0, /))]
    fn crc32c_2(py: Python<'_>, data: &Bound<'_, PyAny>, value: u32) -> PyResult<u32> {
        struct Buffer(ffi::Py_buffer);

        impl Drop for Buffer {
            fn drop(&mut self) {
                unsafe { ffi::PyBuffer_Release(&raw mut self.0) };
            }
        }

        fn get_buffer(py: Python<'_>, object: &Bound<'_, PyAny>) -> PyResult<Buffer> {
            let mut buf = MaybeUninit::<ffi::Py_buffer>::uninit();
            if unsafe {
                ffi::PyObject_GetBuffer(object.as_ptr(), buf.as_mut_ptr(), ffi::PyBUF_SIMPLE)
            } != 0
            {
                return Err(PyErr::fetch(py));
            }
            Ok(Buffer(unsafe { buf.assume_init() }))
        }

        let buf = get_buffer(py, data)?;
        let slice =
            unsafe { core::slice::from_raw_parts(buf.0.buf.cast::<u8>(), buf.0.len as usize) };

        Ok(fallback::crc32c(value, slice, slice.len()))
    }
}
