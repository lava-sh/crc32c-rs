import argparse
import os
import sys
from collections.abc import Callable

import archspec.cpu
import crc32c
import crc32c_rs
import fastcrc
import google_crc32c
import pyperf

KiB = 1024

PAYLOADS = {
    "512 B": 512,
    "1 KiB": KiB,
    "64 KiB": 64 * KiB,
    "512 KiB": 512 * KiB,
    "1 MiB": KiB * KiB,
    "16 MiB": 16 * KiB * KiB,
    "32 MiB": 32 * KiB * KiB,
    "64 MiB": 64 * KiB * KiB,
    "128 MiB": 128 * KiB * KiB,
    "256 MiB": 256 * KiB * KiB,
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


def parse_own_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--impl")
    parser.add_argument("--size")
    args, rest = parser.parse_known_args()
    sys.argv = [sys.argv[0], *rest]
    return args


def main() -> None:
    args = parse_own_args()

    impls = dict(get_impls())
    sizes = dict(PAYLOADS)

    if args.impl is not None:
        if args.impl not in impls:
            msg = f"unknown impl: {args.impl}. Available: {list(impls)}"
            raise SystemExit(msg)
        impls = {args.impl: impls[args.impl]}

    if args.size is not None:
        if args.size not in sizes:
            msg = f"unknown size: {args.size}. Available: {list(sizes)}"
            raise SystemExit(msg)
        sizes = {args.size: sizes[args.size]}

    # check correctness
    expected = 0xE3069283
    for name, fn in impls.items():
        assert fn(b"123456789") == expected, name  # noqa: S101

    runner = pyperf.Runner()

    for size, nbytes in sizes.items():
        data = os.urandom(nbytes)
        for name, fn in impls.items():
            runner.bench_func(f"{name} [{size}]", fn, data)


if __name__ == "__main__":
    main()
