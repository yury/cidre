use crate::{arc, cf, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

define_cf_type!(AttrString(cf::Type));
impl AttrString {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFAttributedStringGetTypeID() }
    }

    #[inline]
    pub fn new_in(str: &cf::String, allocator: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFAttributedStringCreate(allocator, str, None) }
    }

    #[inline]
    pub fn new(str: &cf::String) -> arc::R<Self> {
        unsafe { std::mem::transmute(CFAttributedStringCreate(None, str, None)) }
    }

    #[inline]
    pub fn with_attrs_in(
        str: &cf::String,
        attrs: &cf::Dictionary,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { CFAttributedStringCreate(allocator, str, Some(attrs)) }
    }

    #[inline]
    pub fn with_attrs(str: &cf::String, attrs: &cf::Dictionary) -> arc::R<Self> {
        unsafe { std::mem::transmute(CFAttributedStringCreate(None, str, Some(attrs))) }
    }

    #[inline]
    pub fn string(&self) -> &cf::String {
        unsafe { CFAttributedStringGetString(self) }
    }

    #[inline]
    pub fn length(&self) -> cf::Index {
        unsafe { CFAttributedStringGetLength(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length() as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn copy_in(&self, allocator: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFAttributedStringCreateCopy(allocator, self) }
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { std::mem::transmute(CFAttributedStringCreateCopy(None, self)) }
    }

    #[inline]
    pub fn copy_mut_in(
        &self,
        max_len: usize,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<AttrStringMut>> {
        unsafe { CFAttributedStringCreateMutableCopy(allocator, max_len as _, self) }
    }

    #[inline]
    pub fn copy_mut(&self) -> arc::R<AttrStringMut> {
        unsafe { std::mem::transmute(CFAttributedStringCreateMutableCopy(None, 0, self)) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::AttrString {
        unsafe { std::mem::transmute(self) }
    }
}

define_cf_type!(AttrStringMut(AttrString));
impl AttrStringMut {
    #[inline]
    pub fn with_max_len_in(
        max_len: usize,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<AttrStringMut>> {
        unsafe { CFAttributedStringCreateMutable(allocator, max_len as _) }
    }

    #[inline]
    pub fn with_max_len(max_len: usize) -> arc::R<AttrStringMut> {
        unsafe { std::mem::transmute(CFAttributedStringCreateMutable(None, max_len as _)) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFAttributedStringGetTypeID() -> cf::TypeId;
    fn CFAttributedStringCreate(
        alloc: Option<&cf::Allocator>,
        str: &cf::String,
        attributes: Option<&cf::Dictionary>,
    ) -> Option<arc::R<AttrString>>;

    fn CFAttributedStringGetString(astr: &AttrString) -> &cf::String;
    fn CFAttributedStringGetLength(astr: &AttrString) -> cf::Index;

    fn CFAttributedStringCreateCopy(
        alloc: Option<&cf::Allocator>,
        astr: &AttrString,
    ) -> Option<arc::R<AttrString>>;

    fn CFAttributedStringCreateMutableCopy(
        alloc: Option<&cf::Allocator>,
        max_len: cf::Index,
        astr: &AttrString,
    ) -> Option<arc::R<AttrStringMut>>;

    fn CFAttributedStringCreateMutable(
        alloc: Option<&cf::Allocator>,
        max_len: cf::Index,
    ) -> Option<arc::R<AttrStringMut>>;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn basics() {
        let str = cf::String::from_str("test");
        let astr = cf::AttrString::new(&str);
        assert!(astr.string().equal(&str));

        let attrs = cf::Dictionary::new();
        let astr = cf::AttrString::with_attrs(&str, &attrs);
        assert!(astr.string().equal(&str));

        let copy = astr.copy();
        assert!(copy.string().equal(&str));

        let mcopy = copy.copy_mut();
        assert!(mcopy.string().equal(&str));
    }
}
