use pyo3::{create_exception, exceptions::PyException};

create_exception!(_crc32c_rs, UnsupportedCPUFeatureError, PyException);
