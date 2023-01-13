use crate::{
    define_obj_type, ns,
    objc::{msg_send, Obj},
};

define_obj_type!(Error(ns::Id));

impl Error {
    #[inline]
    pub fn code(&self) -> ns::Integer {
        unsafe { self.call0(msg_send::code) }
    }

    #[inline]
    pub fn domain(&self) -> &ns::ErrorDomain {
        unsafe { self.call0(msg_send::domain) }
    }
}

define_obj_type!(Domain(ns::String));
