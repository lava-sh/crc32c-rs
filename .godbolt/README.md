## `.godbolt/`

Each directory contains standalone `file.c` and `file.rs` files used when
porting code from C to Rust and checking the generated assembly on
[Compiler Explorer](https://godbolt.org).

The first two lines of every file are required:

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

- [avx512_pclmulqdq](https://godbolt.org/z/znY6Eee38)
- [avx512_vpclmulqdq](https://godbolt.org/z/7zhGr9M7b)
- [neon64](https://godbolt.org/z/jK9an5sYx)
- [neon64_sha3](https://godbolt.org/z/4KKrKWKv4)
- [see42_pclmulqdq](https://godbolt.org/z/c8ov13bo9)

If the short links stop working, regenerate them:

```console
uv run .godbolt/godbolt_links.py
```
