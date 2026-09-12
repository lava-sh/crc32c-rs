use core::fmt;

use crc32c_benchmarks::{
    Kernel, available, dispatched,
    payloads::{KIB, MIB, ONE_MIB, SIZES, SMALLEST, Size, payload},
};
use divan::{Bencher, black_box};

fn main() {
    divan::main();
}

/// One kernel measured on one payload size.
#[derive(Clone, Copy)]
struct Case {
    kernel: Kernel,
    size: Size,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.kernel, self.size)
    }
}

/// Every kernel this CPU can execute, over the whole payload sweep.
fn cases() -> Vec<Case> {
    available()
        .into_iter()
        .flat_map(|kernel| SIZES.map(|size| Case { kernel, size }))
        .collect()
}

/// Runtime-dispatched implementation, the one behind `crc32c_rs.crc32c`.
#[divan::bench(args = SIZES)]
fn crc32c(bencher: Bencher, size: Size) {
    let crc32c = dispatched();
    let data = payload(size);

    bencher.bench(|| crc32c(black_box(data), black_box(0)));
}

/// Every kernel available on this machine, not just the dispatched one.
///
/// The dispatcher picks a single kernel per CPU model, so this is what shows
/// whether that choice is still the fastest one on the runner, and how the
/// other implementations behave on the same hardware.
#[divan::bench(args = cases())]
fn crc32c_kernel(bencher: Bencher, case: Case) {
    let data = payload(case.size);

    bencher.bench(|| case.kernel.run(black_box(data), black_box(0)));
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
