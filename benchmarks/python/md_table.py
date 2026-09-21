import platform
import re
import sys
from dataclasses import dataclass
from pathlib import Path

import cpuinfo
import orjson
from tabulate import tabulate

BENCHMARK_START = "<!-- BEGIN BENCHMARK -->"
BENCHMARK_END = "<!-- END BENCHMARK -->"

TO_GB = 1024

NAME_RE = re.compile(r"^(.*?)\s*\[(.*?)]\s*$")
SIZE_RE = re.compile(r"(\d+(?:\.\d+)?)\s*([BKMGT]?)i?B", re.IGNORECASE)
SIZE_LABEL_RE = re.compile(r"([KMGT])iB", re.IGNORECASE)
BLOCK_RE = re.compile(
    rf"{re.escape(BENCHMARK_START)}.*?{re.escape(BENCHMARK_END)}",
    re.DOTALL,
)

DEFAULT_IMPLS = (
    "fastcrc.crc32.iscsi",
    "google_crc32c.value",
    "crc32c.crc32c",
    "crc32c_rs.crc32c",
)


@dataclass
class Result:
    size: str
    name: str
    time: float
    throughput: float

    @property
    def elapsed(self) -> str:
        return f"{self.time * 1e6:.4f} µs" if self.time else "n/a"  # noqa: RUF001

    @property
    def speed(self) -> str:
        if self.throughput < TO_GB:
            return f"{self.throughput:.1f} MB/s"
        return f"{self.throughput / TO_GB:.1f} GB/s"

    def row(self, fastest: "Result") -> list[str]:
        name = f"`{self.name}`"
        elapsed = self.elapsed
        speed = self.speed
        relative = f"{self.throughput / (fastest.throughput or 1.0):.2f}x"

        if self is fastest:
            name = f"**{name} 🥇**"
            elapsed = f"**{elapsed}**"
            speed = f"**{speed}**"
            relative = f"**{relative}**"

        return [name, elapsed, speed, relative]


@dataclass
class Table:
    title: str
    headers: list[str]
    rows: list[list[str]]

    def render(self) -> list[str]:
        table = tabulate(
            self.rows,
            headers=self.headers,
            tablefmt="github",
            colalign=("left",) * len(self.headers),
            disable_numparse=True,
        )
        return [f"### {self.title}\n", table, ""]


def parse_size_to_bytes(size: str) -> int:
    match = SIZE_RE.fullmatch(size.strip())

    if not match:
        return 0

    power = "BKMGT".index(match.group(2).upper())
    return int(float(match.group(1)) * 1024**power)


def display_size(size: str) -> str:
    return SIZE_LABEL_RE.sub(lambda m: f"{m.group(1).upper()}B", size)


def build_system_info(meta: dict) -> dict[str, str]:
    return {
        "OS": platform.platform(),
        "CPU": cpuinfo.get_cpu_info().get("brand_raw") or platform.processor(),
        "Python": sys.version.split()[0],
        "Timer": meta.get("timer") or "unknown",
    }


def build_results(benchmarks: list[dict], root_meta: dict) -> list[Result]:
    results = []
    for bench in benchmarks:
        meta = (root_meta or {}) | (bench.get("metadata") or {})
        match = NAME_RE.match(meta.get("name", "unknown"))
        name, size = (
            (match.group(1).strip(), match.group(2).strip())
            if match
            else (meta.get("name", "unknown"), "unknown")
        )

        samples = [
            value for run in bench.get("runs", []) for value in run.get("values", [])
        ]
        size_bytes = parse_size_to_bytes(size)

        if not samples or not size_bytes:
            continue

        avg_time = sum(samples) / len(samples)
        throughput = size_bytes / avg_time / 1024**2

        results.append(Result(size=size, name=name, time=avg_time, throughput=throughput))

    return results


def render_system_info(info: dict[str, str]) -> Table:
    rows = [[f"**{key}**", value] for key, value in info.items()]
    return Table("System Information", ["Property", "Value"], rows)


def render_size(size: str, results: list[Result]) -> Table:
    rows = sorted(results, key=lambda r: r.throughput, reverse=True)
    headers = ["Library", "Time", "Throughput", "Relative"]
    return Table(display_size(size), headers, [r.row(rows[0]) for r in rows])


def render_sizes(results: list[Result]) -> list[str]:
    sizes = sorted({r.size for r in results}, key=lambda s: (parse_size_to_bytes(s), s))
    lines: list[str] = []

    for size in sizes:
        lines.extend(render_size(size, [r for r in results if r.size == size]).render())

    return lines


def render_details(title: str, body: list[str], *, opened: bool) -> list[str]:
    return [
        f"<details{' open' if opened else ''}>",
        f"<summary>{title}</summary>",
        "",
        *body,
        "",
        "</details>",
        "",
    ]


def generate_markdown(system_info: dict[str, str], results: list[Result]) -> str:
    default = [r for r in results if r.name in DEFAULT_IMPLS]
    lines = [
        *render_system_info(system_info).render(),
        *render_details("Default benchmark", render_sizes(default), opened=True),
        *render_details("Detailed benchmark", render_sizes(results), opened=False),
    ]
    return "\n".join(lines)


def update_readme(markdown_content: str) -> None:
    readme_path = Path(__file__).with_name("README.md")
    content = readme_path.read_text(encoding="utf-8")
    replacement = f"{BENCHMARK_START}\n{markdown_content.rstrip()}\n{BENCHMARK_END}"
    readme_path.write_text(BLOCK_RE.sub(replacement, content), encoding="utf-8")


def main() -> None:
    json_path = (
        Path(sys.argv[1])
        if len(sys.argv) > 1
        else Path(__file__).with_suffix(".json")
    )  # fmt: skip
    data = orjson.loads(json_path.read_bytes())
    benchmarks = data["benchmarks"]
    root_meta = data.get("metadata", {})
    results = build_results(benchmarks, root_meta)

    if not results:
        print("No data to display.")
        return

    system_info = build_system_info(root_meta)
    update_readme(generate_markdown(system_info, results))
    print("Tables updated in README.md")


if __name__ == "__main__":
    main()
