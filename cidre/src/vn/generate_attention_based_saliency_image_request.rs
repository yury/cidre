use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub GenAttentionBasedSaliencyImageRequest(vn::ImageBasedRequest),
    VN_GENERATE_ATTENTION_BAED_SALIENCY_IMAGE_REQUEST
);

impl GenAttentionBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::SaliencyImageObservation>>>;
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_GENERATE_ATTENTION_BAED_SALIENCY_IMAGE_REQUEST:
        &'static objc::Class<GenAttentionBasedSaliencyImageRequest>;
}
