use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    RecognizeAnimalsRequest(vn::ImageBasedRequest),
    VN_RECOGNIZE_ANIMALS_REQUEST
);

define_obj_type!(AnimalIdentifier(ns::String));

impl AnimalIdentifier {
    #[inline]
    pub fn dog() -> &'static Self {
        unsafe { VNAnimalIdentifierDog }
    }

    #[inline]
    pub fn cat() -> &'static Self {
        unsafe { VNAnimalIdentifierCat }
    }
}

impl RecognizeAnimalsRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::RecognizedObjectObservation>>;

    pub fn supported_identifiers(&self) -> Result<arc::R<ns::Array<AnimalIdentifier>>, &ns::Error> {
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

    /// # Safety
    /// use `supported_identifiers()`
    #[objc::msg_send(supportedIdentifiersAndReturnError:)]
    pub unsafe fn supported_identifiers_error_ar(
        &self,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::Rar<ns::Array<AnimalIdentifier>>>;

    #[objc::rar_retain()]
    pub unsafe fn supported_identifiers_error(
        &self,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<ns::Array<AnimalIdentifier>>>;
}

#[link(name = "Vision", kind = "framework")]
extern "C" {
    static VNAnimalIdentifierDog: &'static AnimalIdentifier;
    static VNAnimalIdentifierCat: &'static AnimalIdentifier;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_RECOGNIZE_ANIMALS_REQUEST: &'static objc::Class<RecognizeAnimalsRequest>;
}

#[cfg(test)]
mod test {
    use crate::vn;

    #[test]
    fn basics() {
        let mut request = vn::RecognizeAnimalsRequest::new();
        let supported_ids = request.supported_identifiers().unwrap();

        assert_eq!(2, supported_ids.len());
        assert!(supported_ids.contains(vn::AnimalIdentifier::cat()));
        assert!(supported_ids.contains(vn::AnimalIdentifier::dog()));

        assert!(request.results().is_none());

        request.set_revision(10);

        let _error = request
            .supported_identifiers()
            .expect_err("should be error");
    }
}
