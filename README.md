<!-- rumdl-disable MD036-->
<div align="center">

# crc32c-rs

_High-performance CRC32C implementation compliant with [RFC 3720 (iSCSI)](https://datatracker.ietf.org/doc/html/rfc3720)_
<!-- rumdl-enable MD036-->

[![PyPI version](https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Fpypi.org%2Fpypi%2Fcrc32c-rs%2Fjson&query=%24.info.version&variant=branded&size=xs&mode=light&logo=python&label=pypi+version)](https://pypi.org/project/crc32c-rs)
[![PyPI downloads](https://shieldcn.dev/pypi/dm/crc32c-rs.svg?variant=branded&size=xs&logo=python&logoColor=ffffff)](https://pypistats.org/packages/crc32c-rs)
[![PyPI requires Python](https://shieldcn.dev/pypi/python/crc32c-rs.svg?variant=branded&size=xs&logo=python&logoColor=ffffff&label=requires+python)](https://pypi.org/project/crc32c-rs)
[![PyPI license](https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Fpypi.org%2Fpypi%2Fcrc32c-rs%2Fjson&query=%24.info.license_expression&variant=branded&size=xs&mode=light&logo=python&logoColor=ffffff&label=license)](https://pypi.org/project/crc32c-rs)

<a href="https://github.com/lava-sh/crc32c-rs/actions?query=branch%3Amain"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/ci/lava-sh/crc32c-rs.svg?workflow=ci.yaml&branch=main&variant=outline&size=xs&animate=pulse&logo=github&label=CI&mode=dark"><img alt="CI" src="https://shieldcn.dev/github/ci/lava-sh/crc32c-rs.svg?workflow=ci.yaml&branch=main&variant=outline&size=xs&animate=pulse&mode=light&theme=zinc&logo=github&label=CI"></picture></a>
<a href="https://github.com/lava-sh/crc32c-rs/commits/main"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/last-commit/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&logo=github&mode=dark"><img alt="Last Commit" src="https://shieldcn.dev/github/last-commit/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=light&theme=zinc&logo=github"></picture></a>
<a href="https://github.com/lava-sh/crc32c-rs/commits/main"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/commits/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&logo=github&mode=dark"><img alt="Commits" src="https://shieldcn.dev/github/commits/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=light&theme=zinc&logo=github"></picture></a>
<a href="https://github.com/lava-sh/crc32c-rs/stargazers"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/stars/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=dark"><img alt="Stars" src="https://shieldcn.dev/github/stars/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=light&theme=zinc"></picture></a>
<a href="https://app.codspeed.io/lava-sh/crc32c-rs?utm_source=badge"><img alt="CodSpeed" src="https://shieldcn.dev/badge/codspeed.svg?variant=branded&font=geist&size=xs&color=FF621F&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA5MCA2NCI%2BPHBhdGggZmlsbD0iY3VycmVudENvbG9yIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0xNi4yIDEyQzQxLjQtNS44IDgyLjEgMS4yIDg2LjEgMkM4OC43IDIuNiA5MC40IDUuMSA4OS44IDcuN0M4OC43IDEyIDc4LjggNDcuNSA0NS4xIDY0QzQ1LjIgNjAuNiA0NS42IDU3LjYgNDYuMiA1NC44QzQ2LjQgNTQuMiA0NS42IDUzLjYgNDUgNTRDNDMgNTUuMiAzOS4zIDU3LjMgMzQuMSA1OS4zQzI3LjUgNjEuOSAyMC41IDYzLjUgMjAuNSA2My41QzIwLjUgNjMuNSAyNC40IDU5LjUgMjguNiA1NUMyOS43IDUzLjkgMjguMyA1MiAyNi45IDUyLjVDMTguNyA1NS43IDguMSA1OC41IDAgNTguNUMzLjQgNTAuOCAxMS4zIDQyLjIgMTcuNiAzNi4yQzE4LjcgMzUuMiAxNy44IDMzLjQgMTYuMyAzMy44QzkuOCAzNS43IDMuMyAzNy45IDMuMyAzNy45QzMuMyAzNy45IDguMyAzMC41IDEzLjQgMjUuNEMxNy4xIDIxLjcgMjIuMyAxOC42IDI0LjkgMTcuMUMyNS40IDE2LjggMjUuMyAxNS45IDI0LjcgMTUuOEMyMiAxNC45IDE5LjIgMTMuNyAxNi4yIDEyWk01OS43IDE2LjVhOC41IDguNSAwIDEgMCAxNyAwYTguNSA4LjUgMCAxIDAtMTcgMFpNNjguMSAyMC40YTMuNCAzLjQgMCAxIDAgNi44IDBhMy40IDMuNCAwIDEgMC02LjggMFoiLz48L3N2Zz4%3D&logoColor=ffffff"></a>
<a href="https://t.me/gh_lava_sh"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Ftg.chirizxc.workers.dev%2Fgh_lava_sh&query=%24.members&suffix=+members&variant=outline&font=geist&size=xs&logo=ri%3AFaTelegramPlane&logoColor=24A1DE&label=t.me/gh_lava_sh&mode=dark"><img alt="Telegram members" src="https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Ftg.chirizxc.workers.dev%2Fgh_lava_sh&query=%24.members&suffix=+members&variant=outline&font=geist&size=xs&mode=light&theme=zinc&logo=ri%3AFaTelegramPlane&logoColor=24A1DE&label=t.me/gh_lava_sh"></picture></a>

</div>

## Features

- [x] [High-performance](https://github.com/lava-sh/crc32c-rs/blob/main/benchmarks%2Fpython%2FREADME.md#results) CRC32C implementation written in Rust

- [x] Runtime SIMD dispatch with CPU-model-tuned implementations (see [below](#basic-usage))

- [x] [Direct access](#direct-implementation-selection) to specific implementations for advanced use cases

## Installation

<p>
  <img
    src="https://thesvg.org/icons/python/default.svg"
    alt="Python"
    height="14"
  />
  Using <a href="https://github.com/pypa/pip">pip</a>:
</p>

```console
pip install crc32c-rs
```

<p>
  <img
    src="https://thesvg.org/icons/uv/default.svg"
    alt="uv"
    height="14"
  />
  Using <a href="https://github.com/astral-sh/uv">uv</a>:
</p>

```console
uv pip install crc32c-rs
```

<p>
  <img
    src="https://thesvg.org/icons/poetry/default.svg"
    alt="Poetry"
    height="14"
  />
  Using <a href="https://github.com/python-poetry/poetry">poetry</a>:
</p>

```console
poetry add crc32c-rs
```

## Examples

### Basic usage

```python
from crc32c_rs import crc32c, Hasher

print(crc32c(b"Hello world!"))  # 2073618257

crc = crc32c(b"Hello")
print(crc32c(b" world!", crc))  # 2073618257

h = Hasher()
h.update(b"Hello")
h.update(b" ")
print(h.checksum)  # 4220938453
h.update(b"world")
h.update(b"!")
print(h.checksum)  # 2073618257
print(h.digest())  # b'{\x98\xe7Q'
print(h.hexdigest())  # 7b98e751
```

By default, `crc32c_rs.crc32c` selects an implementation at runtime. Dispatch is keyed by CPU model
first: the vendor and CPUID model (family/model) are looked up, and if the CPU is a known one, its
microarchitecture-tuned implementation is used whenever the required instruction sets are present.
Only for unknown CPUs does dispatch fall back to a generic feature-based choice.

#### x86/x86_64

Model-tuned selection (checked in this order):

|    CPU model    | With `AVX-512VL + VPCLMULQDQ` | With `AVX-512VL + PCLMULQDQ` | With `SSE4.2 + PCLMULQDQ` |
|:---------------:|:-----------------------------:|:----------------------------:|:-------------------------:|
| Sapphire Rapids |           `v3s1_s3`           |              ×               |         `v8s3x3`          |
|      Genoa      |           `v3s2x4`            |              ×               |         `v1s3x2`          |
|    Ice Lake     |           `v4s5x3`            |              ×               |         `v7s3x3`          |
|  Cascade Lake   |               ×               |          `v9s3x4e`           |         `v8s3x3`          |
|      Milan      |               ×               |              ×               |         `v1s4x2`          |
|      Rome       |               ×               |              ×               |         `v1s3x3`          |

Unknown model: `AVX-512VL + VPCLMULQDQ` -> `v4s5x3`,
else `AVX-512VL + PCLMULQDQ` -> `v9s3x4e`,
else `SSE4.2 + PCLMULQDQ` -> `v8s3x3`, else fallback.

#### AArch64/ARM64EC

1. `CRC + AES + SHA3` -> `v9s3x2e_s3`
2. `CRC + AES` -> `v12e_v1` on Apple, `v3s4x2e_v2` elsewhere
3. fallback

#### Other platforms

Portable fallback.

### Direct implementation selection

The default `crc32c_rs.crc32c` function selects an implementation
at runtime, but you can also call a specific implementation directly:

| Function                             |        Required CPU features        |
|--------------------------------------|:-----------------------------------:|
| `crc32c`                             |        Runtime CPU dispatch         |
| `crc32c_fallback`                    |       No special CPU features       |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3` | `AVX-512F + AVX-512VL + VPCLMULQDQ` |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`  | `AVX-512F + AVX-512VL + VPCLMULQDQ` |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`  | `AVX-512F + AVX-512VL + VPCLMULQDQ` |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`  |       `AVX-512VL + PCLMULQDQ`       |
| `crc32c_sse42_pclmulqdq_v1s3x2`      |        `SSE4.2 + PCLMULQDQ`         |
| `crc32c_sse42_pclmulqdq_v1s3x3`      |        `SSE4.2 + PCLMULQDQ`         |
| `crc32c_sse42_pclmulqdq_v1s4x2`      |        `SSE4.2 + PCLMULQDQ`         |
| `crc32c_sse42_pclmulqdq_v7s3x3`      |        `SSE4.2 + PCLMULQDQ`         |
| `crc32c_sse42_pclmulqdq_v8s3x3`      |        `SSE4.2 + PCLMULQDQ`         |
| `crc32c_aes_crc_v12e_v1`             |             `CRC + AES`             |
| `crc32c_aes_v3s4x2e_v2`              |             `CRC + AES`             |
| `crc32c_aes_sha3_v9s3x2e_s3`         |         `CRC + AES + SHA3`          |

Architecture-specific implementations are available only on compatible builds.
If the current processor does not support the required features, calling one of
these functions raises `crc32c_rs.UnsupportedCPUFeatureError`.

```python
from crc32c_rs import crc32c_avx512vl_vpclmulqdq_v3s1_s3

checksum = crc32c_avx512vl_vpclmulqdq_v3s1_s3(b"Hello world!")
print(checksum)  # 2073618257
```

<div align="center">

## Contributors

[![lava-sh/crc32c-rs contributors](https://shieldcn.dev/contributors/lava-sh/crc32c-rs.svg?title=false&theme=slate&size=80&bots=true&titleAlign=center&mode=light&font=geist&border=false&image=https%3A%2F%2Fimages.wallpaperscraft.ru%2Fimage%2Fsingle%2Foblaka_nebo_ogni_1647475_3840x2400.jpg&overlay=0.3)](https://github.com/lava-sh/crc32c-rs/graphs/contributors)

</div>
