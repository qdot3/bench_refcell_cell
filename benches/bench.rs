use std::{
    cell::{Cell, RefCell},
    hint::black_box,
    time::Duration,
};

use cache_size::{l1_cache_line_size, l1_cache_size};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn add_1(c: &mut Criterion) {
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

    let mut b = c.benchmark_group(format!(
        "Add 1 (words vs time) (L1: {l1_size}, L1 Line: {l1_line_size}, word: {word_size})"
    ));
    b.sample_size(500).measurement_time(Duration::from_secs(10));

    macro_rules! bench {
        ($( $e:expr ),*) => {$(
            let src = black_box([0usize; $e]);

            let cell = Cell::new(src);
            b.bench_with_input(BenchmarkId::new("Cell", $e), &cell, |b, i| {
                b.iter(|| {
                    let mut v = i.get();
                    v.iter_mut().for_each(|v| *v = v.wrapping_add(black_box(1)));
                    i.set(v);
                })
            });

            let ref_cell = RefCell::new(src);
            b.bench_with_input(BenchmarkId::new("RefCell", $e), &ref_cell, |b, i| {
                b.iter(|| {
                    i.borrow_mut().iter_mut().for_each(|v| *v = v.wrapping_add(black_box(1)));
                })
            });
        )*};
    }
    bench!(
        2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48
    );
}

criterion_group!(benches, add_1);
criterion_main!(benches);
