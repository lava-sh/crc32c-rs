use crc32c_benchmarks::{
    dispatched, fallback,
    payloads::{KIB, MIB, ONE_MIB, SIZES, SMALLEST, Size, payload},
};
use divan::{Bencher, black_box};

fn main() {
    divan::main();
}

/// Runtime-dispatched implementation, the one behind `crc32c_rs.crc32c`.
#[divan::bench(args = SIZES)]
fn crc32c(bencher: Bencher, size: Size) {
    let crc32c = dispatched();
    let data = payload(size);

    bencher.bench(|| crc32c(black_box(data), black_box(0)));
}

/// Portable table-based implementation, used when no SIMD ISA is available.
#[divan::bench(args = SIZES)]
fn crc32c_fallback(bencher: Bencher, size: Size) {
    let data = payload(size);

    bencher.bench(|| fallback(black_box(data), black_box(0)));
}

/// Streaming usage: feed a 1 MiB payload as 64 KiB chunks.
#[divan::bench]
fn crc32c_chained(bencher: Bencher) {
    let crc32c = dispatched();
    let chunks: Vec<&[u8]> = payload(ONE_MIB).chunks(64 * KIB).collect();

    bencher.bench(|| {
        let mut value = 0;

        for chunk in &chunks {
            value = crc32c(black_box(chunk), value);
        }

        value
    });
}

/// Unaligned input: the kernels have a dedicated head/tail path for it.
#[divan::bench]
fn crc32c_unaligned(bencher: Bencher) {
    let crc32c = dispatched();
    let data = &payload(ONE_MIB)[1..MIB - 1];

    bencher.bench(|| crc32c(black_box(data), black_box(0)));
}

/// Per-call overhead, dominated by the dispatch and the length checks.
#[divan::bench]
fn crc32c_many_small_calls(bencher: Bencher) {
    let crc32c = dispatched();
    let data = payload(SMALLEST);

    bencher.bench(|| {
        let mut value = 0;

        for _ in 0..1000 {
            value = crc32c(black_box(data), value);
        }

        value
    });
}

/// Lower bound of the call overhead.
#[divan::bench]
fn crc32c_empty(bencher: Bencher) {
    let crc32c = dispatched();

    bencher.bench(|| crc32c(black_box(&[]), black_box(0)));
}
