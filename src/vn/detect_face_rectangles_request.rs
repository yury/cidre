use std::mem::transmute;

use crate::{cf, define_obj_type, ns, vn};

define_obj_type!(DetectFaceRectanglesRequest(vn::ImageBasedRequest));

impl DetectFaceRectanglesRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;
    pub const REVISION_3: usize = 3;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::FaceObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    #[inline]
    pub fn new() -> cf::Retained<Self> {
        unsafe { VNDetectFaceRectanglesRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &ns::Id) -> Option<&cf::ArrayOf<vn::Observation>>;

    fn VNDetectFaceRectanglesRequest_new() -> cf::Retained<DetectFaceRectanglesRequest>;
}
