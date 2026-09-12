use core::fmt;
use std::sync::LazyLock;

pub const KIB: usize = 1024;
pub const MIB: usize = 1024 * KIB;

/// A payload size, named after the way it is reported by CodSpeed.
#[derive(Clone, Copy)]
pub struct Size {
    pub name: &'static str,
    pub bytes: usize,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

const fn size(name: &'static str, bytes: usize) -> Size {
    Size { name, bytes }
}

/// Smallest payload of the sweep, used on its own by the per-call overhead
/// benchmark.
pub const SMALLEST: Size = size("64B", 64);

/// Mid-sized payload, used on its own by the chunked and unaligned
/// benchmarks.
pub const ONE_MIB: Size = size("1MiB", MIB);

pub const SIZES: [Size; 11] = [
    SMALLEST,
    size("512B", 512),
    size("1KiB", KIB),
    size("64KiB", 64 * KIB),
    size("512KiB", 512 * KIB),
    ONE_MIB,
    size("16MiB", 16 * MIB),
    size("32MiB", 32 * MIB),
    size("64MiB", 64 * MIB),
    size("128MiB", 128 * MIB),
    size("256MiB", 256 * MIB),
];

const LARGEST: usize = SIZES[SIZES.len() - 1].bytes;

/// CRC32C runs in constant time per byte, so the multi-megabyte payloads are
/// tiled from a single pseudo-random block instead of being drawn byte by
/// byte: the contents do not change what the kernels do, and tiling keeps the
/// setup out of the way of the measured section.
///
/// The generator is seeded with a fixed value, so two CodSpeed runs only
/// differ by the code being measured.
static BLOCK: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut state: u64 = 0xC32C_C32C_C32C_C32C;
    let mut seed = Vec::with_capacity(MIB);

    while seed.len() < MIB {
        // xorshift64, good enough to keep the payload incompressible and
        // reproducible without pulling in a dependency.
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;

        seed.extend_from_slice(&state.to_le_bytes());
    }

    seed.repeat(LARGEST / MIB)
});

/// Payload of `size`, always taken from the same block of bytes.
#[must_use]
pub fn payload(size: Size) -> &'static [u8] {
    &BLOCK[..size.bytes]
}
