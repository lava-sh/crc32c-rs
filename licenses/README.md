## Vendor attribution

This project contains code ported from other open-source projects.

All original licenses are included in the `licenses/` directory.

### Ported libraries

#### [fast-crc32][gh-fast-crc32] - original C implementation by [@corsix][gh-corsix]

- License: MIT / zlib
- Source: [https://github.com/corsix/fast-crc32][gh-fast-crc32]
- Ported algorithms:
  - SSE4.2 + PCLMULQDQ (v7s3x3) -> `src/arch/see42_pclmulqdq.rs`
  - AVX512 + PCLMULQDQ (v9s3x4e) -> `src/arch/avx512_pclmulqdq.rs`
  - AVX512 + VPCLMULQDQ (v3s1_s3) -> `src/arch/avx512_vpclmulqdq.rs`
  - AArch64 NEON + CRC + PMULL (v3s4x2e_v2) -> `src/arch/neon64.rs`
  - AArch64 NEON + CRC + PMULL + EOR3/SHA3 (v9s3x2e_s3) -> `src/arch/neon64_sha3.rs`

##### How to update?

Regenerate C code using [fast-crc32 generator][gh-fast-crc32]:

```bash
make generate

./generate -i sse -p crc32c -a v7s3x3
./generate -i avx512_pclmulqdq -p crc32c -a v7s3x3
./generate -i avx512_vpclmulqdq -p crc32c -a v3s1_s3
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
