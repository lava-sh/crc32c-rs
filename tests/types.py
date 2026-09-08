__all__ = (
    "CrcImpl",
    "ReadableBuffer",
)

import sys
from collections.abc import Callable
from typing import TypeAlias

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    Buffer = bytes | bytearray | memoryview

ReadableBuffer: TypeAlias = Buffer

CrcImpl: TypeAlias = list[tuple[str, Callable[..., int]]]
CrcFn: TypeAlias = Callable[..., int]
