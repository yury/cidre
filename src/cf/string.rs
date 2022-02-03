use super::{runtime::Ltb, Allocator, Index, OptionFlags, Retained, Type, TypeID};

use crate::define_cf_type;

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

define_cf_type!(String(Type));

impl String {
    #[inline]
    pub fn show_str(&self) {
        unsafe { CFShowStr(self) }
    }

    #[inline]
    pub fn get_length(&self) -> Index {
        unsafe { CFStringGetLength(self) }
    }

    #[inline]
    pub fn has_suffix(&self, suffix: &String) -> bool {
        unsafe { CFStringHasSuffix(self, suffix) }
    }

    #[inline]
    pub fn has_prefix(&self, prefix: &String) -> bool {
        unsafe { CFStringHasPrefix(self, prefix) }
    }

    #[inline]
    pub fn create_copy(&self, alloc: Option<&Allocator>) -> Option<Retained<String>> {
        unsafe { CFStringCreateCopy(alloc, self) }
    }
}

define_cf_type!(MutableString(String));

impl MutableString {
    #[inline]
    pub fn append(&mut self, appended_string: &String) {
        unsafe { CFStringAppend(self, appended_string) }
    }

    #[inline]
    pub fn create(alloc: Option<&Allocator>, max_length: Index) -> Option<Retained<Self>> {
        unsafe { CFStringCreateMutable(alloc, max_length) }
    }
}

extern "C" {
    fn CFStringGetTypeID() -> TypeID;
    fn CFStringGetLength(the_string: &String) -> Index;
    fn CFStringCreateMutable(
        alloc: Option<&Allocator>,
        max_length: Index,
    ) -> Option<Retained<MutableString>>;
    fn CFStringCreateCopy(
        alloc: Option<&Allocator>,
        the_string: &String,
    ) -> Option<Retained<String>>;
    fn CFStringHasPrefix(the_string: &String, prefix: &String) -> bool;
    fn CFStringHasSuffix(the_string: &String, prefix: &String) -> bool;
    fn CFStringCreateMutableCopy(
        alloc: Option<&Allocator>,
        max_length: Index,
        the_string: &String,
    ) -> Option<Retained<MutableString>>;
    fn CFStringGetCharacterAtIndex(the_string: &String, idx: Index) -> UniChar;
    fn CFStringAppend(the_string: &mut MutableString, appended_string: &String);
    fn CFStringTrimWhitespace(the_string: &mut MutableString);

    fn CFStringCreateWithBytesNoCopy<'a>(
        alloc: Option<&Allocator>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: StringEncoding,
        is_external_representation: bool,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Ltb<'a, Retained<String>>>;

    fn CFShowStr(str: &String);
}
