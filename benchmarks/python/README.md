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

## Run `benchmarks/python/run.py`

```bash
cd benchmarks/python
python run.py -o results.json
# generate markdown tables
python md_table.py results.json
```

## Results

<!-- BEGIN BENCHMARK -->
### System Information

| Property   | Value                                          |
|------------|------------------------------------------------|
| **OS**     | Windows-10-10.0.19045-SP0                      |
| **CPU**    | 11th Gen Intel(R) Core(TM) i5-11300H @ 3.10GHz |
| **Python** | 3.14.7                                         |
| **Timer**  | QueryPerformanceCounter(), resolution: 100 ns  |

<details open>
<summary>Default benchmark</summary>

### 32 B

| Library                      | Time          | Throughput     | Relative   |
|------------------------------|---------------|----------------|------------|
| **`fastcrc.crc32.iscsi` 🥇** | **0.0623 µs** | **489.5 MB/s** | **1.00x**  |
| `crc32c_rs.crc32c`           | 0.0633 µs     | 482.3 MB/s     | 0.99x      |
| `google_crc32c.value`        | 0.0714 µs     | 427.1 MB/s     | 0.87x      |
| `crc32c.crc32c`              | 0.0782 µs     | 390.1 MB/s     | 0.80x      |

### 512 B

| Library                   | Time          | Throughput   | Relative   |
|---------------------------|---------------|--------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **0.0741 µs** | **6.4 GB/s** | **1.00x**  |
| `fastcrc.crc32.iscsi`     | 0.0764 µs     | 6.2 GB/s     | 0.97x      |
| `google_crc32c.value`     | 0.1034 µs     | 4.6 GB/s     | 0.72x      |
| `crc32c.crc32c`           | 0.1107 µs     | 4.3 GB/s     | 0.67x      |

### 1 KB

| Library                      | Time          | Throughput    | Relative   |
|------------------------------|---------------|---------------|------------|
| **`fastcrc.crc32.iscsi` 🥇** | **0.0941 µs** | **10.1 GB/s** | **1.00x**  |
| `crc32c_rs.crc32c`           | 0.1015 µs     | 9.4 GB/s      | 0.93x      |
| `google_crc32c.value`        | 0.1071 µs     | 8.9 GB/s      | 0.88x      |
| `crc32c.crc32c`              | 0.1159 µs     | 8.2 GB/s      | 0.81x      |

### 64 KB

| Library                   | Time          | Throughput    | Relative   |
|---------------------------|---------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **1.0313 µs** | **59.2 GB/s** | **1.00x**  |
| `fastcrc.crc32.iscsi`     | 1.1481 µs     | 53.2 GB/s     | 0.90x      |
| `crc32c.crc32c`           | 2.2574 µs     | 27.0 GB/s     | 0.46x      |
| `google_crc32c.value`     | 4.1228 µs     | 14.8 GB/s     | 0.25x      |

### 512 KB

| Library                   | Time          | Throughput    | Relative   |
|---------------------------|---------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **6.4596 µs** | **75.6 GB/s** | **1.00x**  |
| `fastcrc.crc32.iscsi`     | 8.1096 µs     | 60.2 GB/s     | 0.80x      |
| `crc32c.crc32c`           | 15.6075 µs    | 31.3 GB/s     | 0.41x      |
| `google_crc32c.value`     | 42.4765 µs    | 11.5 GB/s     | 0.15x      |

### 1 MB

| Library                   | Time           | Throughput    | Relative   |
|---------------------------|----------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **12.6453 µs** | **77.2 GB/s** | **1.00x**  |
| `fastcrc.crc32.iscsi`     | 15.9984 µs     | 61.0 GB/s     | 0.79x      |
| `crc32c.crc32c`           | 30.9808 µs     | 31.5 GB/s     | 0.41x      |
| `google_crc32c.value`     | 82.1534 µs     | 11.9 GB/s     | 0.15x      |

### 16 MB

| Library                   | Time            | Throughput    | Relative   |
|---------------------------|-----------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **627.9698 µs** | **24.9 GB/s** | **1.00x**  |
| `crc32c.crc32c`           | 656.9574 µs     | 23.8 GB/s     | 0.96x      |
| `fastcrc.crc32.iscsi`     | 795.1731 µs     | 19.6 GB/s     | 0.79x      |
| `google_crc32c.value`     | 1565.4902 µs    | 10.0 GB/s     | 0.40x      |

### 32 MB

