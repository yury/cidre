use crate::{arc, av, cm, define_obj_type, ns, objc};

#[cfg(feature = "dispatch")]
use crate::dispatch;

use super::Output;

#[objc::protocol(AVCaptureVideoDataOutputSampleBufferDelegate)]
pub trait VideoDataOutputSampleBufDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(captureOutput:didOutputSampleBuffer:fromConnection:)]
    fn capture_output_did_output_sample_buf_from_connection(
        &mut self,
        output: &av::CaptureOutput,
        sample_buffer: &cm::SampleBuf,
        connection: &av::CaptureConnection,
    );

    #[objc::optional]
    #[objc::msg_send(captureOutput:didDropSampleBuffer:fromConnection:)]
    fn capture_output_did_drop_sample_buf_from_connection(
        &mut self,
        output: &av::CaptureOutput,
        sample_buffer: &cm::SampleBuf,
        connection: &av::CaptureConnection,
    );
}

define_obj_type!(
    #[doc(alias = "AVCaptureVideoDataOuput")]
    pub VideoDataOutput(Output), AV_CAPTURE_VIDEO_DATA_OUTPUT
);

impl VideoDataOutput {
    #[cfg(feature = "dispatch")]
    #[objc::msg_send(setSampleBufferDelegate:queue:)]
    pub fn set_sample_buf_delegate<D: VideoDataOutputSampleBufDelegate>(
        &mut self,
        delegate: Option<&D>,
        queue: Option<&dispatch::Queue>,
    );

    #[objc::msg_send(alwaysDiscardsLateVideoFrames)]
    pub fn always_discard_late_video_frames(&self) -> bool;

    #[objc::msg_send(setAlwaysDiscardsLateVideoFrames:)]
    pub fn set_always_discard_late_video_frames(&mut self, value: bool);

    /// Indicates whether the receiver automatically configures the size of output buffers.
    #[objc::msg_send(automaticallyConfiguresOutputBufferDimensions)]
    pub fn automatically_configures_output_buf_dimensions(&self) -> bool;

    #[objc::msg_send(setAutomaticallyConfiguresOutputBufferDimensions:)]
    pub fn set_automatically_configures_output_buf_dimensions(&mut self, value: bool);

    #[objc::msg_send(deliversPreviewSizedOutputBuffers)]
    pub fn delivers_preview_sized_output_bufs(&self) -> bool;

    #[objc::msg_send(setDeliversPreviewSizedOutputBuffers:)]
    pub fn set_delivers_preview_sized_output_bufs(&mut self, value: bool);

    /// Indicates the supported video pixel formats that can be specified in videoSettings.
    ///
    /// The value of this property is an NSArray of NSNumbers that can be used as values
    /// for the kCVPixelBufferPixelFormatTypeKey in the receiver's videoSettings property.
    /// The formats are listed in an unspecified order. This list can may change if the
    /// activeFormat of the AVCaptureDevice connected to the receiver changes.
    #[objc::msg_send(availableVideoCVPixelFormatTypes)]
    pub fn available_video_cv_pixel_formats(&self) -> arc::R<ns::Array<ns::Number>>;

    /// Indicates the supported video codec formats that can be specified in
    /// setOutputSettingsForConnection:.
    ///
    /// The value of this property is an NSArray of AVVideoCodecTypes that can be
    /// used as values for the AVVideoCodecKey in the receiver's
    /// setOutputSettingsForConnection: dictionary. The array of available video codecs
    /// may change depending on the current session preset. The first codec in the array
    /// is used by default when recording a file.
    #[objc::msg_send(availableVideoCodecTypes)]
    pub fn available_video_codecs(&self) -> arc::R<ns::Array<av::VideoCodec>>;

    /// The dispatch queue on which all sample buffer delegate methods will be called.
    #[cfg(feature = "dispatch")]
    #[objc::msg_send(sampleBufferCallbackQueue)]
    pub fn sample_buf_callback_queue(&self) -> Option<arc::R<dispatch::Queue>>;

    #[objc::msg_send(videoSettings)]
    pub fn video_settings(&self) -> arc::R<ns::Dictionary<ns::String, ns::Id>>;

    #[objc::msg_send(setVideoSettings:)]
    pub unsafe fn set_video_settings_throws(
        &mut self,
        val: Option<&ns::Dictionary<ns::String, ns::Id>>,
    );

    pub fn set_video_settings<'ear>(
        &mut self,
        val: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.set_video_settings_throws(val)) }
    }

    #[objc::msg_send(recommendedVideoSettingsForAssetWriterWithOutputFileType:)]
    pub fn recommended_video_settings_for_asset_writer_with_output_file_type(
        &self,
        output_file_type: &av::FileType,
    ) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(recommendedVideoSettingsForVideoCodecType:assetWriterOutputFileType:)]
    pub fn recommended_video_settings_for_video_codec_asset_writer_output_file_type(
        &self,
        codec_type: &av::VideoCodec,
        output_file_type: &av::FileType,
    ) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_CAPTURE_VIDEO_DATA_OUTPUT: &'static objc::Class<VideoDataOutput>;
}
