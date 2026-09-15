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

| Property   | Value                                               |
|------------|-----------------------------------------------------|
| **OS**     | Windows-2025Server-10.0.26100-SP0                   |
| **CPU**    | Intel64 Family 6 Model 140 Stepping 1, GenuineIntel |
| **Python** | 3.14.7 (64-bit) revision 823f032                    |
| **Timer**  | QueryPerformanceCounter(), resolution: 100 ns       |

### 32 B

| Library                                | Time          | Throughput     | Relative   |
|----------------------------------------|---------------|----------------|------------|
| **`crc32c_sse42_pclmulqdq_v7s3x3` ⭐** | **0.0579 µs** | **526.8 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 0.0581 µs     | 525.0 MB/s     | 1.00x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 0.0582 µs     | 524.1 MB/s     | 0.99x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 0.0584 µs     | 522.4 MB/s     | 0.99x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 0.0585 µs     | 522.0 MB/s     | 0.99x      |
| `crc32c_rs.crc32c_fallback`            | 0.0586 µs     | 520.9 MB/s     | 0.99x      |
| `crc32c_rs.crc32c`                     | 0.0589 µs     | 517.9 MB/s     | 0.98x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 0.0590 µs     | 517.3 MB/s     | 0.98x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`        | 0.0594 µs     | 514.0 MB/s     | 0.98x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 0.0594 µs     | 513.9 MB/s     | 0.98x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 0.0596 µs     | 512.5 MB/s     | 0.97x      |
| `google_crc32c.value`                  | 0.0734 µs     | 415.5 MB/s     | 0.79x      |
| `crc32c.crc32c`                        | 0.0794 µs     | 384.3 MB/s     | 0.73x      |
| `fastcrc.crc32.iscsi`                  | 0.1576 µs     | 193.6 MB/s     | 0.37x      |

### 512 B

| Library                                     | Time          | Throughput      | Relative   |
|---------------------------------------------|---------------|-----------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` ⭐** | **0.0717 µs** | **6807.6 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 0.0742 µs     | 6577.2 MB/s     | 0.97x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 0.0786 µs     | 6215.6 MB/s     | 0.91x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 0.0830 µs     | 5885.6 MB/s     | 0.86x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 0.0909 µs     | 5372.5 MB/s     | 0.79x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 0.0910 µs     | 5363.5 MB/s     | 0.79x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 0.0912 µs     | 5356.1 MB/s     | 0.79x      |
| `crc32c_rs.crc32c`                          | 0.0926 µs     | 5274.0 MB/s     | 0.77x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 0.0984 µs     | 4962.4 MB/s     | 0.73x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 0.0996 µs     | 4901.7 MB/s     | 0.72x      |
| `google_crc32c.value`                       | 0.1061 µs     | 4604.0 MB/s     | 0.68x      |
| `crc32c.crc32c`                             | 0.1116 µs     | 4375.9 MB/s     | 0.64x      |
| `crc32c_rs.crc32c_fallback`                 | 0.1856 µs     | 2631.2 MB/s     | 0.39x      |
| `fastcrc.crc32.iscsi`                       | 1.2926 µs     | 377.7 MB/s      | 0.06x      |

### 1 KiB

