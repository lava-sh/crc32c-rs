import builtins
import sys
from typing import TypeAlias

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

ReadableBuffer: TypeAlias = Buffer

__version__: str

class UnsupportedCPUFeatureError(RuntimeError):
    """Raised when the current CPU does not support required instructions."""


def _crc32c(data: ReadableBuffer, value: builtins.int = 0, /) -> builtins.int:
    """Compute CRC32C using the fastest supported runtime-selected backend."""


def _crc32c_fallback(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """Compute CRC32C with the portable slicing-by-16 fallback."""


def _crc32c_avx512vl_pclmulqdq_v9s3x4e(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use AVX-512VL plus PCLMULQDQ with v9s3x4e scheduling.

    The suffix means 9 vector accumulators, 3 scalar accumulators, 4
    vector loads per accumulator, and pointer-based loop termination (``e``).
    """


def _crc32c_avx512vl_vpclmulqdq_v3s1_s3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use AVX-512VL plus VPCLMULQDQ with v3s1_s3 scheduling.

    ``v3s1_s3`` means 3 vector accumulators, 1 scalar accumulator, 3
    scalar loads per loop, followed by the scalar tail schedule.
    """


def _crc32c_avx512vl_vpclmulqdq_v3s2x4(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use AVX-512VL plus VPCLMULQDQ with v3s2x4 scheduling.

    ``v3s2x4`` means 3 vector accumulators and 2 scalar accumulators;
    ``x4`` selects 4 vector loads per vector accumulator each iteration.
    """


def _crc32c_avx512vl_vpclmulqdq_v4s5x3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use AVX-512VL plus VPCLMULQDQ with v4s5x3 scheduling.

    ``v4s5x3`` means 4 vector accumulators, 5 scalar accumulators, and
    3 vector loads per vector accumulator each iteration.
    """


def _crc32c_sse42_pclmulqdq_v1s3x2(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use SSE4.2 plus PCLMULQDQ with the AMD v1s3x2 schedule."""


def _crc32c_sse42_pclmulqdq_v1s3x3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use SSE4.2 plus PCLMULQDQ with the AMD v1s3x3 schedule."""


def _crc32c_sse42_pclmulqdq_v1s4x2(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use SSE4.2 plus PCLMULQDQ with the AMD v1s4x2 schedule."""


def _crc32c_sse42_pclmulqdq_v7s3x3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use SSE4.2 plus PCLMULQDQ with the v7s3x3 schedule."""


def _crc32c_sse42_pclmulqdq_v8s3x3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use SSE4.2 plus PCLMULQDQ with the v8s3x3 schedule."""


def _crc32c_sse42_s3k4096e(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use the SSE4.2 3-way scalar schedule with a 4096-byte outer block.

    ``s3`` is 3 scalar accumulators, ``k4096`` is the outer block size,
    and ``e`` means pointer-based inner-loop termination.
    """


def _crc32c_aes_crc_v12e_v1(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use ARM CRC plus AES/PMULL with the Apple v12e_v1 schedule.

    ``v12`` is 12 vector accumulators, ``e`` means pointer-based loop
    termination, and ``v1`` is the 1-vector tail schedule.
    """


def _crc32c_aes_v3s4x2e_v2(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use ARM CRC plus AES/PMULL with the v3s4x2e_v2 schedule.

    ``v3s4x2`` means 3 vector accumulators, 4 scalar accumulators, and
    2 vector loads per accumulator; ``e`` is pointer-based termination and
    ``v2`` is the 2-vector tail schedule.
    """


def _crc32c_aes_sha3_v9s3x2e_s3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use ARM CRC, AES/PMULL, and SHA3/EOR3 with v9s3x2e_s3 scheduling."""


def _crc32c_crc_neon_s3k95760_s3(
    data: ReadableBuffer, value: builtins.int = 0, /
) -> builtins.int:
    """Use ARM CRC plus PMULL with a 3-way 95,760-byte schedule."""


def crc32c(data: ReadableBuffer, value: builtins.int = 0, /) -> builtins.int: ...
def crc32c_fallback(data: ReadableBuffer, value: builtins.int = 0, /) -> builtins.int: ...
