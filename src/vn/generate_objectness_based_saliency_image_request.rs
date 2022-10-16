use std::mem::transmute;

use crate::{cf, define_obj_type, objc, vn};

define_obj_type!(GenerateObjectnessBasedSaliencyImageRequest(
    vn::ImageBasedRequest
));

impl GenerateObjectnessBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::SaliencyImageObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    #[inline]
    pub fn new() -> cf::Retained<Self> {
        unsafe { VNGenerateObjectnessBasedSaliencyImageRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &objc::Id) -> Option<&cf::Array>;

    fn VNGenerateObjectnessBasedSaliencyImageRequest_new(
    ) -> cf::Retained<GenerateObjectnessBasedSaliencyImageRequest>;
}
