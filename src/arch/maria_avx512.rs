/* Copyright (c) 2024, MariaDB plc

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; version 2 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1335  USA */

#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::cmp::Ordering;

// This implementation is based on
// crc32_by16_vclmul_avx512 and crc32_refl_by16_vclmul_avx512
// in https://github.com/intel/intel-ipsec-mb/ with some optimizations.
// The // comments in crc32_avx512() correspond to assembler labels.

// table of constants corresponding to a CRC polynomial up to degree 32
#[repr(C, align(64))]
struct Refl32c {
    b2048: [u64; 2],
    b1024: [u64; 2],
    align_b896: [u64; 4],
    // includes b786, b640
    b896: [u64; 6],
    b512: [u64; 2],
    b384: [u64; 2],
    b256: [u64; 2],
    b128: [u64; 2],
    zeropad_for_b384: [u64; 2],
    b64: [u64; 2],
    b32: [u64; 2],
}

const _: () = {
    assert!(core::mem::offset_of!(Refl32c, b896) == 64);
    assert!(core::mem::offset_of!(Refl32c, b512) == 112);
    assert!(core::mem::offset_of!(Refl32c, b384) == 128);
};

// Castagnoli CRC-32C (reflected polynomial 0x1EDC6F41)
#[rustfmt::skip]
static REFL32C: Refl32c = Refl32c {
    b2048: [0x0000_0000_b9e0_2b86, 0x0000_0000_dcb1_7aa4],
    b1024: [0x0000_0000_0d3b_6092, 0x0000_0000_6992_cea2],
    align_b896: [0, 0, 0, 0],
    b896: [
        0x0000_0000_47db_8317,
        0x0000_0000_2ad9_1c30,
        0x0000_0000_0715_ce53,
        0x0000_0000_c49f_4f67,
        0x0000_0000_39d3_b296,
        0x0000_0000_083a_6eec,
    ],
    b512: [0x0000_0000_9e4a_ddf8, 0x0000_0000_740e_ef02],
    b384: [0x0000_0000_ddc0_152b, 0x0000_0000_1c29_1d04],
    b256: [0x0000_0000_ba4f_c28e, 0x0000_0000_3da6_d0cb],
    b128: [0x0000_0000_493c_7d27, 0x0000_0000_f20c_0dfe],
    zeropad_for_b384: [0, 0],
    b64: [0x0000_0000_493c_7d27, 0x0000_0000_dd45_aab8],
    b32: [0x0000_0000_dea7_13f0, 0x0000_0001_05ec_76f0],
};

// Some ternary functions
mod ternary {
    const A: u8 = 0b1111_0000;
    const B: u8 = 0b1100_1100;
    const C: u8 = 0b1010_1010;

    pub const XOR3: i32 = (A ^ B ^ C) as i32;
    pub const XNOR3: i32 = (!(A ^ B ^ C)) as i32;
    pub const XOR2_AND: i32 = ((A ^ B) & C) as i32;
}

// @return a^b^c
#[inline]
#[target_feature(enable = "avx512vl")]
fn xor3_128(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
    _mm_ternarylogic_epi64::<{ ternary::XOR3 }>(a, b, c)
}

// @return ~(a^b^c)
#[inline]
#[target_feature(enable = "avx512vl")]
fn xnor3_128(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
    _mm_ternarylogic_epi64::<{ ternary::XNOR3 }>(a, b, c)
}

// @return a^b^c
#[inline]
#[target_feature(enable = "avx512f")]
fn xor3_512(a: __m512i, b: __m512i, c: __m512i) -> __m512i {
    _mm512_ternarylogic_epi64::<{ ternary::XOR3 }>(a, b, c)
}

// @return (a^b)&c
#[inline]
#[target_feature(enable = "avx512vl")]
fn xor2_and_128(a: __m128i, b: __m128i, c: __m128i) -> __m128i {
    _mm_ternarylogic_epi64::<{ ternary::XOR2_AND }>(a, b, c)
}

// Load 64 bytes
#[inline]
#[target_feature(enable = "avx512bw")]
fn load512(b: *const u8) -> __m512i {
    unsafe { _mm512_loadu_epi8(b.cast()) }
}

