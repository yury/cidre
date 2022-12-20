use std::{ffi::c_void, time::Duration};

use cidre::{
    at,
    cat::{audio::MPEG4ObjectID, AudioFormatFlags, AudioFormatID},
    cf,
    cm::{self, SampleBuffer},
    dispatch,
    os::Status,
    sc::{self, stream::StreamOutput},
    vt::{self, compression_properties::keys, EncodeInfoFlags},
};

#[repr(C)]
struct FrameCounter {
    counter: usize,
    session: cf::Retained<vt::CompressionSession>,
    audio_converter: Option<at::AudioConverterRef>,
}

impl FrameCounter {
    pub fn _counter(&self) -> usize {
        self.counter
    }
}

fn configured_converter(input_asbd: &at::audio::StreamBasicDescription) -> at::AudioConverterRef {
    let output_asbd = at::audio::StreamBasicDescription {
        //sample_rate: 32_000.0,
        // sample_rate: 44_100.0,
        sample_rate: 48_000.0,
        format_id: AudioFormatID::MPEG4_AAC,
        format_flags: AudioFormatFlags(MPEG4ObjectID::AAC_LC.0 as _),
        bytes_per_packet: 0,
        frames_per_packet: 1024,
        bytes_per_frame: 0,
        channels_per_frame: 2,
        bits_per_channel: 0,
        reserved: 0,
    };

    let c = at::AudioConverterRef::with_formats(input_asbd, &output_asbd).unwrap();

    println!("{:?}", c.applicable_encode_bit_rates());
    println!("{:?}", c.applicable_encode_sample_rates());

    return c;
}

impl StreamOutput for FrameCounter {
    extern "C" fn stream_did_output_sample_buffer_of_type(
        &mut self,
        _stream: &sc::Stream,
        sample_buffer: &cm::SampleBuffer,
        of_type: sc::OutputType,
    ) {
        if of_type == sc::OutputType::Audio {
            if self.audio_converter.is_none() {
                self.audio_converter = Some(configured_converter(
                    sample_buffer
                        .format_description()
                        .unwrap()
                        .stream_basic_description()
                        .unwrap(),
                ));
            }
            return;
        }

        self.counter += 1;
        // why without println is not working well?
        // println!("frame {:?}", self.counter);

        let img = sample_buffer.image_buffer();
        if img.is_none() {
            return;
        }
        let img = unsafe { img.unwrap_unchecked() };
        let pts = sample_buffer.presentation_time_stamp();
        let dur = sample_buffer.duration();

        let mut flags = None;

        let res = self
            .session
            .encode_frame(img, pts, dur, None, std::ptr::null_mut(), &mut flags);
        if res.is_err() {
            println!("err {:?}", res);
        }
    }
}

struct RecordContext {
    frames_count: usize,
    format_desc: Option<cf::Retained<cm::VideoFormatDescription>>,
}

impl RecordContext {
    pub fn handle_sample_buffer(&mut self, buffer: &SampleBuffer) {
        if self.frames_count % 1000 == 0 {
            if self.format_desc.is_none() {
                let desc = buffer.format_description().unwrap() as &cm::VideoFormatDescription;

                let buf = desc
                    .as_be_image_desc_cm_buffer(Some(cm::ImageDescriptionFlavor::iso_family()))
                    .unwrap();
                let slice = buf.data_pointer().unwrap();
                println!("format desc {:?} len: {}", slice, slice.len());
                let extensions = desc.extension_atoms().unwrap();
                let hvcc = cf::String::from_str("hvcC");
                let value = extensions.get(&hvcc).unwrap().as_data();
                println!("format desc {:?}", extensions);
                println!("atoms {:?}", value.as_slice());
                // store current format description
                self.format_desc = Some(desc.retained());
            }
        }

        self.frames_count += 1;
    }
}

