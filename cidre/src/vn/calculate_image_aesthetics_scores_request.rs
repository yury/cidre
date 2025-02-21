use crate::{api, arc, define_obj_type, ns, objc, vn};

impl arc::A<CalcImageAestheticsScoresRequest> {
    #[objc::msg_send(initWithCompletionHandler:)]
    pub fn init_with_ch(
        self,
        ch: Option<&mut vn::RequestCh<CalcImageAestheticsScoresRequest>>,
    ) -> arc::R<CalcImageAestheticsScoresRequest>;
}

define_obj_type!(
    #[doc(alias = "VNCalculateImageAestheticsScoresRequest")]
    pub CalcImageAestheticsScoresRequest(vn::ImageBasedRequest),
    VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST,
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0)]
);

impl CalcImageAestheticsScoresRequest {
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_ch(ch: &mut vn::RequestCh<Self>) -> arc::R<Self> {
        Self::alloc().init_with_ch(Some(ch))
    }

    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with(f: impl FnMut(&mut Self, Option<&ns::Error>) + 'static) -> arc::R<Self> {
        // this scope is marker for api::available to make result optional and skip body
        {
            let mut block = vn::RequestCh::<Self>::new2(f);
            Self::with_ch(&mut block)
        }
    }

    #[doc(alias = "VNCalculateImageAestheticsScoresRequestRevision1")]
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::ImageAestheticsScoresObservation>>>;
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST:
        &'static objc::Class<CalcImageAestheticsScoresRequest>;
}
