import logging
import platform
from collections.abc import Callable

import cpuinfo
import crc32c_rs
import pytest

logger = logging.getLogger(__name__)


@pytest.fixture(scope="session")
def crc_impl() -> list[tuple[str, Callable[..., int]]]:
    cpu_info = cpuinfo.get_cpu_info()
    logger.info("CPU: %s", cpu_info.get("brand_raw") or platform.processor())
    logger.info("")

    impls = []

    for attr in dir(crc32c_rs):
        if not attr.startswith("crc32c"):
            continue
        fn = getattr(crc32c_rs, attr)
        try:
            fn(b"abc")
        except crc32c_rs.UnsupportedCPUFeatureError:
            continue
        impls.append((attr, fn))
        logger.info("✅ crc32c_rs.%s available", attr)

    logger.info("")
    logger.info(
        "Implementations: %s",
        ", ".join(f"crc32c_rs.{name}" for name, _ in impls),
    )
    return impls
