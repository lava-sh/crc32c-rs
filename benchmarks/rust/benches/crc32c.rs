use core::fmt;
use std::env::consts::{ARCH, OS};

use crc32c_benchmarks::{
    Kernel, available,
    payloads::{SIZES, Size, payload},
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
        write!(f, "{OS}/{ARCH}/{}/{}", self.kernel, self.size)
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
