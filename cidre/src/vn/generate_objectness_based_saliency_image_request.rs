use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub GenObjectnessBasedSaliencyImageRequest(vn::ImageBasedRequest),
    VN_GENERATE_OBJECTNESS_BASED_SALIENCY_IMAGE_REQUEST
);

impl GenObjectnessBasedSaliencyImageRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::SaliencyImageObservation>>>;
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_GENERATE_OBJECTNESS_BASED_SALIENCY_IMAGE_REQUEST:
        &'static objc::Class<GenObjectnessBasedSaliencyImageRequest>;
}
