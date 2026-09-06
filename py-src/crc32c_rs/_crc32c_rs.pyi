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
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int: ...

def _crc32c_see42_pclmulqdq(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int: ...

def _crc32c_aes_sha3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int: ...

def _crc32c_aes(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int: ...

def _crc32c_avx512_pclmulqdq(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int: ...

def _crc32c_avx512_vpclmulqdq(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int: ...
