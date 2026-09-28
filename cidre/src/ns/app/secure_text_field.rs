use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSSecureTextField")]
    pub SecureTextField(ns::TextField),
    sym NSSecureTextField
);
