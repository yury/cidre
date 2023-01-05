use crate::{arc, define_obj_type, msg_send, ns};

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
    #[inline]
    pub fn length(&self) -> usize {
        msg_send!("ns", self, ns_length)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    #[inline]
    pub fn lowercased(&self) -> arc::R<Self> {
        msg_send!("ns", self, ns_lowercaseString)
    }

    #[inline]
    pub fn to_i32(&self) -> i32 {
        msg_send!("ns", self, ns_intValue)
    }

    #[inline]
    pub fn to_i64(&self) -> i64 {
        msg_send!("ns", self, ns_longLongValue)
    }

    #[inline]
    pub fn to_f32(&self) -> f32 {
        msg_send!("ns", self, ns_floatValue)
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        msg_send!("ns", self, ns_doubleValue)
    }

    #[inline]
    pub fn to_bool(&self) -> bool {
        msg_send!("ns", self, ns_boolValue)
    }

    /// The ns::Integer value of the string.
    ///
    /// The ns::Integer value of the string, assuming a decimal representation and skipping
    /// whitespace at the beginning of the string. This property is 0 if the string doesn’t
    /// begin with a valid decimal text representation of a number.
    /// This property uses formatting information stored in the non-localized value;
    /// use an ns::Scanner object for localized scanning of numeric values from a string.
    #[inline]
    pub fn to_integer(&self) -> ns::Integer {
        msg_send!("ns", self, ns_integerValue)
    }

    #[inline]
    pub fn copy_mut(&self) -> arc::R<ns::StringMut> {
        msg_send!("ns", self, ns_mutableCopy)
    }

    #[inline]
    pub fn with_str(str: &str) -> arc::R<Self> {
        unsafe {
            NSString_initWithBytes_length_encoding(str.as_ptr(), str.len(), Encoding::UTF8).unwrap()
        }
    }

    #[inline]
    pub fn substring(&self, range: std::ops::Range<usize>) -> arc::R<ns::String> {
        let range: ns::Range = range.into();
        msg_send!("ns", self, ns_substringWithRange, range)
    }
}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        msg_send!("ns", self, ns_isEqualToString, other)
    }
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

            assert_eq!(s.length(), 4);
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
        });
    }
}
