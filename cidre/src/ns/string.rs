use std::{borrow::Cow, fmt};

use crate::{
    arc, define_obj_type, ns,
    objc::{self, Obj},
};

#[cfg(feature = "cf")]
use crate::cf;

use super::{
    objc_runtime::{
        ns_class_from_ns_string, ns_selector_from_ns_string, ns_string_from_class,
        ns_string_from_selector,
    },
    Class,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Encoding {
    ASCII = 1,
    UTF8 = 4,
    MacOSRoman = 30,
}

define_obj_type!(
    #[doc(alias = "NSString")]
    pub String(ns::Id), NS_STRING
);

define_obj_type!(
    #[doc(alias = "NSMutableString")]
    pub StringMut(String), NS_MUTABLE_STRING
);

impl arc::A<String> {
    #[objc::msg_send(initWithBytes:length:encoding:)]
    pub fn init_with_bytes_length_encoding(
        self,
        bytes: *const u8,
        length: usize,
        encoding: ns::StringEncoding,
    ) -> Option<arc::R<String>>;

    #[objc::msg_send(initWithBytesNoCopy:length:encoding:freeWhenDone:)]
    pub fn init_with_bytes_no_copy_length_encoding_free_when_done(
        self,
        bytes: *const u8,
        length: usize,
        encoding: ns::StringEncoding,
        free_when_done: bool,
    ) -> Option<arc::R<String>>;
}

impl String {
    #[inline]
    pub fn with_str(str: &str) -> arc::R<Self> {
        unsafe {
            Self::alloc()
                .init_with_bytes_length_encoding(str.as_ptr(), str.len(), Encoding::UTF8)
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn with_str_no_copy(str: &str) -> arc::R<Self> {
        unsafe {
            Self::alloc()
                .init_with_bytes_no_copy_length_encoding_free_when_done(
                    str.as_ptr(),
                    str.len(),
                    Encoding::UTF8,
                    false,
                )
                .unwrap_unchecked()
        }
    }

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(lowercaseString)]
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

    #[objc::msg_send(substringWithRange:)]
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
    pub unsafe fn utf8_chars_ar(&self) -> *const i8;

    #[cfg(feature = "cf")]
    pub fn as_cf(&self) -> &cf::String {
        unsafe { std::mem::transmute(self) }
    }

    #[objc::msg_send(isEqualToString:)]
    pub fn eq_ns_string(&self, other: &Self) -> bool;

    #[inline]
    pub fn to_selector(&self) -> Option<&'static objc::Sel> {
        ns_selector_from_ns_string(self)
    }

    #[inline]
    pub fn to_class(&self) -> Option<&'static objc::Class<ns::Id>> {
        ns_class_from_ns_string(self)
    }

    #[inline]
    pub fn from_class<T: Obj>(cls: &objc::Class<T>) -> arc::R<Self> {
        ns_string_from_class(cls)
    }

    #[inline]
    pub fn from_selector(sel: &objc::Sel) -> arc::R<Self> {
        ns_string_from_selector(sel)
    }
}

// impl std::ops::Index<std::ops::Range<usize>> for String {
//     type Output = arc::Rar<String>;

//     /// Return autoreleased
//     #[inline]
//     fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
//         &self.substring_with_range_ar(index.into())
//     }
// }

impl arc::A<StringMut> {
    #[objc::msg_send(initWithCapacity:)]
    pub fn with_capacity(self, capacity: usize) -> arc::R<StringMut>;
}

impl StringMut {
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        Self::alloc().with_capacity(capacity)
    }

    #[objc::msg_send(replaceCharactersInRange:withString:)]
    pub unsafe fn replace_characters_in_throws(
        &mut self,
        range: ns::Range,
        with_string: &ns::String,
    );

    pub fn replace_characters_in<'ar>(
        &mut self,
        range: ns::Range,
        with_string: &ns::String,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.replace_characters_in_throws(range, with_string) })
    }

    #[objc::msg_send(insertString:atIndex:)]
    pub unsafe fn insert_string_at_throws(&mut self, string: &ns::String, at_index: ns::UInteger);

    pub fn insert_string_at<'ar>(
        &mut self,
        string: &ns::String,
        at_index: ns::UInteger,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.insert_string_at_throws(string, at_index) })
    }

    #[objc::msg_send(deleteCharactersInRange:)]
    pub fn delete_characters_in_throws(&mut self, range: ns::Range);

    pub fn delete_characters_in<'ar>(
        &mut self,
        range: ns::Range,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| self.delete_characters_in_throws(range))
    }

    #[objc::msg_send(append:)]
    pub fn append(&mut self, string: &ns::String);

    #[objc::msg_send(setString:)]
    pub fn set(&mut self, string: &ns::String);

    #[inline]
    pub fn as_cf_mut(&self) -> &cf::StringMut {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_STRING: &'static Class<String>;
    static NS_MUTABLE_STRING: &'static Class<StringMut>;
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.as_cf() == other
    }
}

impl fmt::Display for String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self.as_cf()))
    }
}

#[macro_export]
macro_rules! nsstr {
    ($f:literal) => {{
        $crate::cfstr!($f).as_ns()
    }};
}

pub use nsstr as str;

#[cfg(test)]
mod tests {
    use crate::{ns, objc::ar_pool};

    #[test]
    fn basics() {
        ar_pool(|| {
            let m = ns::StringMut::new();
            assert!(m.is_empty());
            let s = ns::str!(c"10.5");

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

            let zero = ns::str!(c"0");
            assert_eq!(zero, &sub);

            // TODO:
            // let sub = &s[3..4];
            // assert_eq!(sub.to_i32(), 5);

            let r = ns::try_catch(|| s.substring(1..10));
            assert!(r.is_err());

            let _l = s.lowercased();
        });
    }

    #[test]
    fn mut_throws() {
        let one = ns::str!(c"1");
        let mut zero = ns::str!(c"0").copy_mut();
        zero.replace_characters_in(ns::Range::new(0, 1), &one)
            .unwrap();
        zero.replace_characters_in(ns::Range::new(0, 10), &one)
            .expect_err("Should be exceptions");
    }

    #[test]
    fn macro_str() {
        let s = ns::str!(c"привет");
        assert_eq!(12, s.len());
    }
}
