use pyo3::{create_exception, exceptions::PyRuntimeError};

create_exception!(
    _crc32c_rs,
    UnsupportedCPUFeatureError,
    PyRuntimeError,
    "Raised when the current CPU does not support required instructions."
);
