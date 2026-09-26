#![allow(clippy::wildcard_imports)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
pub static SHIFT_TABLE: [u32; 256] = [
    0xd94f3c0b, 0x00000001, 0x493c7d27, 0xf20c0dfe, 0xba4fc28e, 0x3da6d0cb, 0xddc0152b, 0x1c291d04,
    0x9e4addf8, 0x740eef02, 0x39d3b296, 0x083a6eec, 0x0715ce53, 0xc49f4f67, 0x47db8317, 0x2ad91c30,
    0x0d3b6092, 0x6992cea2, 0xc96cfdc0, 0x7e908048, 0x878a92a7, 0x1b3d8f29, 0xdaece73e, 0xf1d0f55e,
    0xab7aff2a, 0xa87ab8a8, 0x2162d385, 0x8462d800, 0x83348832, 0x71d111a8, 0x299847d5, 0xffd852c6,
    0xb9e02b86, 0xdcb17aa4, 0x18b33a4e, 0xf37c5aee, 0xb6dd949b, 0x6051d5a2, 0x78d9ccb7, 0x18b0d4ff,
    0xbac2fd7b, 0x21f3d99c, 0xa60ce07b, 0x8f158014, 0xce7f39f4, 0xa00457f7, 0x61d82e56, 0x8d6d2c43,
    0xd270f1a2, 0x00ac29cf, 0xc619809d, 0xe9adf796, 0x2b3cac5d, 0x96638b34, 0x65863b64, 0xe0e9f351,
    0x1b03397f, 0x9af01f2d, 0xebb883bd, 0x2cff42cf, 0xb3e32c28, 0x88f25a3a, 0x064f7f26, 0x4e36f0b0,
    0xdd7e3b0c, 0xbd6f81f8, 0xf285651c, 0x91c9bd4b, 0x10746f3c, 0x885f087b, 0xc7a68855, 0x4c144932,
    0x271d9844, 0x52148f02, 0x8e766a0c, 0xa3c6f37a, 0x93a5f730, 0xd7c0557f, 0x6cb08e5c, 0x63ded06a,
    0x6b749fb2, 0x4d56973c, 0x1393e203, 0x9669c9df, 0xcec3662e, 0xe417f38a, 0x96c515bb, 0x4b9e0f71,
    0xe6fc4e6a, 0xd104b8fc, 0x8227bb8a, 0x5b397730, 0xb0cd4768, 0xe78eb416, 0x39c7ff35, 0x61ff0e01,
    0xd7a4825c, 0x8d96551c, 0x0ab3844b, 0x0bf80dd2, 0x0167d312, 0x8821abed, 0xf6076544, 0x6a45d2b2,
    0x26f6a60a, 0xd8d26619, 0xa741c1bf, 0xde87806c, 0x98d8d9cb, 0x14338754, 0x49c3cc9c, 0x5bd2011f,
    0x68bce87a, 0xdd07448e, 0x57a3d037, 0xdde8f5b9, 0x6956fc3b, 0xa3e3e02c, 0x42d98888, 0xd73c7bea,
    0x3771e98f, 0x80ff0093, 0xb42ae3d9, 0x8fe4c34d, 0x2178513a, 0xdf99fc11, 0xe0ac139e, 0x6c23e841,
    0x170076fa, 0xfe314258, 0x444dd413, 0x0d8373a0, 0x6f345e45, 0x19e3635e, 0x41d17b64, 0x29f268b4,
    0xff0dba97, 0x1dc0632a, 0xa2b73df1, 0x1614f396, 0xf872e54c, 0x9e2993d3, 0x1e41e9fc, 0x6bebd73c,
    0x86d8e4d2, 0x63ae91e6, 0x651bd98b, 0xf8c9da7a, 0x5bb8f1bc, 0x945a19c1, 0xa90fd27a, 0xee8213b7,
    0xb3af077a, 0x93781dc7, 0x4984d782, 0xccc4a1b9, 0xca6ef3ac, 0xa2c2d971, 0x234e0b26, 0x1cad4452,
    0xdd66cbbb, 0x74922601, 0x4597456a, 0xc55f7eab, 0xe9e28eb4, 0xa1962329, 0x7b3ff57a, 0x2d370749,
    0xc9c8b782, 0x397d84a1, 0x3f70cc6f, 0x79113270, 0x93e106a4, 0xbc817803, 0x62ec6c6d, 0x88eb3c07,
    0xd813b325, 0x6e4cb630, 0x0df04680, 0x71971d5c, 0x2342001e, 0xf33b8bc6, 0x0a2a8d7e, 0x9fb3bbc0,
    0x6d9a4957, 0x6ef22b23, 0xe8b6368b, 0xce2df768, 0xd2c3ed1a, 0xe53a4fc7, 0x995a5724, 0xbe60a91a,
    0x9ef68d35, 0x1dfa0a15, 0x0c139b31, 0x8ec52396, 0xf2271e60, 0x0e766b11, 0x0b0bf8ca, 0x475846a4,
    0x2664fd8b, 0xb2a3dfa6, 0xed64812d, 0xdc1a160c, 0x02ee03b2, 0x79afdf1c, 0x8604ae0f, 0x07ac6e46,
    0x363bd6b3, 0x15f85253, 0x135c83fd, 0x1bec24dd, 0x5fabe670, 0x4c36cd5b, 0x35ec3279, 0xe0a22e29,
    0x00bcf5f6, 0x7c2b6ed9, 0x8ae00689, 0x06ff88fd, 0x17f27698, 0xf7317cf0, 0x58ca5f00, 0x61b6e40b,
    0xaa7c7ad5, 0xde8a97f8, 0xb5cfca28, 0x88f61445, 0xded288f8, 0xd4520e9e, 0x59f229bc, 0x0c592bd5,
    0x6d390dec, 0x38edfaf3, 0x37170390, 0x72cbfcdb, 0x6353c1cc, 0x348331a5, 0xc4584f5c, 0xc3977c19,
    0xf48642e9, 0xdafaea7c, 0x531377e2, 0x73db4c04, 0xdd35bc8d, 0x72675ce8, 0xb25b29f2, 0x3ec2ff83,
    0x9a5ede41, 0xe8c7a017, 0xa563905d, 0xcf4bfaef, 0x45cddf4e, 0x6bde1ac7, 0xacfa3103, 0xae1175c2,
];

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn clmul_scalar(a: u32, b: u32) -> __m128i {
    _mm_clmulepi64_si128::<0>(
        _mm_cvtsi32_si128(a.cast_signed()),
        _mm_cvtsi32_si128(b.cast_signed()),
    )
}

