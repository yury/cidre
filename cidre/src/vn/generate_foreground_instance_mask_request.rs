use crate::{api, arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNGenerateForegroundInstanceMaskRequest")]
    pub GenForegroundInstanceMaskRequest(vn::ImageBasedRequest),
    VN_GENERATE_FOREGROUND_INSTANCE_MASK_REQUEST,
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0, visionos = 1.0)]
);

impl GenForegroundInstanceMaskRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::InstanceMaskObservation>>>;
}

extern "C" {
    static VN_GENERATE_FOREGROUND_INSTANCE_MASK_REQUEST:
        &'static objc::Class<GenForegroundInstanceMaskRequest>;
}
