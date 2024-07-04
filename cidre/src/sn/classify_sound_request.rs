use crate::{arc, cm, define_cls, define_obj_type, ns, objc, sn};

define_obj_type!(
    #[doc(alias = "SNClassifySoundRequest")]
    pub ClassifySoundRequest(sn::Request)
);

impl arc::A<ClassifySoundRequest> {
    #[objc::msg_send(initWithClassifierIdentifier:error:)]
    pub unsafe fn init_with_classifier_id_err<'ear>(
        self,
        id: &sn::Id,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ClassifySoundRequest>>;
}

impl ClassifySoundRequest {
    define_cls!(SN_CLASSIFY_SOUND_REQUEST);

    pub fn with_classifier_id<'ear>(id: &sn::Id) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_classifier_id_err(id, err) })
    }

    pub fn v1<'ear>() -> Result<arc::R<Self>, &'ear ns::Error> {
        Self::with_classifier_id(sn::Id::v1())
    }

    #[objc::msg_send(overlapFactor)]
    pub fn overlap_factor(&self) -> f64;

    #[objc::msg_send(setOverlapFactor:)]
    pub fn set_overlap_factor(&mut self, val: f64);

    #[objc::msg_send(windowDuration)]
    pub fn window_duration(&self) -> cm::Time;

    #[objc::msg_send(setWindowDuration:)]
    pub fn set_window_duration(&self, val: cm::Time);

    #[objc::msg_send(windowDurationConstraint)]
    pub fn window_duration_constraint(&self) -> arc::R<sn::TimeDurationConstraint>;

    #[objc::msg_send(knownClassifications)]
    pub fn known_classifications(&self) -> arc::R<ns::Array<ns::String>>;
}

#[link(name = "sn", kind = "static")]
extern "C" {
    static SN_CLASSIFY_SOUND_REQUEST: &'static objc::Class<ClassifySoundRequest>;
}

#[cfg(test)]
mod tests {
    use crate::sn;

    #[test]
    fn basics() {
        let classify_request = sn::ClassifySoundRequest::with_classifier_id(sn::Id::v1()).unwrap();
        assert!(!classify_request.known_classifications().is_empty());
        // 303 ....
    }
}