| Library                                     | Time          | Throughput       | Relative   |
|---------------------------------------------|---------------|------------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` ⭐** | **0.0801 µs** | **12195.7 MB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 0.1009 µs     | 9680.7 MB/s      | 0.79x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 0.1028 µs     | 9499.9 MB/s      | 0.78x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 0.1030 µs     | 9481.5 MB/s      | 0.78x      |
| `google_crc32c.value`                       | 0.1036 µs     | 9422.4 MB/s      | 0.77x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 0.1044 µs     | 9352.3 MB/s      | 0.77x      |
| `crc32c_rs.crc32c`                          | 0.1089 µs     | 8964.3 MB/s      | 0.74x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 0.1100 µs     | 8876.6 MB/s      | 0.73x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 0.1142 µs     | 8548.3 MB/s      | 0.70x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 0.1155 µs     | 8458.4 MB/s      | 0.69x      |
| `crc32c.crc32c`                             | 0.1192 µs     | 8194.3 MB/s      | 0.67x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 0.1203 µs     | 8120.4 MB/s      | 0.67x      |
| `crc32c_rs.crc32c_fallback`                 | 0.3005 µs     | 3249.4 MB/s      | 0.27x      |
| `fastcrc.crc32.iscsi`                       | 2.4768 µs     | 394.3 MB/s       | 0.03x      |

### 64 KiB

| Library                                     | Time          | Throughput       | Relative   |
|---------------------------------------------|---------------|------------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` ⭐** | **0.7930 µs** | **78814.7 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 1.0289 µs     | 60745.0 MB/s     | 0.77x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 1.0997 µs     | 56832.5 MB/s     | 0.72x      |
| `crc32c_rs.crc32c`                          | 1.1326 µs     | 55184.0 MB/s     | 0.70x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 1.6540 µs     | 37786.7 MB/s     | 0.48x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 1.7845 µs     | 35023.9 MB/s     | 0.44x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 1.9016 µs     | 32867.1 MB/s     | 0.42x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 2.0765 µs     | 30098.7 MB/s     | 0.38x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 2.1723 µs     | 28771.1 MB/s     | 0.37x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 2.2576 µs     | 27683.9 MB/s     | 0.35x      |
| `google_crc32c.value`                       | 2.5950 µs     | 24084.6 MB/s     | 0.31x      |
| `crc32c.crc32c`                             | 2.9249 µs     | 21368.0 MB/s     | 0.27x      |
| `crc32c_rs.crc32c_fallback`                 | 17.0741 µs    | 3660.5 MB/s      | 0.05x      |
| `fastcrc.crc32.iscsi`                       | 152.9782 µs   | 408.6 MB/s       | 0.01x      |

### 512 KiB

