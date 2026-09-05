#!/bin/sh
set -e

cd /app

uv pip install --group maturin --system

maturin build --out dist --features mimalloc

uv pip install crc32c-rs --no-index --find-links wheels --force-reinstall --system

python -c "import crc32c_rs; print(crc32c_rs.__version__)"