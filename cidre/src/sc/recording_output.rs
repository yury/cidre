use crate::{api, arc, av, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;

define_obj_type!(
    /// Configuration for recording stream content to a movie file.
    #[doc(alias = "SCRecordingOutputConfiguration")]
    pub RecordingOutputCfg(ns::Id),
    SC_RECORDING_OUTPUT_CONFIGURATION,
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
);

impl RecordingOutputCfg {
    /// The destination URL for the recording file.
    #[objc::msg_send(outputURL)]
    pub fn output_url(&self) -> Option<arc::R<ns::Url>>;

    /// Sets the destination URL for the recording file.
    #[objc::msg_send(setOutputURL:)]
    pub fn set_output_url(&mut self, val: &ns::Url);

    /// The video codec used for recording.
    #[objc::msg_send(videoCodecType)]
    pub fn video_codec(&self) -> arc::R<av::VideoCodec>;

    /// The file container type used for recording.
    #[objc::msg_send(outputFileType)]
    pub fn output_file_type(&self) -> arc::R<av::FileType>;

    /// The video codecs supported by the current configuration.
    #[objc::msg_send(availableVideoCodecTypes)]
    pub fn available_video_codecs(&self) -> arc::R<ns::Array<av::VideoCodec>>;

    /// The output file types supported by the current configuration.
    #[objc::msg_send(availableOutputFileTypes)]
    pub fn available_output_file_types(&self) -> arc::R<ns::Array<av::FileType>>;

    /// Whether system audio and microphone audio are mixed into one track.
    #[objc::msg_send(mixesAudioWithMicrophone)]
    #[api::available(macos = 27.0, maccatalyst = 27.0, ios = 27.0, visionos = 27.0)]
    pub fn mixes_audio_with_mic(&self) -> bool;

    /// Sets whether system audio and microphone audio are mixed into one track.
    #[objc::msg_send(setMixesAudioWithMicrophone:)]
    #[api::available(macos = 27.0, maccatalyst = 27.0, ios = 27.0, visionos = 27.0)]
    pub fn set_mixes_audio_with_mic(&mut self, val: bool);
}

/// Receives recording output lifecycle callbacks.
#[objc::protocol(SCRecordingOutputDelegate)]
pub trait Delegate: objc::Obj {
    /// Called after recording starts successfully.
    #[objc::optional]
    #[objc::msg_send(recordingOutputDidStartRecording:)]
    fn recording_output_did_start_recording(&mut self, recording_output: &mut RecordingOutput);

    /// Called when recording fails.
    #[objc::optional]
    #[objc::msg_send(recordingOutput:didFailWithError:)]
    fn recording_output_did_fail_with_err(
        &mut self,
        recording_output: &mut RecordingOutput,
        error: &ns::Error,
    );

    /// Called after recording finishes successfully.
    #[objc::optional]
    #[objc::msg_send(recordingOutputDidFinishRecording:)]
    fn recording_output_did_finish_recording(&mut self, recording_output: &mut RecordingOutput);
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}

define_obj_type!(
    /// An output that records stream content to a file.
    #[doc(alias = "SCRecordingOutput")]
    pub RecordingOutput(ns::Id)
);

impl arc::A<RecordingOutput> {
    /// Initializes a recording output with a configuration and delegate.
    #[objc::msg_send(initWithConfiguration:delegate:)]
    pub fn init_with_cfg_delegate<D: Delegate>(
        self,
        cfg: &RecordingOutputCfg,
        delegate: &D,
    ) -> arc::Retained<RecordingOutput>;
}

impl RecordingOutput {
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    crate::define_cls!(SC_RECORDING_OUTPUT);

    /// Indicates current duration of recording to the output file.
    #[cfg(feature = "cm")]
    #[objc::msg_send(recordedDuration)]
    pub fn recorded_duration(&self) -> cm::Time;

    /// Indicates current size, in bytes, of the data recorded to the output file.
    #[objc::msg_send(recordedFileSize)]
    pub fn recorded_file_size(&self) -> isize;

    #[inline]
    /// Creates a recording output with a configuration and delegate.
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn with_cfg(cfg: &RecordingOutputCfg, delegate: &impl Delegate) -> arc::R<Self> {
        Self::alloc().init_with_cfg_delegate(cfg, delegate)
    }
}

unsafe extern "C" {
    static SC_RECORDING_OUTPUT_CONFIGURATION: &'static objc::Class<RecordingOutputCfg>;
    static SC_RECORDING_OUTPUT: &'static objc::Class<RecordingOutput>;
}

#[cfg(test)]
mod tests {
    use crate::{av, sc};

    #[cfg(not(feature = "macos_15_0"))]
    use crate::api;

    #[cfg(feature = "macos_15_0")]
    #[test]
    fn basics() {
        let cfg = sc::RecordingOutputCfg::new();
        assert!(cfg.output_url().is_none());

        let codec = cfg.video_codec();

        assert_eq!(&codec, av::VideoCodec::h264());

        let available_codecs = cfg.available_video_codecs();
        assert!(!available_codecs.is_empty());

        let available_file_types = cfg.available_output_file_types();
        assert!(!available_file_types.is_empty());
    }

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