// Load 16 bytes
#[inline]
#[target_feature(enable = "avx512vl")]
fn load128(b: *const u8) -> __m128i {
    unsafe { _mm_loadu_epi64(b.cast()) }
}

#[inline]
#[target_feature(enable = "avx512vl")]
fn load_tab(b: *const [u64; 2]) -> __m128i {
    unsafe { _mm_load_epi32(b.cast()) }
}

#[inline]
#[target_feature(enable = "avx512f")]
fn xor512(a: __m512i, b: __m512i) -> __m512i {
    _mm512_xor_epi64(a, b)
}

#[inline]
#[target_feature(enable = "avx512vl")]
fn xor256(a: __m256i, b: __m256i) -> __m256i {
    _mm256_xor_epi64(a, b)
}

#[inline]
#[target_feature(enable = "avx512vl")]
fn xor128(a: __m128i, b: __m128i) -> __m128i {
    _mm_xor_epi64(a, b)
}

#[inline]
#[target_feature(enable = "sse2")]
fn and128(a: __m128i, b: __m128i) -> __m128i {
    _mm_and_si128(a, b)
}

// Combine 512 data bits with CRC
#[inline]
#[target_feature(enable = "avx512f,vpclmulqdq")]
fn combine512(a: __m512i, tab: __m512i, b: __m512i) -> __m512i {
    xor3_512(
        b,
        _mm512_clmulepi64_epi128::<0x01>(a, tab),
        _mm512_clmulepi64_epi128::<0x10>(a, tab),
    )
}

// Pick and zero-extend 128 bits of a 512-bit vector (vextracti32x4)
#[inline]
#[target_feature(enable = "avx512dq")]
fn extract512_128<const BITS: i32>(a: __m512i) -> __m512i {
    _mm512_zextsi128_si512(_mm512_extracti64x2_epi64::<BITS>(a))
}

#[repr(align(16))]
struct Align16<T>(T);

#[rustfmt::skip]
static SHUFFLE128: Align16<[u64; 4]> = Align16([
    0x8786_8584_8382_8100,
    0x8f8e_8d8c_8b8a_8988,
    0x0706_0504_0302_0100,
    0x000e_0d0c_0b0a_0908,
]);

#[rustfmt::skip]
static SHIFT128: Align16<[u64; 4]> = Align16([
    0x8786_8584_8382_8100,
    0x8f8e_8d8c_8b8a_8988,
    0x0706_0504_0302_0100,
    0x000e_0d0c_0b0a_0908,
]);

#[rustfmt::skip]
static SIZE_MASK: [u16; 16] = [
    0x0001, 0x0003, 0x0007, 0x000f, 0x001f, 0x003f, 0x007f, 0x00ff, //
    0x01ff, 0x03ff, 0x07ff, 0x0fff, 0x1fff, 0x3fff, 0x7fff, 0xffff,
];

#[rustfmt::skip]
static SHIFT_1_TO_3_REFLECT: [i8; 7 + 11] = [
    -1, -1, -1, -1, -1, -1, -1, //
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
];

// get_last_two_xmms
#[inline]
#[target_feature(enable = "avx512vl,vpclmulqdq")]
fn get_last_two_xmms(crc2: __m128i, b: __m128i, buf: *const u8, size: isize) -> u32 {
    let d = load128(unsafe { buf.offset(size - 16) });
    let mut s = load128(unsafe { (&raw const SHUFFLE128).cast::<u8>().offset(size) });
    let crc_out = _mm_shuffle_epi8(crc2, s);
    s = xor128(s, _mm_set1_epi32(0x8080_8080_u32.cast_signed()));
    let crc_out = xor3_128(
        _mm_blendv_epi8(_mm_shuffle_epi8(crc2, s), d, s),
        _mm_clmulepi64_si128::<0x01>(crc_out, b),
        _mm_clmulepi64_si128::<0x10>(crc_out, b),
    );
    done_128(crc_out)
}

// done_128
#[inline]
#[target_feature(enable = "avx512vl,vpclmulqdq")]
fn done_128(mut crc_out: __m128i) -> u32 {
    let b = load_tab(&raw const REFL32C.b64);
    let crc_tmp = xor128(
        _mm_clmulepi64_si128::<0x00>(crc_out, b),
        _mm_srli_si128(crc_out, 8),
    );
    crc_out = _mm_slli_si128(crc_tmp, 4);
    crc_out = _mm_clmulepi64_si128::<0x10>(crc_out, b);
    crc_out = xor128(crc_out, crc_tmp);
    barrett(crc_out)
}

