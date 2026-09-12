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
cd benchmark
python run.py -o results.json
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

| Library                                     | Time         | Throughput       | Relative  |
|---------------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                       | 0.0216 s     | 451.3 MB/s       | 0.04x     |
| `google_crc32c.value`                       | 0.0018 s     | 5524.2 MB/s      | 0.52x     |
| `crc32c.crc32c`                             | 0.0014 s     | 6888.8 MB/s      | 0.65x     |
| `crc32c_rs.crc32c`                          | 0.0015 s     | 6566.9 MB/s      | 0.62x     |
| `crc32c_rs.crc32c_fallback`                 | 0.0032 s     | 3092.8 MB/s      | 0.29x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 0.0013 s     | 7579.7 MB/s      | 0.71x     |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 0.0014 s     | 7201.3 MB/s      | 0.68x     |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 0.0014 s     | 6865.6 MB/s      | 0.65x     |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 0.0012 s     | 8388.3 MB/s      | 0.79x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 0.0012 s     | 8268.2 MB/s      | 0.78x     |
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` ⭐** | **0.0009 s** | **10618.3 MB/s** | **1.00x** |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 0.0012 s     | 8255.0 MB/s      | 0.78x     |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 0.0014 s     | 7149.1 MB/s      | 0.67x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 0.0014 s     | 7217.8 MB/s      | 0.68x     |

### 1 MiB

| Library                                    | Time         | Throughput       | Relative  |
|--------------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                      | 0.2014 s     | 496.6 MB/s       | 0.01x     |
| `google_crc32c.value`                      | 0.0048 s     | 20973.2 MB/s     | 0.28x     |
| `crc32c.crc32c`                            | 0.0035 s     | 28702.6 MB/s     | 0.38x     |
| `crc32c_rs.crc32c`                         | 0.0018 s     | 55614.3 MB/s     | 0.74x     |
| `crc32c_rs.crc32c_fallback`                | 0.0240 s     | 4165.8 MB/s      | 0.06x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 0.0034 s     | 29011.6 MB/s     | 0.38x     |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 0.0027 s     | 36778.2 MB/s     | 0.49x     |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 0.0029 s     | 34585.3 MB/s     | 0.46x     |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 0.0021 s     | 47348.5 MB/s     | 0.63x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 0.0020 s     | 48983.6 MB/s     | 0.65x     |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 0.0016 s     | 63159.2 MB/s     | 0.84x     |
| **`crc32c_avx512vl_vpclmulqdq_v3s2x4` ⭐** | **0.0013 s** | **75637.2 MB/s** | **1.00x** |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`        | 0.0014 s     | 69013.1 MB/s     | 0.91x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`        | 0.0021 s     | 48271.9 MB/s     | 0.64x     |

### 16 MiB

| Library                                | Time         | Throughput       | Relative  |
|----------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                  | 0.3290 s     | 486.4 MB/s       | 0.02x     |
| `google_crc32c.value`                  | 0.0187 s     | 8570.4 MB/s      | 0.33x     |
| `crc32c.crc32c`                        | 0.0075 s     | 21475.4 MB/s     | 0.84x     |
| `crc32c_rs.crc32c`                     | 0.0068 s     | 23568.2 MB/s     | 0.92x     |
| `crc32c_rs.crc32c_fallback`            | 0.0385 s     | 4161.0 MB/s      | 0.16x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 0.0073 s     | 21814.4 MB/s     | 0.85x     |
| **`crc32c_sse42_pclmulqdq_v1s3x3` ⭐** | **0.0062 s** | **25611.1 MB/s** | **1.00x** |
| `crc32c_sse42_pclmulqdq_v1s4x2`        | 0.0064 s     | 25044.2 MB/s     | 0.98x     |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 0.0076 s     | 21180.0 MB/s     | 0.83x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 0.0077 s     | 20662.5 MB/s     | 0.81x     |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 0.0086 s     | 18697.7 MB/s     | 0.73x     |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 0.0074 s     | 21526.8 MB/s     | 0.84x     |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 0.0070 s     | 22821.0 MB/s     | 0.89x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 0.0074 s     | 21669.1 MB/s     | 0.85x     |

### 64 B

| Library                                | Time         | Throughput      | Relative  |
|----------------------------------------|--------------|-----------------|-----------|
| `fastcrc.crc32.iscsi`                  | 0.0021 s     | 287.3 MB/s      | 0.27x     |
| `google_crc32c.value`                  | 0.0008 s     | 722.9 MB/s      | 0.69x     |
| `crc32c.crc32c`                        | 0.0010 s     | 640.4 MB/s      | 0.61x     |
| `crc32c_rs.crc32c`                     | 0.0007 s     | 860.1 MB/s      | 0.82x     |
| `crc32c_rs.crc32c_fallback`            | 0.0007 s     | 848.9 MB/s      | 0.81x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 0.0007 s     | 931.7 MB/s      | 0.89x     |
| **`crc32c_sse42_pclmulqdq_v1s3x3` ⭐** | **0.0006 s** | **1051.4 MB/s** | **1.00x** |
| `crc32c_sse42_pclmulqdq_v1s4x2`        | 0.0006 s     | 1050.9 MB/s     | 1.00x     |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 0.0006 s     | 1039.3 MB/s     | 0.99x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 0.0006 s     | 1027.5 MB/s     | 0.98x     |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 0.0009 s     | 668.7 MB/s      | 0.64x     |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 0.0009 s     | 647.6 MB/s      | 0.62x     |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 0.0007 s     | 938.7 MB/s      | 0.89x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 0.0006 s     | 991.2 MB/s      | 0.94x     |

### 64 KiB

| Library                                    | Time         | Throughput       | Relative  |
|--------------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                      | 0.1301 s     | 480.3 MB/s       | 0.01x     |
| `google_crc32c.value`                      | 0.0023 s     | 26700.3 MB/s     | 0.44x     |
| `crc32c.crc32c`                            | 0.0023 s     | 26880.6 MB/s     | 0.44x     |
| `crc32c_rs.crc32c`                         | 0.0011 s     | 58663.4 MB/s     | 0.96x     |
| `crc32c_rs.crc32c_fallback`                | 0.0141 s     | 4421.4 MB/s      | 0.07x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 0.0022 s     | 28520.6 MB/s     | 0.47x     |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 0.0020 s     | 31698.5 MB/s     | 0.52x     |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 0.0019 s     | 32858.4 MB/s     | 0.54x     |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 0.0014 s     | 45253.8 MB/s     | 0.74x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 0.0015 s     | 42837.6 MB/s     | 0.70x     |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 0.0012 s     | 54262.9 MB/s     | 0.89x     |
| **`crc32c_avx512vl_vpclmulqdq_v3s2x4` ⭐** | **0.0010 s** | **61160.6 MB/s** | **1.00x** |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`        | 0.0010 s     | 60339.8 MB/s     | 0.99x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`        | 0.0015 s     | 41153.6 MB/s     | 0.67x     |

### 128 MiB

| Library                                | Time         | Throughput       | Relative  |
|----------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                  | 2.6383 s     | 485.2 MB/s       | 0.02x     |
| `google_crc32c.value`                  | 0.0880 s     | 14545.4 MB/s     | 0.64x     |
| `crc32c.crc32c`                        | 0.0690 s     | 18556.4 MB/s     | 0.82x     |
| `crc32c_rs.crc32c`                     | 0.0625 s     | 20486.1 MB/s     | 0.91x     |
| `crc32c_rs.crc32c_fallback`            | 0.3099 s     | 4129.9 MB/s      | 0.18x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 0.0637 s     | 20107.9 MB/s     | 0.89x     |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 0.0593 s     | 21593.1 MB/s     | 0.96x     |
| **`crc32c_sse42_pclmulqdq_v1s4x2` ⭐** | **0.0566 s** | **22605.5 MB/s** | **1.00x** |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 0.0672 s     | 19050.9 MB/s     | 0.84x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 0.0665 s     | 19250.1 MB/s     | 0.85x     |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 0.0772 s     | 16580.3 MB/s     | 0.73x     |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 0.0657 s     | 19492.8 MB/s     | 0.86x     |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 0.0626 s     | 20451.5 MB/s     | 0.90x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 0.0644 s     | 19890.2 MB/s     | 0.88x     |

### 512 MiB

| Library                                | Time         | Throughput       | Relative  |
|----------------------------------------|--------------|------------------|-----------|
| `fastcrc.crc32.iscsi`                  | 10.5936 s    | 483.3 MB/s       | 0.02x     |
| `google_crc32c.value`                  | 0.3583 s     | 14288.3 MB/s     | 0.61x     |
| `crc32c.crc32c`                        | 0.2686 s     | 19062.6 MB/s     | 0.82x     |
| `crc32c_rs.crc32c`                     | 0.2524 s     | 20287.7 MB/s     | 0.87x     |
| `crc32c_rs.crc32c_fallback`            | 1.2442 s     | 4115.1 MB/s      | 0.18x     |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 0.2595 s     | 19729.8 MB/s     | 0.84x     |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 0.2323 s     | 22040.5 MB/s     | 0.94x     |
| **`crc32c_sse42_pclmulqdq_v1s4x2` ⭐** | **0.2191 s** | **23364.5 MB/s** | **1.00x** |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 0.2601 s     | 19683.4 MB/s     | 0.84x     |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 0.2588 s     | 19785.5 MB/s     | 0.85x     |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 0.3018 s     | 16966.0 MB/s     | 0.73x     |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 0.2592 s     | 19756.7 MB/s     | 0.85x     |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 0.2493 s     | 20538.0 MB/s     | 0.88x     |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 0.2494 s     | 20530.4 MB/s     | 0.88x     |
<!-- END BENCHMARK -->
