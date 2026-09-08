import array
import math
from typing import Any

import pytest
from crc32c_rs import Hasher, crc32c_fallback

from .types import ReadableBuffer

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
def test_hasher_init(data: ReadableBuffer, expected: int) -> None:
    h = Hasher(data)
    assert h.checksum == expected
    assert h.digest() == expected.to_bytes(4, "big")
    assert h.hexdigest() == format(expected, "08x")


def test_hasher_init_empty() -> None:
    h = Hasher()
    assert h.checksum == 0
    assert h.digest() == b"\x00\x00\x00\x00"
    assert h.hexdigest() == "00000000"


def test_hasher_update() -> None:
    h = Hasher()
    h.update(b"123")
    h.update(b"456")
    h.update(b"789")
    assert h.checksum == 0xE3069283
    assert h.digest() == b"\xe3\x06\x92\x83"


def test_hasher_update_chain() -> None:
    h = Hasher(b"123")
    h.update(b"456")
    h.update(b"789")
    assert h.digest() == b"\xe3\x06\x92\x83"


@pytest.mark.parametrize(
    ("data", "chunks"),
    [
        (b"123456789", [b"123", b"456", b"789"]),
        (b"a" * 100, [b"a" * 33, b"a" * 33, b"a" * 34]),
        (b"x" * 1024, [b"x" * 512, b"x" * 512]),
    ],
)
def test_hasher_update_chunks(data: ReadableBuffer, chunks: list[ReadableBuffer]) -> None:
    h1 = Hasher(data)
    h2 = Hasher()
    for chunk in chunks:
        h2.update(chunk)
    assert h1.checksum == h2.checksum


def test_hasher_digest() -> None:
    h = Hasher(b"123456789")
    assert h.digest() == b"\xe3\x06\x92\x83"
    assert len(h.digest()) == 4


def test_hasher_hexdigest() -> None:
    h = Hasher(b"123456789")
    assert h.hexdigest() == "e3069283"
    assert len(h.hexdigest()) == 8


def test_hasher_copy() -> None:
    h1 = Hasher(b"123456789")
    h2 = h1.copy()

    assert h1.checksum == h2.checksum
    assert h1.digest() == h2.digest()

    h2.update(b"extra")
    assert h1.checksum != h2.checksum


def test_hasher_copy_with_custom_fn() -> None:
    h1 = Hasher(b"123456789", crc32c_fallback)
    h2 = h1.copy()

    assert h1.checksum == h2.checksum
    assert h1.digest() == h2.digest()

    h1.update(b"extra")
    h2.update(b"extra")
    assert h1.checksum == h2.checksum


def test_hasher_properties() -> None:
    h = Hasher()
    assert h.digest_size == 4
    assert h.block_size == 1


def test_hasher_checksum_property() -> None:
    h = Hasher(b"123456789")
    assert h.checksum == 0xE3069283

    h.update(b"extra")
    assert h.checksum != 0xE3069283


@pytest.mark.parametrize(
    "invalid_data",
    [
        12345,
        None,
        {"key": "value"},
        math.pi,
        [1, 2, 3],
    ],
    ids=["int", "none", "dict", "float", "list"],
)
def test_hasher_init_invalid(invalid_data: Any) -> None:
    with pytest.raises(TypeError):
        Hasher(invalid_data)  # type: ignore[arg-type]


@pytest.mark.parametrize(
    "invalid_data",
    [
        12345,
        None,
        {"key": "value"},
        math.pi,
        [1, 2, 3],
    ],
    ids=["int", "none", "dict", "float", "list"],
)
def test_hasher_update_invalid(invalid_data: Any) -> None:
    h = Hasher()
    with pytest.raises(TypeError):
        h.update(invalid_data)  # type: ignore[arg-type]


def test_hasher_custom_function() -> None:
    def my_crc(data: ReadableBuffer, value: int = 0) -> int:
        return value + len(bytes(data))  # type: ignore[arg-type]

    h = Hasher(b"test", my_crc)
    assert h.checksum == 4
    h.update(b"test")
    assert h.checksum == 8


def test_hasher_custom_function_from_module() -> None:
    h = Hasher(b"123456789", crc32c_fallback)
    assert h.checksum == 0xE3069283


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
def test_hasher_large_data(size: int) -> None:
    data = b"a" * size
    h = Hasher(data)
    expected = crc32c_fallback(data)
    assert h.checksum == expected


@pytest.mark.parametrize(
    "size",
    [
        1024,
        1024 * 1024,
        10 * 1024 * 1024,
    ],
)
def test_hasher_multiple_updates_large(size: int) -> None:
    data = b"a" * size
    chunk_size = 4096
    chunks = [data[i : i + chunk_size] for i in range(0, size, chunk_size)]

    h = Hasher()
    for chunk in chunks:
        h.update(chunk)

    expected = crc32c_fallback(data)
    assert h.checksum == expected


def test_hasher_memoryview_slice() -> None:
    data = b"a" * 32
    h1 = Hasher(data[10:26])

    mv = memoryview(data)[10:26]
    h2 = Hasher(mv)

    assert h1.checksum == h2.checksum
    assert len(mv) == 16


def test_hasher_compare_with_fallback() -> None:
    test_data = [
        b"",
        b"123456789",
        b"a" * 100,
        b"x" * 1024,
        b"y" * 1024 * 1024,
    ]

    for data in test_data:
        h = Hasher(data)
        expected = crc32c_fallback(data)
        assert h.checksum == expected


def test_hasher_repr() -> None:
    h = Hasher(b"123456789")
    repr_str = repr(h)
    assert "Hasher" in repr_str
    assert "e3069283" in repr_str.lower()


def test_hasher_update_returns_none() -> None:
    h = Hasher()
    result = h.update(b"test")
    assert result is None


def test_hasher_copy_independence() -> None:
    h1 = Hasher(b"123")
    h2 = h1.copy()

    h1.update(b"456")
    h2.update(b"789")

    assert h1.checksum != h2.checksum
    assert h1.digest() != h2.digest()
