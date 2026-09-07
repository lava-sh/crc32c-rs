<!-- rumdl-disable MD036-->
<div align="center">

# crc32c-rs

_High-performance CRC32C implementation compliant with RFC 3720 (iSCSI)_
<!-- rumdl-enable MD036-->

[![PyPI version](https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Fpypi.org%2Fpypi%2Fcrc32c-rs%2Fjson&query=%24.info.version&variant=branded&size=xs&mode=light&logo=python&label=pypi+version)](https://pypi.org/project/crc32c-rs)
[![PyPI downloads](https://shieldcn.dev/pypi/dm/crc32c-rs.svg?variant=branded&size=xs&logo=python&logoColor=ffffff)](https://pypistats.org/packages/crc32c-rs)
[![PyPI requires Python](https://shieldcn.dev/pypi/python/crc32c-rs.svg?variant=branded&size=xs&logo=python&logoColor=ffffff&label=requires+python)](https://pypi.org/project/crc32c-rs)
[![PyPI license](https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Fpypi.org%2Fpypi%2Fcrc32c-rs%2Fjson&query=%24.info.license_expression&variant=branded&size=xs&mode=light&logo=python&logoColor=ffffff&label=license)](https://pypi.org/project/crc32c-rs)

<a href="https://github.com/lava-sh/crc32c-rs/actions?query=branch%3Amain"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/ci/lava-sh/crc32c-rs.svg?workflow=ci.yaml&branch=main&variant=outline&size=xs&animate=pulse&logo=github&label=CI&mode=dark"><img alt="CI" src="https://shieldcn.dev/github/ci/lava-sh/crc32c-rs.svg?workflow=ci.yaml&branch=main&variant=outline&size=xs&animate=pulse&mode=light&theme=zinc&logo=github&label=CI"></picture></a>
<a href="https://github.com/lava-sh/crc32c-rs/commits/main"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/last-commit/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&logo=github&mode=dark"><img alt="Last Commit" src="https://shieldcn.dev/github/last-commit/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=light&theme=zinc&logo=github"></picture></a>
<a href="https://github.com/lava-sh/crc32c-rs/commits/main"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/commits/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&logo=github&mode=dark"><img alt="Commits" src="https://shieldcn.dev/github/commits/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=light&theme=zinc&logo=github"></picture></a>
<a href="https://github.com/lava-sh/crc32c-rs/stargazers"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/stars/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=dark"><img alt="Stars" src="https://shieldcn.dev/github/stars/lava-sh/crc32c-rs.svg?variant=outline&font=geist&size=xs&mode=light&theme=zinc"></picture></a>
<a href="https://t.me/gh_lava_sh"><picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Ftg.chirizxc.workers.dev%2Fgh_lava_sh&query=%24.members&suffix=+members&variant=outline&font=geist&size=xs&logo=ri%3AFaTelegramPlane&logoColor=24A1DE&label=t.me/gh_lava_sh&mode=dark"><img alt="Telegram members" src="https://shieldcn.dev/badge/dynamic/json.svg?url=https%3A%2F%2Ftg.chirizxc.workers.dev%2Fgh_lava_sh&query=%24.members&suffix=+members&variant=outline&font=geist&size=xs&mode=light&theme=zinc&logo=ri%3AFaTelegramPlane&logoColor=24A1DE&label=t.me/gh_lava_sh"></picture></a>

</div>

## Features

- High-performance CRC32C implementation written in Rust

- Runtime dispatch to the fastest available implementation:
  - x86/x86_64: `SSE4.2 + PCLMULQDQ`, `AVX-512VL + PCLMULQDQ`, or
    `AVX-512F + AVX-512VL + VPCLMULQDQ`
  - AArch64/ARM64EC: `CRC + AES`, with an optimized `SHA3` variant when available
  - Other platforms: a portable fallback implementation

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
from crc32c_rs import crc32c

print(crc32c(b"Hello world!"))  # 2073618257

crc = crc32c(b"Hello")
print(crc32c(b" world!", crc))  # 2073618257
```

By default, `crc32c_rs.crc32c` detects the CPU's supported instruction
sets at runtime and selects the fastest available implementation. It
checks the implementations from the highest acceleration level to the lowest:

#### x86/x86_64

1. `AVX-512F + AVX-512VL + VPCLMULQDQ`
2. `AVX-512VL + PCLMULQDQ`
3. `SSE4.2 + PCLMULQDQ`
4. fallback

#### AArch64/ARM64EC

1. `CRC + AES + SHA3`
2. `CRC + AES`
3. fallback

#### Other platforms

Portable fallback.

### Direct implementation selection

The default `crc32c_rs.crc32c` function selects an implementation
at runtime, but you can also call a specific implementation directly:

| Function                             | Required CPU features               |
|--------------------------------------|-------------------------------------|
| `crc32c`                             | Runtime CPU dispatch                |
| `crc32c_fallback`                    | No special CPU features             |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3` | `AVX-512F + AVX-512VL + VPCLMULQDQ` |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`  | `AVX-512F + AVX-512VL + VPCLMULQDQ` |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`  | `AVX-512F + AVX-512VL + VPCLMULQDQ` |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`  | `AVX-512VL + PCLMULQDQ`             |
| `crc32c_sse42_pclmulqdq_v1s3x2`      | `SSE4.2 + PCLMULQDQ`                |
| `crc32c_sse42_pclmulqdq_v1s3x3`      | `SSE4.2 + PCLMULQDQ`                |
| `crc32c_sse42_pclmulqdq_v1s4x2`      | `SSE4.2 + PCLMULQDQ`                |
| `crc32c_sse42_pclmulqdq_v7s3x3`      | `SSE4.2 + PCLMULQDQ`                |
| `crc32c_sse42_pclmulqdq_v8s3x3`      | `SSE4.2 + PCLMULQDQ`                |
| `crc32c_sse42_s3k4096e`              | `SSE4.2 + PCLMULQDQ`                |
| `crc32c_aes_crc_v12e_v1`             | `CRC + AES`                         |
| `crc32c_aes_v3s4x2e_v2`              | `CRC + AES`                         |
| `crc32c_aes_sha3_v9s3x2e_s3`         | `CRC + AES + SHA3`                  |
| `crc32c_crc_neon_s3k95760_s3`        | `CRC + AES`                         |

Architecture-specific implementations are available only on compatible builds.
If the current processor does not support the required features, calling one of
these functions raises `crc32c_rs.UnsupportedCPUFeatureError`.

```python
from crc32c_rs import crc32c_avx512vl_vpclmulqdq_v3s1_s3

checksum = crc32c_avx512vl_vpclmulqdq_v3s1_s3(b"Hello world!")
print(checksum)  # 2073618257
```

### CPU and implementation notes

> [!NOTE]
> The following single-core CRC32C benchmark results are taken from the
> [`corsix/fast-crc32` README](https://github.com/corsix/fast-crc32/blob/main/README.md).
> These are benchmarks from that project, not from `crc32c-rs`.

| Processor                          | Instruction set                         | Closest backend                          |          Speed |
|------------------------------------|-----------------------------------------|------------------------------------------|---------------:|
| Apple M1                           | `CRC + AES`                             | `crc32c_crc_neon_s3k95760_s3`            |     25.50 GB/s |
| Apple M1                           | `CRC + AES`                             | `crc32c_aes_crc_v12e_v1`                 |     77.69 GB/s |
| **Apple M1**                       | **`CRC + AES + SHA3`**                  | **`crc32c_aes_sha3_v9s3x2e_s3`**         | **85.05 GB/s** |
| GCP Tau T2A (Ampere Altra Arm)     | `CRC + AES`                             | `crc32c_crc_neon_s3k95760_s3`            |     23.57 GB/s |
| GCP Tau T2A (Ampere Altra Arm)     | `CRC + AES`                             | `crc32c_aes_crc_v12e_v1`                 |     21.81 GB/s |
| **GCP Tau T2A (Ampere Altra Arm)** | **`CRC + AES`**                         | **`crc32c_aes_v3s4x2e_v2`**              | **35.87 GB/s** |
| Intel Cascade Lake                 | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_s3k4096e`                  |     15.11 GB/s |
| Intel Cascade Lake                 | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_pclmulqdq_v8s3x3`          |     28.55 GB/s |
| **Intel Cascade Lake**             | **`AVX-512VL + PCLMULQDQ`**             | **`crc32c_avx512vl_pclmulqdq_v9s3x4e`**  | **31.55 GB/s** |
| Intel Ice Lake                     | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_s3k4096e`                  |     19.80 GB/s |
| Intel Ice Lake                     | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_pclmulqdq_v7s3x3`          |     42.58 GB/s |
| **Intel Ice Lake**                 | **`AVX-512F + AVX-512VL + VPCLMULQDQ`** | **`crc32c_avx512vl_vpclmulqdq_v4s5x3`**  | **63.98 GB/s** |
| Intel Sapphire Rapids              | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_s3k4096e`                  |     17.59 GB/s |
| Intel Sapphire Rapids              | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_pclmulqdq_v8s3x3`          |     36.98 GB/s |
| **Intel Sapphire Rapids**          | **`AVX-512F + AVX-512VL + VPCLMULQDQ`** | **`crc32c_avx512vl_vpclmulqdq_v3s1_s3`** | **97.30 GB/s** |
| AMD EPYC Rome                      | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_s3k4096e`                  |     23.08 GB/s |
| **AMD EPYC Rome**                  | **`SSE4.2 + PCLMULQDQ`**                | **`crc32c_sse42_pclmulqdq_v1s3x3`**      | **31.16 GB/s** |
| AMD EPYC Milan                     | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_s3k4096e`                  |     23.55 GB/s |
| **AMD EPYC Milan**                 | **`SSE4.2 + PCLMULQDQ`**                | **`crc32c_sse42_pclmulqdq_v1s4x2`**      | **31.76 GB/s** |
| AMD EPYC Genoa                     | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_s3k4096e`                  |     26.56 GB/s |
| AMD EPYC Genoa                     | `SSE4.2 + PCLMULQDQ`                    | `crc32c_sse42_pclmulqdq_v1s3x2`          |     36.20 GB/s |
| **AMD EPYC Genoa**                 | **`AVX-512F + AVX-512VL + VPCLMULQDQ`** | **`crc32c_avx512vl_vpclmulqdq_v3s2x4`**  | **71.95 GB/s** |

<div align="center">

## Contributors

[![lava-sh/crc32c-rs contributors](https://shieldcn.dev/contributors/lava-sh/crc32c-rs.svg?title=false&theme=slate&size=80&bots=true&titleAlign=center&mode=light&font=geist&border=false&image=https%3A%2F%2Fimages.wallpaperscraft.ru%2Fimage%2Fsingle%2Foblaka_nebo_ogni_1647475_3840x2400.jpg&overlay=0.3)](https://github.com/lava-sh/crc32c-rs/graphs/contributors)

</div>