| Library                   | Time             | Throughput    | Relative   |
|---------------------------|------------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **1341.7847 µs** | **23.3 GB/s** | **1.00x**  |
| `crc32c.crc32c`           | 1372.5685 µs     | 22.8 GB/s     | 0.98x      |
| `fastcrc.crc32.iscsi`     | 1720.7099 µs     | 18.2 GB/s     | 0.78x      |
| `google_crc32c.value`     | 2271.1680 µs     | 13.8 GB/s     | 0.59x      |

### 64 MB

| Library                   | Time             | Throughput    | Relative   |
|---------------------------|------------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **2855.3065 µs** | **21.9 GB/s** | **1.00x**  |
| `crc32c.crc32c`           | 3139.9890 µs     | 19.9 GB/s     | 0.91x      |
| `fastcrc.crc32.iscsi`     | 3330.8735 µs     | 18.8 GB/s     | 0.86x      |
| `google_crc32c.value`     | 3940.4863 µs     | 15.9 GB/s     | 0.72x      |

### 128 MB

| Library                   | Time             | Throughput    | Relative   |
|---------------------------|------------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **5880.4603 µs** | **21.3 GB/s** | **1.00x**  |
| `crc32c.crc32c`           | 6476.5202 µs     | 19.3 GB/s     | 0.91x      |
| `fastcrc.crc32.iscsi`     | 6859.3068 µs     | 18.2 GB/s     | 0.86x      |
| `google_crc32c.value`     | 8190.3155 µs     | 15.3 GB/s     | 0.72x      |

### 256 MB

| Library                   | Time              | Throughput    | Relative   |
|---------------------------|-------------------|---------------|------------|
| **`crc32c_rs.crc32c` 🥇** | **11859.7002 µs** | **21.1 GB/s** | **1.00x**  |
| `crc32c.crc32c`           | 12987.3793 µs     | 19.2 GB/s     | 0.91x      |
| `fastcrc.crc32.iscsi`     | 13773.1654 µs     | 18.2 GB/s     | 0.86x      |
| `google_crc32c.value`     | 16212.3140 µs     | 15.4 GB/s     | 0.73x      |


</details>

<details>
<summary>Detailed benchmark</summary>

### 32 B

| Library                                | Time          | Throughput     | Relative   |
|----------------------------------------|---------------|----------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s3x3` 🥇** | **0.0592 µs** | **515.3 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 0.0594 µs     | 514.1 MB/s     | 1.00x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 0.0598 µs     | 510.7 MB/s     | 0.99x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 0.0600 µs     | 508.8 MB/s     | 0.99x      |
| `crc32c_sse42`                         | 0.0601 µs     | 507.8 MB/s     | 0.99x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 0.0611 µs     | 499.1 MB/s     | 0.97x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 0.0612 µs     | 498.8 MB/s     | 0.97x      |
| `fastcrc.crc32.iscsi`                  | 0.0623 µs     | 489.5 MB/s     | 0.95x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`        | 0.0629 µs     | 484.9 MB/s     | 0.94x      |
| `crc32c_rs.crc32c`                     | 0.0633 µs     | 482.3 MB/s     | 0.94x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 0.0638 µs     | 478.1 MB/s     | 0.93x      |
| `crc32c_rs.crc32c_fallback`            | 0.0652 µs     | 468.1 MB/s     | 0.91x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 0.0664 µs     | 459.3 MB/s     | 0.89x      |
| `google_crc32c.value`                  | 0.0714 µs     | 427.1 MB/s     | 0.83x      |
| `crc32c.crc32c`                        | 0.0782 µs     | 390.1 MB/s     | 0.76x      |

### 512 B

