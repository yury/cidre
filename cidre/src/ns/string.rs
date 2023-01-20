use std::{borrow::Cow, ffi::CStr, fmt, str::from_utf8_unchecked};

use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Encoding {
    ASCII = 1,
    UTF8 = 4,
    MacOSRoman = 30,
}

define_obj_type!(String(ns::Id));
define_obj_type!(StringMut(String));

impl String {
    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(lowercaseString)]
    pub fn lowercased_ar(&self) -> arc::Rar<Self>;

    #[objc::rar_retain()]
    pub fn lowercased(&self) -> arc::R<Self>;

    #[objc::msg_send(intValue)]
    pub fn to_i32(&self) -> i32;

    #[objc::msg_send(longLongValue)]
    pub fn to_i64(&self) -> i64;

    #[objc::msg_send(floatValue)]
    pub fn to_f32(&self) -> f32;

    #[objc::msg_send(doubleValue)]
    pub fn to_f64(&self) -> f64;

    #[objc::msg_send(boolValue)]
    pub fn to_bool(&self) -> bool;

    /// The ns::Integer value of the string.
    ///
    /// The ns::Integer value of the string, assuming a decimal representation and skipping
    /// whitespace at the beginning of the string. This property is 0 if the string doesn’t
    /// begin with a valid decimal text representation of a number.
    /// This property uses formatting information stored in the non-localized value;
    /// use an ns::Scanner object for localized scanning of numeric values from a string.
    #[objc::msg_send(integerValue)]
    pub fn to_integer(&self) -> ns::Integer;

    // TODO: check Rar
    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::R<ns::StringMut>;

    #[inline]
    pub fn with_str(str: &str) -> arc::R<Self> {
        // we expect no '\0' inside string.
        debug_assert!(str.find('\0').is_none());
        unsafe {
            NSString_initWithBytes_length_encoding(str.as_ptr(), str.len(), Encoding::UTF8).unwrap()
        }
    }

    #[inline]
    pub fn with_str_no_copy(str: &str) -> arc::R<Self> {
        unsafe {
            NSString_initWithBytesNoCopy_length_encoding_freeWhenDone(
                str.as_ptr(),
                str.len(),
                Encoding::UTF8,
                false,
            )
            .unwrap()
        }
    }

    #[objc::msg_send(substringWithRange:)]
    pub fn substring_with_range_ar(&self, range: ns::Range) -> arc::Rar<Self>;

    #[objc::rar_retain()]
    pub fn substring_with_range(&self, range: ns::Range) -> arc::R<Self>;

    #[inline]
    pub fn substring(&self, range: std::ops::Range<usize>) -> arc::R<ns::String> {
        self.substring_with_range(range.into())
    }

    #[objc::msg_send(cStringUsingEncoding:)]
    pub fn c_string(&self, encoding: Encoding) -> *const i8;

    #[objc::msg_send(lengthOfBytesUsingEncoding:)]
    pub fn len_of_bytes(&self, encoding: Encoding) -> ns::UInteger;

    #[objc::msg_send(UTF8String)]
    pub unsafe fn utf8_chars(&self) -> *const i8;
}

impl PartialEq for String {
    #[objc::msg_send(isEqualToString:)]
    fn eq(&self, other: &Self) -> bool;
}

impl std::ops::Index<std::ops::Range<usize>> for String {
    type Output = String;

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        let res = self.substring(index);
        res.autoreleased()
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSString_initWithBytes_length_encoding(
        bytes: *const u8,
        length: usize,
        encoding: Encoding,
    ) -> Option<arc::R<String>>;

    fn NSString_initWithBytesNoCopy_length_encoding_freeWhenDone(
        bytes: *const u8,
        length: usize,
        encoding: Encoding,
        free_when_done: bool,
    ) -> Option<arc::R<String>>;
}

impl fmt::Display for String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self))
    }
}

impl<'a> From<&'a String> for Cow<'a, str> {
    fn from(nsstr: &'a String) -> Self {
        unsafe {
            let c_str = nsstr.utf8_chars();
            if c_str.is_null() {
                let bytes_required = nsstr.len_of_bytes(Encoding::UTF8);

                let mut buffer = Vec::with_capacity(bytes_required as _);
                buffer.set_len(bytes_required as _);
                todo!();
                // let mut used_buf_len: Index = 0;
                // CFStringGetBytes(
                //     cfstr,
                //     range,
                //     Encoding::UTF8,
                //     0,
                //     false,
                //     buffer.as_mut_ptr(),
                //     buffer.len() as _,
                //     &mut used_buf_len,
                // );

                // debug_assert_eq!(bytes_required, used_buf_len);

                Cow::Owned(std::string::String::from_utf8_unchecked(buffer))
            } else {
                let cstr = CStr::from_ptr(c_str);
                Cow::Borrowed(from_utf8_unchecked(cstr.to_bytes()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ns::{self, try_catch},
        objc::autoreleasepool,
    };

    #[test]
    fn basics() {
        autoreleasepool(|| {
            let s = ns::String::with_str("10.5");

            assert_eq!(s.len(), 4);
            assert!(!s.is_empty());

            assert_eq!(s.to_i32(), 10);
            assert_eq!(s.to_integer(), 10);
            assert_eq!(s.to_f32(), 10.5f32);
            assert_eq!(s.to_f64(), 10.5f64);
            assert_eq!(s.to_bool(), true);

            let sub = s.substring(1..2);
            assert_eq!(sub.to_i32(), 0);
            assert_eq!(sub.to_bool(), false);

            let zero = ns::String::with_str("0");
            assert_eq!(&zero, &sub);

            let sub = &s[3..4];
            assert_eq!(sub.to_i32(), 5);

            let r = try_catch(|| s.substring(1..10));
            assert!(r.is_err());

            let _l = s.lowercased();
        });
    }
}
