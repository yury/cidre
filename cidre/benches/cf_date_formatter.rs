use cidre::{cf, ns, objc::ar_pool};
use criterion::{criterion_group, criterion_main, Criterion};

// TODO: investigate why ns::Iso8601DateFormatter is faster...

pub fn criterion_benchmark(c: &mut Criterion) {
    let date = cf::Date::new();
    let cf_iso_formatter = cf::DateFormatter::new_iso_8601();
    let ns_iso_formatter = ns::Iso8601DateFormatter::new();
    let cf_formatter = cf::DateFormatter::with_styles(
        cf::DateFormatterStyle::No,
        cf::DateFormatterStyle::Short,
        None,
    );
    let mut ns_formatter = ns::DateFormatter::new();
    ns_formatter.set_date_style(ns::DateFormatterStyle::No);
    ns_formatter.set_time_style(ns::DateFormatterStyle::Short);

    let n = criterion::black_box(10);

    c.bench_function("cf_iso_date_formatter_with_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for _i in 0..n {
                    cf_iso_formatter.string_from_date(&date);
                }
            })
        })
    });

    c.bench_function("ns_iso_date_formatter_with_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for _i in 0..n {
                    ns_iso_formatter.string_from_date(date.as_ns());
                }
            })
        })
    });

    c.bench_function("cf_date_formatter_with_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for _i in 0..n {
                    cf_formatter.string_from_date(&date);
                }
            })
        })
    });

    c.bench_function("ns_date_formatter_with_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for _i in 0..n {
                    ns_formatter.string_from_date(date.as_ns());
                }
            })
        })
    });

    c.bench_function("cf_iso_date_formatter_with_new_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    let date = cf::Date::new_at(i as _);
                    cf_iso_formatter.string_from_date(&date);
                }
            })
        })
    });

    c.bench_function("ns_iso_date_formatter_with_new_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    let date = ns::Date::with_time_interval_since_1970(i as _);
                    ns_iso_formatter.string_from_date(&date);
                }
            })
        })
    });

    c.bench_function("cf_date_formatter_with_new_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    let date = cf::Date::new_at(i as _);
                    cf_formatter.string_from_date(&date);
                }
            })
        })
    });

    c.bench_function("cf_date_formatter_with_new_system_time", |b| {
        b.iter(|| {
            ar_pool(|| {
                for _i in 0..n {
                    let time = std::time::SystemTime::now();
                    cf_formatter.string_from_system_time(&time).unwrap();
                }
            })
        })
    });

    c.bench_function("ns_date_formatter_with_new_date", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    let date = ns::Date::with_time_interval_since_1970(i as _);
                    ns_formatter.string_from_date(&date);
                }
            })
        })
    });

    c.bench_function("cf_date_formatter_with_abs_time", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    cf_formatter.string_from_abs_time(i as _);
                }
            })
        })
    });

    c.bench_function("ns_iso_date_formatter_with_new_date_ar", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    let date = ns::Date::with_time_interval_since_1970(i as _);
                    ns_iso_formatter.string_from_date_ar(&date);
                }
            })
        })
    });

    c.bench_function("cf_iso_date_formatter_with_abs_time", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    cf_iso_formatter.string_from_abs_time(i as _);
                }
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