| Library                              | Time          | Throughput   | Relative   |
|--------------------------------------|---------------|--------------|------------|
| **`crc32c_sse42` 🥇**                | **0.0735 µs** | **6.5 GB/s** | **1.00x**  |
| `crc32c_rs.crc32c`                   | 0.0741 µs     | 6.4 GB/s     | 0.99x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`  | 0.0746 µs     | 6.4 GB/s     | 0.99x      |
| `fastcrc.crc32.iscsi`                | 0.0764 µs     | 6.2 GB/s     | 0.96x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3` | 0.0795 µs     | 6.0 GB/s     | 0.92x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`  | 0.0829 µs     | 5.8 GB/s     | 0.89x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`      | 0.0870 µs     | 5.5 GB/s     | 0.85x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`      | 0.0945 µs     | 5.0 GB/s     | 0.78x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`  | 0.0974 µs     | 4.9 GB/s     | 0.76x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`      | 0.0984 µs     | 4.8 GB/s     | 0.75x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`      | 0.1013 µs     | 4.7 GB/s     | 0.73x      |
| `google_crc32c.value`                | 0.1034 µs     | 4.6 GB/s     | 0.71x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`      | 0.1055 µs     | 4.5 GB/s     | 0.70x      |
| `crc32c.crc32c`                      | 0.1107 µs     | 4.3 GB/s     | 0.66x      |
| `crc32c_rs.crc32c_fallback`          | 0.1664 µs     | 2.9 GB/s     | 0.44x      |

### 1 KB

| Library                                     | Time          | Throughput    | Relative   |
|---------------------------------------------|---------------|---------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` 🥇** | **0.0864 µs** | **11.0 GB/s** | **1.00x**  |
| `crc32c_sse42`                              | 0.0911 µs     | 10.5 GB/s     | 0.95x      |
| `fastcrc.crc32.iscsi`                       | 0.0941 µs     | 10.1 GB/s     | 0.92x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 0.0988 µs     | 9.6 GB/s      | 0.87x      |
| `crc32c_rs.crc32c`                          | 0.1015 µs     | 9.4 GB/s      | 0.85x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 0.1066 µs     | 8.9 GB/s      | 0.81x      |
| `google_crc32c.value`                       | 0.1071 µs     | 8.9 GB/s      | 0.81x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 0.1097 µs     | 8.7 GB/s      | 0.79x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 0.1107 µs     | 8.6 GB/s      | 0.78x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 0.1156 µs     | 8.3 GB/s      | 0.75x      |
| `crc32c.crc32c`                             | 0.1159 µs     | 8.2 GB/s      | 0.75x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 0.1178 µs     | 8.1 GB/s      | 0.73x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 0.1189 µs     | 8.0 GB/s      | 0.73x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 0.1238 µs     | 7.7 GB/s      | 0.70x      |
| `crc32c_rs.crc32c_fallback`                 | 0.2725 µs     | 3.5 GB/s      | 0.32x      |

### 64 KB

| Library                                    | Time          | Throughput    | Relative   |
|--------------------------------------------|---------------|---------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s2x4` 🥇** | **1.0172 µs** | **60.0 GB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`        | 1.0300 µs     | 59.3 GB/s     | 0.99x      |
| `crc32c_rs.crc32c`                         | 1.0313 µs     | 59.2 GB/s     | 0.99x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 1.1361 µs     | 53.7 GB/s     | 0.90x      |
| `fastcrc.crc32.iscsi`                      | 1.1481 µs     | 53.2 GB/s     | 0.89x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`        | 1.3509 µs     | 45.2 GB/s     | 0.75x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 1.3735 µs     | 44.4 GB/s     | 0.74x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 1.4464 µs     | 42.2 GB/s     | 0.70x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 1.7596 µs     | 34.7 GB/s     | 0.58x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 1.7729 µs     | 34.4 GB/s     | 0.57x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 2.1587 µs     | 28.3 GB/s     | 0.47x      |
| `crc32c_sse42`                             | 2.2165 µs     | 27.5 GB/s     | 0.46x      |
| `crc32c.crc32c`                            | 2.2574 µs     | 27.0 GB/s     | 0.45x      |
| `google_crc32c.value`                      | 4.1228 µs     | 14.8 GB/s     | 0.25x      |
| `crc32c_rs.crc32c_fallback`                | 13.9659 µs    | 4.4 GB/s      | 0.07x      |

### 512 KB

| Library                                    | Time          | Throughput    | Relative   |
|--------------------------------------------|---------------|---------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v4s5x3` 🥇** | **6.4513 µs** | **75.7 GB/s** | **1.00x**  |
| `crc32c_rs.crc32c`                         | 6.4596 µs     | 75.6 GB/s     | 1.00x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`        | 6.6848 µs     | 73.0 GB/s     | 0.97x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 7.8894 µs     | 61.9 GB/s     | 0.82x      |
| `fastcrc.crc32.iscsi`                      | 8.1096 µs     | 60.2 GB/s     | 0.80x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`        | 9.2768 µs     | 52.6 GB/s     | 0.70x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 9.5667 µs     | 51.0 GB/s     | 0.67x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 10.0364 µs    | 48.7 GB/s     | 0.64x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 12.5512 µs    | 38.9 GB/s     | 0.51x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 12.6365 µs    | 38.6 GB/s     | 0.51x      |
| `crc32c_sse42`                             | 15.5008 µs    | 31.5 GB/s     | 0.42x      |
| `crc32c.crc32c`                            | 15.6075 µs    | 31.3 GB/s     | 0.41x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 15.9925 µs    | 30.5 GB/s     | 0.40x      |
| `google_crc32c.value`                      | 42.4765 µs    | 11.5 GB/s     | 0.15x      |
| `crc32c_rs.crc32c_fallback`                | 110.4923 µs   | 4.4 GB/s      | 0.06x      |