| Library                                     | Time          | Throughput        | Relative   |
|---------------------------------------------|---------------|-------------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` ⭐** | **4.9894 µs** | **100213.1 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 6.4720 µs     | 77255.7 MB/s      | 0.77x      |
| `crc32c_rs.crc32c`                          | 6.5046 µs     | 76868.5 MB/s      | 0.77x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 7.4246 µs     | 67343.4 MB/s      | 0.67x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 11.6787 µs    | 42813.0 MB/s      | 0.43x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 12.0069 µs    | 41642.6 MB/s      | 0.42x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 12.5568 µs    | 39819.1 MB/s      | 0.40x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 14.6194 µs    | 34201.2 MB/s      | 0.34x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 15.5197 µs    | 32217.2 MB/s      | 0.32x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 15.8942 µs    | 31458.0 MB/s      | 0.31x      |
| `crc32c.crc32c`                             | 19.6433 µs    | 25454.0 MB/s      | 0.25x      |
| `google_crc32c.value`                       | 24.5221 µs    | 20389.8 MB/s      | 0.20x      |
| `crc32c_rs.crc32c_fallback`                 | 128.7177 µs   | 3884.5 MB/s       | 0.04x      |
| `fastcrc.crc32.iscsi`                       | 1219.5592 µs  | 410.0 MB/s        | 0.00x      |

### 1 MiB

| Library                                     | Time          | Throughput        | Relative   |
|---------------------------------------------|---------------|-------------------|------------|
| **`crc32c_avx512vl_vpclmulqdq_v3s1_s3` ⭐** | **9.7826 µs** | **102221.9 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`         | 12.6561 µs    | 79013.3 MB/s      | 0.77x      |
| `crc32c_rs.crc32c`                          | 12.7017 µs    | 78729.5 MB/s      | 0.77x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`         | 14.6068 µs    | 68461.3 MB/s      | 0.67x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`         | 23.2691 µs    | 42975.4 MB/s      | 0.42x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`             | 23.6272 µs    | 42324.1 MB/s      | 0.41x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`             | 25.0039 µs    | 39993.8 MB/s      | 0.39x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`             | 29.0401 µs    | 34435.1 MB/s      | 0.34x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`             | 30.9586 µs    | 32301.2 MB/s      | 0.32x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`             | 31.5883 µs    | 31657.3 MB/s      | 0.31x      |
| `crc32c.crc32c`                             | 38.9708 µs    | 25660.2 MB/s      | 0.25x      |
| `google_crc32c.value`                       | 79.3531 µs    | 12601.9 MB/s      | 0.12x      |
| `crc32c_rs.crc32c_fallback`                 | 256.8827 µs   | 3892.8 MB/s       | 0.04x      |
| `fastcrc.crc32.iscsi`                       | 2425.0191 µs  | 412.4 MB/s        | 0.00x      |

### 16 MiB

| Library                                    | Time            | Throughput       | Relative   |
|--------------------------------------------|-----------------|------------------|------------|
| **`crc32c_avx512vl_pclmulqdq_v9s3x4e` ⭐** | **469.4988 µs** | **34078.9 MB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 474.6545 µs     | 33708.7 MB/s     | 0.99x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 478.8302 µs     | 33414.8 MB/s     | 0.98x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 494.1993 µs     | 32375.6 MB/s     | 0.95x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 506.2562 µs     | 31604.6 MB/s     | 0.93x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`        | 506.7061 µs     | 31576.5 MB/s     | 0.93x      |
| `crc32c_rs.crc32c`                         | 508.0142 µs     | 31495.2 MB/s     | 0.92x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 508.3913 µs     | 31471.8 MB/s     | 0.92x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`        | 508.4388 µs     | 31468.9 MB/s     | 0.92x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 522.1822 µs     | 30640.6 MB/s     | 0.90x      |
| `crc32c.crc32c`                            | 646.9202 µs     | 24732.6 MB/s     | 0.73x      |
| `google_crc32c.value`                      | 2309.2751 µs    | 6928.6 MB/s      | 0.20x      |
| `crc32c_rs.crc32c_fallback`                | 4140.4928 µs    | 3864.3 MB/s      | 0.11x      |
| `fastcrc.crc32.iscsi`                      | 38785.4879 µs   | 412.5 MB/s       | 0.01x      |

### 32 MiB

| Library                                    | Time            | Throughput       | Relative   |
|--------------------------------------------|-----------------|------------------|------------|
| **`crc32c_avx512vl_pclmulqdq_v9s3x4e` ⭐** | **938.5954 µs** | **34093.5 MB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v7s3x3`            | 957.9953 µs     | 33403.1 MB/s     | 0.98x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`            | 960.8313 µs     | 33304.5 MB/s     | 0.98x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`            | 985.7993 µs     | 32461.0 MB/s     | 0.95x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`            | 1008.2432 µs    | 31738.4 MB/s     | 0.93x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`        | 1028.1957 µs    | 31122.5 MB/s     | 0.91x      |
| `crc32c_rs.crc32c`                         | 1031.0352 µs    | 31036.8 MB/s     | 0.91x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`        | 1034.2888 µs    | 30939.1 MB/s     | 0.91x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`       | 1036.1362 µs    | 30884.0 MB/s     | 0.91x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`            | 1039.0897 µs    | 30796.2 MB/s     | 0.90x      |
| `crc32c.crc32c`                            | 1289.5545 µs    | 24814.8 MB/s     | 0.73x      |
| `google_crc32c.value`                      | 4699.1857 µs    | 6809.7 MB/s      | 0.20x      |
| `crc32c_rs.crc32c_fallback`                | 8231.6567 µs    | 3887.4 MB/s      | 0.11x      |
| `fastcrc.crc32.iscsi`                      | 77569.6208 µs   | 412.5 MB/s       | 0.01x      |

### 64 MiB

| Library                                | Time             | Throughput       | Relative   |
|----------------------------------------|------------------|------------------|------------|
| **`crc32c_sse42_pclmulqdq_v7s3x3` ⭐** | **1975.4668 µs** | **32397.4 MB/s** | **1.00x**  |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 1980.4982 µs     | 32315.1 MB/s     | 1.00x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 1991.4385 µs     | 32137.6 MB/s     | 0.99x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`        | 2030.2861 µs     | 31522.7 MB/s     | 0.97x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 2056.4952 µs     | 31120.9 MB/s     | 0.96x      |
| `crc32c_rs.crc32c`                     | 2062.2238 µs     | 31034.5 MB/s     | 0.96x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 2121.6454 µs     | 30165.3 MB/s     | 0.93x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 2123.5253 µs     | 30138.6 MB/s     | 0.93x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 2123.6542 µs     | 30136.7 MB/s     | 0.93x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 2137.8919 µs     | 29936.0 MB/s     | 0.92x      |
| `crc32c.crc32c`                        | 2598.5494 µs     | 24629.1 MB/s     | 0.76x      |
| `google_crc32c.value`                  | 9627.8564 µs     | 6647.4 MB/s      | 0.21x      |
| `crc32c_rs.crc32c_fallback`            | 17038.8721 µs    | 3756.1 MB/s      | 0.12x      |
| `fastcrc.crc32.iscsi`                  | 154922.5383 µs   | 413.1 MB/s       | 0.01x      |

