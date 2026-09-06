# To run the benchmarks

## Create and activate a virtual environment

<p>
  <span style="white-space: nowrap;">
    <img
      src="https://thesvg.org/icons/linux/default.svg"
      alt="linux"
      height="14"
    />
    Linux /
    <picture>
      <source
        media="(prefers-color-scheme: dark)"
        srcset="https://thesvg.org/icons/apple/default.svg"
      />
      <img
        src="https://thesvg.org/icons/apple/mono.svg"
        alt="macos"
        height="14"
      />
    </picture>
    MacOS:
  </span>
</p>

```bash
python3 -m venv .venv
# or uv venv .venv --seed

source .venv/bin/activate
```

<p>
  <img
    src="https://thesvg.org/icons/windows/default.svg"
    alt="windows"
    height="14"
  />
  Windows:
</p>

```bash
py -m venv .venv
# or uv venv .venv --seed

.venv\scripts\activate
```

## Install benchmark dependencies

<p>
  <img
    src="https://thesvg.org/icons/python/default.svg"
    alt="Python"
    height="14"
  />
  Using <a href="https://github.com/pypa/pip">pip</a>:
</p>

```bash
pip install . --group bench
```

<p>
  <img
    src="https://thesvg.org/icons/uv/default.svg"
    alt="uv"
    height="14"
  />
  Using <a href="https://github.com/astral-sh/uv">uv</a>:
</p>

```bash
uv pip install . --group bench
```

## Run `benchmark/run.py`

```bash
python benchmark/run.py
```

## Results

<!-- BEGIN BENCHMARK -->
### System Information

| Property         | Value                                                            |
|------------------|------------------------------------------------------------------|
| **OS**           | Windows 10 (10.0.19045)                                          |
| **Architecture** | AMD64                                                            |
| **Python**       | 3.14.7 (main, Aug  7 2026, 02:26:22) [MSC v.1944 64 bit (AMD64)] |
| **CPU**          | icelake                                                          |
| **Vendor**       | GenuineIntel                                                     |
| **Family**       | x86_64                                                           |

### 1 KiB

| Library                              | Time         | Throughput       | Relative  |
|--------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                | 0.0207 s     | 472.5 MB/s       | 0.05x     |
| `crc32c.crc32c`                      | 0.0013 s     | 7332.1 MB/s      | 0.70x     |
| `google_crc32c.value`                | 0.0014 s     | 7232.2 MB/s      | 0.69x     |
| `crc32c_rs.fallback`                 | 0.0032 s     | 3043.1 MB/s      | 0.29x     |
| **`crc32c_rs.avx512_vpclmulqdq` ⭐** | **0.0009 s** | **10426.7 MB/s** | **1.00x** |
| `crc32c_rs.avx512_pclmulqdq`         | 0.0014 s     | 6950.6 MB/s      | 0.67x     |
| `crc32c_rs.sse42_pclmulqdq`          | 0.0012 s     | 8339.6 MB/s      | 0.80x     |

### 1 MiB

| Library                              | Time         | Throughput       | Relative  |
|--------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                | 0.2032 s     | 492.2 MB/s       | 0.01x     |
| `crc32c.crc32c`                      | 0.0035 s     | 28319.0 MB/s     | 0.45x     |
| `google_crc32c.value`                | 0.0055 s     | 18107.7 MB/s     | 0.28x     |
| `crc32c_rs.fallback`                 | 0.0261 s     | 3834.8 MB/s      | 0.06x     |
| **`crc32c_rs.avx512_vpclmulqdq` ⭐** | **0.0016 s** | **63584.9 MB/s** | **1.00x** |
| `crc32c_rs.avx512_pclmulqdq`         | 0.0020 s     | 50175.6 MB/s     | 0.79x     |
| `crc32c_rs.sse42_pclmulqdq`          | 0.0019 s     | 52645.4 MB/s     | 0.83x     |

### 16 MiB

