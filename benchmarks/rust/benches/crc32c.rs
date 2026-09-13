use core::fmt;
use std::env::consts::ARCH;

use crc32c_benchmarks::{
    Kernel, available, dispatched,
    payloads::{KIB, MIB, ONE_MIB, SIZES, Size, payload},
};
use divan::{Bencher, black_box};

fn main() {
    divan::main();
}

#[derive(Clone, Copy)]
struct Case {
    kernel: Kernel,
    size: Size,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{ARCH}/{}/{}", self.kernel, self.size)
    }
}

fn cases() -> Vec<Case> {
    available()
        .into_iter()
        .flat_map(|kernel| SIZES.map(|size| Case { kernel, size }))
        .collect()
}

#[divan::bench(args = cases())]
fn crc32c_kernel(bencher: Bencher, case: Case) {
    let data = payload(case.size);

    bencher.bench(|| case.kernel.run(black_box(data), black_box(0)));
}

#[divan::bench(args = [ARCH])]
fn crc32c_chained(bencher: Bencher, _arch: &str) {
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

#[divan::bench(args = [ARCH])]
fn crc32c_unaligned(bencher: Bencher, _arch: &str) {
    let crc32c = dispatched();
    let data = &payload(ONE_MIB)[1..MIB - 1];

    bencher.bench(|| crc32c(black_box(data), black_box(0)));
}
