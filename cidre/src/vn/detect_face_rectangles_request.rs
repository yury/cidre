use crate::{arc, define_obj_type, msg_send, ns, objc, vn};

define_obj_type!(
    DetectFaceRectanglesRequest(vn::ImageBasedRequest),
    VN_DETECT_FACE_RECTANGLES_REQUEST
);

impl DetectFaceRectanglesRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;
    pub const REVISION_3: usize = 3;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::FaceObservation>>;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_DETECT_FACE_RECTANGLES_REQUEST: &'static objc::Class<DetectFaceRectanglesRequest>;
}
