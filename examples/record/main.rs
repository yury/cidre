use std::ffi::c_void;

use cidre::{
    av, cf,
    cm::{self, SampleBuffer},
    cv, dispatch,
    os::Status,
    sc::{self, stream::StreamOutput},
    vt::{
        self,
        compression_properties::{h264_entropy_mode, keys, profile_level},
        EncodeInfoFlags,
    },
};

#[repr(C)]
struct FameCounter {
    counter: u32,
    session: cf::Retained<'static, vt::CompressionSession>,
}

impl FameCounter {
    pub fn counter(&self) -> u32 {
        self.counter
    }
}

impl StreamOutput for FameCounter {
    extern "C" fn stream_did_output_sample_buffer_of_type(
        &mut self,
        stream: &sc::Stream,
        sample_buffer: &cm::SampleBuffer,
        of_type: sc::OutputType,
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

extern "C" fn callback(
    ctx: *mut c_void,
    _: *mut c_void,
    status: Status,
    flags: EncodeInfoFlags,
    buffer: Option<&SampleBuffer>,
) {
    // println!("compressed");
}

#[tokio::main]
async fn main() {
    let q = dispatch::Queue::serial_with_autoreleasepool();
    let content = sc::ShareableContent::current().await.expect("content");
    let ref display = content.displays()[0];
    let mut cfg = sc::StreamConfiguration::new();
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);

    let mut session = vt::CompressionSession::new::<c_void>(
        display.width() as u32 * 2,
        display.height() as u32 * 2,
        cm::VideoCodecType::H264,
        None,
        None,
        Some(callback),
        std::ptr::null_mut(),
    )
    .unwrap();

    let bool_true = cf::Boolean::value_true();
    let bool_false = cf::Boolean::value_false();
    let expected_fr = cf::Number::from_i32(60);
    let frame_delay_count = cf::Number::from_i32(0);

    let mut props = cf::MutableDictionary::with_capacity(10);
    props.insert(keys::real_time(), &bool_true);
    props.insert(keys::allow_frame_reordering(), &bool_false);
    props.insert(
        keys::profile_level(),
        profile_level::h264::main_auto_level(),
    );
    props.insert(keys::allow_open_gop(), &bool_false);
    props.insert(keys::h264_entropy_mode(), h264_entropy_mode::cabac());
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
    stream.start().await.expect_err("already started");

    dispatch::main()
}
