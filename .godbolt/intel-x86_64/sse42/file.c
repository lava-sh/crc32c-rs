// x86-64 clang 23.1.0
// -O3 -msse4.2

/* crc32c.c -- compute CRC-32C using the Intel crc32 instruction
 * Copyright (C) 2013, 2015, 2021, 2023 Mark Adler
 * Version 1.5  7 Dec 2023  Mark Adler
 *
 * SIMD-only excerpt of https://github.com/madler/brotli/blob/master/crc32c.c:
 * the software fallback, the SSE 4.2 dispatch and the test harness are
 * dropped, the hardware routine is exported as crc32c_hw.  zlib licensed.
 */

#include <pthread.h>
#include <stddef.h>
#include <stdint.h>

/* CRC-32C (iSCSI) polynomial in reversed bit order. */
#define POLY 0x82f63b78

/* Multiply a matrix times a vector over the Galois field of two elements,
   GF(2).  Each element is a bit in an unsigned integer.  mat must have at
   least as many entries as the power of two for most significant one bit in
   vec. */
static inline uint32_t gf2_matrix_times(uint32_t *mat, uint32_t vec) {
    uint32_t sum = 0;
    while (vec) {
        if (vec & 1)
            sum ^= *mat;
        vec >>= 1;
        mat++;
    }
    return sum;
}

/* Multiply a matrix by itself over GF(2).  Both mat and square must have 32
   rows. */
static inline void gf2_matrix_square(uint32_t *square, uint32_t *mat) {
    for (unsigned n = 0; n < 32; n++)
        square[n] = gf2_matrix_times(mat, mat[n]);
}

/* Construct an operator to apply len zeros to a crc.  len must be a power of
   two.  If len is not a power of two, then the result is the same as for the
   largest power of two less than len.  The result for len == 0 is the same as
   for len == 1.  A version of this routine could be easily written for any
   len, but that is not needed for this application. */
static void crc32c_zeros_op(uint32_t *even, size_t len) {
    uint32_t odd[32];       /* odd-power-of-two zeros operator */

    /* put operator for one zero bit in odd */
    odd[0] = POLY;              /* CRC-32C polynomial */
    uint32_t row = 1;
    for (unsigned n = 1; n < 32; n++) {
        odd[n] = row;
        row <<= 1;
    }

    /* put operator for two zero bits in even */
    gf2_matrix_square(even, odd);

    /* put operator for four zero bits in odd */
    gf2_matrix_square(odd, even);

    /* first square will put the operator for one zero byte (eight zero bits),
       in even -- next square puts operator for two zero bytes in odd, and so
       on, until len has been rotated down to zero */
    do {
        gf2_matrix_square(even, odd);
        len >>= 1;
        if (len == 0)
            return;
        gf2_matrix_square(odd, even);
        len >>= 1;
    } while (len);

    /* answer ended up in odd -- copy to even */
    for (unsigned n = 0; n < 32; n++)
        even[n] = odd[n];
}

/* Take a length and build four lookup tables for applying the zeros operator
   for that length, byte-by-byte on the operand. */
static void crc32c_zeros(uint32_t zeros[][256], size_t len) {
    uint32_t op[32];

    crc32c_zeros_op(op, len);
    for (unsigned n = 0; n < 256; n++) {
        zeros[0][n] = gf2_matrix_times(op, n);
        zeros[1][n] = gf2_matrix_times(op, n << 8);
        zeros[2][n] = gf2_matrix_times(op, n << 16);
        zeros[3][n] = gf2_matrix_times(op, n << 24);
    }
}

/* Apply the zeros operator table to crc. */
static inline uint32_t crc32c_shift(uint32_t zeros[][256], uint32_t crc) {
    return zeros[0][crc & 0xff] ^ zeros[1][(crc >> 8) & 0xff] ^
           zeros[2][(crc >> 16) & 0xff] ^ zeros[3][crc >> 24];
}

