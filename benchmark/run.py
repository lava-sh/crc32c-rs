import gc
import os
import platform
import re
import sys
import time
from collections.abc import Callable
from pathlib import Path

import archspec.cpu
import crc32c
import crc32c_rs
import fastcrc
import google_crc32c
import tabulate as tab
from archspec.cpu import Microarchitecture
from markdown_it import MarkdownIt
from tabulate import tabulate

PAYLOADS = {
    "64 B": os.urandom(64),
    "1 KiB": os.urandom(1024),
    "64 KiB": os.urandom(64 * 1024),
    "1 MiB": os.urandom(1024 * 1024),
    "16 MiB": os.urandom(16 * 1024 * 1024),
    "128 MiB": os.urandom(128 * 1024 * 1024),
    "512 MiB": os.urandom(512 * 1024 * 1024),
}

EXPECTED_CRC32C = 0xE3069283
BYTES_PER_KIB = 1024

BENCHMARK_START = "<!-- BEGIN BENCHMARK -->"
BENCHMARK_END = "<!-- END BENCHMARK -->"


def get_available_implementations() -> list[tuple[str, Callable]]:
    host = archspec.cpu.host()
    features = set(host.features)

    impls = [
        ("fastcrc.crc32.iscsi", fastcrc.crc32.iscsi),
        ("crc32c.crc32c", crc32c.crc32c),
        ("google_crc32c.value", google_crc32c.value),
        ("crc32c_rs.fallback", crc32c_rs.crc32c_fallback),
    ]

    if (
            hasattr(crc32c_rs, "crc32c_avx512_vpclmulqdq") and
            {"avx512f", "avx512vl", "vpclmulqdq"}.issubset(features)
    ):  # fmt: skip
        impls.append(("crc32c_rs.avx512_vpclmulqdq", crc32c_rs.crc32c_avx512_vpclmulqdq))

    if (
            hasattr(crc32c_rs, "crc32c_avx512_pclmulqdq") and
            {"avx512vl", "pclmulqdq"}.issubset(features)
    ):  # fmt: skip
        impls.append(("crc32c_rs.avx512_pclmulqdq", crc32c_rs.crc32c_avx512_pclmulqdq))

    if (
            hasattr(crc32c_rs, "crc32c_see42_pclmulqdq") and
            {"sse4_2", "pclmulqdq"}.issubset(features)
    ):  # fmt: skip
        impls.append(("crc32c_rs.sse42_pclmulqdq", crc32c_rs.crc32c_see42_pclmulqdq))

    if (
            hasattr(crc32c_rs, "crc32c_neon64") and
            {"aes", "crc32"}.issubset(features)
    ):  # fmt: skip
        impls.append(("crc32c_rs.neon64", crc32c_rs.crc32c_neon64))

    if (
            hasattr(crc32c_rs, "crc32c_neon64_sha3") and
            {"aes", "crc32", "sha3"}.issubset(features)
    ):  # fmt: skip
        impls.append(("crc32c_rs.neon64_sha3", crc32c_rs.crc32c_neon64_sha3))

    return impls


def check_correctness(impls: list[tuple[str, Callable]]) -> None:
    data = b"123456789"

    for name, fn in impls:
        result = fn(data)
        if result != EXPECTED_CRC32C:
            msg = f"{name}: expected {EXPECTED_CRC32C:08X}, got {result:08X}"
            raise AssertionError(msg)


def benchmark(fn: Callable, data: bytes, warmup: int, iterations: int) -> float:
    for _ in range(warmup):
        fn(data)

    gc_enabled = gc.isenabled()
    gc.disable()

    try:
        start = time.perf_counter_ns()
        for _ in range(iterations):
            fn(data)
        elapsed_ns = time.perf_counter_ns() - start
    finally:
        if gc_enabled:
            gc.enable()

    return elapsed_ns / 1e9


def get_iterations(data_len: int) -> int:
    if data_len <= BYTES_PER_KIB:
        return 10_000
    if data_len <= 64 * BYTES_PER_KIB:
        return 1_000
    if data_len <= BYTES_PER_KIB * BYTES_PER_KIB:
        return 100
    return 10


def get_system_info() -> dict[str, str | Microarchitecture]:
    host = archspec.cpu.host()
    return {
        "OS": f"{platform.system()} {platform.release()} ({platform.version()})",
        "Architecture": platform.machine(),
        "Python": sys.version,
        "CPU": host.name,
        "Vendor": host.vendor,
        "Family": host.family,
    }


