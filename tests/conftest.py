import logging
from collections.abc import Callable

import archspec.cpu
import pytest
from crc32c_rs import crc32c_fallback

from .types import CrcFn

logger = logging.getLogger(__name__)

crc32c_avx512_vpclmulqdq: CrcFn | None = None
crc32c_avx512_pclmulqdq: CrcFn | None = None
crc32c_see42_pclmulqdq: CrcFn | None = None

try:
    from crc32c_rs import (
        crc32c_avx512_pclmulqdq,
        crc32c_avx512_vpclmulqdq,
        crc32c_see42_pclmulqdq,
    )
except ImportError:
    pass


@pytest.fixture(scope="session")
def crc_impl() -> list[tuple[str, Callable[..., int]]]:
    host = archspec.cpu.host()

    logger.info("CPU: %s", host.name)
    logger.info("Vendor: %s", host.vendor)
    logger.info("Family: %s", host.family)
    logger.info("Microarchitecture: %s", getattr(host, "microarchitecture", "unknown"))
    logger.info("Features: %s", " ".join(sorted(host.features)))

    impls = []

    if crc32c_avx512_vpclmulqdq is not None:
        impls.append(("avx512_vpclmulqdq", crc32c_avx512_vpclmulqdq))
        logger.info("✅ crc32c_rs.avx512_vpclmulqdq available")

    if crc32c_avx512_pclmulqdq is not None:
        impls.append(("avx512_pclmulqdq", crc32c_avx512_pclmulqdq))
        logger.info("✅ crc32c_rs.avx512_pclmulqdq available")

    if crc32c_see42_pclmulqdq is not None:
        impls.append(("sse42_pclmulqdq", crc32c_see42_pclmulqdq))
        logger.info("✅ crc32c_rs.sse42_pclmulqdq available")

    impls.append(("fallback", crc32c_fallback))
    return impls
