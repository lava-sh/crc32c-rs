use pyo3::{create_exception, exceptions::PyRuntimeException};

create_exception!(
    _crc32c_rs,
    UnsupportedCPUFeatureError,
    PyRuntimeException,
    "Raised when the current CPU does not support required instructions."
);
