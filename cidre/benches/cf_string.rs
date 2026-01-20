use cidre::cf;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cf::String::from_str tagged", |b| {
        b.iter(|| assert!(cf::String::from_str(black_box("hello")).is_tagged_ptr()))
    });

    c.bench_function("cf::String::from_str_no_copy tagged", |b| {
        b.iter(|| {
            assert!(unsafe { cf::String::from_str_no_copy(black_box("hello")) }.is_tagged_ptr())
        })
    });

    let string = "very long string that can't be tagged cf::String".to_string();

    c.bench_function("cf::String::from_str", |b| {
        b.iter(|| assert!(!cf::String::from_str(black_box(&string)).is_tagged_ptr()))
    });

    c.bench_function("cf::String::from_str_no_copy", |b| {
        b.iter(|| {
            assert!(!unsafe { cf::String::from_str_no_copy(black_box(&string)) }.is_tagged_ptr())
        })
    });

    c.bench_function("cf::String cf::str!", |b| {
        b.iter(|| {
            assert!(!cf::str!(c"very long string that can't be tagged cf::String").is_tagged_ptr())
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
