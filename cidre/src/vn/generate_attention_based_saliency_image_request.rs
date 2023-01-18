use crate::{arc, cf, define_obj_type, msg_send, vn};

define_obj_type!(GenerateAttentionBasedSaliencyImageRequest(
    vn::ImageBasedRequest
));

impl GenerateAttentionBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::SaliencyImageObservation>> {
        msg_send!("vn", self, sel_results)
    }

    pub fn new() -> arc::R<Self> {
        unsafe { VNGenerateAttentionBasedSaliencyImageRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn VNGenerateAttentionBasedSaliencyImageRequest_new(
    ) -> arc::R<GenerateAttentionBasedSaliencyImageRequest>;
}
