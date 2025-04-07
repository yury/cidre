use cidre::{ns, objc, objc::Obj};
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let _str = ns::String::new();
    let str = ns::str!(c"hello");

    c.bench_function("impl_call:msg_send", |b| {
        b.iter(|| {
            let len = str.len();
            assert_eq!(len, 5);
        })
    });

    let impl_fn = unsafe {
        str.class()
            .method_impl(objc::sel_reg_name(c"length".as_ptr()))
    };
    assert!(!impl_fn.is_null());

    let impl_fn: extern "C-unwind" fn(ptr: *const ns::String) -> usize =
        unsafe { std::mem::transmute(impl_fn) };

    c.bench_function("impl_call:direct", |b| {
        b.iter(|| {
            let len = impl_fn(str);
            assert_eq!(len, 5);
        })
    });

    let cf_str = str.as_cf();

    c.bench_function("impl_call:cf_call", |b| {
        b.iter(|| {
            let len = cf_str.len();
            assert_eq!(len, 5);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
