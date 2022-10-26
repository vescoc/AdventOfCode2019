use criterion::{criterion_group, criterion_main, Criterion};

use day24::*;

fn criterion_benchmarks(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(part_1));
    c.bench_function("part 2", |b| b.iter(part_2));
}

criterion_group!(benches, criterion_benchmarks);
criterion_main!(benches);