#[inline]
#[target_feature(enable = "sse4.2")]
fn mm_crc32_u64(crc: u32, v: u64) -> u32 {
    #[cfg(target_arch = "x86")]
    {
        let lo = _mm_crc32_u32(crc, v as u32);
        _mm_crc32_u32(lo, (v >> 32) as u32)
    }
    #[cfg(target_arch = "x86_64")]
    {
        _mm_crc32_u64(u64::from(crc), v) as u32
    }
}

#[inline]
#[target_feature(enable = "sse4.2")]
fn mm_extract_epi64<const IMM1: i32>(a: __m128i) -> u64 {
    const { assert!(IMM1 == 0 || IMM1 == 1) };
    #[cfg(target_arch = "x86")]
    {
        let arr: [u64; 2] = unsafe { core::mem::transmute(a) };
        arr[IMM1 as usize]
    }
    #[cfg(target_arch = "x86_64")]
    {
        _mm_extract_epi64::<IMM1>(a).cast_unsigned()
    }
}

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
fn crc_shift(crc: u32, nbytes: usize) -> u64 {
    mm_extract_epi64::<0>(clmul_scalar(crc, SHIFT_TABLE[nbytes >> 3]))
}

#[inline]
#[target_feature(enable = "sse4.2,pclmulqdq")]
pub unsafe fn crc32c_small(mut crc0: u32, mut buf: *const u8, mut len: usize) -> u32 {
    let klen = ((len - 8) / 48) * 16;
    let mut crc1 = 0_u32;
    let mut crc2 = 0_u32;
    loop {
        crc0 = mm_crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        crc1 = mm_crc32_u64(crc1, unsafe {
            buf.add(klen).cast::<u64>().read_unaligned()
        });
        crc2 = mm_crc32_u64(crc2, unsafe {
            buf.add(klen * 2).cast::<u64>().read_unaligned()
        });
        crc0 = mm_crc32_u64(crc0, unsafe { buf.add(8).cast::<u64>().read_unaligned() });
        crc1 = mm_crc32_u64(crc1, unsafe {
            buf.add(klen + 8).cast::<u64>().read_unaligned()
        });
        crc2 = mm_crc32_u64(crc2, unsafe {
            buf.add(klen * 2 + 8).cast::<u64>().read_unaligned()
        });
        buf = unsafe { buf.add(16) };
        len -= 48;
        if len < 56 {
            break;
        }
    }
    let vc = crc_shift(crc0, klen * 2 + 8) ^ crc_shift(crc1, klen + 8);
    buf = unsafe { buf.add(klen * 2) };
    crc0 = crc2;
    crc0 = mm_crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() } ^ vc);
    buf = unsafe { buf.add(8) };
    len -= 8;

    while len >= 8 {
        crc0 = mm_crc32_u64(crc0, unsafe { buf.cast::<u64>().read_unaligned() });
        buf = unsafe { buf.add(8) };
        len -= 8;
    }
    while len != 0 {
        crc0 = _mm_crc32_u8(crc0, unsafe { *buf });
        buf = unsafe { buf.add(1) };
        len -= 1;
    }
    crc0
}
