use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    DetectFaceCaptureQualityRequest(vn::ImageBasedRequest),
    VN_DETECT_FACE_CAPTURE_QUALITY_REQUEST
);

impl DetectFaceCaptureQualityRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::FaceObservation>>;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_DETECT_FACE_CAPTURE_QUALITY_REQUEST:
        &'static objc::Class<DetectFaceCaptureQualityRequest>;
}

#[cfg(test)]
mod tests {
    use crate::vn;
    #[test]
    fn basics() {
        let mut request = vn::DetectFaceCaptureQualityRequest::new();
        request.set_revision(vn::DetectFaceCaptureQualityRequest::REVISION_2);
    }
}
