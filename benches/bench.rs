use std::{
    cell::{Cell, RefCell},
    hint::black_box,
    time::Duration,
};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn reverse(c: &mut Criterion) {
    let mut b = c.benchmark_group("Reverse");
    b.sample_size(500).measurement_time(Duration::from_secs(10));

    macro_rules! bench {
        ($( $e:expr ),*) => {$(
            let cell = Cell::new([0u8; $e]);
            b.bench_with_input(BenchmarkId::new("Cell", $e), &black_box(cell), |b, i| {
                b.iter(|| {
                    let mut v = i.get();
                    v.iter_mut().for_each(|v| *v = v.wrapping_add(black_box(1)));
                    i.set(v);
                })
            });

            let ref_cell = RefCell::new([0u8; $e]);
            b.bench_with_input(BenchmarkId::new("RefCell", $e), &black_box(ref_cell), |b, i| {
                b.iter(|| {
                    i.borrow_mut().iter_mut().for_each(|v| *v = v.wrapping_add(black_box(1)));
                })
            });
        )*};
    }
    bench!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
}

criterion_group!(benches, reverse);
criterion_main!(benches);
