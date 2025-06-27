use cidre::vimage;
use criterion::{Criterion, criterion_group, criterion_main};
use half::{self, slice::HalfFloatSliceExt};

pub fn criterion_benchmark(c: &mut Criterion) {
    const WIDTH: usize = 1024;
    c.bench_function("f32_to_f16::vimage::1024::DO_NOT_TILE", |b| {
        let f32_buf = vimage::AllocatedBuf::with_size(WIDTH, 1, 32).unwrap();
        let ptr = f32_buf.data as *mut f32;
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, WIDTH) };
        for v in slice.iter_mut() {
            *v = 0.5;
        }
        let mut f16_buf = vimage::AllocatedBuf::with_size(WIDTH, 1, 16).unwrap();
        b.iter(|| {
            f16_buf
                .to_f16_from_f32(&f32_buf, vimage::Flags::DO_NOT_TILE)
                .unwrap();
        });
        let ptr = f16_buf.data as *mut half::f16;
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, WIDTH) };
        for v in slice.iter() {
            let r = v.to_f32();
            assert_eq!(0.5, r);
        }
    });
    c.bench_function("f32_to_f16::vimage::1024::NO_FLAGS", |b| {
        let f32_buf = vimage::AllocatedBuf::with_size(WIDTH, 1, 32).unwrap();
        let mut f16_buf = vimage::AllocatedBuf::with_size(WIDTH, 1, 16).unwrap();
        b.iter(|| {
            f16_buf
                .to_f16_from_f32(&f32_buf, vimage::Flags::default())
                .unwrap();
        });
    });
    c.bench_function("f32_to_f16::vimage::1024::inplace", |b| {
        let mut f32_buf = vimage::AllocatedBuf::with_size(WIDTH, 1, 32).unwrap();
        b.iter(|| {
            f32_buf
                .inplace_to_f16_from_f32(vimage::Flags::DO_NOT_TILE)
                .unwrap();
        });
    });
    c.bench_function("f32_to_f16::slice::1024", |b| {
        let f32_buf = [0.5f32; WIDTH];
        let mut f16_buf = [half::f16::ZERO; WIDTH];
        b.iter(|| vimage::Buf::slice_f32_to_f16(&f32_buf, &mut f16_buf).unwrap());
    });
    c.bench_function("f32_to_f16::half::1024", |b| {
        let f32_buf = [0.5f32; WIDTH];
        let mut f16_buf = [half::f16::ZERO; WIDTH];
        b.iter(|| {
            f16_buf.convert_from_f32_slice(&f32_buf);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
