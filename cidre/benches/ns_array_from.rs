use cidre::{arc, ns, objc::ar_pool};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("array_with_buf_inside", |b| {
        b.iter(|| {
            ar_pool(|| {
                let _v: arc::R<ns::Array<ns::Number>> = [10i32].into();
            })
        })
    });

    c.bench_function("array_with_vec_inside", |b| {
        b.iter(|| {
            ar_pool(|| {
                let _v: arc::R<ns::Array<ns::Number>> = [10i64][..].into();
            })
        })
    });

    c.bench_function("array_with_mut", |b| {
        b.iter(|| {
            ar_pool(|| {
                let mut arr = ns::ArrayMut::with_capacity(1);
                arr.push(ns::Number::with_u64(10u64).as_ref());
                let _v: arc::R<ns::Array<ns::Number>> = arr.freeze();
            })
        })
    });

    c.bench_function("array_with_vec_inside_5", |b| {
        b.iter(|| {
            ar_pool(|| {
                let _v: arc::R<ns::Array<ns::Number>> = [10i64, 11, 12, 13, 14][..].into();
            })
        })
    });

    c.bench_function("array_with_buf_inside_5", |b| {
        b.iter(|| {
            ar_pool(|| {
                let _v: arc::R<ns::Array<ns::Number>> = [10i32, 11, 12, 13, 14][..].into();
            })
        })
    });

    c.bench_function("array_with_mut_5", |b| {
        b.iter(|| {
            ar_pool(|| {
                let mut arr = ns::ArrayMut::with_capacity(5);
                for i in [10i64, 11, 12, 13, 14].iter() {
                    arr.push(ns::Number::with_i64(*i).as_ref());
                }
                let _v: arc::R<ns::Array<ns::Number>> = arr.freeze();
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
