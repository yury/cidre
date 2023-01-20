use crate::{define_obj_type, ns, objc};

define_obj_type!(Error(ns::Id));

impl Error {
    #[objc::msg_send2(code)]
    pub fn code(&self) -> ns::Integer;

    #[objc::msg_send2(domain)]
    pub fn domain(&self) -> &ns::ErrorDomain;
}

define_obj_type!(Domain(ns::String));
