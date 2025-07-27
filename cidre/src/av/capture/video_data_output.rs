use crate::{arc, av, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;

#[cfg(feature = "dispatch")]
use crate::dispatch;

use super::Output;

#[cfg(feature = "cm")]
#[objc::protocol(AVCaptureVideoDataOutputSampleBufferDelegate)]
pub trait VideoDataOutputSampleBufDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(captureOutput:didOutputSampleBuffer:fromConnection:)]
    fn capture_output_did_output_sample_buf_from_connection(
        &mut self,
        output: &av::CaptureOutput,
        sample_buf: &cm::SampleBuf,
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
    #[cfg(feature = "cm")]
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
    pub fn automatically_configures_output_buf_dims(&self) -> bool;

    #[objc::msg_send(setAutomaticallyConfiguresOutputBufferDimensions:)]
    pub fn set_automatically_configures_output_buf_dims(&mut self, value: bool);

    #[objc::msg_send(deliversPreviewSizedOutputBuffers)]
    pub fn delivers_preview_sized_output_bufs(&self) -> bool;

    /// Throws unless automatically_configures_output_buf_dims has been set to false.
    #[objc::msg_send(setDeliversPreviewSizedOutputBuffers:)]
    pub unsafe fn set_delivers_preview_sized_output_bufs_throws(&mut self, value: bool);

    pub fn set_delivers_preview_sized_output_bufs<'ear>(
        &mut self,
        value: bool,
    ) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.set_delivers_preview_sized_output_bufs_throws(value)) }
    }

    /// Indicates whether the receiver should prepare the cellular radio for imminent network activity.
    ///
    /// Apps that scan video data output buffers for information that will result in network activity
    /// (such as detecting a QRCode containing a URL) should set this property true to allow the cellular
    /// radio to prepare for an imminent network request. Enabling this property requires a lengthy reconfiguration
    ///  of the capture render pipeline, so you should set this property to YES before calling -[AVCaptureSession startRunning].
    ///
    /// Using this API requires your app to adopt the entitlement `com.apple.developer.avfoundation.video-data-output-prepares-cellular-radio-for-machine-readable-code-scanning`.
    #[objc::msg_send(preparesCellularRadioForNetworkConnection)]
    #[objc::available(ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn prepares_cellular_radio_for_network_connection(&self) -> bool;

    #[objc::msg_send(setPreparesCellularRadioForNetworkConnection:)]
    #[objc::available(ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn set_prepares_cellular_radio_for_network_connection(&mut self, val: bool);

    #[objc::msg_send(preservesDynamicHDRMetadata)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn preserves_dynamic_hdr_metadata(&self) -> bool;

    #[objc::msg_send(setPreservesDynamicHDRMetadata:)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn set_preserves_dynamic_hdr_metadata(&mut self, val: bool);

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
    pub fn video_settings(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

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

    /// Indicates the recommended media timescale for the video track.
    ///
    /// This will return a recommended media timescale based on the active capture session's inputs.
    /// It will not be less than 600. It may or may not be a multiple of 600.
    #[objc::msg_send(recommendedMediaTimeScaleForAssetWriter)]
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn recommended_media_time_scale_for_asset_writer(&self) -> cm::TimeScale;
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_CAPTURE_VIDEO_DATA_OUTPUT: &'static objc::Class<VideoDataOutput>;
}
