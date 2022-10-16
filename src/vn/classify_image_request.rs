use std::mem::transmute;

use crate::{cf, define_obj_type, objc, vn};

define_obj_type!(ClassifyImageRequest(vn::ImageBasedRequest));

impl ClassifyImageRequest {
    pub const REVISION_1: usize = 1;

    pub fn results(&self) -> Option<&cf::ArrayOf<vn::ClassificationObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    pub fn supported_identifiers<'a, 'ar>(
        &'a self,
    ) -> Result<&'a cf::ArrayOf<cf::String>, &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = rsel_supportedIdentifiersAndReturnError(self, &mut error);
            if res.is_some() {
                Ok(transmute(res))
            } else {
                Err(error.unwrap())
            }
        }
    }

    pub fn new() -> cf::Retained<Self> {
        unsafe { VNClassifyImageRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &objc::Id) -> Option<&cf::Array>;

    fn rsel_supportedIdentifiersAndReturnError<'a, 'ar>(
        id: &'a objc::Id,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<&'a cf::ArrayOf<cf::String>>;

    fn VNClassifyImageRequest_new() -> cf::Retained<ClassifyImageRequest>;
}

#[cfg(test)]
mod test {
    use crate::vn;

    #[test]
    fn basics() {
        let mut request = vn::ClassifyImageRequest::new();
        let supported_ids = request.supported_identifiers().unwrap();
        supported_ids.show();

        request.set_revision(10);

        let error = request
            .supported_identifiers()
            .expect_err("should be error");

        error.show();
    }
}