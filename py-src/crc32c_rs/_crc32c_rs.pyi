import builtins

from _typeshed import ReadableBuffer

__version__: str

class UnsupportedCPUFeatureError(RuntimeError):
    """Raised when the current CPU does not support required instructions."""

# fmt: off
def _crc32c(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        Determined by the implementation selected at runtime.

    Algorithm:
        The implementation is chosen at runtime from the CPU vendor, model, and
        the SIMD instruction sets the CPU reports; if no vectorized
        implementation fits, ``crc32c_fallback`` is used.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_fallback(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        Base instruction set only.

    Algorithm:
        Slice-by-16: sixteen 256-entry tables fold the next 16 bytes with one
        lookup each. The outer loop runs four such blocks, 64 bytes, and
        prefetches 256 bytes ahead; the remaining 8+ bytes go through the same
        tables with a length-indexed stride.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_avx512vl_pclmulqdq_v9s3x4e(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The AVX-512VL and PCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 240 bytes per iteration: nine 16-byte accumulators multiplied by a
        constant with ``pclmulqdq`` and three-way XORed by VPTERNLOGQ, plus
        three scalar accumulators advancing by four 8-byte ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_avx512vl_vpclmulqdq_v3s1_s3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The AVX-512F, AVX-512VL, and VPCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 200 bytes per iteration: three 64-byte accumulators multiplied with
        ``vpclmulqdq`` plus a single 8-byte ``crc32`` step, then drains what is
        left of the input with three scalar accumulators, 24 bytes per
        iteration.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_avx512vl_vpclmulqdq_v3s2x4(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The AVX-512F, AVX-512VL, and VPCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 256 bytes per iteration: three 64-byte accumulators multiplied with
        ``vpclmulqdq`` and two scalar accumulators advancing by four 8-byte
        ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_avx512vl_vpclmulqdq_v4s5x3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The AVX-512F, AVX-512VL, and VPCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 376 bytes per iteration: four 64-byte accumulators multiplied with
        ``vpclmulqdq``, three-way XORed by VPTERNLOGQ, plus five scalar
        accumulators advancing by three 8-byte ``crc32`` steps each. Inputs of
        256-1024 bytes take a separate small-input path.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_sse42(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The SSE4.2 instruction set extension, without PCLMULQDQ.

    Algorithm:
        Three interleaved streams advance 8 bytes per instruction each and are
        merged with precomputed shift tables in rounds of 64, 128, 256, and 8192
        bytes; what is left over is walked 8 bytes, then one byte, at a time.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_sse42_pclmulqdq_v1s3x2(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The SSE4.2 and PCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 64 bytes per iteration: one 16-byte accumulator multiplied with
        ``pclmulqdq`` plus three scalar accumulators advancing by two 8-byte
        ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_sse42_pclmulqdq_v1s3x3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The SSE4.2 and PCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 88 bytes per iteration: one 16-byte accumulator multiplied with
        ``pclmulqdq`` plus three scalar accumulators advancing by three 8-byte
        ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_sse42_pclmulqdq_v1s4x2(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The SSE4.2 and PCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 80 bytes per iteration: one 16-byte accumulator multiplied with
        ``pclmulqdq`` plus four scalar accumulators advancing by two 8-byte
        ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_sse42_pclmulqdq_v7s3x3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The SSE4.2 and PCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 184 bytes per iteration: seven 16-byte accumulators multiplied with
        ``pclmulqdq`` plus three scalar accumulators advancing by three 8-byte
        ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_sse42_pclmulqdq_v8s3x3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The SSE4.2 and PCLMULQDQ instruction set extensions.

    Algorithm:
        Folds 200 bytes per iteration: eight 16-byte accumulators multiplied with
        ``pclmulqdq`` plus three scalar accumulators advancing by three 8-byte
        ``crc32`` steps each.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_aes_crc_v12e_v1(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The CRC and AES instruction set extensions.

    Algorithm:
        Twelve 16-byte accumulators advance 192 bytes per iteration, each
        multiplied with ``pmull`` and folded with the next block in one step, so
        a pointer comparison ends the loop. The tail reduces the twelve
        accumulators to one and continues 16 bytes at a time.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_aes_v3s4x2e_v2(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The CRC and AES instruction set extensions.

    Algorithm:
        Blends both pipes: three 16-byte ``pmull`` accumulators and four scalar
        accumulators advancing by two 8-byte ``crc32`` steps each. It folds
        112 bytes per iteration, 64 of them through the scalar pipe, which is
        more than the vector pipe handles.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """

def _crc32c_aes_sha3_v9s3x2e_s3(
    data: ReadableBuffer,
    value: builtins.int = 0,
    /,
) -> builtins.int:
    """
    Compute the CRC32C checksum of ``data``.

    Requires:
        The CRC, AES, and SHA3 instruction set extensions.

    Algorithm:
        Folds 192 bytes per iteration: nine 16-byte ``pmull`` accumulators plus
        three scalar accumulators advancing by two 8-byte ``crc32`` steps each.
        The three-way XOR is a single ``eor3``.

    Args:
        data: Bytes to checksum.
        value: Checksum of the preceding data, ``0`` for a fresh run.

    Returns:
        The CRC32C checksum of ``data``, continued from ``value``.
    """
