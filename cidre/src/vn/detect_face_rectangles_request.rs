use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNDetectFaceRectanglesRequest")]
    pub DetectFaceRectanglesRequest(vn::ImageBasedRequest),
    VN_DETECT_FACE_RECTANGLES_REQUEST
);

impl DetectFaceRectanglesRequest {
    #[doc(alias = "VNDetectFaceRectanglesRequestRevision1")]
    pub const REVISION_1: usize = 1;

    #[doc(alias = "VNDetectFaceRectanglesRequestRevision2")]
    pub const REVISION_2: usize = 2;

    #[doc(alias = "VNDetectFaceRectanglesRequestRevision3")]
    pub const REVISION_3: usize = 3;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::FaceObservation>>>;
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_DETECT_FACE_RECTANGLES_REQUEST: &'static objc::Class<DetectFaceRectanglesRequest>;
}
