use pyo3::{prelude::*, types::PyBytes};

use crate::crc32c_rs;

#[pyclass(name = "_Hasher")]
pub struct Hasher {
    value: u32,
    py_fn: Option<Py<PyAny>>,
}

impl Hasher {
    fn compute(
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        value: u32,
        py_fn: Option<&Py<PyAny>>,
    ) -> PyResult<u32> {
        match py_fn {
            Some(f) => f.bind(py).call1((data, value))?.extract::<u32>(),
            None => crc32c_rs::crc32c(py, data, value),
        }
    }
}

#[pymethods]
impl Hasher {
    #[new]
    #[pyo3(signature = (data = None, r#fn = None))]
    fn new(
        py: Python<'_>,
        data: Option<&Bound<'_, PyAny>>,
        r#fn: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let py_fn = r#fn.map(Py::from);
        let value = match data {
            Some(data) => Self::compute(py, data, 0, py_fn.as_ref())?,
            None => 0,
        };
        Ok(Self { value, py_fn })
    }

    fn update(&mut self, py: Python<'_>, data: &Bound<'_, PyAny>) -> PyResult<()> {
        self.value = Self::compute(py, data, self.value, self.py_fn.as_ref())?;
        Ok(())
    }

    fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.value.to_be_bytes())
    }

    fn hexdigest(&self) -> String {
        format!("{:08x}", self.value)
    }

    fn copy(&self, py: Python<'_>) -> Self {
        Self {
            value: self.value,
            py_fn: self.py_fn.as_ref().map(|f| f.clone_ref(py)),
        }
    }

    #[allow(clippy::unused_self)]
    #[getter]
    const fn digest_size(&self) -> u32 {
        4
    }

    #[allow(clippy::unused_self)]
    #[getter]
    const fn block_size(&self) -> u32 {
        1
    }

    #[getter]
    const fn checksum(&self) -> u32 {
        self.value
    }
}
