use cidre::{cf, ns};
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let cf_str = cf::str!(c"string");
    let ns_str = ns::str!(c"string");

    let value = ns::Data::new();

    let cf_dictionary = cf::DictionaryOf::with_keys_values(&[cf_str], &[value.as_cf()]);
    let ns_dictionary = ns::Dictionary::with_keys_values(&[ns_str], &[value.as_ref()]);

    c.bench_function("indexing_ns_dictionary_get", |b| {
        b.iter(|| {
            let _v = ns_dictionary.get(ns_str).unwrap();
        })
    });
    c.bench_function("indexing_cf_dictionary_get", |b| {
        b.iter(|| {
            let _v = cf_dictionary.get(cf_str).unwrap();
        })
    });

    c.bench_function("indexing_ns_dictionary_as_cf", |b| {
        let cf: &cf::DictionaryOf<cf::String, cf::Type> =
            unsafe { std::mem::transmute(ns_dictionary.as_ref()) };
        b.iter(|| {
            let _v = cf.get(cf_str).unwrap();
        })
    });
    c.bench_function("indexing_cf_dictionary_as_ns", |b| {
        let ns: &ns::Dictionary<ns::String, ns::Id> =
            unsafe { std::mem::transmute(cf_dictionary.as_ref()) };
        b.iter(|| {
            let _v = ns.get(ns_str).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
