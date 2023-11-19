use crate::{arc, av, cm, define_obj_type, dispatch, ns, objc};

use super::Output;

#[objc::obj_trait]
pub trait AudioDataOutputSampleBufDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(captureOutput:didOutputSampleBuffer:fromConnection:)]
    fn capture_output_did_output_sample_buf_from_connection(
        &mut self,
        output: &av::CaptureOutput,
        sample_buf: &cm::SampleBuf,
        connection: &av::CaptureConnection,
    );
}

define_obj_type!(pub AudioDataOutput(Output), AV_CAPTURE_AUDIO_DATA_OUTPUT);
impl AudioDataOutput {
    #[objc::msg_send(setSampleBufferDelegate:queue:)]
    pub fn set_sample_buf_delegate<D: AudioDataOutputSampleBufDelegate>(
        &mut self,
        delegate: Option<&D>,
        queue: Option<&dispatch::Queue>,
    );

    /// The dispatch queue on which all sample buffer delegate methods will be called.
    #[objc::msg_send(sampleBufferCallbackQueue)]
    pub fn sample_buf_callback_queue(&self) -> Option<&dispatch::Queue>;

    #[objc::msg_send(recommendedAudioSettingsForAssetWriterWithOutputFileType:)]
    pub fn recommended_audio_settings_for_asset_writer_with_output_file_type<'a>(
        &'a self,
        output_file_type: &av::FileType,
    ) -> Option<&'a ns::Dictionary<ns::String, ns::Id>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_AUDIO_DATA_OUTPUT: &'static objc::Class<AudioDataOutput>;
}
