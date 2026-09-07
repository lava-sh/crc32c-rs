__all__ = (
    "UnsupportedCPUFeatureError",
    "__version__",
    "crc32c",
    "crc32c_fallback",
)

from ._crc32c_rs import (
    UnsupportedCPUFeatureError,
    __version__,
    _crc32c as crc32c,
    _crc32c_fallback as crc32c_fallback,
)

try:  # noqa: RUF067
    from ._crc32c_rs import (
        _crc32c_avx512vl_pclmulqdq_v9s3x4e as crc32c_avx512vl_pclmulqdq_v9s3x4e,
        _crc32c_avx512vl_vpclmulqdq_v3s1_s3 as crc32c_avx512vl_vpclmulqdq_v3s1_s3,
        _crc32c_avx512vl_vpclmulqdq_v3s2x4 as crc32c_avx512vl_vpclmulqdq_v3s2x4,
        _crc32c_avx512vl_vpclmulqdq_v4s5x3 as crc32c_avx512vl_vpclmulqdq_v4s5x3,
        _crc32c_sse42_pclmulqdq_v1s3x2 as crc32c_sse42_pclmulqdq_v1s3x2,
        _crc32c_sse42_pclmulqdq_v1s3x3 as crc32c_sse42_pclmulqdq_v1s3x3,
        _crc32c_sse42_pclmulqdq_v1s4x2 as crc32c_sse42_pclmulqdq_v1s4x2,
        _crc32c_sse42_pclmulqdq_v7s3x3 as crc32c_sse42_pclmulqdq_v7s3x3,
        _crc32c_sse42_pclmulqdq_v8s3x3 as crc32c_sse42_pclmulqdq_v8s3x3,
        _crc32c_sse42_s3k4096e as crc32c_sse42_s3k4096e,
    )
except ImportError:
    pass
else:
    __all__ += (
        "crc32c_avx512vl_pclmulqdq_v9s3x4e",
        "crc32c_avx512vl_vpclmulqdq_v3s1_s3",
        "crc32c_avx512vl_vpclmulqdq_v3s2x4",
        "crc32c_avx512vl_vpclmulqdq_v4s5x3",
        "crc32c_sse42_pclmulqdq_v1s3x2",
        "crc32c_sse42_pclmulqdq_v1s3x3",
        "crc32c_sse42_pclmulqdq_v1s4x2",
        "crc32c_sse42_pclmulqdq_v7s3x3",
        "crc32c_sse42_pclmulqdq_v8s3x3",
        "crc32c_sse42_s3k4096e",
    )

try:  # noqa: RUF067
    from ._crc32c_rs import (
        _crc32c_aes_crc_v12e_v1 as crc32c_aes_crc_v12e_v1,
        _crc32c_aes_sha3_v9s3x2e_s3 as crc32c_aes_sha3_v9s3x2e_s3,
        _crc32c_aes_v3s4x2e_v2 as crc32c_aes_v3s4x2e_v2,
        _crc32c_crc_neon_s3k95760_s3 as crc32c_crc_neon_s3k95760_s3,
    )
except ImportError:
    pass
else:
    __all__ += (
        "crc32c_aes_crc_v12e_v1",
        "crc32c_aes_sha3_v9s3x2e_s3",
        "crc32c_aes_v3s4x2e_v2",
        "crc32c_crc_neon_s3k95760_s3",
    )
