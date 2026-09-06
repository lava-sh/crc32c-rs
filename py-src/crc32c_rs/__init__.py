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
        _crc32c_avx512_pclmulqdq as crc32c_avx512_pclmulqdq,
        _crc32c_avx512_vpclmulqdq as crc32c_avx512_vpclmulqdq,
        _crc32c_see42_pclmulqdq as crc32c_see42_pclmulqdq,
    )
except ImportError:
    pass
else:
    __all__ += (  # type: ignore[assignment]
        "crc32c_avx512_pclmulqdq",
        "crc32c_avx512_vpclmulqdq",
        "crc32c_see42_pclmulqdq",
    )

try:  # noqa: RUF067
    from ._crc32c_rs import (
        _crc32c_neon64 as crc32c_neon64,
        _crc32c_neon64_sha3 as crc32c_neon64_sha3,
    )
except ImportError:
    pass
else:
    __all__ += (  # type: ignore[assignment]
        "crc32c_neon64",
        "crc32c_neon64_sha3",
    )