### 128 MiB

| Library                                | Time             | Throughput       | Relative   |
|----------------------------------------|------------------|------------------|------------|
| **`crc32c_sse42_pclmulqdq_v1s4x2` ⭐** | **4566.5220 µs** | **28030.1 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`    | 4949.4185 µs     | 25861.6 MB/s     | 0.92x      |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`    | 5053.4914 µs     | 25329.0 MB/s     | 0.90x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`        | 5101.6107 µs     | 25090.1 MB/s     | 0.90x      |
| `crc32c_rs.crc32c`                     | 5131.6498 µs     | 24943.2 MB/s     | 0.89x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`        | 5983.3615 µs     | 21392.7 MB/s     | 0.76x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`    | 6115.7387 µs     | 20929.6 MB/s     | 0.75x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3`   | 6196.1632 µs     | 20657.9 MB/s     | 0.74x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`        | 7032.8166 µs     | 18200.4 MB/s     | 0.65x      |
| `crc32c.crc32c`                        | 7083.2825 µs     | 18070.7 MB/s     | 0.64x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`        | 7428.5848 µs     | 17230.7 MB/s     | 0.61x      |
| `google_crc32c.value`                  | 19267.8152 µs    | 6643.2 MB/s      | 0.24x      |
| `crc32c_rs.crc32c_fallback`            | 35417.4704 µs    | 3614.0 MB/s      | 0.13x      |
| `fastcrc.crc32.iscsi`                  | 316146.2217 µs   | 404.9 MB/s       | 0.01x      |

### 256 MiB

| Library                              | Time              | Throughput       | Relative   |
|--------------------------------------|-------------------|------------------|------------|
| **`crc32c_rs.crc32c` ⭐**            | **16813.9240 µs** | **15225.5 MB/s** | **1.00x**  |
| `crc32c_avx512vl_vpclmulqdq_v4s5x3`  | 17042.4869 µs     | 15021.3 MB/s     | 0.99x      |
| `crc32c_avx512vl_vpclmulqdq_v3s2x4`  | 17576.2067 µs     | 14565.1 MB/s     | 0.96x      |
| `crc32c_sse42_pclmulqdq_v1s4x2`      | 18926.5863 µs     | 13525.9 MB/s     | 0.89x      |
| `crc32c_sse42_pclmulqdq_v1s3x3`      | 19031.7115 µs     | 13451.2 MB/s     | 0.88x      |
| `crc32c_avx512vl_vpclmulqdq_v3s1_s3` | 19638.8408 µs     | 13035.4 MB/s     | 0.86x      |
| `crc32c_sse42_pclmulqdq_v1s3x2`      | 19808.5281 µs     | 12923.7 MB/s     | 0.85x      |
| `crc32c_avx512vl_pclmulqdq_v9s3x4e`  | 19946.6854 µs     | 12834.2 MB/s     | 0.84x      |
| `crc32c.crc32c`                      | 20785.2554 µs     | 12316.4 MB/s     | 0.81x      |
| `crc32c_sse42_pclmulqdq_v8s3x3`      | 21423.5633 µs     | 11949.5 MB/s     | 0.78x      |
| `crc32c_sse42_pclmulqdq_v7s3x3`      | 21595.7531 µs     | 11854.2 MB/s     | 0.78x      |
| `google_crc32c.value`                | 33256.9083 µs     | 7697.6 MB/s      | 0.51x      |
| `crc32c_rs.crc32c_fallback`          | 69100.6700 µs     | 3704.7 MB/s      | 0.24x      |
| `fastcrc.crc32.iscsi`                | 625749.8450 µs    | 409.1 MB/s       | 0.03x      |
<!-- END BENCHMARK -->
