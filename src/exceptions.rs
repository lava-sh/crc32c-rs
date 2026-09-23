use pyo3::{create_exception, exceptions::PyException};

create_exception!(
    _crc32c_rs,
    UnsupportedCPUFeatureError,
    PyException,
    "Raised when the current CPU does not support required instructions."
);
