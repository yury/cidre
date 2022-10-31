use core::fmt;
use std::{borrow::Cow, ffi::CStr, os::raw::c_char, str::from_utf8_unchecked};

use super::{Allocator, Index, OptionFlags, Range, Retained, Type, TypeId};

use crate::{define_cf_type, define_options, UniChar};

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Encoding(u32);

impl Encoding {
    pub const ASCII: Self = Self(0x0600);
    pub const UTF8: Self = Self(0x08000100);
}

define_options!(CompareFlags(OptionFlags));

impl CompareFlags {
    pub const NONE: Self = Self(OptionFlags(0));

    pub const CASE_INSENSITIVE: Self = Self(OptionFlags(1));
    pub const BACKWARDS: Self = Self(OptionFlags(4));
    pub const ANCHORED: Self = Self(OptionFlags(8));
    pub const NON_LITERAL: Self = Self(OptionFlags(16));
    pub const LOCALIZED: Self = Self(OptionFlags(32));
    pub const NUMERACALLY: Self = Self(OptionFlags(64));
    pub const DIACRITIC_INSENITIVE: Self = Self(OptionFlags(128));
    pub const WIDTH_INSENSITIVE: Self = Self(OptionFlags(256));
    pub const FORCED_ORDERING: Self = Self(OptionFlags(512));
}

define_cf_type!(String(Type));

