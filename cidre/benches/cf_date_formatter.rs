use cidre::{cf, ns, objc::ar_pool};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let date = cf::Date::current();
    let cf_formatter =
        cf::DateFormatter::new_iso_8601(cf::Iso8601DateFormatOpts::WITH_INTERNET_DATE_TIME);

    let ns_formatter = ns::Iso8601DateFormatter::new();

    let n = 100;

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

    c.bench_function("ns_date_formatter_with_new_date_ar", |b| {
        b.iter(|| {
            ar_pool(|| {
                for i in 0..n {
                    let date = ns::Date::with_time_interval_since_1970(i as _);
                    ns_formatter.string_from_date_ar(&date);
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
