use cidre::ns;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let num = i64::MAX - 1;

    c.bench_function("array_new_with_alloc_init", |b| {
        b.iter(|| ns::Array::<ns::Id>::new())
    });
    c.bench_function("array_new_with_new", |b| {
        b.iter(|| ns::Array::<ns::Id>::_new())
    });

    c.bench_function("alloc_init", |b| {
        b.iter(|| {
            ns::Number::with_i64(num);
        })
    });
    c.bench_function("alloc_init_tagged", |b| {
        b.iter(|| {
            ns::Number::with_i64(1);
        })
    });
    c.bench_function("alloc_tagged", |b| {
        b.iter(|| {
            ns::Number::tagged_i32(10);
        })
    });
    c.bench_function("alloc_tagged_alloc", |b| {
        b.iter(|| {
            ns::Number::tagged_i32_alloc(10);
        })
    });
    c.bench_function("alloc_with_retain_auto", |b| {
        b.iter(|| {
            ns::Number::with_i64_retain_auto(num);
        })
    });
    c.bench_function("alloc_with_fn_call", |b| {
        b.iter(|| {
            ns::Number::with_i64_call(num);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
