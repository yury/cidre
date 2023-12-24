use crate::{arc, cf, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSAttributedString")]
    pub AttrString(ns::Id),
    NS_ATTRIBUTED_STRING
);

impl arc::A<AttrString> {
    #[objc::msg_send(initWithString:)]
    pub fn init_with_string(self, str: &ns::String) -> arc::R<AttrString>;
}

impl AttrString {
    /// For performance reasons, this property returns the current backing store of
    /// the attributed string object. If you want to maintain a snapshot of this as
    /// you manipulate the returned string, you should make a copy of the appropriate substring.
    #[objc::msg_send(string)]
    pub fn string(&self) -> &ns::String;

    #[inline]
    pub fn with_string(str: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(str)
    }

    pub fn as_cf(&self) -> &cf::AttrString {
        unsafe { std::mem::transmute(self) }
    }
}

define_obj_type!(
    #[doc(alias = "NSMutableAttributedString")]
    pub AttrStringMut(AttrString)
);

extern "C" {
    static NS_ATTRIBUTED_STRING: &'static objc::Class<AttrString>;
}
