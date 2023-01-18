use crate::{arc, cf, define_obj_type, msg_send, vn};

define_obj_type!(GenerateObjectnessBasedSaliencyImageRequest(
    vn::ImageBasedRequest
));

impl GenerateObjectnessBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::SaliencyImageObservation>> {
        msg_send!("vn", self, sel_results)
    }

    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { VNGenerateObjectnessBasedSaliencyImageRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn VNGenerateObjectnessBasedSaliencyImageRequest_new(
    ) -> arc::R<GenerateObjectnessBasedSaliencyImageRequest>;
}
