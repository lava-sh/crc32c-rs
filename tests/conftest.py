import logging
from collections.abc import Callable

import archspec.cpu
import pytest
from crc32c_rs import crc32c_fallback

logger = logging.getLogger(__name__)


def _import(module_name: str, pattern: str) -> dict[str, Callable]:
    impls = {}
    try:
        module = __import__(module_name, fromlist=["*"])
        impls = {
            name: getattr(module, name)
            for name in dir(module)
            if name.startswith(pattern)
        }
    except ImportError:
        pass
    return impls


@pytest.fixture(scope="session")
def crc_impl() -> list[tuple[str, Callable[..., int]]]:
    host = archspec.cpu.host()
    features = set(host.features)

    logger.info("CPU: %s", host.name)
    logger.info("Vendor: %s", host.vendor)
    logger.info("Family: %s", host.family)
    logger.info("Features: %s", " ".join(sorted(features)))
    logger.info("")

    impls_ = _import("crc32c_rs", "crc32c_")

    requirements = {
        "crc32c_avx512vl_vpclmulqdq_v3s1_s3": {"avx512f", "avx512vl", "vpclmulqdq"},
        "crc32c_avx512vl_vpclmulqdq_v3s2x4": {"avx512f", "avx512vl", "vpclmulqdq"},
        "crc32c_avx512vl_vpclmulqdq_v4s5x3": {"avx512f", "avx512vl", "vpclmulqdq"},
        "crc32c_avx512vl_pclmulqdq_v9s3x4e": {"avx512vl", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v1s3x2": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v1s3x3": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v1s4x2": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v7s3x3": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v8s3x3": {"sse4_2", "pclmulqdq"},
        "crc32c_aes_crc_v12e_v1": {"aes", "crc32"},
        "crc32c_aes_v3s4x2e_v2": {"aes", "crc32"},
        "crc32c_aes_sha3_v9s3x2e_s3": {"aes", "crc32", "sha3"},
    }

    impls = []

    for name, required in requirements.items():
        implementation = impls_.get(name)
        if implementation is not None and required.issubset(features):
            impls.append((name, implementation))
            logger.info("crc32c_rs.%s available", name)

    impls.append(("crc32c_fallback", crc32c_fallback))
    logger.info("")
    logger.info(
        "Implementations: %s",
        ", ".join(f"crc32c_rs.{name}" for name, _ in impls),
    )
    return impls
