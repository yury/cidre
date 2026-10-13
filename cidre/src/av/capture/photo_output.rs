use crate::{api, arc, av, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;
#[cfg(feature = "cv")]
use crate::cv;

/// Constants indicating how photo quality should be prioritized against speed.
#[doc(alias = "AVCapturePhotoQualityPrioritization")]
#[repr(isize)]
pub enum PhotoQualityPrioritization {
    /// Indicates that speed of photo delivery is most important, even at the expense of quality.
    #[doc(alias = "AVCapturePhotoQualityPrioritizationSpeed")]
    Speed = 1,

    /// Indicates that photo quality and speed of delivery are balanced in priority.
    #[doc(alias = "AVCapturePhotoQualityPrioritizationBalanced")]
    Balanced = 2,

    /// Indicates that photo quality is paramount, even at the expense of shot-to-shot time.
    #[doc(alias = "AVCapturePhotoQualityPrioritizationQuality")]
    Quality = 3,
}

define_obj_type!(
    #[doc(alias = "AVCapturePhotoOutput")]
    pub PhotoOutput(av::CaptureOutput),
    AV_CAPTURE_PHOTO_OUTPUT,
    #[api::available(macos = 10.15, ios = 10.0, maccatalyst = 14.0, tvos = 17.0)]
);

impl PhotoOutput {
    /// Indicates the highest quality the receiver should be prepared to output on a capture-by-capture basis.
    #[objc::msg_send(maxPhotoQualityPrioritization)]
    pub fn max_photo_quality_prioritization(&self) -> PhotoQualityPrioritization;

    #[objc::msg_send(setMaxPhotoQualityPrioritization:)]
    pub fn set_max_photo_quality_prioritization(&mut self, val: PhotoQualityPrioritization);

    #[objc::msg_send(isFastCapturePrioritizationSupported)]
    pub fn is_fast_capture_prioritization_supported(&self) -> bool;

    /// The largest photos the output is prepared to deliver: one of the active format's
    /// `supported_max_photo_dims`. Set before the session runs: changing it reconfigures the
    /// capture pipeline.
    #[cfg(feature = "cm")]
    #[objc::msg_send(maxPhotoDimensions)]
    #[api::available(ios = 16.0, macos = 13.0, maccatalyst = 16.0, tvos = 17.0)]
    pub fn max_photo_dims(&self) -> cm::VideoDimensions;

    #[cfg(feature = "cm")]
    #[objc::msg_send(setMaxPhotoDimensions:)]
    #[api::available(ios = 16.0, macos = 13.0, maccatalyst = 16.0, tvos = 17.0)]
    pub fn set_max_photo_dims(&mut self, val: cm::VideoDimensions);

    #[objc::msg_send(capturePhotoWithSettings:delegate:)]
    pub unsafe fn capture_photo_throws<D: PhotoCaptureDelegate>(
        &mut self,
        settings: &PhotoSettings,
        delegate: &D,
    );

    /// Takes a photo as `settings` say; `delegate` hears its progress and gets the photo (on a
    /// queue of the output's). Settings are used once: a new instance for each photo. Throws
    /// for settings the output cannot satisfy, or a delegate without the callbacks they need.
    pub fn capture_photo<'ear, D: PhotoCaptureDelegate>(
        &mut self,
        settings: &PhotoSettings,
        delegate: &D,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.capture_photo_throws(settings, delegate) })
    }
}

/// Hears the progress of a photo taken with [`PhotoOutput::capture_photo`], on a queue of the
/// output's: `will_begin_capture` first, `did_finish_capture` last.
#[objc::protocol(AVCapturePhotoCaptureDelegate)]
pub trait PhotoCaptureDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(captureOutput:willBeginCaptureForResolvedSettings:)]
    fn capture_output_will_begin_capture(
        &mut self,
        output: &PhotoOutput,
        resolved_settings: &ResolvedPhotoSettings,
    );

    #[objc::optional]
    #[objc::msg_send(captureOutput:willCapturePhotoForResolvedSettings:)]
    fn capture_output_will_capture_photo(
        &mut self,
        output: &PhotoOutput,
        resolved_settings: &ResolvedPhotoSettings,
    );

    #[objc::optional]
    #[objc::msg_send(captureOutput:didCapturePhotoForResolvedSettings:)]
    fn capture_output_did_capture_photo(
        &mut self,
        output: &PhotoOutput,
        resolved_settings: &ResolvedPhotoSettings,
    );

    #[objc::optional]
    #[objc::msg_send(captureOutput:didFinishProcessingPhoto:error:)]
    fn capture_output_did_finish_processing_photo(
        &mut self,
        output: &PhotoOutput,
        photo: &Photo,
        error: Option<&ns::Error>,
    );

    #[objc::optional]
    #[objc::msg_send(captureOutput:didFinishCaptureForResolvedSettings:error:)]
    fn capture_output_did_finish_capture(
        &mut self,
        output: &PhotoOutput,
        resolved_settings: &ResolvedPhotoSettings,
        error: Option<&ns::Error>,
    );
}

define_obj_type!(
    /// The settings a photo was actually taken with, matched to its [`PhotoSettings`] by
    /// `unique_id`.
    #[doc(alias = "AVCaptureResolvedPhotoSettings")]
    pub ResolvedPhotoSettings(ns::Id)
);

impl ResolvedPhotoSettings {
    #[objc::msg_send(uniqueID)]
    pub fn unique_id(&self) -> i64;

