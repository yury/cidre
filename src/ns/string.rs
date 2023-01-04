use crate::{cf, define_obj_type, msg_send, ns};

define_obj_type!(String(ns::Id));
define_obj_type!(MutableString(String));

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
    pub fn lowercased(&self) -> cf::Retained<Self> {
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
    pub fn mutable_copy(&self) -> cf::Retained<ns::String> {
        msg_send!("ns", self, ns_mutableCopy)
    }
}
