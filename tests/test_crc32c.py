import array

import pytest
import pytest_check
from crc32c_rs import crc32c_fallback

from .types import CrcImpl, ReadableBuffer

GIL_MINSIZE = 32 * 1024


@pytest.mark.parametrize(
    ("data", "expected"),
    [
        (b"", 0),
        (b"123456789", 0xE3069283),
        (bytearray(b"123456789"), 0xE3069283),
        (memoryview(b"123456789"), 0xE3069283),
        (array.array("B", b"123456789"), 0xE3069283),
    ],
    ids=["empty", "bytes", "bytearray", "memoryview", "array"],
)
def test_crc32c_buffer(crc_impl: CrcImpl, data: ReadableBuffer, expected: int) -> None:
    for _, crc in crc_impl:
        with pytest_check.check:
            assert crc(data) == expected


def test_crc32c_memoryview_slice(crc_impl: CrcImpl) -> None:
    data = b"a" * 32
    for _, crc in crc_impl:
        with pytest_check.check:
            expected = crc(data[10:26])
            mv = memoryview(data)[10:26]
            assert crc(mv) == expected
            assert len(mv) == 16


def test_crc32c_not_a_buffer(crc_impl: CrcImpl) -> None:
    for _, crc in crc_impl:
        with pytest_check.check, pytest.raises(TypeError):
            crc(12345)
        with pytest_check.check, pytest.raises(TypeError):
            crc(None)
        with pytest_check.check, pytest.raises(TypeError):
            crc({"key": "value"})


@pytest.mark.parametrize(
    "size",
    [
        GIL_MINSIZE - 1,
        GIL_MINSIZE,
        GIL_MINSIZE + 1,
        GIL_MINSIZE * 2,
    ],
    ids=["below_gil", "at_gil", "above_gil", "double_gil"],
)
def test_crc32c_gil_threshold(crc_impl: CrcImpl, size: int) -> None:
    data = b"a" * size
    expected = crc32c_fallback(data)
    for _, crc in crc_impl:
        with pytest_check.check:
            assert crc(data) == expected
