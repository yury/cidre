use std::mem::transmute;

use crate::{cf, define_obj_type, ns, vn};

define_obj_type!(GenerateAttentionBasedSaliencyImageRequest(
    vn::ImageBasedRequest
));

impl GenerateAttentionBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    pub fn results(&self) -> Option<&cf::ArrayOf<vn::SaliencyImageObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    pub fn new() -> cf::Retained<Self> {
        unsafe { VNGenerateAttentionBasedSaliencyImageRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &ns::Id) -> Option<&cf::ArrayOf<vn::Observation>>;

    fn VNGenerateAttentionBasedSaliencyImageRequest_new(
    ) -> cf::Retained<GenerateAttentionBasedSaliencyImageRequest>;
}
