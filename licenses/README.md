## Vendor attribution

This project contains code ported from other open-source projects.

All original licenses are included in the `licenses/` directory.

### Ported libraries

#### [fast-crc32][gh-fast-crc32] - original C implementation by [@corsix][gh-corsix]

- License: MIT / zlib
- Source: [https://github.com/corsix/fast-crc32][gh-fast-crc32]
- Ported algorithms:
  - SSE4.2 + PCLMULQDQ (`v1s3x2`) -> `src/arch/sse42_pclmulqdq_v1s3x2.rs`
  - SSE4.2 + PCLMULQDQ (`v1s3x3`) -> `src/arch/sse42_pclmulqdq_v1s3x3.rs`
  - SSE4.2 + PCLMULQDQ (`v1s4x2`) -> `src/arch/sse42_pclmulqdq_v1s4x2.rs`
  - SSE4.2 + PCLMULQDQ (`v7s3x3`) -> `src/arch/sse42_pclmulqdq_v7s3x3.rs`
  - SSE4.2 + PCLMULQDQ (`v8s3x3`) -> `src/arch/sse42_pclmulqdq_v8s3x3.rs`
  - AVX-512VL + PCLMULQDQ (`v9s3x4e`) -> `src/arch/avx512vl_pclmulqdq_v9s3x4e.rs`
  - AVX-512VL + VPCLMULQDQ (`v3s1_s3`) -> `src/arch/avx512vl_vpclmulqdq_v3s1_s3.rs`
  - AVX-512VL + VPCLMULQDQ (`v3s2x4`) -> `src/arch/avx512vl_vpclmulqdq_v3s2x4.rs`
  - AVX-512VL + VPCLMULQDQ (`v4s5x3`) -> `src/arch/avx512vl_vpclmulqdq_v4s5x3.rs`
  - AArch64 NEON + CRC + PMULL (`v3s4x2e_v2`) -> `src/arch/aes_v3s4x2e_v2.rs`
  - AArch64 NEON + CRC + PMULL (`v12e_v1`) -> `src/arch/aes_crc_v12e_v1.rs`
  - AArch64 NEON + CRC + PMULL + EOR3/SHA3 (`v9s3x2e_s3`) -> `src/arch/aes_sha3_v9s3x2e_s3.rs`

The suffix follows `corsix/fast-crc32`: `v` is the number of vector
accumulators, `s` is the number of scalar accumulators, `x` is the load ratio,
`k` is an outer-loop block size, and `e` means pointer-based loop termination.
A trailing variant such as `_v1`, `_v2`, or `_s3` names the tail schedule used
after the main schedule.

##### How to update?

Regenerate C code using [fast-crc32 generator][gh-fast-crc32]:

```bash
make generate

# AMD x86-64
./generate -i avx512_vpclmulqdq -p crc32c -a v3s2x4
./generate -i sse -p crc32c -a v1s3x2
./generate -i sse -p crc32c -a v1s3x3
./generate -i sse -p crc32c -a v1s4x2

# Intel x86-64
./generate -i avx512_vpclmulqdq -p crc32c -a v3s1_s3
./generate -i avx512_vpclmulqdq -p crc32c -a v4s5x3
./generate -i avx512 -p crc32c -a v9s3x4e
./generate -i sse -p crc32c -a v7s3x3
./generate -i sse -p crc32c -a v8s3x3

# AArch64
./generate -i neon -p crc32c -a v12e_v1
./generate -i neon -p crc32c -a v3s4x2e_v2
./generate -i neon_eor3 -p crc32c -a v9s3x2e_s3
```

Port to Rust line by line

#### [crc-fast-rust][gh-crc-fast-rust] - original Rust implementation

Original by [@awesomized][gh-awesomized], maintained fork by [@MuntasirSZN][gh-MuntasirSZN].

- License: MIT / zlib / Apache
- Source: [https://github.com/MuntasirSZN/crc-fast-rus][gh-crc-fast-rust]
- Ported algorithms:
  - Scalar (Fallback) -> `src/arch/fallback.rs`

[gh-corsix]: https://github.com/corsix
[gh-awesomized]: https://github.com/awesomized
[gh-MuntasirSZN]: https://github.com/MuntasirSZN

[gh-fast-crc32]: https://github.com/corsix/fast-crc32/tree/13f5289ceb6065d014f9e1e64f161ea7a926c3b7
[gh-crc-fast-rust]: https://github.com/MuntasirSZN/crc-fast-rust/tree/e3f3c613c3e158b2b82d576347ffc6e5e07ac5ba
