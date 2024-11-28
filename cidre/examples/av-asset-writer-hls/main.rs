use std::{fs, io::Write, path::PathBuf};

use cidre::{
    arc, av, av::AssetWriterDelegate, cm, define_obj_type, dispatch, ns, objc, objc::Obj, sc,
    sc::StreamOutput, ut,
};

#[repr(C)]
struct OutputContext {
    input: arc::R<av::AssetWriterInput>,
    writer: arc::R<av::AssetWriter>,
}

define_obj_type!(
    OutputDelegate + sc::StreamOutputImpl,
    OutputContext,
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
            eprint!("s");
            return;
        }
        let ctx = self.inner_mut();
        if ctx.input.is_ready_for_more_media_data() {
            let res = unsafe { ctx.input.append_sample_buf_throws(sample_buffer) };
            if res {
                eprint!(".");
            } else {
                panic!("{:?}", ctx.writer.error());
            }
        } else {
            eprint!("x");
        }
    }
}

struct SegmentWriter {
    n: u32,
    dir: PathBuf,
    base_name: String,
    target_dur: u32,
}

impl SegmentWriter {
    fn init_path(&self) -> PathBuf {
        self.dir.join(format!("{}.mp4", self.base_name))
    }

    fn playlist_path(&self) -> PathBuf {
        self.dir.join(format!("{}.m3u8", self.base_name))
    }

    fn segment_path(&self) -> PathBuf {
        self.dir.join(self.segment_name())
    }

    fn segment_name(&self) -> String {
        format!("{}{}.m4s", self.base_name, self.n)
    }

    fn write_init(&mut self, data: &[u8]) {
        fs::write(self.init_path(), data).unwrap();

        let target_dur = self.target_dur;
        let base_name = &self.base_name;
        fs::write(
            self.playlist_path(),
            format!(
                "#EXTM3U
#EXT-X-VERSION:7
#EXT-X-TARGETDURATION:{target_dur}
#EXT-X-MEDIA-SEQUENCE:0
#EXT-X-MAP:URI=\"{base_name}.mp4\"
"
            ),
        )
        .unwrap();
    }

    fn write_segment(&mut self, data: &[u8], duration: f64) {
        fs::write(self.segment_path(), data).unwrap();
        let mut file = fs::File::options()
            .append(true)
            .open(self.playlist_path())
            .unwrap();
        let segment_name = self.segment_name();
        writeln!(file, "\n#EXTINF:{duration},\n{segment_name}").unwrap();
    }

    fn write_end(&mut self) {
        let mut file = fs::File::options()
            .append(true)
            .open(self.playlist_path())
            .unwrap();
        writeln!(file, "\n#EXT-X-ENDLIST").unwrap();
    }
}

define_obj_type!(
    WriterDelegate + av::AssetWriterDelegateImpl,
    SegmentWriter,
    WRITER_DELEGATE_CLS
);

impl AssetWriterDelegate for WriterDelegate {}

#[objc::add_methods]
impl av::AssetWriterDelegateImpl for WriterDelegate {
    extern "C" fn impl_asset_writer_did_output_segment_data_with_report(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _writer: &av::AssetWriter,
        segment_data: &ns::Data,
        segment_type: av::AssetSegmentType,
        segment_report: Option<&av::AssetSegmentReport>,
    ) {
        let ctx = self.inner_mut();
        match segment_type {
            av::AssetSegmentType::Initialization => {
                ctx.write_init(segment_data.as_slice());
            }
            av::AssetSegmentType::Separable => {
                let duration = segment_report
                    .unwrap()
                    .track_reports()
                    .get(0)
                    .unwrap()
                    .duration()
                    .as_secs();
                ctx.write_segment(segment_data.as_slice(), duration);
                eprintln!("[{}]{duration}", ctx.n);
                ctx.n += 1;
            }
        }
    }
}

fn video_settings() -> arc::R<ns::DictionaryMut<ns::String, ns::Id>> {
    objc::ar_pool(|| {
        let assist =
            av::OutputSettingsAssistant::with_preset(av::OutputSettingsPreset::hevc_1920x1080())
                .unwrap();
        let mut settings = assist.video_settings().unwrap().copy_mut();

        let props_key = av::video_settings_keys::compression_props();
        let v = &settings[props_key];

        // we need to turn off frame reordering for hls
        if let Some(cfg) = v.try_cast(ns::Dictionary::<ns::String, ns::Id>::cls()) {
            let mut cfg = cfg.copy_mut();
            let key = ns::str!(c"AllowFrameReordering");
            cfg.set_obj_for_key(ns::Number::with_i8(0).as_ref(), key);
            settings.set_obj_for_key(&cfg, props_key);
        }
        settings
    })
}

#[tokio::main]
async fn main() {
    const FPS: i32 = 30;
    const TARGET_DUR: u32 = 6;

    let mut delegate = WriterDelegate::with(SegmentWriter {
        n: 0,
        dir: "/tmp/".into(),
        base_name: "hls".into(),
        target_dur: TARGET_DUR,
    });

    let mut input = av::AssetWriterInput::with_media_type_and_output_settings(
        av::MediaType::video(),
        Some(video_settings().as_ref()),
    )
    .unwrap();
    input.set_expects_media_data_in_real_time(true);

    let mut writer = av::AssetWriter::with_content_type(ut::Type::mpeg4movie()).unwrap();
    writer.add_input(&input).unwrap();
    writer.set_delegate(Some(delegate.as_ref()));
    // writer.set_output_file_type_profile(Some(av::FileTypeProfile::mpeg4_cmaf_compliant()));
    writer.set_output_file_type_profile(Some(av::FileTypeProfile::mpeg4_apple_hls()));
    writer.set_preferred_output_segment_interval(cm::Time::with_secs(TARGET_DUR as _, 1));

    // let start_time = cm::Time::with_secs(ca::current_media_time(), 1000000000);
    let start_time = cm::Time::zero();
    writer.set_initial_segment_start_time(start_time);
    let queue = dispatch::Queue::serial_with_ar_pool();

    let content = sc::ShareableContent::current().await.expect("content");
    let display = content.displays().get(0).unwrap();
    let mut cfg = sc::StreamCfg::new();
    cfg.set_minimum_frame_interval(cm::Time::new(1, FPS));
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);
    let windows = ns::Array::new();
    let filter = sc::ContentFilter::with_display_excluding_windows(&display, &windows);
    let stream = sc::Stream::new(&filter, &cfg);
    let output = OutputDelegate::with(OutputContext {
        input,
        writer: writer.clone(),
    });
    stream
        .add_stream_output(output.as_ref(), sc::OutputType::Screen, Some(&queue))
        .expect("failed to add output");

    if writer.start_writing() {
        writer.start_session_at_src_time(start_time);
        stream.start().await.unwrap();
        tokio::signal::ctrl_c().await.unwrap();
        stream.stop().await.unwrap();

        writer.finish_writing();
        delegate.inner_mut().write_end();
    } else {
        eprintln!("failed? {:?}", writer.error());
    }
}