### 1 MB

| Library                                    | Time           | Throughput    | Relative   |
|--------------------------------------------|----------------|---------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v4s5x3` 🥇** | **12.6375 µs** | **77.3 GB/s** | **1.00x**  |
| `crc32c_rs.crc32c`                         | 12.6453 µs     | 77.2 GB/s     | 1.00x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`        | 13.1888 µs     | 74.0 GB/s     | 0.96x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 15.6317 µs     | 62.5 GB/s     | 0.81x      |
| `fastcrc.crc32.iscsi`                      | 15.9984 µs     | 61.0 GB/s     | 0.79x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`        | 18.3386 µs     | 53.3 GB/s     | 0.69x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 18.9165 µs     | 51.6 GB/s     | 0.67x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 19.8674 µs     | 49.2 GB/s     | 0.64x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 24.9040 µs     | 39.2 GB/s     | 0.51x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 25.0628 µs     | 39.0 GB/s     | 0.50x      |
| `crc32c_sse42`                             | 30.8826 µs     | 31.6 GB/s     | 0.41x      |
| `crc32c.crc32c`                            | 30.9808 µs     | 31.5 GB/s     | 0.41x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 31.5321 µs     | 31.0 GB/s     | 0.40x      |
| `google_crc32c.value`                      | 82.1534 µs     | 11.9 GB/s     | 0.15x      |
| `crc32c_rs.crc32c_fallback`                | 220.9111 µs    | 4.4 GB/s      | 0.06x      |

### 16 MB

| Library                                | Time            | Throughput    | Relative   |
|----------------------------------------|-----------------|---------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s4x2` 🥇** | **563.0549 µs** | **27.8 GB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 598.6931 µs     | 26.1 GB/s     | 0.94x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 624.6256 µs     | 25.0 GB/s     | 0.90x      |
| `crc32c_rs.crc32c`                     | 627.9698 µs     | 24.9 GB/s     | 0.90x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 640.1733 µs     | 24.4 GB/s     | 0.88x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 650.8747 µs     | 24.0 GB/s     | 0.87x      |
| `crc32c.crc32c`                        | 656.9574 µs     | 23.8 GB/s     | 0.86x      |
| `crc32c_sse42`                         | 657.5249 µs     | 23.8 GB/s     | 0.86x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 682.2219 µs     | 22.9 GB/s     | 0.83x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 687.5498 µs     | 22.7 GB/s     | 0.82x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 698.9156 µs     | 22.4 GB/s     | 0.81x      |
| `fastcrc.crc32.iscsi`                  | 795.1731 µs     | 19.6 GB/s     | 0.71x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 836.1911 µs     | 18.7 GB/s     | 0.67x      |
| `google_crc32c.value`                  | 1565.4902 µs    | 10.0 GB/s     | 0.36x      |
| `crc32c_rs.crc32c_fallback`            | 3655.1055 µs    | 4.3 GB/s      | 0.15x      |

### 32 MB

