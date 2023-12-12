use std::time::Duration;

use cidre::{
    arc, av, av::AssetWriterDelegate, cm, define_obj_type, dispatch, ns, objc, sc,
    sc::StreamOutput, ut,
};

#[repr(C)]
struct Context {
    input: arc::R<av::AssetWriterInput>,
    writer: arc::R<av::AssetWriter>,
}

define_obj_type!(
    OutputDelegate + sc::StreamOutputImpl,
    Context,
    OUTPUT_DELEGATE_CLS
);

impl sc::StreamOutput for OutputDelegate {}

#[objc::add_methods]
impl sc::StreamOutputImpl for OutputDelegate {
    extern "C" fn impl_stream_did_output_sample_buf(
        &mut self,
        _cmd: Option<&cidre::objc::Sel>,
        _stream: &sc::Stream,
        sample_buffer: &mut cm::SampleBuf,
        _kind: sc::OutputType,
    ) {
        if sample_buffer.image_buf().is_none() {
            // skipping sample buffers without image buffers
            eprint!("n");
            return;
        }
        let ctx = self.inner_mut();
        if ctx.input.is_ready_for_more_media_data() {
            let res = unsafe { ctx.input.append_sample_buf_throws(sample_buffer) };
            if res {
                eprint!(".");
            } else {
                eprint!("{:?}", ctx.writer.error());
            }
        } else {
            eprint!("x");
        }
    }
}

define_obj_type!(
    WriterDelegate + av::AssetWriterDelegateImpl,
    usize,
    WRITER_DELEGATE_CLS
);

impl AssetWriterDelegate for WriterDelegate {}

#[objc::add_methods]
impl av::AssetWriterDelegateImpl for WriterDelegate {
    extern "C" fn impl_asset_writer_did_output_segment_data_with_report(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _writer: &av::AssetWriter,
        _segment_data: &ns::Data,
        segment_type: av::AssetSegmentType,
        segment_report: Option<&av::AssetSegmentReport>,
    ) {
        eprintln!("{segment_type:?} {segment_report:?}");
        if let Some(report) = segment_report {
            let report = &report.track_reports()[0];
            eprintln!("duration: {:?}", report.duration().seconds());
        }
    }
}

#[tokio::main]
async fn main() {
    const FPS: i32 = 30;

    let delegate = WriterDelegate::with(0);
    let mut writer = av::AssetWriter::with_content_type(ut::Type::mpeg4movie()).unwrap();

    let assist =
        av::OutputSettingsAssistant::with_preset(av::OutputSettingsPreset::hevc_1920x1080())
            .unwrap();
    let mut input = av::AssetWriterInput::with_media_type_and_output_settings(
        av::MediaType::video(),
        assist.video_settings().as_deref(),
    )
    .unwrap();
    input.set_expects_media_data_in_real_time(true);

    writer.add_input(&input).unwrap();
    writer.set_delegate(Some(delegate.as_ref()));
    writer.set_output_file_type_profile(Some(av::FileTypeProfile::mpeg4_apple_hls()));
    writer.set_preferred_output_segment_interval(cm::Time::with_seconds(6.0, 1));
    writer.set_initial_segment_start_time(cm::Time::zero());
    let queue = dispatch::Queue::serial_with_autoreleasepool();

    let content = sc::ShareableContent::current().await.expect("content");
    let ref display = content.displays()[0];
    let mut cfg = sc::StreamCfg::new();
    cfg.set_minimum_frame_interval(cm::Time::new(1, FPS));
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);
    let windows = ns::Array::new();
    let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
    let stream = sc::Stream::new(&filter, &cfg);
    let output = OutputDelegate::with(Context {
        input,
        writer: writer.clone(),
    });
    stream
        .add_stream_output(output.as_ref(), sc::OutputType::Screen, Some(&queue))
        .expect("failed to add output");
    if writer.start_writing() {
        writer.start_session_at_source_time(cm::Time::zero());
        stream.start().await.unwrap();
    } else {
        eprintln!("failed? {:?}", writer.error());
    }
    eprintln!("started");

    tokio::time::sleep(Duration::from_secs(100)).await;
    eprintln!("ended");
}
