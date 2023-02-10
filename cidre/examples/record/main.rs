use std::{collections::VecDeque, ffi::c_void, time::Duration};

use cidre::{
    arc, at,
    cat::{audio::MPEG4ObjectID, AudioFormatFlags, AudioFormatID},
    cf, cm, define_obj_type, dispatch, ns, objc, os,
    sc::{self, stream::Output, stream::OutputImpl},
    vt::{self, compression_properties::keys, EncodeInfoFlags},
};

#[repr(C)]
pub struct FrameCounterInner {
    video_counter: usize,
    audio_counter: usize,
    audio_queue: AudioQueue,
    session: arc::R<vt::CompressionSession>,
    audio_converter: at::AudioConverterRef,
}

impl FrameCounterInner {
    pub fn _video_counter(&self) -> usize {
        self.video_counter
    }
}

define_obj_type!(FrameCounter + OutputImpl, FrameCounterInner, FRAME_COUNTER);

impl Output for FrameCounter {}

#[objc::add_methods]
impl OutputImpl for FrameCounter {
    extern "C" fn impl_stream_did_output_sample_buffer(
        &mut self,
        _cmd: Option<&cidre::objc::Sel>,
        _stream: &sc::Stream,
        sample_buffer: &mut cm::SampleBuffer,
        kind: sc::OutputType,
    ) {
        let mut inner = self.inner_mut();

        if kind == sc::OutputType::Audio {
            if inner.audio_counter == 0 {
                inner.audio_converter = configured_converter(
                    sample_buffer
                        .format_description()
                        .unwrap()
                        .stream_basic_description()
                        .unwrap(),
                );
            }

            inner.audio_queue.enque(sample_buffer);

            if inner.audio_queue.is_ready() {
                let mut data = [0u8; 2000];
                let buffer = at::AudioBuffer {
                    number_channels: 1,
                    data_bytes_size: data.len() as _,
                    data: data.as_mut_ptr(),
                };
                let buffers = [buffer];
                let mut buf = at::audio::BufferList {
                    number_buffers: buffers.len() as _,
                    buffers,
                };

                let mut size = 1u32;

                inner
                    .audio_converter
                    .fill_complex_buf(convert_audio, &mut inner.audio_queue, &mut size, &mut buf)
                    .unwrap();

                // println!("size {}", buf.buffers[0].data_bytes_size,);
            }

            inner.audio_counter += 1;
            return;
        }

        inner.video_counter += 1;

        let img = sample_buffer.image_buffer();
        if img.is_none() {
            return;
        }
        let img = unsafe { img.unwrap_unchecked() };
        let pts = sample_buffer.presentation_time_stamp();
        let dur = sample_buffer.duration();

        let mut flags = None;

        let res = inner
            .session
            .encode_frame(img, pts, dur, None, std::ptr::null_mut(), &mut flags);
        if res.is_err() {
            println!("err {:?}", res);
        }
    }
}

fn default_converter() -> at::AudioConverterRef {
    let output_asbd = at::audio::StreamBasicDescription {
        //sample_rate: 32_000.0,
        // sample_rate: 44_100.0,
        sample_rate: 48_000.0,
        format_id: AudioFormatID::MPEG4_AAC,
        format_flags: Default::default(),
        // format_flags: AudioFormatFlags(MPEG4ObjectID::AAC_LC.0 as _),
        bytes_per_packet: 0,
        frames_per_packet: 1024,
        bytes_per_frame: 0,
        channels_per_frame: 2,
        bits_per_channel: 0,
        reserved: 0,
    };
    let input_asbd = at::audio::StreamBasicDescription {
        //sample_rate: 32_000.0,
        // sample_rate: 44_100.0,
        sample_rate: 48_000.0,
        format_id: AudioFormatID::LINEAR_PCM,
        //format_flags: AudioFormatFlags(41),
        format_flags: AudioFormatFlags::IS_FLOAT
            | AudioFormatFlags::IS_PACKED
            | AudioFormatFlags::IS_NON_INTERLEAVED,
        bytes_per_packet: 4,
        frames_per_packet: 1,
        bytes_per_frame: 4,
        channels_per_frame: 2,
        bits_per_channel: 32,
        reserved: 0,
    };
    at::AudioConverterRef::with_formats(&input_asbd, &output_asbd).unwrap()
}

fn configured_converter(input_asbd: &at::audio::StreamBasicDescription) -> at::AudioConverterRef {
    // https://www.youtube.com/watch?v=yArrLvMYng8
    let output_asbd = at::audio::StreamBasicDescription {
        //sample_rate: 32_000.0,
        // sample_rate: 44_100.0,
        sample_rate: 48_000.0,
        format_id: AudioFormatID::MPEG4_AAC_HE,
        //format_flags: AudioFormatFlags(MPEG4ObjectID::AAC_LC.0 as _),
        format_flags: AudioFormatFlags(0),
        bytes_per_packet: 0,
        frames_per_packet: 1024,
        bytes_per_frame: 0,
        channels_per_frame: 2,
        bits_per_channel: 0,
        reserved: 0,
    };

    at::AudioConverterRef::with_formats(input_asbd, &output_asbd).unwrap()
}

struct AudioQueue {
    queue: VecDeque<arc::R<cm::SampleBuffer>>,
    last_buffer_offset: i32,
    input_asbd: at::audio::StreamBasicDescription,
}

