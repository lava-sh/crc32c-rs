import os
from collections.abc import Callable

import archspec.cpu
import crc32c
import crc32c_rs
import fastcrc
import google_crc32c
import pyperf

KiB = 1024

PAYLOADS = {
    "512 B": os.urandom(512),
    "1 KiB": os.urandom(KiB),
    "64 KiB": os.urandom(64 * KiB),
    "512 KiB": os.urandom(512 * KiB),
    "1 MiB": os.urandom(KiB * KiB),
    "16 MiB": os.urandom(16 * KiB * KiB),
    "32 MiB": os.urandom(32 * KiB * KiB),
    "64 MiB": os.urandom(64 * KiB * KiB),
    "128 MiB": os.urandom(128 * KiB * KiB),
    "256 MiB": os.urandom(256 * KiB * KiB),
}


def get_impls() -> list[tuple[str, Callable]]:
    features = set(archspec.cpu.host().features)

    impls = [
        ("fastcrc.crc32.iscsi", fastcrc.crc32.iscsi),
        ("google_crc32c.value", google_crc32c.value),
        ("crc32c.crc32c", crc32c.crc32c),
        ("crc32c_rs.crc32c", crc32c_rs.crc32c),
        ("crc32c_rs.crc32c_fallback", crc32c_rs.crc32c_fallback),
    ]

    configs = [
        ({"sse4_2", "pclmulqdq"}, [
            "crc32c_sse42_pclmulqdq_v1s3x2",
            "crc32c_sse42_pclmulqdq_v1s3x3",
            "crc32c_sse42_pclmulqdq_v1s4x2",
            "crc32c_sse42_pclmulqdq_v7s3x3",
            "crc32c_sse42_pclmulqdq_v8s3x3",
        ]),
        ({"avx512f", "avx512vl", "vpclmulqdq"}, [
            "crc32c_avx512vl_vpclmulqdq_v3s1_s3",
            "crc32c_avx512vl_vpclmulqdq_v3s2x4",
            "crc32c_avx512vl_vpclmulqdq_v4s5x3",
        ]),
        ({"avx512vl", "pclmulqdq"}, [
            "crc32c_avx512vl_pclmulqdq_v9s3x4e",
        ]),
        ({"aes", "crc32"}, [
            "crc32c_aes_crc_v12e_v1",
            "crc32c_aes_v3s4x2e_v2",
        ]),
        ({"aes", "crc32", "sha3"}, [
            "crc32c_aes_sha3_v9s3x2e_s3",
        ]),
    ]  # fmt: skip

    for required, attrs in configs:
        if required.issubset(features):
            impls.extend(
               (attr, getattr(crc32c_rs, attr))
               for attr in attrs
               if hasattr(crc32c_rs, attr)
           )

    return impls


def main() -> None:
    impls = get_impls()

    # check correctness
    for _, fn in impls:
        result = fn(b"123456789")
        expected = 0xE3069283
        assert result == expected  # noqa: S101

    runner = pyperf.Runner(
        processes=5,
        warmups=2,
        values=25,
    )

    for size, data in PAYLOADS.items():
        for name, fn in impls:
            runner.bench_func(f"{name} [{size}]", fn, data)


if __name__ == "__main__":
    main()
