use cidre::ns;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

pub fn criterion_benchmark(c: &mut Criterion) {
    let item: ns::Integer = 42;
    let section: ns::Integer = 3;
    let item_usize = item as usize;
    let section_usize = section as usize;

    c.bench_function("index_path_with_item", |b| {
        b.iter(|| {
            let _ip = ns::IndexPath::with_item(black_box(item), black_box(section));
        })
    });

    c.bench_function("index_path_with_index_push", |b| {
        b.iter(|| {
            let ip = ns::IndexPath::with_index(black_box(section_usize));
            let _ip = ip.push(black_box(item_usize));
        })
    });

    c.bench_function("index_path_with_indexes_2", |b| {
        b.iter(|| {
            let indexes = [black_box(section_usize), black_box(item_usize)];
            let _ip = ns::IndexPath::with_indexes(&indexes);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
