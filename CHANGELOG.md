# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0).

## [Unreleased] - ReleaseDate

### Performance improvements

* Add a `crc32c_small` fast path for small inputs to `avx512vl_vpclmulqdq_v3s1_s3`, `avx512vl_pclmulqdq_v9s3x4e`, `sse42_pclmulqdq_v7s3x3`, and `sse42_pclmulqdq_v1s3x3`. (by [@chirizxc][gh-chirizxc])
* Use AVX-512 folding instead of the multi-stream `crc32` loop in `crc32c_small` of `avx512vl_vpclmulqdq_v4s5x3` and `avx512vl_vpclmulqdq_v3s2x4`. (by [@chirizxc][gh-chirizxc])
* Fold the remaining whole 64-byte blocks in `crc32c_small` of `avx512vl_vpclmulqdq_v4s5x3`, `avx512vl_vpclmulqdq_v3s2x4`, and `avx512vl_pclmulqdq_v9s3x4e`, so the scalar `crc32` loop only handles the final < 64 bytes. (by [@chirizxc][gh-chirizxc])

### Misc

* Port the `aarch64` kernels from inline assembly to intrinsics. (by [@chirizxc][gh-chirizxc])
* Move the `x86` and `aarch64` sources into subdirectories of `src/arch`. (by [@chirizxc][gh-chirizxc])
* Clean up the `#[target_feature]` attributes of the `x86` kernels and the `.godbolt` sources. (by [@chirizxc][gh-chirizxc])
* Update the `.godbolt` links. (by [@chirizxc][gh-chirizxc])
* Bump PyO3 (0.29.2 -> 0.29.3) and the Python dependencies. (by [@chirizxc][gh-chirizxc])
* Update CI: bump the WASM nightly toolchain and Pyodide, and extend the WASM tests. (by [@chirizxc][gh-chirizxc])

## [0.0.5] - 27.09.2026

### Performance improvements

* Speed up `crc32c_sse42` on medium inputs. (by [@chirizxc][gh-chirizxc])
* Speed up the fallback implementation on inputs with 8 or more remaining bytes. (by [@chirizxc][gh-chirizxc])

### Features

* Add docstrings to the `_crc32c_rs` stubs. (by [@chirizxc][gh-chirizxc])
* Add `__doc__` for `UnsupportedCPUFeatureError` class. (by [@chirizxc][gh-chirizxc])

### Misc

* Clean up and deduplicate the `src/arch/*` implementations and the `.godbolt` sources. (by [@chirizxc][gh-chirizxc])
* Update the benchmark results and the Python/Rust benchmark harness. (by [@chirizxc][gh-chirizxc])
* Update the godbolt links, including the fixed ARM links. (by [@chirizxc][gh-chirizxc])
* Log a more precise CPU name in the tests. (by [@chirizxc][gh-chirizxc])

## [0.0.4] - 17.09.2026

### Performance improvements

* Only take the `crc32c_small` fast path from the length where it is faster than the 8-byte walk, and decide that before the alignment prologue shortens the input. (by [@chirizxc][gh-chirizxc])

### Features

* Improve CPU detection for AMD processors. (by [@chirizxc][gh-chirizxc])

### Misc

* Import `ReadableBuffer` from `_typeshed` in `_crc32c_rs.pyi`. (by [@chirizxc][gh-chirizxc])

## [0.0.3] - 15.09.2026

### Performance improvements

* Fast paths for `avx512vl_vpclmulqdq_v4s5x3`, `avx512vl_vpclmulqdq_v3s2x4`, `aes_crc_v12e_v1`, `aes_sha3_v9s3x2e_s3`, and `aes_v3s4x2e_v2` for small inputs (32B - 1KiB). (by [@chirizxc][gh-chirizxc])

### Features

* New `sse42` implementation: SSE4.2-only, without PCLMULQDQ, exposed as `crc32c_sse42`. (by [@chirizxc][gh-chirizxc])
* Prefer `aes_sha3_v9s3x2e_s3` only on Apple CPUs; other vendors use `aes_crc_v3s4x2e_v2` when CRC+AES are available. (by [@chirizxc][gh-chirizxc])

### Bug Fixes

* Correct crc32c fallback implementation on big-endian. (by [@chirizxc][gh-chirizxc])

## [0.0.2] - 09.09.2026

### Bug Fixes

* Remove the incorrect `_Hasher` class from `_crc32c_rs.pyi`. (by [@chirizxc][gh-chirizxc])
* Fix Intel CPU model grouping for Ice Lake. (by [@chirizxc][gh-chirizxc])

### Misc

* Fix the link to the unreleased version in the changelog. (by [@chirizxc][gh-chirizxc])

## [0.0.1] - 08.09.2026

First release.

[gh-chirizxc]: https://github.com/chirizxc

[Unreleased]: https://github.com/lava-sh/crc32c-rs/compare/0.0.5...HEAD

[0.0.5]: https://github.com/lava-sh/crc32c-rs/compare/0.0.4...0.0.5
[0.0.4]: https://github.com/lava-sh/crc32c-rs/compare/0.0.3...0.0.4
[0.0.3]: https://github.com/lava-sh/crc32c-rs/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/lava-sh/crc32c-rs/compare/0.0.1...0.0.2
[0.0.1]: https://github.com/lava-sh/crc32c-rs/compare/1771e0a03a38848d12afa15ab09ae1a05a6325b0...0.0.1
