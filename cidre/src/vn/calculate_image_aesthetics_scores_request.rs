use crate::{api, arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNCalculateImageAestheticsScoresRequest")]
    pub CalcImageAestheticsScoresRequest(vn::ImageBasedRequest),
    VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST,
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0)]
);

impl CalcImageAestheticsScoresRequest {
    #[doc(alias = "VNCalculateImageAestheticsScoresRequestRevision1")]
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::ImageAestheticsScoresObservation>>>;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST:
        &'static objc::Class<CalcImageAestheticsScoresRequest>;
}
