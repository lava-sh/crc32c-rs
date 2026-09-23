## `.godbolt/`

Each implementation directory contains a standalone `file.c` and `file.rs` pair used when porting
code from C to Rust and comparing generated assembly on [Compiler Explorer](https://godbolt.org).

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

## Generated links

### AMD x86-64

- [avx512vl_vpclmulqdq-v3s2x4](https://godbolt.org/z/zz7s4jKM1)
- [sse42_pclmulqdq-v1s3x2](https://godbolt.org/z/55Efb41Ma)
- [sse42_pclmulqdq-v1s3x3](https://godbolt.org/z/fbec7qKG5)
- [sse42_pclmulqdq-v1s4x2](https://godbolt.org/z/eaadcv9db)
- [sse42_s3k4096e](https://godbolt.org/z/TPs1e6zss)

### ARM

- [aes_crc-v12e_v1](https://godbolt.org/z/9o9rnrKrx)
- [aes_crc-v3s4x2e_v2](https://godbolt.org/z/3r9sMd3ne)
- [aes_crc_sha3-v9s3x2e_s3](https://godbolt.org/z/6qqrdjMKG)

### Intel x86-64

- [avx512vl_pclmulqdq-v9s3x4e](https://godbolt.org/z/rbTsEbaTc)
- [avx512vl_vpclmulqdq-v3s1_s3](https://godbolt.org/z/8qvn479fv)
- [avx512vl_vpclmulqdq-v4s5x3](https://godbolt.org/z/jeYYv6YPG)
- [sse42](https://godbolt.org/z/7zooT8s7b)
- [sse42_pclmulqdq-v7s3x3](https://godbolt.org/z/jK5zjYqnW)
- [sse42_pclmulqdq-v8s3x3](https://godbolt.org/z/35e553n6G)

If the short links stop working, regenerate them:

```console
uv run .godbolt/godbolt_links.py
```
