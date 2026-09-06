import array

import pytest
from crc32c_rs import crc32c

from .helpers import ReadableBuffer

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
)
def test_crc32c_buffer(data: ReadableBuffer, expected: int) -> None:
    assert crc32c(data) == expected


def test_crc32c_memoryview_slice() -> None:
    data = b"a" * 32
    expected = crc32c(data[10:26])

    mv = memoryview(data)[10:26]
    assert crc32c(mv) == expected
    assert len(mv) == 16  # 26 - 10 = 16


def test_crc32c_not_a_buffer() -> None:
    with pytest.raises(TypeError):
        crc32c(12345)  # ty: ignore[invalid-argument-type]

    with pytest.raises(TypeError):
        crc32c(None)  # ty: ignore[invalid-argument-type]

    with pytest.raises(TypeError):
        crc32c({"key": "value"})  # ty: ignore[invalid-argument-type]


@pytest.mark.parametrize(
    "size",
    [
        GIL_MINSIZE - 1,   # Just below threshold (no GIL release)
        GIL_MINSIZE,       # At threshold
        GIL_MINSIZE + 1,   # Just above threshold (with GIL release)
        GIL_MINSIZE * 2,   # Well above threshold
    ],
    ids=[
        "below_gil_minsize",
        "at_gil_minsize",
        "above_gil_minsize",
        "double_gil_minsize",
    ],
)
def test_crc32c_gil_threshold(size: int) -> None:
    data = b"a" * size
    expected = crc32c(data)
    assert crc32c(data) == expected
