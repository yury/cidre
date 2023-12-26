use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNRecognizeAnimalsRequest")]
    pub RecognizeAnimalsRequest(vn::ImageBasedRequest),
    VN_RECOGNIZE_ANIMALS_REQUEST
);

define_obj_type!(pub AnimalId(ns::String));

impl AnimalId {
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

    pub fn supported_ids(&self) -> Result<arc::R<ns::Array<AnimalId>>, &ns::Error> {
        unsafe {
            let mut err = None;
            let res = self.supported_ids_err(&mut err);
            if res.is_some() {
                println!("some");
                Ok(res.unwrap_unchecked())
            } else {
                println!("none");
                Err(err.unwrap())
            }
        }
    }

    /// # Safety
    /// use `supported_ids()`
    #[objc::msg_send(supportedIdentifiersAndReturnError:)]
    pub unsafe fn supported_ids_err_ar<'ear>(
        &self,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Rar<ns::Array<AnimalId>>>;

    #[objc::rar_retain]
    pub unsafe fn supported_ids_err<'ear>(
        &self,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<AnimalId>>>;
}

#[link(name = "Vision", kind = "framework")]
extern "C" {
    static VNAnimalIdentifierDog: &'static AnimalId;
    static VNAnimalIdentifierCat: &'static AnimalId;
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
        println!("1");
        let supported_ids = request.supported_ids().unwrap();
        println!("2");

        assert_eq!(2, supported_ids.len());
        println!("3");
        assert!(supported_ids.contains(vn::AnimalId::cat()));
        println!("4");
        assert!(supported_ids.contains(vn::AnimalId::dog()));

        println!("5");
        assert!(request.results().is_none());
        println!("6");

        request.set_revision(10);
        println!("7");

        let _error = request.supported_ids().expect_err("should be error");
        println!("8");
    }
}
