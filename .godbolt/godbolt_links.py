# /// script
# dependencies = [
#   "zapros == 0.17.0",
# ]
# ///

from dataclasses import dataclass
from http import HTTPStatus
from pathlib import Path
from typing import Any

from zapros import Client

API_ROOT = "https://godbolt.org"
SHORTENER_PATH = "/api/shortener"
COMPILERS_PATH = "/api/compilers/{language}"
ROOT = Path(__file__).resolve().parent
LANGUAGES = {
    ".rs": "rust",
    ".c": "c++",
    ".cc": "c++",
    ".cpp": "c++",
    ".cxx": "c++",
}
SOURCE_SUFFIXES = set(LANGUAGES) - {".rs"}
MIN_HEADER_LINES = 2


@dataclass(frozen=True)
class Source:
    path: Path
    language: str
    compiler_name: str
    options: str
    code: str


def parse_source(path: Path) -> Source:
    code = path.read_text(encoding="utf-8")
    lines = code.splitlines()
    if len(lines) < MIN_HEADER_LINES:
        message = f"{path}: expected compiler and flags in the first two lines"
        raise ValueError(message)

    compiler_name = parse_header(lines[0], path, 1)
    options = parse_header(lines[1], path, 2)

    language = LANGUAGES.get(path.suffix.lower())
    if language is None:
        message = f"{path}: unsupported source extension {path.suffix!r}"
        raise ValueError(message)

    return Source(path, language, compiler_name, options, code)


def parse_header(line: str, path: Path, line_number: int) -> str:
    value = line[2:].strip() if line.startswith("//") else ""
    if not value:
        message = f"{path}:{line_number}: expected a // header containing compiler/flags"
        raise ValueError(message)
    return value


def check_response(response: Any, action: str) -> None:
    if response.status >= HTTPStatus.BAD_REQUEST:
        message = f"{action} failed with HTTP {response.status}"
        raise RuntimeError(message)


def compiler_catalog(client: Client, language: str) -> dict[str, str]:
    response = client.get(
        API_ROOT + COMPILERS_PATH.format(language=language),
    )
    check_response(response, "Compiler catalog request")

    catalog = {}
    for line in response.text.splitlines()[1:]:
        compiler_id, separator, display_name = line.partition("|")
        if separator:
            catalog[display_name.strip().casefold()] = compiler_id.strip()
    return catalog


def resolve_compiler(
    source: Source,
    catalogs: dict[str, dict[str, str]],
) -> str:
    catalog = catalogs[source.language]
    compiler_name = source.compiler_name.casefold()
    compiler_id = catalog.get(compiler_name)
    if compiler_id is None:
        available = sorted(name for name in catalog if compiler_name in name)
        hint = f" Similar names: {', '.join(available[:5])}." if available else ""
        message = (
            f"{source.path}: compiler {source.compiler_name!r} was not found for "
            f"language {source.language!r}.{hint}"
        )
        raise ValueError(message)
    return compiler_id


def component(component_name: str, state: dict[str, Any]) -> dict[str, Any]:
    return {
        "type": "component",
        "componentName": component_name,
        "componentState": state,
    }


def make_config(sources: list[Source], compiler_ids: list[str]) -> dict[str, Any]:
    editors = []
    compilers = []

    for editor_id, (source, compiler_id) in enumerate(
        zip(sources, compiler_ids, strict=True),
        start=1,
    ):
        editor = component(
            "codeEditor",
            {
                "id": editor_id,
                "source": source.code,
                "options": {"compileOnChange": True},
                "lang": source.language,
            },
        )
        editors.append(editor)
        compilers.append(
            component(
                "compiler",
                {
                    "source": editor_id,
                    "compiler": compiler_id,
                    "lang": source.language,
                    "filters": {"debugCalls": True},
                    "options": source.options,
                },
            ),
        )

    return {
        "version": 4,
        "content": [
            {
                "type": "row",
                "content": [
                    {"type": "column", "content": editors},
                    {"type": "column", "content": compilers},
                ],
            },
        ],
    }


def create_short_link(client: Client, config: dict[str, Any]) -> str:
    response = client.post(
        API_ROOT + SHORTENER_PATH,
        json={"config": config},
    )
    check_response(response, "Short link request")

    payload = response.json
    url = payload.get("url")
    if not isinstance(url, str) or not url:
        message = f"Compiler Explorer returned an unexpected response: {payload!r}"
        raise ValueError(message)
    return url if url.startswith("http") else API_ROOT + url


def find_pairs(root: Path) -> list[tuple[Path, Path]]:
    source_directories = {
        path.parent
        for path in (*root.rglob("file.c"), *root.rglob("file.rs"))
        if path.is_file()
    }
    pairs = []

    for directory in sorted(source_directories):
        c_path = directory / "file.c"
        rust_path = directory / "file.rs"
        has_c = c_path.is_file()
        has_rust = rust_path.is_file()
        if has_c != has_rust:
            missing = "file.rs" if has_c else "file.c"
            message = f"{directory}: missing paired {missing}"
            raise ValueError(message)
        pairs.append((c_path, rust_path))
    return pairs


def main() -> None:
    pairs = find_pairs(ROOT)
    if not pairs:
        message = f"No source pairs found in {ROOT}"
        raise SystemExit(message)

    parsed_pairs = [
        [parse_source(rust_path), parse_source(c_path)]
        for c_path, rust_path in pairs
    ]

    with Client() as client:
        catalogs = {
            language: compiler_catalog(client, language)
            for language in {source.language for pair in parsed_pairs for source in pair}
        }

        for sources in parsed_pairs:
            compiler_ids = [resolve_compiler(source, catalogs) for source in sources]
            link = create_short_link(client, make_config(sources, compiler_ids))
            directory = sources[0].path.parent.relative_to(ROOT).as_posix()
            print(f"{directory}: {link}")


if __name__ == "__main__":
    main()
