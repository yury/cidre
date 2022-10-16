use std::mem::transmute;

use crate::{cf, define_obj_type, objc, vn};

define_obj_type!(GenerateImageFeaturePrintRequest(vn::ImageBasedRequest));

impl GenerateImageFeaturePrintRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::FeaturePrintObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    #[inline]
    pub fn image_crop_and_scale_option(&self) -> vn::ImageCropAndScaleOption {
        unsafe { rsel_imageCropAndScaleOption(self) }
    }

    #[inline]
    pub fn set_image_crop_and_scale_option(&mut self, value: vn::ImageCropAndScaleOption) {
        unsafe { wsel_setImageCropAndScaleOption(self, value) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &objc::Id) -> Option<&cf::Array>;

    fn rsel_imageCropAndScaleOption(id: &objc::Id) -> vn::ImageCropAndScaleOption;
    fn wsel_setImageCropAndScaleOption(id: &mut objc::Id, value: vn::ImageCropAndScaleOption);
}
