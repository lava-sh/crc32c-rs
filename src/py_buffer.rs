use core::mem::MaybeUninit;

use pyo3::{Bound, PyAny, PyErr, PyResult, Python, ffi};

pub struct PyBuffer(ffi::Py_buffer);

impl Drop for PyBuffer {
    fn drop(&mut self) {
        // SAFETY: the buffer was initialized successfully by
        // `PyObject_GetBuffer` and is released exactly once here.
        unsafe { ffi::PyBuffer_Release(&raw mut self.0) };
    }
}

impl PyBuffer {
    pub fn get(py: Python<'_>, object: &Bound<'_, PyAny>) -> PyResult<Self> {
        let mut buffer = MaybeUninit::<ffi::Py_buffer>::uninit();
        // SAFETY: on success, `Py_buffer` is fully initialized;
        // released once via `PyBuffer_Release`.
        let get_buffer = unsafe {
            ffi::PyObject_GetBuffer(object.as_ptr(), buffer.as_mut_ptr(), ffi::PyBUF_SIMPLE)
        };
        if get_buffer != 0 {
            return Err(PyErr::fetch(py));
        }
        // SAFETY: `PyObject_GetBuffer` returned success, so
        // it initialized the complete `Py_buffer` structure.
        Ok(Self(unsafe { buffer.assume_init() }))
    }

    #[inline]
    pub fn ptr_len(&self) -> (*const u8, usize) {
        (self.0.buf.cast::<u8>(), self.0.len as usize)
    }
}
