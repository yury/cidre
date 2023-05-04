use crate::{cf, define_cls, define_obj_type, ns, objc};

define_obj_type!(Null(ns::Id));

impl Null {
    define_cls!(NS_NULL);

    #[objc::cls_msg_send(null)]
    pub fn value() -> &'static Self;

    #[inline]
    pub fn as_cf(&self) -> &cf::Null {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_NULL: &'static objc::Class<Null>;
}
