# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0).

## [Unreleased] - ReleaseDate

### Added

* Fastpath for `avx512vl_vpclmulqdq_v4s5x3`, `avx512vl_vpclmulqdq_v3s2x4` on small inputs (32 B - 1 KiB). (by [@chirizxc][gh-chirizxc])

### Fixes

* Correct crc32c fallback implementation on big-endian. (by [@chirizxc][gh-chirizxc])

## [0.0.2] - 09.09.2026

### Fixes

* Fix the link to the unreleased version in the changelog. (by [@chirizxc][gh-chirizxc])
* Remove the incorrect `_Hasher` class from `_crc32c_rs.pyi`. (by [@chirizxc][gh-chirizxc])
* Fix Intel CPU model grouping for Ice Lake. (by [@chirizxc][gh-chirizxc])

## [0.0.1] - 08.09.2026

First release.

[gh-chirizxc]: https://github.com/chirizxc

[Unreleased]: https://github.com/lava-sh/crc32c-rs/compare/0.0.2...HEAD

[0.0.2]: https://github.com/lava-sh/crc32c-rs/compare/0.0.1...0.0.2
[0.0.1]: https://github.com/lava-sh/crc32c-rs/compare/1771e0a03a38848d12afa15ab09ae1a05a6325b0...0.0.1
