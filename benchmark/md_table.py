import platform
import re
import sys
from dataclasses import dataclass
from pathlib import Path

import orjson
from tabulate import tabulate

BENCHMARK_START = "<!-- BEGIN BENCHMARK -->"
BENCHMARK_END = "<!-- END BENCHMARK -->"

SIZE_MULTIPLIERS = {
    "B": 1,
    "KB": 1000,
    "KIB": 1024,
    "MB": 1000**2,
    "MIB": 1024**2,
    "GB": 1000**3,
    "GIB": 1024**3,
}

NAME_RE = re.compile(r"^(.*?)\s*\[(.*?)]\s*$")
SIZE_RE = re.compile(
    r"(\d+(?:\.\d+)?)\s*(B|KiB|MiB|GiB|KB|MB|GB)?",
    re.IGNORECASE,
)
BLOCK_RE = re.compile(
    rf"{re.escape(BENCHMARK_START)}.*?{re.escape(BENCHMARK_END)}",
    re.DOTALL,
)


@dataclass
class Result:
    size: str
    name: str
    time: float
    throughput: float

    @property
    def elapsed(self) -> str:
        return f"{self.time * 1e6:.4f} µs" if self.time else "n/a"

    @property
    def speed(self) -> str:
        return f"{self.throughput:.1f} MB/s"

    def row(self, fastest: "Result") -> list[str]:
        name = f"`{self.name}`"
        elapsed = self.elapsed
        speed = self.speed
        relative = f"{self.throughput / (fastest.throughput or 1.0):.2f}x"

        if self is fastest:
            name = f"**{name} ⭐**"
            elapsed = f"**{elapsed}**"
            speed = f"**{speed}**"
            relative = f"**{relative}**"

        return [name, elapsed, speed, relative]


def parse_size_to_bytes(size: str) -> int:
    match = SIZE_RE.match(size)
    if not match:
        return 0
    value = float(match.group(1))
    unit = (match.group(2) or "B").upper()
    return int(value * SIZE_MULTIPLIERS.get(unit, 1))


def merged_meta(*layers: dict) -> dict:
    out: dict = {}
    for layer in layers:
        if layer:
            out.update(layer)
    return out


def build_system_info(meta: dict) -> dict[str, str]:
    return {
        "OS": meta.get("platform") or platform.platform(),
        "CPU": meta.get("cpu_model_name") or platform.processor() or platform.machine(),
        "Python": meta.get("python_version") or sys.version.split()[0],
        "Timer": meta.get("timer") or "unknown",
    }


def build_results(benchmarks: list[dict], root_meta: dict) -> list[Result]:
    results = []
    for bench in benchmarks:
        meta = merged_meta(root_meta, bench.get("metadata", {}))
        match = NAME_RE.match(meta.get("name", "unknown"))
        name, size = (
            (match.group(1).strip(), match.group(2).strip())
            if match
            else (meta.get("name", "unknown"), "unknown")
        )

        size_bytes = parse_size_to_bytes(size)
        samples: list[float] = []
        for run in bench.get("runs", []):
            samples.extend(run.get("values", []))

        if not samples or not size_bytes:
            continue

        avg_time = sum(samples) / len(samples)
        throughput = size_bytes / avg_time / 1024 / 1024

        results.append(Result(size=size, name=name, time=avg_time, throughput=throughput))
    return results


def render_system_info(info: dict[str, str]) -> list[str]:
    rows = [[f"**{key}**", value] for key, value in info.items()]
    table = tabulate(
        rows,
        headers=["Property", "Value"],
        tablefmt="github",
        colalign=("left", "left"),
        disable_numparse=True,
    )
    return ["### System Information\n", table, ""]


def render_size(size: str, results: list[Result]) -> list[str]:
    rows = sorted(results, key=lambda r: r.throughput, reverse=True)
    fastest = rows[0]
    table = tabulate(
        [r.row(fastest) for r in rows],
        headers=["Library", "Time", "Throughput", "Relative"],
        tablefmt="github",
        colalign=("left", "left", "left", "left"),
        disable_numparse=True,
    )
    return [f"### {size}\n", table, ""]


def generate_markdown(system_info: dict[str, str], results: list[Result]) -> str:
    sizes = sorted({r.size for r in results}, key=lambda s: (parse_size_to_bytes(s), s))
    lines = render_system_info(system_info)
    for size in sizes:
        lines.extend(render_size(size, [r for r in results if r.size == size]))
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
