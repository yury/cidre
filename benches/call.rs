use cidre::ns;
use criterion::{criterion_group, criterion_main, Criterion};


pub fn criterion_benchmark(c: &mut Criterion) {

    let pi = ns::ProcessInfo::current();


    c.bench_function("sel", |b| {
        b.iter(|| {
          pi.processor_count()
            // let c:usize = unsafe { pi.sel(sel_processor_count()) };
        })
    });

    c.bench_function("fn_call", |b| {
        b.iter(|| {
            pi.processor_count2()
        })
    });

    c.bench_function("ext sel", |b| {
      b.iter(|| {
          pi.processor_count3()
      })
  });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
