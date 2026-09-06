import pytest
from crc32c_rs import crc32c

from .helpers import ReadableBuffer


@pytest.mark.parametrize(
    "data",
    [
        b"123456789",
        bytearray(b"123456789"),
        memoryview(b"123456789"),
    ],
    ids=["bytes", "bytearray", "memoryview"],
)
def test_crc32c_buffer(data: ReadableBuffer) -> None:
    expected = 0xE3069283
    assert crc32c(data) == expected
