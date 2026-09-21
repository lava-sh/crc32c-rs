# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0).

## [Unreleased] - ReleaseDate

### Changed

* Add docstrings to the `_crc32c_rs` stubs. (by [@chirizxc][gh-chirizxc])
* Speed up `crc32c_sse42` on medium inputs. (by [@chirizxc][gh-chirizxc])
* Speed up the fallback implementation on inputs with 8 or more remaining bytes. (by [@chirizxc][gh-chirizxc])

## [0.0.4] - 17.09.2026

### Changed

* Only take the `crc32c_small` fast path from the length where it is faster than the 8-byte walk, and decide that before the alignment prologue shortens the input. (by [@chirizxc][gh-chirizxc])

* Import `ReadableBuffer` from `_typeshed` in `_crc32c_rs.pyi`. (by [@chirizxc][gh-chirizxc])

* Improve CPU detection for AMD processors. (by [@chirizxc][gh-chirizxc])

## [0.0.3] - 15.09.2026

### Added

* Fast paths for `avx512vl_vpclmulqdq_v4s5x3`, `avx512vl_vpclmulqdq_v3s2x4`, `aes_crc_v12e_v1`, `aes_sha3_v9s3x2e_s3`, and `aes_v3s4x2e_v2` for small inputs (32B - 1KiB). (by [@chirizxc][gh-chirizxc])
* New `sse42` implementation: SSE4.2-only, without PCLMULQDQ, exposed as `crc32c_sse42`. (by [@chirizxc][gh-chirizxc])

### Fixes

* Correct crc32c fallback implementation on big-endian. (by [@chirizxc][gh-chirizxc])
* Prefer `aes_sha3_v9s3x2e_s3` only on Apple CPUs; other vendors use `aes_crc_v3s4x2e_v2` when CRC+AES are available. (by [@chirizxc][gh-chirizxc])

## [0.0.2] - 09.09.2026

### Fixes

* Fix the link to the unreleased version in the changelog. (by [@chirizxc][gh-chirizxc])
* Remove the incorrect `_Hasher` class from `_crc32c_rs.pyi`. (by [@chirizxc][gh-chirizxc])
* Fix Intel CPU model grouping for Ice Lake. (by [@chirizxc][gh-chirizxc])

## [0.0.1] - 08.09.2026

First release.

[gh-chirizxc]: https://github.com/chirizxc

[Unreleased]: https://github.com/lava-sh/crc32c-rs/compare/0.0.4...HEAD

[0.0.4]: https://github.com/lava-sh/crc32c-rs/compare/0.0.3...0.0.4
[0.0.3]: https://github.com/lava-sh/crc32c-rs/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/lava-sh/crc32c-rs/compare/0.0.1...0.0.2
[0.0.1]: https://github.com/lava-sh/crc32c-rs/compare/1771e0a03a38848d12afa15ab09ae1a05a6325b0...0.0.1
