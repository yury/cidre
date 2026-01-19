use std::ffi::c_void;

use cidre::dispatch;
use criterion::{Criterion, criterion_group, criterion_main};

extern "C-unwind" fn work(_ctx: *mut u8) {}

pub fn criterion_benchmark(c: &mut Criterion) {
    let queue = dispatch::Queue::serial_with_ar_pool();

    c.bench_function("dispatch::queue.barrier_sync_f", |b| {
        b.iter(|| queue.barrier_sync_f(std::ptr::null_mut(), work))
    });

    c.bench_function("dispatch::queue.sync_f", |b| {
        b.iter(|| queue.sync_f(std::ptr::null_mut(), work))
    });

    c.bench_function("dispatch::queue.sync_fn", |b| {
        extern "C" fn foo(_ctx: *const c_void) {}
        b.iter(|| queue.sync_fn(foo));
    });

    c.bench_function("dispatch::queue.barrier_async_and_wait_f", |b| {
        b.iter(|| queue.barrier_async_and_wait_f(std::ptr::null_mut(), work))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