    /// The photo's dimensions.
    #[cfg(feature = "cm")]
    #[objc::msg_send(photoDimensions)]
    pub fn photo_dims(&self) -> cm::VideoDimensions;
}

define_obj_type!(
    /// A photo taken with a [`PhotoOutput`].
    #[doc(alias = "AVCapturePhoto")]
    pub Photo(ns::Id)
);

impl Photo {
    /// When the photo was taken (the device clock's time).
    #[cfg(feature = "cm")]
    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> cm::Time;

    /// The photo's pixels, where an uncompressed format was asked for; `None` for an encoded
    /// one.
    #[cfg(feature = "cv")]
    #[objc::msg_send(pixelBuffer)]
    pub fn pixel_buf(&self) -> Option<&cv::PixelBuf>;

    #[objc::msg_send(resolvedSettings)]
    pub fn resolved_settings(&self) -> arc::R<ResolvedPhotoSettings>;

    /// The photo as a file (HEIC or JPEG, with its metadata) to write or add to the Photos
    /// library.
    #[objc::msg_send(fileDataRepresentation)]
    #[api::available(ios = 11.0, macos = 10.15, maccatalyst = 14.0, tvos = 17.0)]
    pub fn file_data_representation(&self) -> Option<arc::R<ns::Data>>;
}

define_obj_type!(
    #[doc(alias = "AVCapturePhotoSettings")]
    pub PhotoSettings(ns::Id),
    AV_CAPTURE_PHOTO_SETTINGS,
    #[api::available(macos = 10.15, ios = 10.0, maccatalyst = 14.0, tvos = 17.0)]
);

impl PhotoSettings {
    /// Settings for a processed photo in `format` (its codec, for one): `None` for the default.
    #[objc::msg_send(photoSettingsWithFormat:)]
    pub fn with_format(format: Option<&ns::Dictionary<ns::String, ns::Id>>) -> arc::R<Self>;

    /// The largest this photo may be: one of the active format's `supported_max_photo_dims`,
    /// no larger than the output's `max_photo_dims`. The smallest supported by default.
    #[cfg(feature = "cm")]
    #[objc::msg_send(maxPhotoDimensions)]
    #[api::available(ios = 16.0, macos = 13.0, maccatalyst = 16.0, tvos = 17.0)]
    pub fn max_photo_dims(&self) -> cm::VideoDimensions;

    #[cfg(feature = "cm")]
    #[objc::msg_send(setMaxPhotoDimensions:)]
    #[api::available(ios = 16.0, macos = 13.0, maccatalyst = 16.0, tvos = 17.0)]
    pub fn set_max_photo_dims(&mut self, val: cm::VideoDimensions);

    /// A 64-bit number that uniquely identifies this instance.
    ///
    /// When you create an instance of av::capture::PhotoSettings, a unique_id is generated automatically.
    /// This unique_id is guaranteed to be unique for the life time of your process.
    #[objc::msg_send(uniqueID)]
    pub fn unique_id(&self) -> i64;

    /// A dictionary of Core Video pixel buffer attributes or av::VideoSettings, analogous to AVCaptureStillImageOutput's outputSettings property.
    ///
    /// The format dictionary you passed to one of the creation methods. May be None if you've specified RAW-only capture.
    #[objc::msg_send(format)]
    pub fn format(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(rawFileFormat)]
    #[api::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn raw_file_format(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    /// The file container for which the processed photo is formatted to be stored.
    #[objc::msg_send(processedFileType)]
    pub fn processed_file_type(&self) -> Option<arc::R<av::FileType>>;

    #[objc::msg_send(flashMode)]
    pub fn flash_mode(&self) -> av::CaptureFlashMode;

    #[objc::msg_send(setFlashMode:)]
    pub fn set_flash_mode(&mut self, val: av::CaptureFlashMode);

    #[objc::msg_send(isAutoRedEyeReductionEnabled)]
    pub fn is_auto_red_eye_reduction_enabled(&self) -> bool;

    #[objc::msg_send(setAutoRedEyeReductionEnabled:)]
    pub fn set_auto_red_eye_reduction_enabled(&mut self, val: bool);
}

unsafe extern "C" {
    static AV_CAPTURE_PHOTO_OUTPUT: &'static objc::Class<PhotoOutput>;
    static AV_CAPTURE_PHOTO_SETTINGS: &'static objc::Class<PhotoSettings>;
}

#[cfg(test)]
mod tests {
    use crate::av::{self, capture};
    #[test]
    fn basics() {
        let mut a = capture::PhotoSettings::new();
        let b = capture::PhotoSettings::new();

        assert_ne!(a.unique_id(), b.unique_id());

        a.format().unwrap();

        let file_type = a.processed_file_type().unwrap();
        assert_eq!(&file_type, av::FileType::jpeg());

        assert_eq!(a.flash_mode(), av::CaptureFlashMode::Off);

        a.set_flash_mode(av::CaptureFlashMode::Auto);

        assert_eq!(a.flash_mode(), av::CaptureFlashMode::Auto);

        assert_eq!(a.is_auto_red_eye_reduction_enabled(), false);

        let c = capture::PhotoSettings::with_format(None);
        assert_ne!(c.unique_id(), a.unique_id());
        assert_eq!(
            c.processed_file_type().as_deref(),
            Some(av::FileType::jpeg())
        );
    }
}
