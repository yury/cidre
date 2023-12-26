use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    pub ClassifyImageRequest(vn::ImageBasedRequest),
    VN_CLASSIFY_IMAGE_REQUEST
);

impl ClassifyImageRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::ClassificationObservation>>;

    #[objc::msg_send(supportedIdentifiersAndReturnError:)]
    pub unsafe fn supported_ids_and_return_err_ar<'ear>(
        &self,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Rar<ns::Array<ns::String>>>;

    #[objc::rar_retain()]
    pub unsafe fn supported_ids_and_return_err<'ear>(
        &self,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    pub fn supported_ids<'ear>(&self) -> Result<arc::R<ns::Array<ns::String>>, &'ear ns::Error> {
        unsafe {
            let mut error = None;
            let res = self.supported_ids_and_return_err(&mut error);
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
    use crate::{objc::ar_pool, vn};

    #[test]
    fn basics() {
        ar_pool(|| {
            let mut request = vn::ClassifyImageRequest::new();
            let supported_ids = request.supported_ids().unwrap();

            assert!(!supported_ids.is_empty());
            assert!(request.results().is_none());

            request.set_revision(10);

            let _err = request.supported_ids().expect_err("should be error");
        })
    }
}
