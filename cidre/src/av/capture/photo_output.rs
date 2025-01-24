use crate::{api, arc, av, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVCapturePhotoOutput")]
    pub PhotoOutput(av::CaptureOutput),
    AV_CAPTURE_PHOTO_OUTPUT,
    #[api::available(macos = 10.15, ios = 10.0, maccatalyst = 14.0, tvos = 17.0)]
);

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

    #[objc::msg_send(processedFileType)]
    pub fn processed_file_type(&self) -> Option<arc::R<av::FileType>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_PHOTO_OUTPUT: &'static objc::Class<PhotoOutput>;
    static AV_CAPTURE_PHOTO_SETTINGS: &'static objc::Class<PhotoSettings>;
}

#[cfg(test)]
mod tests {
    use crate::av::{self, capture};
    #[test]
    fn basics() {
        let a = capture::PhotoSettings::new();
        let b = capture::PhotoSettings::new();

        assert_ne!(a.unique_id(), b.unique_id());

        a.format().unwrap();

        let file_type = a.processed_file_type().unwrap();
        assert_eq!(&file_type, av::FileType::jpeg());
    }
}