extern "C" fn callback(
    ctx: *mut RecordContext,
    _: *mut c_void,
    status: Status,
    flags: EncodeInfoFlags,
    buffer: Option<&SampleBuffer>,
) {
    if status.is_err() || buffer.is_none() {
        println!("status {:?} Flags: {:#b}", status, flags);
        return;
    }
    unsafe {
        let ctx = ctx.as_mut().unwrap_unchecked();
        ctx.handle_sample_buffer(buffer.unwrap_unchecked());
    }
}

#[tokio::main]
async fn main() {
    const FPS: i32 = 60;

    let q = dispatch::Queue::serial_with_autoreleasepool();
    let content = sc::ShareableContent::current().await.expect("content");
    let ref display = content.displays()[0];
    let mut cfg = sc::StreamConfiguration::new();
    cfg.set_minimum_frame_interval(cm::Time::new(1, FPS));
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);

    // audio
    cfg.set_captures_audio(true);

    let input = Box::new(RecordContext {
        frames_count: 0,
        format_desc: None,
    });

    let memory_pool = cm::MemoryPool::new();
    let memory_pool_allocator = memory_pool.pool_allocator();

    let mut session = vt::CompressionSession::new(
        1440 * 2, // display.width() as u32 * 2,
        900 * 2,  // display.height() as u32 * 2,
        cm::VideoCodecType::HEVC,
        None,
        None,
        Some(memory_pool_allocator),
        Some(callback),
        Box::into_raw(input) as _,
    )
    .unwrap();

    println!(
        "rendering with {}x{}",
        display.width() * 2,
        display.height() * 2
    );

    let expected_bitrate = cf::Number::from_i32(4_500_000);
    let bool_true = cf::Boolean::value_true();
    let bool_false = cf::Boolean::value_false();
    let expected_fr = cf::Number::from_i32(FPS);
    let frame_delay_count = cf::Number::from_i32(0);
    let max_key_frame_interval = cf::Number::from_i32(FPS * 5);
    let max_key_frame_interval_duration = cf::Number::from_f64(5f64).unwrap();
    let rate_limit = cf::Array::from_type_refs(&[
        &cf::Number::from_i32(200_000),
        &cf::Number::from_f64(0.1f64).unwrap(),
        &cf::Number::from_i32(5_000_000),
        &cf::Number::from_f64(1.0f64).unwrap(),
    ])
    .unwrap();

    let mut props = cf::MutableDictionary::with_capacity(10);
    props.insert(keys::real_time(), bool_true);
    props.insert(keys::allow_frame_reordering(), bool_false);
    props.insert(keys::max_key_frame_interval(), &max_key_frame_interval);
    props.insert(
        keys::max_key_frame_interval_duration(),
        &max_key_frame_interval_duration,
    );
    props.insert(keys::data_rate_limits(), &rate_limit);
    props.insert(keys::avarage_bit_rate(), &expected_bitrate);
    //props.insert(keys::constant_bit_rate(), &expected_bitrate);
    // props.insert(
    //     keys::profile_level(),
    //     profile_level::hevc::main_auto_level(),
    // );
    // props.insert(keys::allow_open_gop(), bool_false);
    // props.insert(keys::h264_entropy_mode(), h264_entropy_mode::cabac());
    props.insert(keys::expected_frame_rate(), &expected_fr);
    props.insert(keys::max_frame_delay_count(), &frame_delay_count);

    session.set_props(&props).unwrap();
    session.prepare().unwrap();

    let windows = cf::ArrayOf::new();
    let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
    let stream = sc::Stream::new(&filter, &cfg);

    let delegate = FrameCounter {
        counter: 0,
        session,
        audio_converter: None,
    };
    let d = delegate.delegate();
    let mut error = None;
    stream.add_stream_output(&d, sc::OutputType::Screen, Some(&q), &mut error);
    stream.add_stream_output(&d, sc::OutputType::Audio, Some(&q), &mut error);
    assert!(error.is_none());
    stream.start().await.expect("started");

    tokio::time::sleep(Duration::from_secs(100_200)).await;

    _ = stream.stop();
}
