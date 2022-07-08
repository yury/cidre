use crate::{av, cf, define_obj_type, dispatch, ns};

use super::Output;

define_obj_type!(VideoDataOutput(Output));

impl VideoDataOutput {
    pub fn new() -> cf::Retained<'static, Self> {
        unsafe { AVCaptureVideoDataOutput_new() }
    }

    pub fn always_discard_late_video_frames(&self) -> bool {
        unsafe { rsel_alwaysDiscardsLateVideoFrames(self) }
    }

    pub fn set_always_discard_late_video_frames(&mut self, value: bool) {
        unsafe { wsel_setAlwaysDiscardsLateVideoFrames(self, value) }
    }

    /// Indicates the supported video pixel formats that can be specified in videoSettings.
    ///
    /// The value of this property is an NSArray of NSNumbers that can be used as values
    /// for the kCVPixelBufferPixelFormatTypeKey in the receiver's videoSettings property.
    /// The formats are listed in an unspecified order. This list can may change if the
    /// activeFormat of the AVCaptureDevice connected to the receiver changes.
    pub fn available_video_cv_pixel_format_types(&self) -> &cf::ArrayOf<cf::Number> {
        unsafe { rsel_availableVideoCVPixelFormatTypes(self) }
    }

    /// Indicates the supported video codec formats that can be specified in
    /// setOutputSettingsForConnection:.
    ///
    /// The value of this property is an NSArray of AVVideoCodecTypes that can be
    /// used as values for the AVVideoCodecKey in the receiver's
    /// setOutputSettingsForConnection: dictionary. The array of available video codecs
    /// may change depending on the current session preset. The first codec in the array
    /// is used by default when recording a file.
    pub fn available_video_codec_types(&self) -> &cf::ArrayOf<av::VideoCodecType> {
        unsafe { rsel_availableVideoCodecTypes(self) }
    }

    /// The dispatch queue on which all sample buffer delegate methods will be called.
    pub fn sample_buffer_callback_queue(&self) -> Option<&dispatch::Queue> {
        unsafe { rsel_sampleBufferCallbackQueue(self) }
    }

    pub fn video_settings(&self) -> Option<&cf::DictionaryOf<cf::String, ns::Id>> {
        unsafe { rsel_videoSettings(self) }
    }

    pub fn recommended_video_settings_for_asset_writer_with_output_file_type(
        &self,
        output_file_type: &av::FileType,
    ) -> Option<&cf::DictionaryOf<cf::String, ns::Id>> {
        unsafe {
            rsel_recommendedVideoSettingsForAssetWriterWithOutputFileType(self, output_file_type)
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVCaptureVideoDataOutput_new() -> cf::Retained<'static, VideoDataOutput>;

    fn rsel_alwaysDiscardsLateVideoFrames(id: &ns::Id) -> bool;
    fn wsel_setAlwaysDiscardsLateVideoFrames(id: &ns::Id, value: bool);

    fn rsel_availableVideoCVPixelFormatTypes(id: &ns::Id) -> &cf::ArrayOf<cf::Number>;
    fn rsel_availableVideoCodecTypes(id: &ns::Id) -> &cf::ArrayOf<av::VideoCodecType>;
    fn rsel_sampleBufferCallbackQueue(id: &ns::Id) -> Option<&dispatch::Queue>;

    fn rsel_videoSettings(id: &ns::Id) -> Option<&cf::DictionaryOf<cf::String, ns::Id>>;
    fn rsel_recommendedVideoSettingsForAssetWriterWithOutputFileType<'a>(
        id: &'a ns::Id,
        output_file_type: &av::FileType,
    ) -> Option<&'a cf::DictionaryOf<cf::String, ns::Id>>;
}
