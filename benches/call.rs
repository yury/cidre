use std::mem::transmute;

use cidre::{ns, objc::Sel};
use criterion::{criterion_group, criterion_main, Criterion};

#[inline(never)]
pub fn sel_processor_count() -> &'static Sel {
    #[link_section = "__TEXT,__objc_methname,cstring_literals"]
    static STR: [u8; 15] = *b"processorCount\0";
    #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
    static SEL: &[u8; 15] = &STR;

    unsafe {
        let ptr = std::ptr::read_volatile(&SEL);
        transmute(ptr)
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let pi = ns::ProcessInfo::current();

    c.bench_function("sel", |b| b.iter(|| pi.processor_count()));

    c.bench_function("fn_call", |b| b.iter(|| pi.processor_count2()));

    c.bench_function("ext sel", |b| b.iter(|| pi.processor_count3()));

    c.bench_function("never_inline", |b| {
        b.iter(|| {
            // pi.processor_count()
            let c: usize = unsafe { pi.sel0(sel_processor_count()) };
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
