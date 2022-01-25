use super::{AllocatorRef, Index, OptionFlags, TypeID, TypeRef};
use std::ops::{Deref, DerefMut};

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

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct StringRef(TypeRef);

#[repr(transparent)]
pub struct String(StringRef);

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct MutableStringRef(StringRef);

#[repr(transparent)]
pub struct MutableString(MutableStringRef);

impl StringRef {
    #[inline]
    pub fn retained(&self) -> String {
        unsafe { String(self.retain()) }
    }

    #[inline]
    pub unsafe fn retain(&self) -> StringRef {
        StringRef(self.0.retain())
    }

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

impl Drop for String {
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}

impl MutableStringRef {
    #[inline]
    pub fn retained(&self) -> MutableString {
        unsafe { MutableString(self.retain()) }
    }

    #[inline]
    pub unsafe fn retain(&self) -> MutableStringRef {
        MutableStringRef(self.0.retain())
    }

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

impl Deref for StringRef {
    type Target = TypeRef;

    #[inline]
    fn deref(&self) -> &TypeRef {
        &self.0
    }
}

impl DerefMut for StringRef {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for String {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for MutableStringRef {
    type Target = StringRef;

    #[inline]
    fn deref(&self) -> &StringRef {
        &self.0
    }
}

impl Deref for MutableString {
    type Target = MutableStringRef;

    #[inline]
    fn deref(&self) -> &MutableStringRef {
        &self.0
    }
}

impl DerefMut for MutableString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for MutableString {
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}

impl DerefMut for MutableStringRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for String {
    type Target = StringRef;

    #[inline]
    fn deref(&self) -> &StringRef {
        &self.0
    }
}

impl TypeRef {
    #[inline]
    pub fn show(&self) {
        unsafe { CFShow(*self) }
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
    fn CFStringCreateCopy(alloc: Option<AllocatorRef>, theString: StringRef) -> Option<String>;
    fn CFStringHasPrefix(the_string: StringRef, prefix: StringRef) -> bool;
    fn CFStringCreateMutableCopy(
        alloc: Option<AllocatorRef>,
        max_length: Index,
        the_string: StringRef,
    ) -> Option<MutableString>;
    fn CFStringGetCharacterAtIndex(theString: StringRef, idx: Index) -> UniChar;
    fn CFStringAppend(theString: MutableStringRef, appended_string: StringRef);
    fn CFStringTrimWhitespace(theString: MutableStringRef);

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
