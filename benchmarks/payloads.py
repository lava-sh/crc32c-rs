__all__ = (
    "KIB",
    "MIB",
    "PAYLOADS",
    "SIZES",
)

import random

KIB = 1024
MIB = 1024 * KIB

# The sweep deliberately crosses every threshold that changes the shape of the
# work: payloads dominated by the Python call overhead (64 B, 512 B), the 32 KiB
# mark above which `crc32c_rs` detaches the GIL, sizes that still fit in L2/L3
# (512 KiB, 1 MiB) and sizes large enough that the SIMD kernel is bound by main
# memory bandwidth (16 MiB and up).
SIZES = {
    "64B": 64,
    "512B": 512,
    "1KiB": KIB,
    "64KiB": 64 * KIB,
    "512KiB": 512 * KIB,
    "1MiB": MIB,
    "16MiB": 16 * MIB,
    "32MiB": 32 * MIB,
    "64MiB": 64 * MIB,
    "128MiB": 128 * MIB,
    "256MiB": 256 * MIB,
}

# A fixed seed keeps the payloads identical from one run to the next, so that
# two CodSpeed runs only differ by the code being measured.
_rng = random.Random(0xC32C)

# CRC32C runs in constant time per byte, so the multi-megabyte payloads are
# tiled from a single random block instead of being drawn byte by byte: this
# keeps collection fast and avoids the multi-hundred-megabyte integer that
# `random.randbytes` would otherwise materialise for them.
_BLOCK = _rng.randbytes(MIB)


def _payload(size: int) -> bytes:
    if size <= MIB:
        return _BLOCK[:size]

    return _BLOCK * (size // MIB)


PAYLOADS: dict[str, bytes] = {name: _payload(size) for name, size in SIZES.items()}
