use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    GenerateImageFeaturePrintRequest(vn::ImageBasedRequest),
    VN_GENERATE_IMAGE_FEAUTRE_PRINT_REQUEST
);

impl GenerateImageFeaturePrintRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::FeaturePrintObservation>>;

    #[objc::msg_send(imageCropAndScaleOption)]
    pub fn image_crop_and_scale_option(&self) -> vn::ImageCropAndScaleOption;

    #[objc::msg_send(setImageCropAndScaleOption:)]
    pub fn set_image_crop_and_scale_option(&mut self, value: vn::ImageCropAndScaleOption);
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_GENERATE_IMAGE_FEAUTRE_PRINT_REQUEST:
        &'static objc::Class<GenerateImageFeaturePrintRequest>;
}
