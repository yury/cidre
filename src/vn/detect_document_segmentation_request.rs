use std::mem::transmute;

use crate::{cf, define_obj_type, objc, vn};

define_obj_type!(DetectDocumentSegmentationRequest(vn::ImageBasedRequest));

impl DetectDocumentSegmentationRequest {
    pub const REVISION_1: usize = 1;

    pub fn results(&self) -> Option<&cf::ArrayOf<vn::RectangleObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    pub fn new() -> cf::Retained<DetectDocumentSegmentationRequest> {
        unsafe { VNDetectDocumentSegmentationRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &objc::Id) -> Option<&cf::Array>;

    fn VNDetectDocumentSegmentationRequest_new() -> cf::Retained<DetectDocumentSegmentationRequest>;
}
