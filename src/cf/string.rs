use super::{AllocatorRef, Index, OptionFlags, TypeID, TypeRef};

use crate::define_ref;

define_ref!(TypeRef, StringRef, String);
define_ref!(StringRef, MutableStringRef, MutableString);

impl TypeRef {
    #[inline]
    pub fn show(&self) {
        unsafe { CFShow(*self) }
    }
}

///```
/// use cidre::cf;
///
/// assert_eq!(cf::string_get_type_id(), 7);
///```
pub fn string_get_type_id() -> TypeID {
    unsafe { CFStringGetTypeID() }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct StringEncoding(u32);

impl StringEncoding {
    pub const UTF8: Self = Self(0x08000100);
}

pub type UniChar = u16;

impl StringRef {
    #[inline]
    pub fn get_length(&self) -> Index {
        unsafe { CFStringGetLength(*self) }
    }

    #[inline]
    pub fn create_mutable_copy(
        &self,
        alloc: Option<AllocatorRef>,
        max_length: Index,
    ) -> Option<MutableString> {
        unsafe { CFStringCreateMutableCopy(alloc, max_length, *self) }
    }

    #[inline]
    pub fn get_character_at_index(&self, idx: Index) -> UniChar {
        unsafe { CFStringGetCharacterAtIndex(*self, idx) }
    }

    #[inline]
    pub fn show_str(&self) {
        unsafe { CFShowStr(*self) }
    }

    #[inline]
    pub fn create_copy(&self, alloc: Option<AllocatorRef>) -> Option<String> {
        unsafe { CFStringCreateCopy(alloc, *self) }
    }

    #[inline]
    pub fn has_prefix(&self, prefix: StringRef) -> bool {
        unsafe { CFStringHasPrefix(*self, prefix) }
    }
}

impl String {
    ///```
    /// use cidre::cf;
    ///
    /// let s = cf::String::from_static_string("nice").expect("CFString");
    /// assert_eq!(4, s.get_length());
    ///```
    #[inline]
    pub fn from_static_string(string: &'static str) -> Option<String> {
        unsafe {
            CFStringCreateWithBytesNoCopy(
                None,
                string.as_ptr(),
                string.len() as _,
                StringEncoding::UTF8,
                false,
                AllocatorRef::null(),
            )
        }
    }
}

impl MutableStringRef {
    #[inline]
    pub fn append_string(&mut self, appending_string: StringRef) {
        unsafe { CFStringAppend(*self, appending_string) }
    }

    #[inline]
    pub fn trim_whitespace(&mut self) {
        unsafe { CFStringTrimWhitespace(*self) }
    }
}

impl MutableString {
    #[inline]
    pub fn create(alloc: Option<AllocatorRef>, max_length: Index) -> Option<MutableString> {
        unsafe { CFStringCreateMutable(alloc, max_length) }
    }
}

#[repr(transparent)]
pub struct StringCompareFlags(OptionFlags);

impl StringCompareFlags {
    pub const NONE: Self = Self(0);
    pub const CASE_INSENSITIVE: Self = Self(1);
    pub const BACKWARDS: Self = Self(4);
    pub const ANCHORED: Self = Self(8);
    pub const NON_LITERAL: Self = Self(16);
    pub const LOCALIZED: Self = Self(32);
    pub const NUMERACALLY: Self = Self(64);
    pub const DIACRITIC_INSENITIVE: Self = Self(128);
    pub const WIDTH_INSENSITIVE: Self = Self(256);
    pub const FORCED_ORDERING: Self = Self(512);
}

extern "C" {
    fn CFStringGetTypeID() -> TypeID;
    fn CFStringGetLength(theString: StringRef) -> Index;
    fn CFStringCreateMutable(
        alloc: Option<AllocatorRef>,
        max_length: Index,
    ) -> Option<MutableString>;
    fn CFStringCreateCopy(alloc: Option<AllocatorRef>, the_string: StringRef) -> Option<String>;
    fn CFStringHasPrefix(the_string: StringRef, prefix: StringRef) -> bool;
    fn CFStringCreateMutableCopy(
        alloc: Option<AllocatorRef>,
        max_length: Index,
        the_string: StringRef,
    ) -> Option<MutableString>;
    fn CFStringGetCharacterAtIndex(the_string: StringRef, idx: Index) -> UniChar;
    fn CFStringAppend(the_string: MutableStringRef, appended_string: StringRef);
    fn CFStringTrimWhitespace(the_string: MutableStringRef);

    fn CFStringCreateWithBytesNoCopy(
        alloc: Option<AllocatorRef>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: StringEncoding,
        is_external_representation: bool,
        contents_deallocator: Option<AllocatorRef>,
    ) -> Option<String>;

    fn CFShow(cf: TypeRef);
    fn CFShowStr(str: StringRef);
}
