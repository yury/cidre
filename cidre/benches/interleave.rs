use std::{ffi::c_void, hint::black_box};

use cidre::{
    at::{self, au, audio},
    av, os, vdsp,
};
use criterion::{Criterion, criterion_group, criterion_main};

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

    c.bench_function("interleave with audio format converter convert", |b| {
        b.iter(|| {
            converter
                .convert_complex_buf(N as u32, &list_a, &mut list_b)
                .unwrap();
        });
        assert_eq!(lr, res);
    });

    extern "C-unwind" fn input_proc(
        _converter: &at::AudioConverter,
        io_number_data_packets: &mut u32,
        io_data: &mut audio::BufList<2>,
        _out_data_packet_descriptions: *mut *mut audio::StreamPacketDesc,
        in_user_data: *mut at::AudioBufList<2>,
    ) -> os::Status {
        *io_number_data_packets = N as u32;
        let list_a = unsafe { in_user_data.as_ref().unwrap() };
        io_data.number_buffers = list_a.number_buffers;
        for i in 0..2 {
            let a = &mut io_data.buffers[i];
            let b = &list_a.buffers[i];
            a.number_channels = b.number_channels;
            a.data_bytes_size = b.data_bytes_size;
            a.data = b.data;
        }
        os::Status::NO_ERR
    }

    c.bench_function("interleave with audio format converter fill", |b| {
        b.iter(|| {
            let mut output_size = N as u32;
            converter
                .fill_complex_buf(input_proc, &mut list_a, &mut output_size, &mut list_b)
                .unwrap();
        });
        assert_eq!(lr, res);
    });

    #[cfg(feature = "macos_26_0")]
    c.bench_function(
        "interleave with audio format converter fill realtime safe",
        |b| {
            b.iter(|| {
                let mut output_size = N as u32;
                converter
                    .fill_complex_buf_realtime_safe(
                        input_proc,
                        &mut list_a,
                        &mut output_size,
                        &mut list_b,
                    )
                    .unwrap()
            });
            assert_eq!(lr, res);
        },
    );

    let from_fmt = av::AudioFormat::with_asbd(&src_asbd).unwrap();
    let to_fmt = av::AudioFormat::with_asbd(&dst_asbd).unwrap();

    let mut pcm_a = av::AudioPcmBuf::with_format(&from_fmt, N as u32).unwrap();
    let mut pcm_b = av::AudioPcmBuf::with_format(&to_fmt, N as u32).unwrap();

    pcm_a
        .set_frame_len(N as u32)
        .expect("Failed to set frame length on buf a");
    pcm_b
        .set_frame_len(N as u32)
        .expect("Failed to set frame length on buf b");

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

    extern "C-unwind" fn render(
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

    c.bench_function("interleave with au::FormatConverter", |b| {
        b.iter(|| {
            converter.render(N as u32, &mut list_b).unwrap();
        })
    });

    c.bench_function("interleave rust", |b| {
        lr.fill(0.0f32);

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
        });
        assert_eq!(lr, res);
    });

    #[cfg(target_arch = "aarch64")]
    c.bench_function("interleave rust neon", |b| {
        lr.fill(0.0f32);

        b.iter(|| {
            unsafe { interleave_f32_neon(&l, &r, &mut lr) };
        });
        assert_eq!(lr, res);
    });

    c.bench_function("interleave vDSP", |b| {
        lr.fill(0.0f32);
        let comp = lr.as_mut_ptr() as *mut vdsp::Complex<f32>;
        let mut comp = unsafe { std::slice::from_raw_parts_mut(comp, N) };

        b.iter(|| {
            vdsp::ztoc_f32(&l, &r, &mut comp);
        });
        assert_eq!(lr, res);
    });
}

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
unsafe fn interleave_f32_neon(l: &[f32], r: &[f32], lr: &mut [f32]) {
    // Ensure arrays have the same length and output is twice the length
    // assert_eq!(
    //     l.len(),
    //     r.len(),
    //     "Input arrays must have equal length"
    // );
    // assert_eq!(
    //     lr.len(),
    //     l.len() * 2,
    //     "Output array must be twice the input length"
    // );

    let length = l.len();
    let mut i = 0;

    // Process 4 elements at a time using NEON
    while i + 3 < length {
        // Load 4 floats from each array into NEON registers
        let vec1 = unsafe { vld1q_f32(l.as_ptr().add(i)) };
        let vec2 = unsafe { vld1q_f32(r.as_ptr().add(i)) };

        // Create a pair for interleaved store
        let pair = float32x4x2_t(vec1, vec2);

        // Store interleaved result to output
        unsafe { vst2q_f32(lr.as_mut_ptr().add(2 * i), pair) };

        i += 4;
    }

    // Handle remaining elements (if length is not a multiple of 4)
    for j in i..length {
        lr[2 * j] = l[j];
        lr[2 * j + 1] = r[j];
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
