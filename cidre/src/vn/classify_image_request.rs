use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    ClassifyImageRequest(vn::ImageBasedRequest),
    VN_CLASSIFY_IMAGE_REQUEST
);

impl ClassifyImageRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::ClassificationObservation>>;

    #[objc::msg_send(supportedIdentifiersAndReturnError:)]
    pub fn supported_identifiers_and_return_err_ar(
        &self,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::Rar<ns::Array<ns::String>>>;

    #[objc::rar_retain()]
    pub fn supported_identifiers_and_return_err(
        &self,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    pub fn supported_identifiers<'ar>(
        &self,
    ) -> Result<arc::R<ns::Array<ns::String>>, &'ar ns::Error> {
        unsafe {
            let mut error = None;
            let res = self.supported_identifiers_and_return_err(&mut error);
            if res.is_some() {
                Ok(res.unwrap_unchecked())
            } else {
                Err(error.unwrap())
            }
        }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_CLASSIFY_IMAGE_REQUEST: &'static objc::Class<ClassifyImageRequest>;
}

#[cfg(test)]
mod test {
    use crate::vn;

    #[test]
    fn basics() {
        let mut request = vn::ClassifyImageRequest::new();
        let supported_ids = request.supported_identifiers().unwrap();

        assert!(!supported_ids.is_empty());
        assert!(request.results().is_none());

        request.set_revision(10);

        request
            .supported_identifiers()
            .expect_err("should be error");
    }
}
