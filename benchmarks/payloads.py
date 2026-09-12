__all__ = (
    "KIB",
    "MIB",
    "PAYLOADS",
    "SIZES",
)

import random

KIB = 1024
MIB = 1024 * KIB

# `crc32c_rs` only detaches the GIL for payloads of at least 32 KiB, so the
# sizes below deliberately cover both sides of that threshold, from a payload
# that is dominated by the Python call overhead (64 B) up to one where the SIMD
# kernel dominates (1 MiB).
SIZES = {
    "64B": 64,
    "1KiB": KIB,
    "64KiB": 64 * KIB,
    "1MiB": MIB,
}

# A fixed seed keeps the payloads identical from one run to the next, so that
# two CodSpeed runs only differ by the code being measured.
_rng = random.Random(0xC32C)

PAYLOADS: dict[str, bytes] = {name: _rng.randbytes(size) for name, size in SIZES.items()}
