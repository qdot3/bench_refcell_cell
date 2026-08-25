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

fn add1_cell<const N: usize>(src: &Cell<[usize; N]>) {
    let mut val = src.get();
    val.iter_mut().for_each(|v| {
        *v = v.wrapping_add(
            // avoid auto-vectorization
            black_box(1),
        )
    });
    src.set(val);
}

fn add1_refcell<const N: usize>(src: &RefCell<[usize; N]>) {
    let mut val = src.borrow_mut();
    val.iter_mut()
        .for_each(|v| *v = v.wrapping_add(black_box(1)));
}

fn add1(c: &mut Criterion) {
    let mut b = c.benchmark_group(format!("Add 1 (words vs time) ({})", cache_info()));

    macro_rules! bench {
        ($( $e:expr ),*) => {$(
            let src = [0; $e];

            let cell = Cell::new(src);
            b.bench_with_input(BenchmarkId::new("Cell", $e), &cell, |b, i| {
                b.iter(|| add1_cell(black_box(i)))
            });

            let refcell = RefCell::new(src);
            b.bench_with_input(BenchmarkId::new("RefCell", $e), &refcell, |b, i| {
                b.iter(|| add1_refcell(black_box(i)))
            });
        )*};
    }
    bench!(
        2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40
    );
}

criterion_group!(benches, add1);
criterion_main!(benches);
