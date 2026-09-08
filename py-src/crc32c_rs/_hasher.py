import sys
from collections.abc import Callable
from typing import Self, TypeAlias

from ._crc32c_rs import _crc32c

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    Buffer = bytes | bytearray | memoryview


ReadableBuffer: TypeAlias = Buffer


class Hasher:
    def __init__(self, data: ReadableBuffer = b"", fn: Callable = _crc32c) -> None:
        self._fn = fn
        self._checksum = self._fn(data)

    @property
    def digest_size(self) -> int:
        return 4

    @property
    def block_size(self) -> int:
        return 1

    @property
    def checksum(self) -> int:
        return self._checksum

    def update(self, data: Buffer) -> None:
        self._checksum = self._fn(data, self._checksum)

    def digest(self) -> bytes:
        return self._checksum.to_bytes(4, "big")

    def hexdigest(self) -> str:
        return self.digest().hex()

    def copy(self) -> Self:
        res = type(self)(fn=self._fn)
        res._checksum = self._checksum  # noqa: SLF001
        return res
