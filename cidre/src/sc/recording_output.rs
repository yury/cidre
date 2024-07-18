use crate::{api, arc, av, cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "SCRecordingOutputConfiguration")]
    pub RecordingOutputCfg(ns::Id),
    SC_RECORDING_OUTPUT_CONFIGURATION,
    #[api::available(macos = 15.0)]
);

impl RecordingOutputCfg {
    #[objc::msg_send(outputURL)]
    pub fn output_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setOutputURL:)]
    pub fn set_output_url(&mut self, val: &ns::Url);

    #[objc::msg_send(videoCodecType)]
    pub fn video_codec(&self) -> arc::R<av::VideoCodec>;

    #[objc::msg_send(outputFileType)]
    pub fn output_file_type(&self) -> arc::R<av::FileType>;

    #[objc::msg_send(availableVideoCodecTypes)]
    pub fn available_video_codecs(&self) -> arc::R<ns::Array<av::VideoCodec>>;

    #[objc::msg_send(availableOutputFileTypes)]
    pub fn available_output_file_types(&self) -> arc::R<ns::Array<av::FileType>>;
}

#[objc::obj_trait]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(recordingOutputDidStartRecording:)]
    fn recording_output_did_start_recording(&mut self, recording_output: &mut RecordingOutput);

    #[objc::optional]
    #[objc::msg_send(recordingOutput:didFailWithError:)]
    fn recording_output_did_fail_with_err(
        &mut self,
        recording_output: &mut RecordingOutput,
        error: &ns::Error,
    );

    #[objc::optional]
    #[objc::msg_send(recordingOutputDidFinishRecording:)]
    fn recording_output_did_finish_recording(&mut self, recording_output: &mut RecordingOutput);
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}

define_obj_type!(
    #[doc(alias = "SCRecordingOutput")]
    pub RecordingOutput(ns::Id)
);

impl arc::A<RecordingOutput> {
    #[objc::msg_send(initWithConfiguration:delegate:)]
    pub fn init_with_cfg_delegate<D: Delegate>(
        self,
        cfg: &RecordingOutputCfg,
        delegate: &D,
    ) -> arc::Retained<RecordingOutput>;
}

impl RecordingOutput {
    #[api::available(macos = 15.0)]
    crate::define_cls!(SC_RECORDING_OUTPUT);

    /// Indicates current duration of recording to the output file.
    #[objc::msg_send(recordedDuration)]
    pub fn recorded_duration(&self) -> cm::Time;

    /// Indicates current size, in bytes, of the data recorded to the output file.
    #[objc::msg_send(recordedFileSize)]
    pub fn recorded_file_size(&self) -> isize;

    #[inline]
    #[api::available(macos = 15.0)]
    pub fn with_cfg(cfg: &RecordingOutputCfg, delegate: &impl Delegate) -> arc::R<Self> {
        Self::alloc().init_with_cfg_delegate(cfg, delegate)
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    static SC_RECORDING_OUTPUT_CONFIGURATION: &'static objc::Class<RecordingOutputCfg>;
    static SC_RECORDING_OUTPUT: &'static objc::Class<RecordingOutput>;
}

#[cfg(test)]
mod tests {
    use crate::{api, av, sc};

    #[cfg(not(feature = "macos_15_0"))]
    #[test]
    fn basics() {
        if api::version!(macos = 15.0) {
            let cfg = sc::RecordingOutputCfg::new().unwrap();
            assert!(cfg.output_url().is_none());

            let codec = cfg.video_codec();

            assert_eq!(&codec, av::VideoCodec::h264());

            let available_codecs = cfg.available_video_codecs();
            assert!(!available_codecs.is_empty());

            let available_file_types = cfg.available_output_file_types();
            assert!(!available_file_types.is_empty());
        } else {
            assert!(sc::RecordingOutputCfg::new().is_none());
        }
    }
}
