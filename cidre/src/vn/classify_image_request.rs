use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNClassifyImageRequest")]
    pub ClassifyImageRequest(vn::ImageBasedRequest),
    VN_CLASSIFY_IMAGE_REQUEST
);

impl ClassifyImageRequest {
    /// Classification with a taxonomy of 1,303 possible identifiers.
    #[doc(alias = "VNClassifyImageRequestRevision1")]
    pub const REVISION_1: usize = 1;

    /// The same taxonomy as `vn::ClassifyImageRequest::REVISION_1` but with improved accuracy, reduced latency and memory utilization.
    #[doc(alias = "VNClassifyImageRequestRevision2")]
    pub const REVISION_2: usize = 2;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::ClassificationObservation>>>;

    #[objc::msg_send(supportedIdentifiersAndReturnError:)]
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

            assert_eq!(vn::ClassifyImageRequest::REVISION_2, request.revision());
            let supported_ids = request.supported_ids().unwrap();

            assert!(!supported_ids.is_empty());
            assert!(request.results().is_none());

            request.set_revision(10);

            let _err = request.supported_ids().expect_err("should be error");
        })
    }
}
