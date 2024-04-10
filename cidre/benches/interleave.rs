use std::{ffi::c_void, hint::black_box};

use cidre::{
    at::{self, au, audio},
    av, os, vdsp,
};
use criterion::{criterion_group, criterion_main, Criterion};

const N: usize = 1024;
pub fn criterion_benchmark(c: &mut Criterion) {
    let src_asbd = audio::StreamBasicDesc::common_f32(48_000.0, 2, false);
    let dst_asbd = audio::StreamBasicDesc::common_f32(48_000.0, 2, true);

    let converter = at::AudioConverterRef::with_formats(&src_asbd, &dst_asbd).unwrap();

    let mut list_a: at::AudioBufList<2> = Default::default();
    let mut list_b: at::AudioBufList<1> = Default::default();
    let mut l = black_box(vec![0.0f32; N]);
    let mut r = black_box(vec![0.0f32; N]);
    for i in 0..N {
        l[i] = i as f32;
        r[i] = -(i as f32);
    }
    list_a.buffers[0].data = l.as_ptr() as _;
    list_a.buffers[0].data_bytes_size = (l.len() * 4) as u32;
    list_a.buffers[1].data = r.as_ptr() as _;
    list_a.buffers[1].data_bytes_size = (r.len() * 4) as u32;

    let mut lr = black_box(vec![0.0f32; N * 2]);
    let mut res = black_box(vec![0.0f32; N * 2]);
    let mut j = 0;
    for i in 0..N {
        res[j] = l[i];
        j += 1;
        res[j] = r[i];
        j += 1;
    }
    list_b.buffers[0].data = lr.as_ptr() as _;
    list_b.buffers[0].data_bytes_size = (lr.len() * 4) as u32;

    c.bench_function("interleave with audio format converter", |b| {
        b.iter(|| {
            converter
                .convert_complex_buf(N as u32, &list_a, &mut list_b)
                .unwrap();
        })
    });

    assert_eq!(lr, res);

    let from_fmt = av::AudioFormat::with_asbd(&src_asbd).unwrap();
    let to_fmt = av::AudioFormat::with_asbd(&dst_asbd).unwrap();

    let mut pcm_a = av::AudioPcmBuf::with_format_and_frame_capacity(&from_fmt, N as u32).unwrap();
    let mut pcm_b = av::AudioPcmBuf::with_format_and_frame_capacity(&to_fmt, N as u32).unwrap();

    pcm_a.set_frame_length(N as u32);
    pcm_b.set_frame_length(N as u32);

    let converter = av::AudioConverter::with_formats(&from_fmt, &to_fmt).unwrap();
    c.bench_function("interleave with av::AudioConverter", |b| {
        b.iter(|| {
            converter
                .convert_to_buf_from_buf(&mut pcm_b, &pcm_a)
                .unwrap();
        })
    });

    let mut converter = au::FormatConverter::new_apple().unwrap();
    converter.set_input_stream_format(&src_asbd).unwrap();
    converter.set_output_stream_format(&dst_asbd).unwrap();

    extern "C" fn render(
        _in_ref_con: *mut c_void,
        _io_action_flags: &mut au::RenderActionFlags,
        _in_timestamp: &at::AudioTimeStamp,
        _in_bus_num: u32,
        _in_number_frames: u32,
        _io_data: *mut at::AudioBufList<N>,
    ) -> os::Status {
        os::Status::NO_ERR
    }

    converter
        .set_input_cb(render, std::ptr::null_mut())
        .unwrap();
    converter
        .unit_mut()
        .set_should_allocate_output_buf(false)
        .unwrap();
    let mut converter = converter.allocate_resources().unwrap();

    c.bench_function("interleave with au format converter", |b| {
        b.iter(|| {
            converter.render(N as u32, &mut list_b).unwrap();
        })
    });

    lr.fill(0.0f32);

    c.bench_function("interleave rust", |b| {
        b.iter(|| {
            assert!(l.len() >= N);
            assert!(r.len() >= N);
            assert!(lr.len() >= N * 2);
            let mut j = 0;
            for i in 0..N {
                lr[j] = l[i];
                lr[j + 1] = r[i];
                j += 2;
            }
        })
    });
    assert_eq!(lr, res);

    lr.fill(0.0f32);

    let comp = lr.as_mut_ptr() as *mut vdsp::Complex<f32>;
    let mut comp = unsafe { std::slice::from_raw_parts_mut(comp, N) };
    c.bench_function("interleave vDSP", |b| {
        b.iter(|| {
            vdsp::ztoc_f32(&l, &r, &mut comp);
        })
    });
    assert_eq!(lr, res);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
