use core::fmt;
use std::{borrow::Cow, ffi::CStr, os::raw::c_char, str::from_utf8_unchecked};

use super::{Index, OptionFlags, Range, Type, TypeId};

use crate::{arc, cf, define_cf_type, define_opts, UniChar};

#[cfg(feature = "ns")]
use crate::ns;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Encoding(u32);

impl Encoding {
    pub const MAC_ROMAN: Self = Self(0);
    pub const ASCII: Self = Self(0x0600);
    pub const UTF8: Self = Self(0x08000100);

    /// The default encoding for the system; untagged 8-bit characters are usually in this encoding
    ///
    /// CFStringGetSystemEncoding
    ///
    /// ```
    /// use cidre::cf;
    ///
    /// let encoding = cf::StringEncoding::system_encoding();
    /// assert_eq!(encoding, cf::StringEncoding::MAC_ROMAN);
    /// ```
    #[inline]
    pub fn system_encoding() -> Self {
        unsafe { CFStringGetSystemEncoding() }
    }
}

define_opts!(pub CompareFlags(OptionFlags));

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

// https://github.com/apportable/Foundation/blob/master/System/CoreFoundation/src/CFString.c

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
    /// let s1 = unsafe { cf::String::from_str_no_copy("nice") };
    /// let s2 = unsafe { cf::String::from_str_no_copy("nice") };
    ///
    /// assert_eq!(4, s1.len());
    /// assert!(s1.equal(&s2));
    ///```
    /// NOTE: cf_string benchmark reveals that it is actually do copy
    #[inline]
    pub unsafe fn from_str_no_copy(str: &str) -> arc::R<Self> {
        let bytes = str.as_bytes();
        unsafe {
            Self::create_with_bytes_no_copy_in(
                bytes,
                Encoding::UTF8,
                false,
                cf::Allocator::null(),
                None,
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
    /// assert_eq!(4, s1.len());
    /// assert!(s1.equal(&s2));
    /// assert!(s3.has_prefix(&s2));
    ///```
    #[inline]
    pub fn from_str(str: &str) -> arc::R<Self> {
        let bytes = str.as_bytes();
        unsafe { Self::create_with_bytes(None, bytes, Encoding::UTF8, false).unwrap_unchecked() }
    }

    #[inline]
    pub fn from_cstr(cstr: &CStr) -> arc::R<Self> {
        unsafe {
            Self::create_with_cstring_in(cstr.to_bytes_with_nul(), Encoding::UTF8, None)
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub unsafe fn from_cstr_no_copy(cstr: &CStr) -> arc::R<Self> {
        Self::create_with_cstring_no_copy_in(
            cstr.to_bytes_with_nul(),
            Encoding::UTF8,
            cf::Allocator::null(),
            None,
        )
        .unwrap_unchecked()
    }

    #[inline]
    pub fn show_str(&self) {
        unsafe { CFShowStr(self) }
    }

    #[inline]
    pub fn len(&self) -> Index {
        unsafe { CFStringGetLength(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn has_suffix(&self, suffix: &Self) -> bool {
        unsafe { CFStringHasSuffix(self, suffix) }
    }

    /// CFStringHasPrefix
    #[inline]
    pub fn has_prefix(&self, prefix: &Self) -> bool {
        unsafe { CFStringHasPrefix(self, prefix) }
    }

    /// CFStringGetCharacterAtIndex
    #[inline]
    pub fn character_at_index(&self, idx: Index) -> UniChar {
        unsafe { CFStringGetCharacterAtIndex(self, idx) }
    }

    /// CFStringCreateCopy
    #[inline]
    pub fn copy_in(&self, alloc: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFStringCreateCopy(alloc, self) }
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { std::mem::transmute(self.copy_in(None)) }
    }

    #[inline]
    pub fn create_with_bytes_no_copy_in(
        bytes: &[u8],
        encoding: Encoding,
        is_external_representation: bool,
        contents_deallocator: Option<&cf::Allocator>,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CFStringCreateWithBytesNoCopy(
                alloc,
                bytes.as_ptr(),
                bytes.len() as _,
                encoding,
                is_external_representation,
                contents_deallocator,
            )
        }
    }

    #[inline]
    pub unsafe fn create_with_cstring_no_copy_in(
        bytes_with_null: &[u8],
        encoding: Encoding,
        contents_deallocator: Option<&cf::Allocator>,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            let c_str = bytes_with_null.as_ptr() as *const i8;
            CFStringCreateWithCStringNoCopy(alloc, c_str, encoding, contents_deallocator)
        }
    }

    #[inline]
    pub fn create_with_cstring_in(
        bytes_with_null: &[u8],
        encoding: Encoding,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            let c_str = bytes_with_null.as_ptr() as *const i8;
            CFStringCreateWithCString(alloc, c_str, encoding)
        }
    }

    #[inline]
    pub fn create_with_bytes(
        alloc: Option<&cf::Allocator>,
        bytes: &[u8],
        encoding: Encoding,
        is_external_representation: bool,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CFStringCreateWithBytes(
                alloc,
                bytes.as_ptr(),
                bytes.len() as _,
                encoding,
                is_external_representation,
            )
        }
    }

    #[inline]
    pub fn copy_mut_in(
        &self,
        max_length: Index,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<StringMut>> {
        unsafe { CFStringCreateMutableCopy(alloc, max_length, self) }
    }

    #[inline]
    pub fn copy_mut(&self, max_length: Index) -> arc::R<StringMut> {
        unsafe { self.copy_mut_in(max_length, None).unwrap_unchecked() }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::String {
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn to_string(&self) -> std::string::String {
        unsafe {
            let range = crate::cf::Range {
                loc: 0,
                len: self.len(),
            };
            let mut bytes_required: Index = 0;
            CFStringGetBytes(
                self,
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
                self,
                range,
                Encoding::UTF8,
                0,
                false,
                buffer.as_mut_ptr(),
                buffer.len() as _,
                &mut used_buf_len,
            );

            debug_assert_eq!(bytes_required, used_buf_len);

            std::string::String::from_utf8_unchecked(buffer)
        }
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
                ) -> &'static $crate::cf::String;
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
                Cow::Owned(cfstr.to_string())
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

define_cf_type!(StringMut(String));

impl StringMut {
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
    pub fn create(alloc: Option<&cf::Allocator>, max_length: Index) -> Option<arc::R<Self>> {
        unsafe { CFStringCreateMutable(alloc, max_length) }
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        let ptr = unsafe { CFStringGetCStringPtr(self, Encoding::UTF8) };
        if ptr.is_null() {
            return false;
        }
        let s = unsafe { CStr::from_ptr(ptr) };
        if let Ok(s) = s.to_str() {
            return s.eq(other);
        }

        false
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFStringGetTypeID() -> TypeId;
    fn CFStringGetLength(the_string: &String) -> Index;
    fn CFStringCreateMutable(
        alloc: Option<&cf::Allocator>,
        max_length: Index,
    ) -> Option<arc::R<StringMut>>;
    fn CFStringCreateCopy(
        alloc: Option<&cf::Allocator>,
        the_string: &String,
    ) -> Option<arc::R<String>>;
    fn CFStringHasPrefix(the_string: &String, prefix: &String) -> bool;
    fn CFStringHasSuffix(the_string: &String, prefix: &String) -> bool;
    fn CFStringCreateMutableCopy(
        alloc: Option<&cf::Allocator>,
        max_length: Index,
        the_string: &String,
    ) -> Option<arc::R<StringMut>>;
    fn CFStringGetCharacterAtIndex(the_string: &String, idx: Index) -> UniChar;

    fn CFStringAppend(the_string: &mut StringMut, appended_string: &String);
    fn CFStringTrim(the_string: &mut StringMut, trim_string: &String);
    fn CFStringTrimWhitespace(the_string: &mut StringMut);

    fn CFStringCreateWithBytesNoCopy(
        alloc: Option<&cf::Allocator>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: Encoding,
        is_external_representation: bool,
        contents_deallocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<String>>;

    fn CFStringCreateWithCStringNoCopy(
        alloc: Option<&cf::Allocator>,
        c_str: *const c_char,
        encoding: Encoding,
        contents_deallocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<String>>;

    fn CFStringCreateWithCString(
        alloc: Option<&cf::Allocator>,
        c_str: *const c_char,
        encoding: Encoding,
    ) -> Option<arc::R<String>>;

    fn CFStringCreateWithBytes(
        alloc: Option<&cf::Allocator>,
        bytes: *const u8,
        num_bytes: Index,
        encoding: Encoding,
        is_external_representation: bool,
    ) -> Option<arc::R<String>>;

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

    fn CFStringGetSystemEncoding() -> Encoding;

    // fn __CFStringMakeConstantString(str: *const c_char) -> &'static String;
}

impl From<&'static str> for arc::R<String> {
    #[inline]
    fn from(s: &'static str) -> Self {
        String::from_str(s)
    }
}

impl From<&std::string::String> for arc::R<String> {
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
        assert_eq!(s.len(), 5);
        let std_str = s.to_string();
        assert_eq!(std_str.chars().count(), 5);

        let ns_str = s.as_ns();
        assert_eq!(&ns_str.to_string(), "hello");
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
