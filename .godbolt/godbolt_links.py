# /// script
# dependencies = [
#   "zapros == 0.19.0",
# ]
# ///

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from zapros import Client, RetryMiddleware, StdNetworkHandler

ROOT = Path(__file__).resolve().parent
SOURCE_NAMES = ("file.c", "file.cc")

BASE_URL = "https://godbolt.org/"
LANGUAGES = {
    ".c": "c",
    ".cc": "c++",
    ".rs": "rust",
}  # fmt: skip
HEADER_LINES = 2
FILTERS = {
    "labels": True,
    "libraryCode": True,
    "directives": True,
    "commentOnly": True,
    "trim": False,
    "debugCalls": False,
    "intel": True,
}
FILTERS_BY_LANGUAGE = {
    "rust": FILTERS,
    "c": FILTERS,
    "c++": {**FILTERS, "demangle": True, "verboseDemangling": True},
}


@dataclass(frozen=True)
class Source:
    path: Path
    language: str
    compiler_name: str
    options: str
    code: str


def parse_source(path: Path) -> Source:
    code = path.read_text(encoding="utf-8")
    headers = code.splitlines()[:HEADER_LINES]
    if (
            len(headers) < HEADER_LINES or
            not all(header.startswith("//") for header in headers)
    ):  # fmt: skip
        msg = f"{path}: expected the compiler name and the flags in the first two lines"
        raise ValueError(msg)

    language = LANGUAGES.get(path.suffix.lower())
    if language is None:
        msg = f"{path}: unsupported source extension {path.suffix!r}"
        raise ValueError(msg)

    compiler_name, options = (header[2:].strip() for header in headers)
    return Source(path, language, compiler_name, options, code)


def compiler_catalog(client: Client, language: str) -> dict[str, str]:
    response = client.get(f"api/compilers/{language}")
    response.raise_for_status()

    catalog = {}
    for line in response.text.splitlines()[1:]:
        compiler_id, separator, display_name = line.partition("|")
        if separator:
            catalog[display_name.strip().casefold()] = compiler_id.strip()
    return catalog


def resolve_compiler(source: Source, catalog: dict[str, str]) -> str:
    compiler_name = source.compiler_name.casefold()
    compiler_id = catalog.get(compiler_name)
    if compiler_id is None:
        available = sorted(name for name in catalog if compiler_name in name)
        hint = f" Similar names: {', '.join(available[:5])}." if available else ""
        msg = (
            f"{source.path}: compiler {source.compiler_name!r} was not found for "
            f"language {source.language!r}.{hint}"
        )
        raise ValueError(msg)
    return compiler_id


def make_config(sources: list[Source], compiler_ids: list[str]) -> dict[str, Any]:
    editors = []
    compilers = []

    for editor_id, (source, compiler_id) in enumerate(
        zip(sources, compiler_ids, strict=True),
        start=1,
    ):
        editors.append({
            "type": "component",
            "componentName": "codeEditor",
            "componentState": {
                "id": editor_id,
                "source": source.code,
                "options": {"compileOnChange": True},
                "lang": source.language,
            },
        })  # fmt: skip
        compilers.append({
            "type": "component",
            "componentName": "compiler",
            "componentState": {
                "source": editor_id,
                "compiler": compiler_id,
                "lang": source.language,
                "filters": FILTERS_BY_LANGUAGE[source.language],
                "options": source.options,
            },
        })  # fmt: skip

    return {
        "version": 4,
        "content": [{
            "type": "row",
            "content": [
                {"type": "column", "content": editors},
                {"type": "column", "content": compilers},
            ],
            }],
     }  # fmt: skip


def create_short_link(client: Client, config: dict[str, Any]) -> str:
    response = client.post("api/shortener", json={"config": config})
    response.raise_for_status()

    payload = response.json
    url = payload.get("url")
    if not isinstance(url, str) or not url:
        msg = f"Compiler Explorer returned an unexpected response: {payload!r}"
        raise ValueError(msg)
    return url if url.startswith("http") else BASE_URL + url.lstrip("/")


def find_pairs(root: Path) -> list[tuple[Path, Path]]:
    dirs = {
        path.parent for name in (*SOURCE_NAMES, "file.rs") for path in root.rglob(name)
    }
    pairs = []

    for directory in sorted(dirs):
        sources = [directory / name for name in SOURCE_NAMES]
        sources = [path for path in sources if path.is_file()]
        rust_path = directory / "file.rs"
        if len(sources) != 1 or not rust_path.is_file():
            msg = f"{directory}: expected one of {SOURCE_NAMES} next to file.rs"
            raise ValueError(msg)
        pairs.append((sources[0], rust_path))

    return pairs


def main() -> None:
    pairs = find_pairs(ROOT)
    if not pairs:
        msg = f"No source pairs found in {ROOT}"
        raise SystemExit(msg)

    parsed_pairs = [[parse_source(rust), parse_source(source)] for source, rust in pairs]
    languages = {source.language for pair in parsed_pairs for source in pair}

    handler = RetryMiddleware(StdNetworkHandler(), max_attempts=3)
    with Client(handler=handler, base_url=BASE_URL) as client:
        catalogs = {
            language: compiler_catalog(client, language) for language in languages
        }

        for sources in parsed_pairs:
            compiler_ids = [
                resolve_compiler(source, catalogs[source.language]) for source in sources
            ]
            link = create_short_link(client, make_config(sources, compiler_ids))
            directory = sources[0].path.parent.relative_to(ROOT).as_posix()
            print(f"{directory}: {link}")


if __name__ == "__main__":
    main()
