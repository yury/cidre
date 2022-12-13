use crate::{cf, define_obj_type, msg_send, vn};

define_obj_type!(ClassifyImageRequest(vn::ImageBasedRequest));

impl ClassifyImageRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::ClassificationObservation>> {
        msg_send!("vn", self, sel_results)
    }

    pub fn supported_identifiers<'a, 'ar>(
        &'a self,
    ) -> Result<&'a cf::ArrayOf<cf::String>, &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = self.supported_identifiers_error(&mut error);
            if res.is_some() {
                Ok(res.unwrap_unchecked())
            } else {
                Err(error.unwrap())
            }
        }
    }

    pub unsafe fn supported_identifiers_error<'ar>(
        &self,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<&'ar cf::ArrayOf<cf::String>> {
        msg_send!("vn", self, sel_supportedIdentifiersAndReturnError, error)
    }

    pub fn new() -> cf::Retained<Self> {
        unsafe { VNClassifyImageRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
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

        assert!(request.results().is_none());

        request.set_revision(10);

        let error = request
            .supported_identifiers()
            .expect_err("should be error");

        error.show();
    }
}
