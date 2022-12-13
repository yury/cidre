use crate::{cf, define_obj_type, msg_send, vn};

define_obj_type!(DetectDocumentSegmentationRequest(vn::ImageBasedRequest));

impl DetectDocumentSegmentationRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::RectangleObservation>> {
        msg_send!("vn", self, sel_results)
    }

    pub fn new() -> cf::Retained<DetectDocumentSegmentationRequest> {
        unsafe { VNDetectDocumentSegmentationRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {

    fn VNDetectDocumentSegmentationRequest_new() -> cf::Retained<DetectDocumentSegmentationRequest>;
}
