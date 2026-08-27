use std::{
    cell::{Cell, RefCell},
    hint::black_box,
};

use cache_size::{l1_cache_line_size, l1_cache_size};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

/// FIXME: This supports x86 architectures only for now
fn cache_info() -> String {
    let l1_size = match l1_cache_size() {
        Some(n) => {
            if n.trailing_zeros() >= 20 {
                format!("{} MiB", n >> 20)
            } else if n.trailing_zeros() >= 10 {
                format!("{} KiB", n >> 10)
            } else {
                format!("{} B", n)
            }
        }
        None => "NA".to_string(),
    };
    let l1_line_size = match l1_cache_line_size() {
        Some(n) => format!("{n} B"),
        None => "NA".to_string(),
    };
    let word_size = format!("{} B", std::mem::size_of::<usize>());

    format!("L1: {l1_size}, L1 Line: {l1_line_size}, word: {word_size}")
}

#[derive(Clone, Copy)]
struct BigInt<const N: usize>([usize; N]);

impl<const N: usize> BigInt<N> {
    fn new() -> Self {
        Self([0; _])
    }

    fn wrapping_mul(&mut self, rhs: usize) {
        let mut carry = 0;
        self.0.iter_mut().for_each(|v| {
            let (c, s) = v.carrying_mul(rhs, carry);
            *v = s;
            carry = c;
        });
    }
}

fn full(c: &mut Criterion) {
    let mut b = c.benchmark_group(format!("Full (size [words] vs time) ({})", cache_info()));

    macro_rules! bench {
        ($( $e:expr ),*$(,)?) => {$(
            let cell = Cell::new(BigInt::<$e>::new());
            b.bench_with_input(BenchmarkId::new("Cell", $e), &cell, |b, i| {
                b.iter(|| {
                    let mut val = i.get();
                    val.wrapping_mul(black_box(1));
                    i.set(val);
                })
            });

            let refcell = RefCell::new(BigInt::<$e>::new());
            b.bench_with_input(BenchmarkId::new("RefCell", $e), &refcell, |b, i| {
                b.iter(|| i.borrow_mut().wrapping_mul(black_box(1)))
            });
        )*};
    }
    bench!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

    b.finish();
}

criterion_group!(benches, full);
criterion_main!(benches);
