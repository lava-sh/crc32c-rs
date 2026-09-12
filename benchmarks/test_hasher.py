import pytest
from crc32c_rs import Hasher
from pytest_codspeed import BenchmarkFixture

from .payloads import KIB, PAYLOADS, SIZES

SIZE_IDS = list(SIZES)


@pytest.mark.parametrize("size", SIZE_IDS)
def test_hasher_init(benchmark: BenchmarkFixture, size: str) -> None:
    data = PAYLOADS[size]
    benchmark(Hasher, data)


@pytest.mark.parametrize("chunk_size", [KIB, 64 * KIB], ids=["1KiB", "64KiB"])
def test_hasher_update(benchmark: BenchmarkFixture, chunk_size: int) -> None:
    """Incremental hashing of a 1 MiB payload, chunk by chunk."""
    payload = PAYLOADS["1MiB"]
    chunks = [payload[i : i + chunk_size] for i in range(0, len(payload), chunk_size)]

    def run() -> int:
        hasher = Hasher()
        for chunk in chunks:
            hasher.update(chunk)
        return hasher.checksum

    benchmark(run)


def test_hasher_digest(benchmark: BenchmarkFixture) -> None:
    hasher = Hasher(PAYLOADS["1KiB"])
    benchmark(hasher.digest)


def test_hasher_hexdigest(benchmark: BenchmarkFixture) -> None:
    hasher = Hasher(PAYLOADS["1KiB"])
    benchmark(hasher.hexdigest)


def test_hasher_copy(benchmark: BenchmarkFixture) -> None:
    hasher = Hasher(PAYLOADS["1KiB"])
    benchmark(hasher.copy)
