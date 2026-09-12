import array

import pytest
from crc32c_rs import crc32c, crc32c_fallback
from pytest_codspeed import BenchmarkFixture

from .payloads import KIB, MIB, PAYLOADS, SIZES

SIZE_IDS = list(SIZES)


@pytest.mark.parametrize("size", SIZE_IDS)
def test_crc32c(benchmark: BenchmarkFixture, size: str) -> None:
    """Runtime-dispatched implementation, the one used by `crc32c_rs.crc32c`."""
    data = PAYLOADS[size]
    benchmark(crc32c, data)


@pytest.mark.parametrize("size", SIZE_IDS)
def test_crc32c_fallback(benchmark: BenchmarkFixture, size: str) -> None:
    """Portable table-based implementation, used when no SIMD ISA is available."""
    data = PAYLOADS[size]
    benchmark(crc32c_fallback, data)


@pytest.mark.parametrize(
    "wrap",
    [bytes, bytearray, memoryview, lambda data: array.array("B", data)],
    ids=["bytes", "bytearray", "memoryview", "array"],
)
def test_crc32c_buffer_protocol(benchmark: BenchmarkFixture, wrap: object) -> None:
    """Cost of acquiring a `Py_buffer` from the various buffer implementations."""
    data = wrap(PAYLOADS["1KiB"])  # ty: ignore[call-non-callable]
    benchmark(crc32c, data)


def test_crc32c_memoryview_slice(benchmark: BenchmarkFixture) -> None:
    data = memoryview(PAYLOADS["1MiB"])[1 : MIB - 1]
    benchmark(crc32c, data)


def test_crc32c_chained(benchmark: BenchmarkFixture) -> None:
    """Streaming usage: feed a 1 MiB payload as 64 KiB chunks."""
    payload = PAYLOADS["1MiB"]
    chunk = 64 * KIB
    chunks = [payload[i : i + chunk] for i in range(0, len(payload), chunk)]

    def run() -> int:
        value = 0
        for part in chunks:
            value = crc32c(part, value)
        return value

    benchmark(run)


def test_crc32c_many_small_calls(benchmark: BenchmarkFixture) -> None:
    """Per-call overhead, dominated by argument parsing and dispatch."""
    data = PAYLOADS["64B"]

    def run() -> int:
        value = 0
        for _ in range(1000):
            value = crc32c(data, value)
        return value

    benchmark(run)


def test_crc32c_empty(benchmark: BenchmarkFixture) -> None:
    """Lower bound of the call overhead."""
    benchmark(crc32c, b"")
