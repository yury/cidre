use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub DetectFaceCaptureQualityRequest(vn::ImageBasedRequest),
    sym VNDetectFaceCaptureQualityRequest
);

impl DetectFaceCaptureQualityRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::FaceObservation>>>;
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
