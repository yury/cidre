use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub DetectDocumentSegmentationRequest(vn::ImageBasedRequest),
    VN_DETECT_DOCUMENT_SEGMENTATION_REQUEST
);

impl DetectDocumentSegmentationRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::RectangleObservation>>>;
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_DETECT_DOCUMENT_SEGMENTATION_REQUEST:
        &'static objc::Class<DetectDocumentSegmentationRequest>;
}
