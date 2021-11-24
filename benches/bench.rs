use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ms_converter::ms;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1d", |b| b.iter(|| ms(black_box("1d")).unwrap()));
    c.bench_function("1.1d", |b| b.iter(|| ms(black_box("1.1d")).unwrap()));
    c.bench_function("100000000ms", |b| {
        b.iter(|| ms(black_box("100000000ms")).unwrap())
    });
    c.bench_function("100000000.1231412ms", |b| {
        b.iter(|| ms(black_box("100000000.1231412ms")).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
