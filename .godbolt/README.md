## `.godbolt/`

Each implementation directory contains a standalone `file.c` and `file.rs` pair
used when porting code from C to Rust and comparing generated assembly on
[Compiler Explorer](https://godbolt.org). The link generator searches
recursively, so architecture and implementation directories may be nested
below `.godbolt/`.

The first two lines of every source file are required:

1. compiler and compiler version;
2. compilation flags.

Examples:

```c
// x86-64 clang 23.1.0
// -O3 -msse4.2 -mpclmul -funroll-loops -finline-functions
```

```rust
// rustc nightly
// -C opt-level=3 -C target-feature=+sse4.2,+pclmulqdq
```

I have not found a clear way to run Rust NEON checks on Godbolt yet.

## Generated links

### AMD x86-64

- [avx512vl_vpclmulqdq-v3s2x4](https://godbolt.org/z/x67jsb3sb)
- [sse42_pclmulqdq-v1s3x2](https://godbolt.org/z/6ns61djba)
- [sse42_pclmulqdq-v1s3x3](https://godbolt.org/z/q5hhTxhzE)
- [sse42_pclmulqdq-v1s4x2](https://godbolt.org/z/4qboaz5eG)

### ARM

- [aes_crc-v12e_v1](https://godbolt.org/z/6zb7vso3o)
- [aes_crc-v3s4x2e_v2](https://godbolt.org/z/Tx7TG3Wq3)
- [aes_crc_sha3-v9s3x2e_s3](https://godbolt.org/z/YGTM5z65v)

### Intel x86-64

- [avx512vl_pclmulqdq-v9s3x4e](https://godbolt.org/z/n88MTPqob)
- [avx512vl_vpclmulqdq-v3s1_s3](https://godbolt.org/z/Wc3cnscne)
- [avx512vl_vpclmulqdq-v4s5x3](https://godbolt.org/z/58evh8TPz)
- [sse42_pclmulqdq-v7s3x3](https://godbolt.org/z/cfb6G8hff)
- [sse42_pclmulqdq-v8s3x3](https://godbolt.org/z/aj3vejv9a)

If the short links stop working, regenerate them from the repository root:

```console
uv run .godbolt/godbolt_links.py
```
