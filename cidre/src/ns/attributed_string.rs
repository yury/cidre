use crate::{arc, cf, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSAttributedStringKey")]
    pub AttrStringKey(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSAttributedStringFormattingContextKey")]
    pub AttrStringFormatCtxKey(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSAttributedString")]
    pub AttrString(ns::Id),
    NS_ATTRIBUTED_STRING
);

impl arc::A<AttrString> {
    #[objc::msg_send(initWithString:)]
    pub fn init_with_string(self, str: &ns::String) -> arc::R<AttrString>;

    #[objc::msg_send(initWithAttributedString:)]
    pub fn init_with_attr_string(self, str: &ns::AttrString) -> arc::R<AttrString>;
}

impl AttrString {
    /// For performance reasons, this property returns the current backing store of
    /// the attributed string object. If you want to maintain a snapshot of this as
    /// you manipulate the returned string, you should make a copy of the appropriate substring.
    #[objc::msg_send(string)]
    pub fn string(&self) -> &ns::String;

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(attributesAtIndex:effectiveRange:)]
    pub unsafe fn attrs_at_throws_ar(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> arc::Rar<ns::Dictionary<AttrStringKey, ns::Id>>;

    #[objc::rar_retain]
    pub unsafe fn attrs_at_throws(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> arc::R<ns::Dictionary<AttrStringKey, ns::Id>>;

    pub fn attrs_at<'ar>(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> Result<arc::R<ns::Dictionary<AttrStringKey, ns::Id>>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.attrs_at_throws(index, effective_range) })
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(str)
    }

    #[inline]
    pub fn with_attr_string(str: &ns::AttrString) -> arc::R<Self> {
        Self::alloc().init_with_attr_string(str)
    }

    #[inline]
    pub fn as_cf(&self) -> &cf::AttrString {
        unsafe { std::mem::transmute(self) }
    }

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<AttrString>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::R<AttrStringMut>;
}

define_obj_type!(
    #[doc(alias = "NSMutableAttributedString")]
    pub AttrStringMut(AttrString),
    NS_MUTABLE_ATTRIBUTED_STRING
);

extern "C" {
    static NS_ATTRIBUTED_STRING: &'static objc::Class<AttrString>;
    static NS_MUTABLE_ATTRIBUTED_STRING: &'static objc::Class<AttrStringMut>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let str = ns::String::with_str("Hello");
        let astr = ns::AttrString::with_string(&str);

        assert_eq!(astr.string(), &str);
        let attrs = astr.attrs_at(0, None).unwrap();
        assert!(attrs.is_empty());

        let copy = astr.copy_mut();
        let copy = copy.copy();
        let copy = ns::AttrString::with_attr_string(&copy);

        assert_eq!(copy.len(), 5);

        // TODO: investigate
        // astr.attrs_at(1000, Some(&ns::Range::new(1000, 1000)))
        // .expect_err("Should be Exception");
    }
}
