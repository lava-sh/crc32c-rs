import builtins
import sys
from typing import TypeAlias

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

ReadableBuffer: TypeAlias = Buffer

__version__: str

class UnsupportedCPUFeatureError(RuntimeError):
    """Raised when the current CPU does not support required instructions."""

def _crc32c(data: ReadableBuffer, value: builtins.int = 0, /) -> builtins.int: ...

def _crc32c_fallback(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_avx512vl_pclmulqdq_v9s3x4e(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_avx512vl_vpclmulqdq_v3s1_s3(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_avx512vl_vpclmulqdq_v3s2x4(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_avx512vl_vpclmulqdq_v4s5x3(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_sse42_pclmulqdq_v1s3x2(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_sse42_pclmulqdq_v1s3x3(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_sse42_pclmulqdq_v1s4x2(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_sse42_pclmulqdq_v7s3x3(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_sse42_pclmulqdq_v8s3x3(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_aes_crc_v12e_v1(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_aes_v3s4x2e_v2(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...

def _crc32c_aes_sha3_v9s3x2e_s3(
    data: ReadableBuffer, value: builtins.int = 0, /,
) -> builtins.int: ...
