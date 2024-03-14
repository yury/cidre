use cidre::{cf, ns};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cf::uuid", |b| {
        b.iter(|| {
            cf::Uuid::new();
        })
    });

    c.bench_function("ns::uuid", |b| {
        b.iter(|| {
            ns::Uuid::new();
        })
    });

    c.bench_function("rust::uuid", |b| {
        b.iter(|| {
            uuid::Uuid::new_v4();
        })
    });

    c.bench_function("cf::uuid::to_cf_string", |b| {
        b.iter(|| {
            cf::Uuid::new().to_cf_string();
        })
    });

    c.bench_function("ns::uuid::string", |b| {
        b.iter(|| {
            ns::Uuid::new().string();
        })
    });

    c.bench_function("rust::uuid::to_string", |b| {
        b.iter(|| {
            uuid::Uuid::new_v4().to_string();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
