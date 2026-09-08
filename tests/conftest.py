import logging
from collections.abc import Callable

import archspec.cpu
import pytest
from crc32c_rs import crc32c_fallback

from .types import CrcFn

logger = logging.getLogger(__name__)

x86_impls: dict[str, CrcFn] = {}
arm_impls: dict[str, CrcFn] = {}

try:
    from crc32c_rs import (
        crc32c_avx512vl_pclmulqdq_v9s3x4e,
        crc32c_avx512vl_vpclmulqdq_v3s1_s3,
        crc32c_avx512vl_vpclmulqdq_v3s2x4,
        crc32c_avx512vl_vpclmulqdq_v4s5x3,
        crc32c_sse42_pclmulqdq_v1s3x2,
        crc32c_sse42_pclmulqdq_v1s3x3,
        crc32c_sse42_pclmulqdq_v1s4x2,
        crc32c_sse42_pclmulqdq_v7s3x3,
        crc32c_sse42_pclmulqdq_v8s3x3,
    )
except ImportError:
    pass
else:
    x86_impls = {
        "avx512vl_pclmulqdq_v9s3x4e": crc32c_avx512vl_pclmulqdq_v9s3x4e,
        "avx512vl_vpclmulqdq_v3s1_s3": crc32c_avx512vl_vpclmulqdq_v3s1_s3,
        "avx512vl_vpclmulqdq_v3s2x4": crc32c_avx512vl_vpclmulqdq_v3s2x4,
        "avx512vl_vpclmulqdq_v4s5x3": crc32c_avx512vl_vpclmulqdq_v4s5x3,
        "sse42_pclmulqdq_v1s3x2": crc32c_sse42_pclmulqdq_v1s3x2,
        "sse42_pclmulqdq_v1s3x3": crc32c_sse42_pclmulqdq_v1s3x3,
        "sse42_pclmulqdq_v1s4x2": crc32c_sse42_pclmulqdq_v1s4x2,
        "sse42_pclmulqdq_v7s3x3": crc32c_sse42_pclmulqdq_v7s3x3,
        "sse42_pclmulqdq_v8s3x3": crc32c_sse42_pclmulqdq_v8s3x3,
    }

try:
    from crc32c_rs import (
        crc32c_aes_crc_v12e_v1,
        crc32c_aes_sha3_v9s3x2e_s3,
        crc32c_aes_v3s4x2e_v2,
    )
except ImportError:
    pass
else:
    arm_impls = {
        "aes_crc_v12e_v1": crc32c_aes_crc_v12e_v1,
        "aes_v3s4x2e_v2": crc32c_aes_v3s4x2e_v2,
        "aes_sha3_v9s3x2e_s3": crc32c_aes_sha3_v9s3x2e_s3,
    }


@pytest.fixture(scope="session")
def crc_impl() -> list[tuple[str, Callable[..., int]]]:
    host = archspec.cpu.host()
    features = set(host.features)

    logger.info("CPU: %s", host.name)
    logger.info("Vendor: %s", host.vendor)
    logger.info("Family: %s", host.family)
    logger.info("Features: %s", " ".join(sorted(features)))
    logger.info("")

    impls: list[tuple[str, CrcFn]] = []

    x86_requirements = {
        "avx512vl_vpclmulqdq_v3s1_s3": {"avx512f", "avx512vl", "vpclmulqdq"},
        "avx512vl_vpclmulqdq_v3s2x4": {"avx512f", "avx512vl", "vpclmulqdq"},
        "avx512vl_vpclmulqdq_v4s5x3": {"avx512f", "avx512vl", "vpclmulqdq"},
        "avx512vl_pclmulqdq_v9s3x4e": {"avx512vl", "pclmulqdq"},
        "sse42_pclmulqdq_v1s3x2": {"sse4_2", "pclmulqdq"},
        "sse42_pclmulqdq_v1s3x3": {"sse4_2", "pclmulqdq"},
        "sse42_pclmulqdq_v1s4x2": {"sse4_2", "pclmulqdq"},
        "sse42_pclmulqdq_v7s3x3": {"sse4_2", "pclmulqdq"},
        "sse42_pclmulqdq_v8s3x3": {"sse4_2", "pclmulqdq"},
    }
    for name, required in x86_requirements.items():
        implementation = x86_impls.get(name)
        if implementation is not None and required.issubset(features):
            impls.append((name, implementation))
            logger.info("crc32c_rs.%s available", name)

    arm_requirements = {name: {"aes", "crc32"} for name in arm_impls}
    for name, required in arm_requirements.items():
        implementation = arm_impls.get(name)
        if implementation is not None and required.issubset(features):
            impls.append((name, implementation))
            logger.info("crc32c_rs.%s available", name)

    impls.append(("fallback", crc32c_fallback))
    logger.info("")
    logger.info(
        "Implementations: %s",
        ", ".join(f"crc32c_rs.{name}" for name, _ in impls),
    )
    return impls
