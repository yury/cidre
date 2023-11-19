use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub GenerateAttentionBasedSaliencyImageRequest(vn::ImageBasedRequest),
    VN_GENERATE_ATTENTION_BAED_SALIENCY_IMAGE_REQUEST
);

impl GenerateAttentionBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::SaliencyImageObservation>>;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_GENERATE_ATTENTION_BAED_SALIENCY_IMAGE_REQUEST:
        &'static objc::Class<GenerateAttentionBasedSaliencyImageRequest>;
}
