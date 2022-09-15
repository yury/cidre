use std::{ffi::c_void, net::SocketAddr, time::Duration};

use cidre::{
    av, cf,
    cm::{self, SampleBuffer},
    dispatch,
    os::Status,
    sc::{self, stream::StreamOutput},
    vt::{
        self,
        compression_properties::{keys, profile_level},
        EncodeInfoFlags,
    },
};

#[repr(C)]
struct FameCounter {
    counter: u32,
    session: cf::Retained<vt::CompressionSession>,
}

impl FameCounter {
    pub fn _counter(&self) -> u32 {
        self.counter
    }
}

impl StreamOutput for FameCounter {
    extern "C" fn stream_did_output_sample_buffer_of_type(
        &mut self,
        _stream: &sc::Stream,
        sample_buffer: &cm::SampleBuffer,
        _of_type: sc::OutputType,
    ) {
        self.counter += 1;
        // why without println is not working well?
        // println!("frame {:?}", self.counter);

        let img = sample_buffer.image_buffer();
        if img.is_none() {
            return;
        }
        let img = img.unwrap();
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

extern "C" fn _callback2(
    ctx: *mut c_void,
    _: *mut c_void,
    status: Status,
    _flags: EncodeInfoFlags,
    buffer: Option<&SampleBuffer>,
) {
    // println!("compressed");
    if status.is_err() || buffer.is_none() {
        println!("status {:?}", status);
        return;
    }

    //    sender: flume::Sender<Cmd>,
    let ctx = ctx as *mut cf::Retained<av::AssetWriterInput>;
    let ctx = unsafe { ctx.as_ref().unwrap() };

    let buf = buffer.unwrap();
    let data_buffer = buf.data_buffer().unwrap();
    let data = data_buffer.data_pointer().unwrap();
    assert_eq!(data.len(), data_buffer.data_len());
    println!("{:?}", data.len());

    if ctx.is_ready_for_more_media_data() {
        ctx.append_sample_buffer(buf);
    }
}

extern "C" fn callback(
    ctx: *mut c_void,
    _: *mut c_void,
    status: Status,
    _flags: EncodeInfoFlags,
    buffer: Option<&SampleBuffer>,
) {
    // println!("compressed");
    if status.is_err() || buffer.is_none() {
        println!("status {:?}", status);
        return;
    }

    let ctx = ctx as *mut flume::Sender<rt::Cmd>;
    let ctx = unsafe { ctx.as_ref().unwrap() };

    let buf = buffer.unwrap().retained();
    ctx.send(rt::Cmd::Schedule {
        kind: rt::MessageKind::VideoKey,
        body: buf.data_buffer().unwrap().retained(),
    })
    .unwrap();
    //let data_buffer = buf.data_buffer().unwrap();
    //let data = data_buffer.data_pointer().unwrap();
    //assert_eq!(data.len(), data_buffer.data_len());
    //println!("{:?}", data.len());
}

#[tokio::main]
async fn main() {
    let q = dispatch::Queue::serial_with_autoreleasepool();
    let content = sc::ShareableContent::current().await.expect("content");
    let ref display = content.displays()[0];
    let mut cfg = sc::StreamConfiguration::new();
    cfg.set_minimum_frame_interval(cm::Time::new(1, 30));
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);

    let format = cm::FormatDescription::video(
        cm::VideoCodecType::H264,
        display.width() as i32 * 2,
        display.height() as i32 * 2,
        None,
    )
    .unwrap();

    let writer_input =
        av::AssetWriterInput::with_media_type_and_format_hint(av::MediaType::video(), &format);
    writer_input.set_expects_media_data_in_real_time(true);
    let url = cf::URL::from_str("file:///Users/yury/bla.mp4").unwrap();

    let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    writer.add_input(&writer_input);
    writer.start_writing();
    let start = cm::Time::with_seconds(0.2f64, 10_000);
    let start = cm::Clock::host_time_clock().time().add(&start);
    writer.start_session_at_source_time(start);

    //let input = Box::new(writer_input);
    //let addr = SocketAddr::V4("127.0.0.1:8080".parse().unwrap());
    // let addr = SocketAddr::V4("192.168.135.174:8080".parse().unwrap());
    //let addr = SocketAddr::V4("10.0.1.10:8080".parse().unwrap());
    let addr = SocketAddr::V4("192.168.135.113:8080".parse().unwrap());
    // let addr = SocketAddr::V4("172.20.10.1:8080".parse().unwrap());

    let sender = rt::create_sender(addr, 0);
    let input = Box::new(sender);

    let mut session = vt::CompressionSession::new::<c_void>(
        1920, // display.width() as u32 * 2,
        1080, // display.height() as u32 * 2,
        cm::VideoCodecType::HEVC,
        None,
        None,
        Some(callback),
        Box::into_raw(input) as _,
    )
    .unwrap();

    println!(
        "rendering with {}x{}",
        display.width() * 2,
        display.height() * 2
    );

    let bool_true = cf::Boolean::value_true();
    let bool_false = cf::Boolean::value_false();
    let expected_fr = cf::Number::from_i32(30);
    let frame_delay_count = cf::Number::from_i32(0);
    let max_key_frame_interval = cf::Number::from_i32(30 * 5);
    let rate_limit =
        cf::Array::from_type_refs(&[&cf::Number::from_i32(3_000_000), &cf::Number::from_i32(1)])
            .unwrap();

    let mut props = cf::MutableDictionary::with_capacity(10);
    props.insert(keys::real_time(), bool_true);
    props.insert(keys::allow_frame_reordering(), bool_false);
    props.insert(keys::max_key_frame_interval(), &max_key_frame_interval);
    props.insert(keys::data_rate_limits(), &rate_limit);
    props.insert(
        keys::profile_level(),
        profile_level::hevc::main_auto_level(),
    );
    // props.insert(keys::allow_open_gop(), bool_false);
    // props.insert(keys::h264_entropy_mode(), h264_entropy_mode::cabac());
    props.insert(keys::expected_frame_rate(), &expected_fr);
    props.insert(keys::max_frame_delay_count(), &frame_delay_count);

    session.set_props(&props).unwrap();
    session.prepare().unwrap();

    let windows = cf::ArrayOf::<sc::Window>::new().unwrap();
    let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
    let stream = sc::Stream::new(&filter, &cfg);

    let delegate = FameCounter {
        counter: 0,
        session,
    };
    let d = delegate.delegate();
    let mut error = None;
    stream.add_stream_output(&d, sc::OutputType::Screen, Some(&q), &mut error);
    assert!(error.is_none());
    stream.start().await.expect("started");

    // cf::RunLoop::run
    tokio::time::sleep(Duration::from_secs(1200)).await;

    //    dispatch::Queue::main().async_with(move || {
    _ = stream.stop();

    writer.end_session_at_source_time(cm::Clock::host_time_clock().time());
    writer.finish_writing();
    //  })
}
