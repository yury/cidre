use cidre::{blocks, dispatch, ns, objc::ar_pool};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let _d = ns::Data::new();
    let n = 5;
    let mut data = dispatch::Data::empty().retained();
    for _ in 0..n {
        let b = dispatch::Data::from_static(b"hello");
        data = dispatch::Data::concat(&data, &b);
    }
    c.bench_function("block_enum_ranges_no_escape", |b| {
        b.iter(|| {
            ar_pool(|| {
                let mut ranges = Vec::with_capacity(n);
                data.as_ns().enum_ranges(|_ptr, range, _done| {
                    ranges.push(range);
                });
                assert_eq!(ranges.len(), n);
            })
        })
    });

    c.bench_function("block_enum_ranges_block_alloc", |b| {
        b.iter(|| {
            ar_pool(|| {
                let mut ranges = Vec::with_capacity(n);
                {
                    let mut closure = |_ptr, range, _done| {
                        ranges.push(range);
                    };
                    let mut block = blocks::NoEscBlock::stack3(&mut closure);
                    data.as_ns().enumerate_byte_ranges_using_block(&mut block);
                }
                assert_eq!(ranges.len(), n);
            })
        })
    });

    c.bench_function("block_data_apply_noescape", |b| {
        b.iter(|| {
            ar_pool(|| {
                let mut ranges = Vec::with_capacity(n);
                data.apply(|_region, offset, _ptr, _size| {
                    ranges.push(offset);
                    true
                });
                assert_eq!(ranges.len(), n);
            })
        })
    });

    c.bench_function("block_enum_ranges_block_empty", |b| {
        let mut block = blocks::NoEscBlock::new3(|_ptr, _range, _done| {});
        b.iter(|| {
            ar_pool(|| {
                data.as_ns().enumerate_byte_ranges_using_block(&mut block);
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
