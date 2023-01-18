use crate::{arc, cf, define_obj_type, msg_send, ns, vn};

define_obj_type!(GenerateImageFeaturePrintRequest(vn::ImageBasedRequest));

impl GenerateImageFeaturePrintRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::FeaturePrintObservation>> {
        msg_send!("vn", self, sel_results)
    }

    #[inline]
    pub fn image_crop_and_scale_option(&self) -> vn::ImageCropAndScaleOption {
        unsafe { rsel_imageCropAndScaleOption(self) }
    }

    #[inline]
    pub fn set_image_crop_and_scale_option(&mut self, value: vn::ImageCropAndScaleOption) {
        unsafe { wsel_setImageCropAndScaleOption(self, value) }
    }

    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { VNGenerateImageFeaturePrintRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_imageCropAndScaleOption(id: &ns::Id) -> vn::ImageCropAndScaleOption;
    fn wsel_setImageCropAndScaleOption(id: &mut ns::Id, value: vn::ImageCropAndScaleOption);
    fn VNGenerateImageFeaturePrintRequest_new() -> arc::R<GenerateImageFeaturePrintRequest>;
}
