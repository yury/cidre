use cidre::{cf, ns};
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let cf_str = cf::str!(c"cf_string");
    let ns_str = ns::str!(c"ns_string");
    let cf_array = cf::ArrayOf::from_slice(&[cf_str]);
    let ns_array = ns::Array::from_slice(&[ns_str]);
    c.bench_function("indexing_ns_array", |b| {
        b.iter(|| {
            let _v = ns_array.get(0).unwrap();
        })
    });
    c.bench_function("indexing_ns_array_throws", |b| {
        b.iter(|| {
            let _v = unsafe { ns_array.get_throws(0) };
        })
    });

    c.bench_function("indexing_cf_array", |b| {
        b.iter(|| {
            let _v = &cf_array[0];
        })
    });

    c.bench_function("indexing_ns_array_with_cf_api", |b| {
        b.iter(|| {
            let _v = &ns_array.as_cf()[0];
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