def generate_benchmark_markdown(system_info: dict[str, str], results: list[dict]) -> str:
    lines = ["### System Information\n"]

    system_rows = [
        [f"**{key}**", str(system_info[key])]
        for key in ["OS", "Architecture", "Python", "CPU", "Vendor", "Family"]
        if key in system_info
    ]
    lines.extend(
        (
            tabulate(
                system_rows,
                headers=["Property", "Value"],
                tablefmt="github",
                colalign=("left", "left"),
                disable_numparse=True,
            ),
            "",
        ),
    )

    sizes = sorted(
        {r["size"] for r in results},
        key=lambda x: (
            int(re.search(r"\d+", x).group()),
            x.split()[1] if len(x.split()) > 1 else "",
        ),
    )

    for size in sizes:
        lines.append(f"### {size}\n")

        size_results = [r for r in results if r["size"] == size]
        fastest = max(
            size_results,
            key=lambda x: float(re.search(r"[\d.]+", x["throughput"]).group()),
        )

        table_rows = []
        for r in size_results:
            name = f"`{r['name']}`"
            elapsed = r["time"]
            throughput = r["throughput"]
            relative = r["relative"]
            if r["name"] == fastest["name"]:
                name = f"**{name} ⭐**"
                elapsed = f"**{elapsed}**"
                throughput = f"**{throughput}**"
                relative = f"**{relative}**"
            table_rows.append([name, elapsed, throughput, relative])

        previous_min_padding = tab.MIN_PADDING
        previous_wide_chars_mode = tab.WIDE_CHARS_MODE
        tab.MIN_PADDING = 1
        tab.WIDE_CHARS_MODE = True
        try:
            lines.append(
                tabulate(
                    table_rows,
                    headers=["Library", "Time", "Throughput", "Relative"],
                    tablefmt="github",
                    colalign=("left", "left", "left", "left"),
                    disable_numparse=True,
                ),
            )
        finally:
            tab.MIN_PADDING = previous_min_padding
            tab.WIDE_CHARS_MODE = previous_wide_chars_mode
        lines.append("")

    return "\n".join(lines)


def update_readme(markdown_content: str) -> None:
    readme_path = Path(__file__).with_name("README.md")
    content = readme_path.read_text(encoding="utf-8")

    tokens = MarkdownIt().parse(content)
    start_token = next(
        token
        for token in tokens
        if token.type == "html_block" and BENCHMARK_START in token.content
    )
    end_token = next(
        token
        for token in tokens
        if token.type == "html_block" and BENCHMARK_END in token.content
    )

    start_line = start_token.map[0]
    end_line = end_token.map[1]
    lines = content.splitlines(keepends=True)
    replacement = f"{BENCHMARK_START}\n{markdown_content.rstrip()}\n{BENCHMARK_END}\n"
    lines[start_line:end_line] = [replacement]

    readme_path.write_text("".join(lines), encoding="utf-8")


def main() -> None:
    system_info = get_system_info()

    print(f"OS: {system_info['OS']}")
    print(f"Architecture: {system_info['Architecture']}")
    print(f"Python: {system_info['Python']}")
    print(f"CPU: {system_info['CPU']}")
    print(f"Vendor: {system_info['Vendor']}")
    print(f"Family: {system_info['Family']}")
    print()

    impls = get_available_implementations()

    if not impls:
        print("No implementations found!")
        return

    check_correctness(impls)

    library_width = max(len("Library"), *(len(name) for name, _ in impls))
    header = (
        f"{'Size':>8}  "
        f"{'Library':<{library_width}}  "
        f"{'Time':>12}  "
        f"{'Throughput':>14}  "
        f"{'Relative':>10}"
    )
    print(header)
    print("-" * len(header))

    green_underline = "\033[32;4m"
    reset_format = "\033[0m"

    all_results = []

    for size_name, data in PAYLOADS.items():
        iterations = get_iterations(len(data))
        results = []

        for name, fn in impls:
            elapsed = benchmark(fn, data, warmup=10, iterations=iterations)
            total_bytes = len(data) * iterations
            throughput = total_bytes / elapsed / 1024 / 1024
            results.append((name, elapsed, throughput))

        fastest = max(throughput for _, _, throughput in results)

        for name, elapsed, throughput in results:
            relative = throughput / fastest
            line = (
                f"{size_name:>8}  "
                f"{name:<{library_width}}  "
                f"{elapsed:>9.4f} s  "
                f"{throughput:>10.1f} MB/s  "
                f"{relative:>8.2f}x"
            )
            if throughput == fastest:
                line = f"{green_underline}{line}{reset_format}"
            print(line)

            all_results.append(
                {
                    "size": size_name,
                    "name": name,
                    "time": f"{elapsed:.4f} s",
                    "throughput": f"{throughput:.1f} MB/s",
                    "relative": f"{relative:.2f}x",
                },
            )
        print()

    markdown = generate_benchmark_markdown(system_info, all_results)
    update_readme(markdown)
    print("\nBenchmark results updated in benchmark/README.md")


if __name__ == "__main__":
    main()
