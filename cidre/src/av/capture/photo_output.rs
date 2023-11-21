use crate::{arc, av, define_obj_type, objc};

define_obj_type!(
    #[doc(alias = "AVCapturePhotoOutput")]
    pub PhotoOutput(av::CaptureOutput),
    AV_CAPTURE_PHOTO_OUTPUT
);

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_PHOTO_OUTPUT: &'static objc::Class<PhotoOutput>;
}