impl String {
    ///```
    /// use cidre::cf;
    ///
    /// assert_eq!(cf::String::type_id(), 7);
    ///```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFStringGetTypeID() }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str_no_copy("nice");
    /// let s2 = cf::String::from_str_no_copy("nice");
    ///
    /// assert_eq!(4, s1.length());
    /// assert!(s1.equal(&s2));
    ///```
    #[inline]
    pub fn from_str_no_copy(str: &str) -> Retained<String> {
        let bytes = str.as_bytes();
        unsafe {
            Self::create_with_bytes_no_copy(
                None,
                bytes,
                bytes.len() as _,
                Encoding::UTF8,
                false,
                Allocator::null(),
            )
            .unwrap_unchecked()
        }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("nice");
    /// let s2 = cf::String::from_str("nice");
    /// let s3 = cf::String::from_str("nice string");
    ///
    /// assert_eq!(4, s1.length());
    /// assert!(s1.equal(&s2));
    /// assert!(s3.has_prefix(&s2));
    ///```
    #[inline]
    pub fn from_str(str: &str) -> Retained<String> {
        let bytes = str.as_bytes();
        unsafe {
            Self::create_with_bytes(None, bytes, bytes.len() as _, Encoding::UTF8, false)
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn from_cstr(cstr: &CStr) -> Retained<String> {
        unsafe {
            Self::create_with_cstring(None, cstr.to_bytes_with_nul(), Encoding::UTF8)
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn from_cstr_no_copy(cstr: &CStr) -> Retained<String> {
        unsafe {
            Self::create_with_cstring_no_copy(
                None,
                cstr.to_bytes_with_nul(),
                Encoding::UTF8,
                Allocator::null(),
            )
            .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn show_str(&self) {
        unsafe { CFShowStr(self) }
    }

    #[inline]
    pub fn length(&self) -> Index {
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
    pub fn get_character_at_index(&self, idx: Index) -> UniChar {
        unsafe { CFStringGetCharacterAtIndex(self, idx) }
    }

    #[inline]
    pub fn create_copy(&self, alloc: Option<&Allocator>) -> Option<Retained<String>> {
        unsafe { CFStringCreateCopy(alloc, self) }
    }

    #[inline]
    pub fn copy(&self) -> Option<Retained<String>> {
        self.create_copy(None)
    }

    #[inline]
    pub fn create_with_bytes_no_copy(
        alloc: Option<&Allocator>,
        bytes: &[u8],
        num_bytes: Index,
        encoding: Encoding,
        is_external_representation: bool,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Retained<String>> {
        unsafe {
            let bytes = bytes.as_ptr();
            CFStringCreateWithBytesNoCopy(
                alloc,
                bytes,
                num_bytes,
                encoding,
                is_external_representation,
                contents_deallocator,
            )
        }
    }

    #[inline]
    pub fn create_with_cstring_no_copy(
        alloc: Option<&Allocator>,
        bytes_with_null: &[u8],
        encoding: Encoding,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Retained<String>> {
        unsafe {
            let c_str = bytes_with_null.as_ptr() as *const i8;
            CFStringCreateWithCStringNoCopy(alloc, c_str, encoding, contents_deallocator)
        }
    }

    #[inline]
    pub fn create_with_cstring(
        alloc: Option<&Allocator>,
        bytes_with_null: &[u8],
        encoding: Encoding,
    ) -> Option<Retained<String>> {
        unsafe {
            let c_str = bytes_with_null.as_ptr() as *const i8;
            CFStringCreateWithCString(alloc, c_str, encoding)
        }
    }

    #[inline]
    pub fn create_with_bytes(
        alloc: Option<&Allocator>,
        bytes: &[u8],
        num_bytes: Index,
        encoding: Encoding,
        is_external_representation: bool,
    ) -> Option<Retained<String>> {
        unsafe {
            let bytes = bytes.as_ptr();
            CFStringCreateWithBytes(
                alloc,
                bytes,
                num_bytes,
                encoding,
                is_external_representation,
            )
        }
    }

    #[inline]
    pub fn create_mutable_copy(
        &self,
        alloc: Option<&Allocator>,
        max_length: Index,
    ) -> Option<Retained<MutableString>> {
        unsafe { CFStringCreateMutableCopy(alloc, max_length, self) }
    }

    #[inline]
    pub fn mutable_copy(&self, max_length: Index) -> Option<Retained<MutableString>> {
        self.create_mutable_copy(None, max_length)
    }
}

#[macro_export]
macro_rules! cfstr {
    ($f:literal) => {
        unsafe {
            // TODO: becnhamrk and mem usage
            // https://opensource.apple.com/source/CF/CF-855.17/CFString.c
            extern "C" {
                fn __CFStringMakeConstantString(
                    str: *const std::os::raw::c_char,
                ) -> &'static crate::cf::String;
            }

            __CFStringMakeConstantString(
                std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($f, "\0").as_bytes())
                    .as_ptr(),
            )
        }
    };
}

impl<'a> From<&'a String> for Cow<'a, str> {
    fn from(cfstr: &'a String) -> Self {
        unsafe {
            let c_str = CFStringGetCStringPtr(cfstr, Encoding::UTF8);
            if c_str.is_null() {
                let range = crate::cf::Range {
                    location: 0,
                    length: cfstr.length(),
                };
                let mut bytes_required: Index = 0;
                CFStringGetBytes(
                    cfstr,
                    range,
                    Encoding::UTF8,
                    0,
                    false,
                    std::ptr::null_mut(),
                    0,
                    &mut bytes_required,
                );

                let mut buffer = Vec::with_capacity(bytes_required as _);
                buffer.set_len(bytes_required as _);
                let mut used_buf_len: Index = 0;
                CFStringGetBytes(
                    cfstr,
                    range,
                    Encoding::UTF8,
                    0,
                    false,
                    buffer.as_mut_ptr(),
                    buffer.len() as _,
                    &mut used_buf_len,
                );

                debug_assert_eq!(bytes_required, used_buf_len);

                Cow::Owned(std::string::String::from_utf8_unchecked(buffer))
            } else {
                let cstr = CStr::from_ptr(c_str);
                Cow::Borrowed(from_utf8_unchecked(cstr.to_bytes()))
            }
        }
    }
}

impl fmt::Display for String {
    /// ```
    /// use cidre::cf;
    ///
    /// let s = cf::String::from_str("nice");
    /// let ss = s.to_string();
    ///
    /// assert_eq!("nice", &ss);
    /// ```
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self))
    }
}

// impl fmt::Debug for String {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         fmt.write_str(&Cow::from(self))
//     }
// }

define_cf_type!(MutableString(String));

impl MutableString {
    #[inline]
    pub fn append(&mut self, appended_string: &String) {
        unsafe { CFStringAppend(self, appended_string) }
    }

    #[inline]
    pub fn trim(&mut self, trim_string: &String) {
        unsafe { CFStringTrim(self, trim_string) }
    }

    #[inline]
    pub fn trim_whitespace(&mut self) {
        unsafe { CFStringTrimWhitespace(self) }
    }

    #[inline]
    pub fn create(alloc: Option<&Allocator>, max_length: Index) -> Option<Retained<Self>> {
        unsafe { CFStringCreateMutable(alloc, max_length) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFStringGetTypeID() -> TypeId;
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
    fn CFStringTrim(the_string: &mut MutableString, trim_string: &String);
    fn CFStringTrimWhitespace(the_string: &mut MutableString);

    fn CFStringCreateWithBytesNoCopy(
        alloc: Option<&Allocator>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: Encoding,
        is_external_representation: bool,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Retained<String>>;

    fn CFStringCreateWithCStringNoCopy(
        alloc: Option<&Allocator>,
        c_str: *const c_char,
        encoding: Encoding,
        contents_deallocator: Option<&Allocator>,
    ) -> Option<Retained<String>>;

    fn CFStringCreateWithCString(
        alloc: Option<&Allocator>,
        c_str: *const c_char,
        encoding: Encoding,
    ) -> Option<Retained<String>>;

    fn CFStringCreateWithBytes(
        alloc: Option<&Allocator>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: Encoding,
        is_external_representation: bool,
    ) -> Option<Retained<String>>;

    fn CFShowStr(str: &String);

    fn CFStringGetCStringPtr(the_string: &String, encoding: Encoding) -> *const c_char;
    fn CFStringGetBytes(
        the_string: &String,
        range: Range,
        encoding: Encoding,
        loss_byte: u8,
        is_external_representation: bool,
        buffer: *mut u8,
        max_buflen: Index,
        used_buf_len: *mut Index,
    ) -> Index;

    // fn __CFStringMakeConstantString(str: *const c_char) -> &'static String;
}

impl From<&'static str> for Retained<String> {
    #[inline]
    fn from(s: &'static str) -> Self {
        String::from_str_no_copy(s)
    }
}

impl From<&std::string::String> for Retained<String> {
    #[inline]
    fn from(s: &std::string::String) -> Self {
        String::from_str(s.as_str())
    }
}

#[cfg(test)]
mod tests {

    use crate::cf;

    #[test]
    fn it_works() {
        let s = cf::String::from_str("hello");
        assert_eq!(s.length(), 5);
        let std_str = s.to_string();
        assert_eq!(std_str.chars().count(), 5);
    }

    #[test]
    fn macro_cfstr() {
        let s = cfstr!("nice");
        println!("rt {}", s.retain_count());
        s.show();
        s.show_str();

        assert_eq!(s.to_string(), "nice".to_string());
    }
}