impl AudioQueue {
    #[inline]
    pub fn enque(&mut self, sbuf: &cm::SampleBuffer) {
        self.queue.push_back(sbuf.retained())
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        self.queue.len() > 2
    }

    pub fn fill_audio_buffer(
        &mut self,
        list: &mut at::audio::BufferList,
    ) -> Result<(), os::Status> {
        let mut left = 1024i32;
        let mut offset: i32 = self.last_buffer_offset as i32;
        let mut out_offset = 0;
        let mut cursor = list.cursor::<2>();
        while let Some(b) = self.queue.pop_front() {
            let samples = b.num_samples() as i32;
            let count = i32::min(samples - offset, left);
            b.copy_pcm_data_into_audio_buffer_list(
                offset,
                count,
                cursor.offset(out_offset, count as _, &self.input_asbd),
            )?;
            left -= count;
            offset = offset + count;
            out_offset += count as usize;
            if offset < samples {
                self.last_buffer_offset = offset;
                self.queue.push_front(b);
                break;
            } else {
                offset = 0;
            }
            if left == 0 {
                break;
            }
        }
        Ok(())
    }
}

extern "C" fn convert_audio(
    _converter: &at::AudioConverter,
    _io_number_data_packets: &mut u32,
    io_data: &mut at::audio::BufferList,
    _out_data_packet_description: *mut at::audio::StreamBasicDescription,
    in_user_data: *mut AudioQueue,
) -> os::Status {
    let q: &mut AudioQueue = unsafe { &mut *in_user_data };

    match q.fill_audio_buffer(io_data) {
        Ok(()) => os::Status(0),
        Err(status) => status,
    }

    //let frames = i32::min(*io_number_data_packets as i32, buf.num_samples() as _);
    //buf.copy_pcm_data_into_audio_buffer_list(0, frames, io_data)
}

struct RecordContext {
    frames_count: usize,
    format_desc: Option<arc::R<cm::VideoFormatDescription>>,
}

impl RecordContext {
    pub fn handle_sample_buffer(&mut self, buffer: &cm::SampleBuffer) {
        if self.frames_count % 1000 == 0 {
            if self.format_desc.is_none() {
                let desc = buffer.format_description().unwrap() as &cm::VideoFormatDescription;

                // let buf = desc
                //     .as_be_image_desc_cm_buffer(Some(cm::ImageDescriptionFlavor::iso_family()))
                //     .unwrap();
                //let slice = buf.data_pointer().unwrap();
                // println!("format desc {:?} len: {}", slice, slice.len());
                // let extensions = desc.extension_atoms().unwrap();
                // let hvcc = cf::String::from_str("hvcC");
                // let value = extensions.get(&hvcc).unwrap().as_data();
                // println!("format desc {:?}", extensions);
                // println!("atoms {:?}", value.as_slice());
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
    status: os::Status,
    flags: EncodeInfoFlags,
    buffer: Option<&cm::SampleBuffer>,
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

    let queue = dispatch::Queue::serial_with_autoreleasepool();
    let content = sc::ShareableContent::current().await.expect("content");
    let ref display = content.displays()[0];
    let mut cfg = sc::StreamConfiguration::new();
    cfg.set_minimum_frame_interval(cm::Time::new(1, FPS));
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);

    // audio
    cfg.set_captures_audio(true);
    cfg.set_excludes_current_process_audio(false);

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
    let max_key_frame_interval_duration = cf::Number::from_f64(5f64);
    let rate_limit = cf::Array::from_type_refs(&[
        &cf::Number::from_i32(200_000),
        &cf::Number::from_f64(0.1f64),
        &cf::Number::from_i32(5_000_000),
        &cf::Number::from_f64(1.0f64),
    ])
    .unwrap();

    let mut props = cf::DictionaryMut::with_capacity(10);
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

    let windows = ns::Array::new();
    let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
    let stream = sc::Stream::new(&filter, &cfg);
    let input_asbd = at::audio::StreamBasicDescription {
        //sample_rate: 32_000.0,
        // sample_rate: 44_100.0,
        sample_rate: 48_000.0,
        format_id: AudioFormatID::LINEAR_PCM,
        //format_flags: AudioFormatFlags(41),
        format_flags: AudioFormatFlags::IS_FLOAT
            | AudioFormatFlags::IS_PACKED
            | AudioFormatFlags::IS_NON_INTERLEAVED,
        bytes_per_packet: 4,
        frames_per_packet: 1,
        bytes_per_frame: 4,
        channels_per_frame: 2,
        bits_per_channel: 32,
        reserved: 0,
    };

    let inner = FrameCounterInner {
        video_counter: 0,
        audio_counter: 0,
        audio_queue: AudioQueue {
            queue: Default::default(),
            last_buffer_offset: 0,
            input_asbd,
        },
        session,
        audio_converter: default_converter(),
    };
    let delegate = FrameCounter::with(inner);
    stream
        .add_stream_output(delegate.as_ref(), sc::OutputType::Screen, Some(&queue))
        .unwrap();
    stream
        .add_stream_output(delegate.as_ref(), sc::OutputType::Audio, Some(&queue))
        .unwrap();

    stream.start().await.unwrap();

    tokio::time::sleep(Duration::from_secs(100_200)).await;

    _ = stream.stop();
}