// barrett
#[inline]
#[target_feature(enable = "avx512vl,vpclmulqdq")]
fn barrett(crc_out: __m128i) -> u32 {
    let b = load_tab(&raw const REFL32C.b32);
    let crc_tmp = crc_out;
    let crc_out = and128(crc_out, _mm_set_epi64x(!0_i64, !0xFFFF_FFFF_i64));
    let crc_out = _mm_clmulepi64_si128::<0x00>(crc_out, b);
    let crc_out = xor2_and_128(crc_out, crc_tmp, _mm_set_epi64x(0, !0_i64));
    let crc_out = xnor3_128(crc_out, crc_tmp, _mm_clmulepi64_si128::<0x10>(crc_out, b));
    _mm_extract_epi32::<2>(crc_out).cast_unsigned()
}

// reduce_64B
#[inline]
#[target_feature(enable = "avx512dq,avx512vl,vpclmulqdq")]
fn reduce_64b(lo: __m512i) -> __m128i {
    let b384 = unsafe { _mm512_load_epi32((&raw const REFL32C.b384).cast()) };
    let crc512 = xor3_512(
        _mm512_clmulepi64_epi128::<0x01>(lo, b384),
        _mm512_clmulepi64_epi128::<0x10>(lo, b384),
        extract512_128::<3>(lo),
    );
    let crc512 = xor512(crc512, _mm512_shuffle_i64x2::<0b0100_1110>(crc512, crc512));
    let crc256 = _mm512_castsi512_si256(crc512);
    xor128(
        _mm256_extracti64x2_epi64::<1>(crc256),
        _mm256_castsi256_si128(crc256),
    )
}

// final_reduction
#[inline]
#[target_feature(enable = "avx512vl,vpclmulqdq")]
fn final_reduction(mut buf: *const u8, mut size: isize, mut crc_out: __m128i) -> u32 {
    let b = load_tab(&raw const REFL32C.b128);

    while size >= 0 {
        // reduction_loop_16B
        crc_out = xor3_128(
            load128(buf),
            _mm_clmulepi64_si128::<0x01>(crc_out, b),
            _mm_clmulepi64_si128::<0x10>(crc_out, b),
        );
        buf = unsafe { buf.add(16) };
        size -= 16;
    }
    // final_reduction_for_128

    size += 16;
    if size != 0 {
        return get_last_two_xmms(crc_out, b, buf, size);
    }

    done_128(crc_out)
}

