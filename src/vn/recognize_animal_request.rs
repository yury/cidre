use crate::{cf, define_cf_type, define_obj_type, msg_send, vn};

define_obj_type!(RecognizeAnimalsRequest(vn::ImageBasedRequest));

define_cf_type!(AnimalIdentifier(cf::String));

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

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::RecognizedObjectObservation>> {
        msg_send!("vn", self, sel_results)
    }

    pub fn supported_identifiers<'a, 'ar>(
        &'a self,
    ) -> Result<&'a cf::ArrayOf<AnimalIdentifier>, &'ar cf::Error> {
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
    pub unsafe fn supported_identifiers_error<'ar>(
        &self,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<&'ar cf::ArrayOf<AnimalIdentifier>> {
        msg_send!("vn", self, sel_supportedIdentifiersAndReturnError, error)
    }

    pub fn new() -> cf::Retained<Self> {
        unsafe { VNRecognizeAnimalsRequest_new() }
    }
}

#[link(name = "Vision", kind = "framework")]
extern "C" {
    static VNAnimalIdentifierDog: &'static AnimalIdentifier;
    static VNAnimalIdentifierCat: &'static AnimalIdentifier;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn VNRecognizeAnimalsRequest_new() -> cf::Retained<RecognizeAnimalsRequest>;
}

#[cfg(test)]
mod test {
    use crate::vn;

    #[test]
    fn basics() {
        let mut request = vn::RecognizeAnimalsRequest::new();
        let supported_ids = request.supported_identifiers().unwrap();
        supported_ids.show();

        assert_eq!(2, supported_ids.len());
        assert!(supported_ids.contains(vn::AnimalIdentifier::cat()));
        assert!(supported_ids.contains(vn::AnimalIdentifier::dog()));

        assert!(request.results().is_none());

        request.set_revision(10);

        let error = request
            .supported_identifiers()
            .expect_err("should be error");

        error.show();
    }
}
