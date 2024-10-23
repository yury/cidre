use crate::{arc, cf, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSNull")]
    pub Null(ns::Id)
);

unsafe impl Send for Null {}

impl Null {
    define_cls!(NS_NULL);

    /// The singleton instance of [`ns::Null`]. Equal to [`cf::Null::value()`].
    ///
    /// use cf::Null::value if possible.
    #[objc::msg_send(null)]
    pub fn value() -> arc::R<Self>;

    #[inline]
    pub fn as_cf(&self) -> &cf::Null {
        unsafe { std::mem::transmute(self) }
    }
}

impl Default for arc::R<Null> {
    fn default() -> Self {
        Null::value()
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_NULL: &'static objc::Class<Null>;
}
