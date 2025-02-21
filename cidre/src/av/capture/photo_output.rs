use crate::{api, arc, av, define_obj_type, ns, objc};

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
}

define_obj_type!(
    #[doc(alias = "AVCapturePhotoSettings")]
    pub PhotoSettings(ns::Id),
    AV_CAPTURE_PHOTO_SETTINGS,
    #[api::available(macos = 10.15, ios = 10.0, maccatalyst = 14.0, tvos = 17.0)]
);

impl PhotoSettings {
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

#[link(name = "av", kind = "static")]
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
    }
}
