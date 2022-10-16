use crate::{cf, define_obj_type, objc, vn};

define_obj_type!(GenerateImageFeaturePrintRequest(vn::ImageBasedRequest));

impl GenerateImageFeaturePrintRequest {
    pub const REVISION_1: usize = 1;

    //pub fn results(&self) -> Option<&cf::ArrayOf<vn::Feat
}

extern "C" {
    fn rsel_results(id: &objc::Id) -> Option<&cf::Array>;
}