| Library                             | Time         | Throughput       | Relative  |
|-------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`               | 0.3225 s     | 496.1 MB/s       | 0.02x     |
| `crc32c.crc32c`                     | 0.0078 s     | 20447.8 MB/s     | 0.94x     |
| `google_crc32c.value`               | 0.0200 s     | 7983.1 MB/s      | 0.37x     |
| `crc32c_rs.fallback`                | 0.0451 s     | 3550.9 MB/s      | 0.16x     |
| `crc32c_rs.avx512_vpclmulqdq`       | 0.0117 s     | 13649.8 MB/s     | 0.63x     |
| **`crc32c_rs.avx512_pclmulqdq` ⭐** | **0.0073 s** | **21825.4 MB/s** | **1.00x** |
| `crc32c_rs.sse42_pclmulqdq`         | 0.0080 s     | 19933.2 MB/s     | 0.91x     |

### 64 B

| Library                            | Time         | Throughput      | Relative  |
|------------------------------------|--------------|-----------------|-----------|
| `fastcrc.crc32.iscsi`              | 0.0021 s     | 297.7 MB/s      | 0.29x     |
| `crc32c.crc32c`                    | 0.0009 s     | 684.5 MB/s      | 0.67x     |
| `google_crc32c.value`              | 0.0008 s     | 741.5 MB/s      | 0.72x     |
| `crc32c_rs.fallback`               | 0.0008 s     | 803.6 MB/s      | 0.79x     |
| `crc32c_rs.avx512_vpclmulqdq`      | 0.0007 s     | 840.8 MB/s      | 0.82x     |
| `crc32c_rs.avx512_pclmulqdq`       | 0.0008 s     | 729.6 MB/s      | 0.71x     |
| **`crc32c_rs.sse42_pclmulqdq` ⭐** | **0.0006 s** | **1023.4 MB/s** | **1.00x** |

### 64 KiB

| Library                              | Time         | Throughput       | Relative  |
|--------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                | 0.1261 s     | 495.8 MB/s       | 0.01x     |
| `crc32c.crc32c`                      | 0.0024 s     | 26594.6 MB/s     | 0.51x     |
| `google_crc32c.value`                | 0.0020 s     | 30526.5 MB/s     | 0.59x     |
| `crc32c_rs.fallback`                 | 0.0168 s     | 3712.0 MB/s      | 0.07x     |
| **`crc32c_rs.avx512_vpclmulqdq` ⭐** | **0.0012 s** | **51742.7 MB/s** | **1.00x** |
| `crc32c_rs.avx512_pclmulqdq`         | 0.0015 s     | 43082.6 MB/s     | 0.83x     |
| `crc32c_rs.sse42_pclmulqdq`          | 0.0016 s     | 39936.1 MB/s     | 0.77x     |

### 128 MiB

| Library                             | Time         | Throughput       | Relative  |
|-------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`               | 2.6003 s     | 492.2 MB/s       | 0.02x     |
| `crc32c.crc32c`                     | 0.0698 s     | 18347.6 MB/s     | 0.90x     |
| `google_crc32c.value`               | 0.0882 s     | 14515.2 MB/s     | 0.71x     |
| `crc32c_rs.fallback`                | 0.3453 s     | 3706.9 MB/s      | 0.18x     |
| `crc32c_rs.avx512_vpclmulqdq`       | 0.0748 s     | 17119.3 MB/s     | 0.84x     |
| **`crc32c_rs.avx512_pclmulqdq` ⭐** | **0.0626 s** | **20452.6 MB/s** | **1.00x** |
| `crc32c_rs.sse42_pclmulqdq`         | 0.0675 s     | 18961.1 MB/s     | 0.93x     |

### 512 MiB

| Library                             | Time         | Throughput       | Relative  |
|-------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`               | 10.4272 s    | 491.0 MB/s       | 0.02x     |
| `crc32c.crc32c`                     | 0.2726 s     | 18779.0 MB/s     | 0.90x     |
| `google_crc32c.value`               | 0.3490 s     | 14672.5 MB/s     | 0.71x     |
| `crc32c_rs.fallback`                | 1.3866 s     | 3692.4 MB/s      | 0.18x     |
| `crc32c_rs.avx512_vpclmulqdq`       | 0.3540 s     | 14464.0 MB/s     | 0.70x     |
| **`crc32c_rs.avx512_pclmulqdq` ⭐** | **0.2465 s** | **20769.8 MB/s** | **1.00x** |
| `crc32c_rs.sse42_pclmulqdq`         | 0.2581 s     | 19836.6 MB/s     | 0.96x     |
<!-- END BENCHMARK -->
