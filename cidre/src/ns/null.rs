use crate::{arc, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cf")]
use crate::cf;

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

    #[cfg(feature = "cf")]
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

impl AsRef<cf::Null> for Null {
    fn as_ref(&self) -> &cf::Null {
        self.as_cf()
    }
}

impl AsRef<cf::Type> for Null {
    fn as_ref(&self) -> &cf::Type {
        self.as_cf()
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_NULL: &'static objc::Class<Null>;
}