| Library                                | Time             | Throughput    | Relative   |
|----------------------------------------|------------------|---------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s4x2` 🥇** | **1269.4716 µs** | **24.6 GB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 1295.9324 µs     | 24.1 GB/s     | 0.98x      |
| `crc32c_rs.crc32c`                     | 1341.7847 µs     | 23.3 GB/s     | 0.95x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 1372.3199 µs     | 22.8 GB/s     | 0.93x      |
| `crc32c.crc32c`                        | 1372.5685 µs     | 22.8 GB/s     | 0.92x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 1384.5138 µs     | 22.6 GB/s     | 0.92x      |
| `crc32c_sse42`                         | 1406.3898 µs     | 22.2 GB/s     | 0.90x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 1413.8978 µs     | 22.1 GB/s     | 0.90x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 1433.0792 µs     | 21.8 GB/s     | 0.89x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 1486.0055 µs     | 21.0 GB/s     | 0.85x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 1526.9066 µs     | 20.5 GB/s     | 0.83x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 1678.0252 µs     | 18.6 GB/s     | 0.76x      |
| `fastcrc.crc32.iscsi`                  | 1720.7099 µs     | 18.2 GB/s     | 0.74x      |
| `google_crc32c.value`                  | 2271.1680 µs     | 13.8 GB/s     | 0.56x      |
| `crc32c_rs.crc32c_fallback`            | 7284.7891 µs     | 4.3 GB/s      | 0.17x      |

### 64 MB

| Library                                | Time             | Throughput    | Relative   |
|----------------------------------------|------------------|---------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s4x2` 🥇** | **2574.2294 µs** | **24.3 GB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 2668.2465 µs     | 23.4 GB/s     | 0.96x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 2854.1975 µs     | 21.9 GB/s     | 0.90x      |
| `crc32c_rs.crc32c`                     | 2855.3065 µs     | 21.9 GB/s     | 0.90x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 2861.4258 µs     | 21.8 GB/s     | 0.90x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 2992.2609 µs     | 20.9 GB/s     | 0.86x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 3057.6502 µs     | 20.4 GB/s     | 0.84x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 3078.5018 µs     | 20.3 GB/s     | 0.84x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 3093.1992 µs     | 20.2 GB/s     | 0.83x      |
| `crc32c.crc32c`                        | 3139.9890 µs     | 19.9 GB/s     | 0.82x      |
| `crc32c_sse42`                         | 3143.0708 µs     | 19.9 GB/s     | 0.82x      |
| `fastcrc.crc32.iscsi`                  | 3330.8735 µs     | 18.8 GB/s     | 0.77x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 3400.1911 µs     | 18.4 GB/s     | 0.76x      |
| `google_crc32c.value`                  | 3940.4863 µs     | 15.9 GB/s     | 0.65x      |
| `crc32c_rs.crc32c_fallback`            | 14926.9884 µs    | 4.2 GB/s      | 0.17x      |

### 128 MB

| Library                                | Time             | Throughput    | Relative   |
|----------------------------------------|------------------|---------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s4x2` 🥇** | **5211.9004 µs** | **24.0 GB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 5384.4418 µs     | 23.2 GB/s     | 0.97x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 5877.4237 µs     | 21.3 GB/s     | 0.89x      |
| `crc32c_rs.crc32c`                     | 5880.4603 µs     | 21.3 GB/s     | 0.89x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 5898.1915 µs     | 21.2 GB/s     | 0.88x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 6090.6620 µs     | 20.5 GB/s     | 0.86x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 6184.5908 µs     | 20.2 GB/s     | 0.84x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 6222.9901 µs     | 20.1 GB/s     | 0.84x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 6295.9945 µs     | 19.9 GB/s     | 0.83x      |
| `crc32c.crc32c`                        | 6476.5202 µs     | 19.3 GB/s     | 0.80x      |
| `crc32c_sse42`                         | 6484.3159 µs     | 19.3 GB/s     | 0.80x      |
| `fastcrc.crc32.iscsi`                  | 6859.3068 µs     | 18.2 GB/s     | 0.76x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 7029.2294 µs     | 17.8 GB/s     | 0.74x      |
| `google_crc32c.value`                  | 8190.3155 µs     | 15.3 GB/s     | 0.64x      |
| `crc32c_rs.crc32c_fallback`            | 29851.4575 µs    | 4.2 GB/s      | 0.17x      |

### 256 MB

| Library                                | Time              | Throughput    | Relative   |
|----------------------------------------|-------------------|---------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s4x2` 🥇** | **10409.2583 µs** | **24.0 GB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 10632.7283 µs     | 23.5 GB/s     | 0.98x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 11823.9811 µs     | 21.1 GB/s     | 0.88x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 11839.8268 µs     | 21.1 GB/s     | 0.88x      |
| `crc32c_rs.crc32c`                     | 11859.7002 µs     | 21.1 GB/s     | 0.88x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 11885.3031 µs     | 21.0 GB/s     | 0.88x      |
| `crc32c_sse42`                         | 12273.6834 µs     | 20.4 GB/s     | 0.85x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 12276.7321 µs     | 20.4 GB/s     | 0.85x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 12322.9564 µs     | 20.3 GB/s     | 0.84x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 12346.1316 µs     | 20.2 GB/s     | 0.84x      |
| `crc32c.crc32c`                        | 12987.3793 µs     | 19.2 GB/s     | 0.80x      |
| `fastcrc.crc32.iscsi`                  | 13773.1654 µs     | 18.2 GB/s     | 0.76x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 14128.3981 µs     | 17.7 GB/s     | 0.74x      |
| `google_crc32c.value`                  | 16212.3140 µs     | 15.4 GB/s     | 0.64x      |
| `crc32c_rs.crc32c_fallback`            | 58788.9875 µs     | 4.3 GB/s      | 0.18x      |


</details>
<!-- END BENCHMARK -->
