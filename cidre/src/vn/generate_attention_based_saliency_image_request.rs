use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub GenAttentionBasedSaliencyImageRequest(vn::ImageBasedRequest),
    sym VNGenerateAttentionBasedSaliencyImageRequest
);

impl GenAttentionBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::SaliencyImageObservation>>>;
}
