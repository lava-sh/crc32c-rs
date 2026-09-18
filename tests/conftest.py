import logging
from collections.abc import Callable
from pathlib import Path

import archspec.cpu
import pytest
from crc32c_rs import crc32c_fallback

logger = logging.getLogger(__name__)

# archspec has no vocabulary for the SVE2 crypto extensions, so a kernel that
# needs them has to be checked against the kernel's own feature list instead.
HWCAP_REQUIREMENTS = {
    "crc32c_sve2_eor3_v9s3x2e_s3": {"sveaes", "svepmull"},
}


def _hwcaps() -> set[str]:
    """Feature names from /proc/cpuinfo, empty when unavailable."""
    try:
        cpuinfo = Path("/proc/cpuinfo").read_text(encoding="utf-8")
    except OSError:
        return set()

    for line in cpuinfo.splitlines():
        if line.startswith("Features"):
            return set(line.partition(":")[2].split())
    return set()


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
    hwcaps = _hwcaps()

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
        "crc32c_sse42": {"sse4_2"},
        "crc32c_sse42_pclmulqdq_v1s3x2": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v1s3x3": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v1s4x2": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v7s3x3": {"sse4_2", "pclmulqdq"},
        "crc32c_sse42_pclmulqdq_v8s3x3": {"sse4_2", "pclmulqdq"},
        "crc32c_aes_crc_v12e_v1": {"aes", "crc32"},
        "crc32c_aes_v3s4x2e_v2": {"aes", "crc32"},
        "crc32c_aes_sha3_v9s3x2e_s3": {"aes", "crc32", "sha3"},
        "crc32c_sve2_eor3_v9s3x2e_s3": {"aes", "crc32", "sve2"},
    }

    impls = []

    for name, required in requirements.items():
        implementation = impls_.get(name)
        if implementation is None or not required.issubset(features):
            continue

        missing = HWCAP_REQUIREMENTS.get(name, set()) - hwcaps
        if missing:
            logger.info(
                "⏭️ crc32c_rs.%s skipped, CPU lacks %s",
                name,
                ", ".join(sorted(missing)),
            )
            continue

        impls.append((name, implementation))
        logger.info("✅ crc32c_rs.%s available", name)

    impls.append(("crc32c_fallback", crc32c_fallback))
    logger.info("")
    logger.info(
        "Implementations: %s",
        ", ".join(f"crc32c_rs.{name}" for name, _ in impls),
    )
    return impls
