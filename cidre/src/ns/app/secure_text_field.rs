use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSecureTextField")]
    pub SecureTextField(ns::TextField),
    NS_SECURE_TEXT_FIELD
);

unsafe extern "C" {
    static NS_SECURE_TEXT_FIELD: &'static objc::Class<SecureTextField>;
}
