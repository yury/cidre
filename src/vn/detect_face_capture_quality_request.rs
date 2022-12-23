use crate::{cf, define_obj_type, msg_send, vn};

define_obj_type!(DetectFaceCaptureQualityRequest(vn::ImageBasedRequest));

impl DetectFaceCaptureQualityRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::FaceObservation>> {
        msg_send!("vn", self, sel_results)
    }

    #[inline]
    pub fn new() -> cf::Retained<Self> {
        unsafe { VNDetectFaceCaptureQualityRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn VNDetectFaceCaptureQualityRequest_new() -> cf::Retained<DetectFaceCaptureQualityRequest>;
}
