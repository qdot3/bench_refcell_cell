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
#[repr(C)]
struct Head<const N: usize> {
    val: usize,
    _pad: [usize; N],
}

impl<const N: usize> Head<N> {
    fn new(val: usize) -> Self {
        Self { val, _pad: [0; _] }
    }

    fn add(&mut self, val: usize) {
        self.val += val
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Tail<const N: usize> {
    _pad: [usize; N],
    val: usize,
}

impl<const N: usize> Tail<N> {
    fn new(val: usize) -> Self {
        Self { val, _pad: [0; _] }
    }

    fn add(&mut self, val: usize) {
        self.val += val
    }
}

fn partial(c: &mut Criterion) {
    let mut b = c.benchmark_group(format!(
        "Partial (padding [words] vs time) ({})",
        cache_info()
    ));

    macro_rules! bench {
        ($( $e:expr ),*$(,)?) => {$(
            let cell = Cell::new(Head::<$e>::new(0));
            b.bench_with_input(BenchmarkId::new("Cell (Head)", $e), &cell, |b, i| {
                b.iter(|| {
                    let mut val = i.get();
                    val.add(black_box(1));
                    i.set(val);
                })
            });

            let refcell = RefCell::new(Head::<$e>::new(0));
            b.bench_with_input(BenchmarkId::new("RefCell (Head)", $e), &refcell, |b, i| {
                b.iter(|| i.borrow_mut().add(black_box(1)))
            });

            let cell = Cell::new(Tail::<$e>::new(0));
            b.bench_with_input(BenchmarkId::new("Cell (Tail)", $e), &cell, |b, i| {
                b.iter(|| {
                    let mut val = i.get();
                    val.add(black_box(1));
                    i.set(val);
                })
            });

            let refcell = RefCell::new(Tail::<$e>::new(0));
            b.bench_with_input(BenchmarkId::new("RefCell (Tail)", $e), &refcell, |b, i| {
                b.iter(|| i.borrow_mut().add(black_box(1)))
            });
        )*};
    }
    bench!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    );
}

criterion_group!(benches, partial);
criterion_main!(benches);