/* Block sizes for three-way parallel crc computation.  LONG and SHORT must
   both be powers of two. */
#define LONG 8192
#define SHORT 256

/* Tables for hardware crc that shift a crc by LONG and SHORT zeros. */
static pthread_once_t crc32c_once_hw = PTHREAD_ONCE_INIT;
static uint32_t crc32c_long[4][256];
static uint32_t crc32c_short[4][256];

/* Initialize tables for shifting crcs. */
static void crc32c_init_hw(void) {
    crc32c_zeros(crc32c_long, LONG);
    crc32c_zeros(crc32c_short, SHORT);
}

/* Compute CRC-32C using the Intel hardware instruction. */
uint32_t crc32c_hw(uint32_t crc, void const *buf, size_t len) {
    /* populate shift tables the first time through */
    pthread_once(&crc32c_once_hw, crc32c_init_hw);

    /* pre-process the crc */
    crc = ~crc;
    uint64_t crc0 = crc;            /* 64-bits for crc32q instruction */

    /* compute the crc for up to seven leading bytes to bring the data pointer
       to an eight-byte boundary */
    unsigned char const *next = buf;
    while (len && ((uintptr_t)next & 7) != 0) {
        __asm__("crc32b\t%1, %0"
                : "+r"(crc0)
                : "m"(*next));
        next++;
        len--;
    }

    /* compute the crc on sets of LONG*3 bytes, executing three independent crc
       instructions, each on LONG bytes -- this is optimized for the Nehalem,
       Westmere, Sandy Bridge, and Ivy Bridge architectures, which have a
       throughput of one crc per cycle, but a latency of three cycles */
    while (len >= LONG*3) {
        uint64_t crc1 = 0;
        uint64_t crc2 = 0;
        unsigned char const * const end = next + LONG;
        do {
            __asm__("crc32q\t%3, %0\n\t"
                    "crc32q\t%4, %1\n\t"
                    "crc32q\t%5, %2"
                    : "+r"(crc0), "+r"(crc1), "+r"(crc2)
                    : "m"(*next), "m"(next[LONG]), "m"(next[2*LONG]));
            next += 8;
        } while (next < end);
        crc0 = crc32c_shift(crc32c_long, crc0) ^ crc1;
        crc0 = crc32c_shift(crc32c_long, crc0) ^ crc2;
        next += LONG*2;
        len -= LONG*3;
    }

    /* do the same thing, but now on SHORT*3 blocks for the remaining data less
       than a LONG*3 block */
    while (len >= SHORT*3) {
        uint64_t crc1 = 0;
        uint64_t crc2 = 0;
        unsigned char const * const end = next + SHORT;
        do {
            __asm__("crc32q\t%3, %0\n\t"
                    "crc32q\t%4, %1\n\t"
                    "crc32q\t%5, %2"
                    : "+r"(crc0), "+r"(crc1), "+r"(crc2)
                    : "m"(*next), "m"(next[SHORT]), "m"(next[2*SHORT]));
            next += 8;
        } while (next < end);
        crc0 = crc32c_shift(crc32c_short, crc0) ^ crc1;
        crc0 = crc32c_shift(crc32c_short, crc0) ^ crc2;
        next += SHORT*2;
        len -= SHORT*3;
    }

    /* compute the crc on the remaining eight-byte units less than a SHORT*3
       block */
    {
        unsigned char const * const end = next + (len - (len & 7));
        while (next < end) {
            __asm__("crc32q\t%1, %0"
                    : "+r"(crc0)
                    : "m"(*next));
            next += 8;
        }
        len &= 7;
    }

    /* compute the crc for up to seven trailing bytes */
    while (len) {
        __asm__("crc32b\t%1, %0"
                : "+r"(crc0)
                : "m"(*next));
        next++;
        len--;
    }

    /* return a post-processed crc */
    return ~crc0;
}
