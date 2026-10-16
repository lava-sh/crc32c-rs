import logging
import platform
from collections.abc import Callable

import cpuinfo
import crc32c_rs
import pytest
from crc32c_rs import UnsupportedCPUFeatureError

logger = logging.getLogger(__name__)


@pytest.fixture(scope="session")
def crc_impl() -> list[tuple[str, Callable[..., int]]]:
    cpu_info = cpuinfo.get_cpu_info()
    logger.info("CPU: %s", cpu_info.get("brand_raw") or platform.processor())
    logger.info("Features: %s", " ".join(sorted(cpu_info.get("flags", []))))
    logger.info("")

    impls = []

    for attr in dir(crc32c_rs):
        if not attr.startswith("crc32c"):
            continue
        implementation = getattr(crc32c_rs, attr)
        try:
            implementation(b"abc")
        except UnsupportedCPUFeatureError:
            continue
        impls.append((attr, implementation))
        logger.info("✅ crc32c_rs.%s available", attr)

    logger.info("")
    logger.info(
        "Implementations: %s",
        ", ".join(f"crc32c_rs.{name}" for name, _ in impls),
    )
    return impls