#[target_feature(enable = "avx512bw,avx512dq,avx512vl,vpclmulqdq")]
pub unsafe fn crc32c(crc: u32, buf: *const u8, size: usize) -> u32 {
    let crc_in = _mm512_zextsi128_si512(_mm_cvtsi32_si128((!crc).cast_signed()));
    let b512 = _mm512_broadcast_i32x4(load_tab(&raw const REFL32C.b512));
    let mut crc_out: __m128i;
    let mut lo: __m512i;
    let mut buf = buf;
    let mut size = size.cast_signed();

    if size >= 256 {
        lo = xor512(load512(buf), crc_in);
        let mut l1 = load512(unsafe { buf.add(64) });

        let b1024 = _mm512_broadcast_i32x4(load_tab(&raw const REFL32C.b1024));
        size -= 256;
        if size >= 256 {
            let mut h0 = load512(unsafe { buf.add(128) });
            let mut hi = load512(unsafe { buf.add(192) });
            let b2048 = _mm512_broadcast_i32x4(load_tab(&raw const REFL32C.b2048));
            size -= 256;
            loop {
                buf = unsafe { buf.add(256) };
                lo = combine512(lo, b2048, load512(buf));
                l1 = combine512(l1, b2048, load512(unsafe { buf.add(64) }));
                h0 = combine512(h0, b2048, load512(unsafe { buf.add(128) }));
                hi = combine512(hi, b2048, load512(unsafe { buf.add(192) }));
                size -= 256;
                if size < 0 {
                    break;
                }
            }

            buf = unsafe { buf.add(256) };
            lo = combine512(lo, b1024, h0);
            l1 = combine512(l1, b1024, hi);
            size += 128;
        } else {
            loop {
                buf = unsafe { buf.add(128) };
                lo = combine512(lo, b1024, load512(buf));
                l1 = combine512(l1, b1024, load512(unsafe { buf.add(64) }));
                size -= 128;
                if size < 0 {
                    break;
                }
            }

            buf = unsafe { buf.add(128) };
        }

        if size >= -64 {
            size += 128;
            lo = combine512(lo, b512, l1);
            lo = combine512(lo, b512, load512(buf));
            loop {
                buf = unsafe { buf.add(64) };
                size -= 64;
                if size < 64 {
                    break;
                }
                lo = combine512(lo, b512, load512(buf));
            }

            // reduce_64B
            crc_out = reduce_64b(lo);
            size -= 16;
            return final_reduction(buf, size, crc_out);
        }

        let b896 = unsafe { _mm512_load_epi32((&raw const REFL32C.b896).cast()) };
        let b384 = unsafe { _mm512_load_epi32((&raw const REFL32C.b384).cast()) };

        let mut c4 = xor3_512(
            _mm512_clmulepi64_epi128::<0x01>(lo, b896),
            _mm512_clmulepi64_epi128::<0x10>(lo, b896),
            _mm512_clmulepi64_epi128::<0x01>(l1, b384),
        );
        c4 = xor3_512(
            c4,
            _mm512_clmulepi64_epi128::<0x10>(l1, b384),
            extract512_128::<3>(l1),
        );

        let mut c2 = _mm512_castsi512_si256(_mm512_shuffle_i64x2::<0b0100_1110>(c4, c4));
        c2 = xor256(c2, _mm512_castsi512_si256(c4));
        crc_out = xor128(
            _mm256_extracti64x2_epi64::<1>(c2),
            _mm256_castsi256_si128(c2),
        );
        size += 128 - 16;
        return final_reduction(buf, size, crc_out);
    }

    let b: __m128i;

    // less_than_256
    if size >= 32 {
        if size >= 64 {
            lo = xor512(load512(buf), crc_in);

            // fold_64_B_loop
            loop {
                buf = unsafe { buf.add(64) };
                size -= 64;
                if size < 64 {
                    break;
                }
                lo = combine512(lo, b512, load512(buf));
            }

            // reduce_64B
            crc_out = reduce_64b(lo);
            size -= 16;
        } else {
            // less_than_64
            crc_out = xor128(load128(buf), _mm512_castsi512_si128(crc_in));
            buf = unsafe { buf.add(16) };
            size -= 32;
        }

        // final_reduction
        return final_reduction(buf, size, crc_out);
    }

    // less_than_32
    if size > 0 {
        match size.cmp(&16) {
            Ordering::Greater => {
                crc_out = xor128(load128(buf), _mm512_castsi512_si128(crc_in));
                buf = unsafe { buf.add(16) };
                size -= 16;
                b = load_tab(&raw const REFL32C.b128);
                get_last_two_xmms(crc_out, b, buf, size)
            }
            Ordering::Less => {
                crc_out = unsafe {
                    _mm_maskz_loadu_epi8(SIZE_MASK[(size - 1).cast_unsigned()], buf.cast())
                };
                crc_out = xor128(crc_out, _mm512_castsi512_si128(crc_in));

                if size >= 4 {
                    crc_out = _mm_shuffle_epi8(
                        crc_out,
                        load128(unsafe { (&raw const SHIFT128).cast::<u8>().offset(size) }),
                    );
                    done_128(crc_out)
                } else {
                    // only_less_than_4
                    // Shift, zero-filling 5 to 7 of the 8-byte crc_out
                    crc_out = _mm_shuffle_epi8(
                        crc_out,
                        load128(unsafe {
                            (&raw const SHIFT_1_TO_3_REFLECT)
                                .cast::<u8>()
                                .offset(size - 1)
                        }),
                    );
                    barrett(crc_out)
                }
            }
            Ordering::Equal => {
                crc_out = xor128(load128(buf), _mm512_castsi512_si128(crc_in));
                done_128(crc_out)
            }
        }
    } else {
        crc
    }
}
