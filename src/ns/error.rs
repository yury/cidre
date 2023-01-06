use crate::{define_obj_type, msg_send, ns};

define_obj_type!(Error(ns::Id));

impl Error {
    pub fn code(&self) -> ns::Integer {
        msg_send!("ns", self, ns_code)
    }
    pub fn domain(&self) -> &ns::ErrorDomain {
        msg_send!("ns", self, ns_domain)
    }
}

define_obj_type!(Domain(ns::String));
