## `.godbolt/`

Each implementation directory contains a standalone `file.c` and `file.rs` pair used when porting code from C to Rust and comparing generated assembly on [Compiler Explorer](https://godbolt.org). The link generator searches recursively, so architecture and implementation directories may be nested below `.godbolt/`.

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

- [avx512vl_vpclmulqdq-v3s2x4](https://godbolt.org/z/s5MY174rh)
- [sse42_pclmulqdq-v1s3x2](https://godbolt.org/z/WsWzGsczY)
- [sse42_pclmulqdq-v1s3x3](https://godbolt.org/z/qTa93j3o8)
- [sse42_pclmulqdq-v1s4x2](https://godbolt.org/z/qaxWKahen)
- [sse42_s3k4096e](https://godbolt.org/z/x1fMnfjEo)

### ARM

- [aes_crc-v12e_v1](https://godbolt.org/z/dej4ds6cq)
- [aes_crc-v3s4x2e_v2](https://godbolt.org/z/fMvYzrTEs)
- [aes_crc_sha3-v9s3x2e_s3](https://godbolt.org/z/T73Mvz6Wf)
- [crc_neon-s3k95760_s3](https://godbolt.org/z/53TseEcYe)

### Intel x86-64

- [avx512vl_pclmulqdq-v9s3x4e](https://godbolt.org/z/j6s9bYfsv)
- [avx512vl_vpclmulqdq-v3s1_s3](https://godbolt.org/z/nWKfWr4cr)
- [avx512vl_vpclmulqdq-v4s5x3](https://godbolt.org/z/3eGWee1zs)
- [sse42_pclmulqdq-v7s3x3](https://godbolt.org/z/ncEbz9Me6)
- [sse42_pclmulqdq-v8s3x3](https://godbolt.org/z/e3rqz6caq)

If the short links stop working, regenerate them from the repository root:

```console
uv run .godbolt/godbolt_links.py
```
