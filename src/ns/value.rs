use crate::{cf, define_obj_type, msg_send, ns};

define_obj_type!(Value(ns::Id));

define_obj_type!(Number(Value));

impl Number {
    #[inline]
    pub fn with_i8(value: i8) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithChar(value) }
    }

    #[inline]
    pub fn with_u8(value: u8) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithUnsignedChar(value) }
    }

    #[inline]
    pub fn with_i16(value: i16) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithShort(value) }
    }

    #[inline]
    pub fn with_u16(value: u16) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithUnsignedShort(value) }
    }

    #[inline]
    pub fn with_i32(value: i32) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithInt(value) }
    }

    #[inline]
    pub fn with_u32(value: u32) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithUnsignedInt(value) }
    }

    #[inline]
    pub fn with_i64(value: i64) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithLongLong(value) }
    }

    #[inline]
    pub fn with_u64(value: u64) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithUnsignedLongLong(value) }
    }

    #[inline]
    pub fn with_f32(value: f32) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithFloat(value) }
    }

    #[inline]
    pub fn with_f64(value: f64) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithDouble(value) }
    }

    #[inline]
    pub fn with_bool(value: bool) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithBool(value) }
    }

    #[inline]
    pub fn with_integer(value: ns::Integer) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithInteger(value) }
    }

    #[inline]
    pub fn with_uinteger(value: ns::UInteger) -> cf::Retained<Self> {
        unsafe { NSNumber_numberWithUnsignedInteger(value) }
    }

    #[doc(alias = "charValue")]
    #[inline]
    pub fn as_i8(&self) -> i8 {
        msg_send!("ns", self, ns_charValue)
    }

    #[doc(alias = "unsignedCharValue")]
    #[inline]
    pub fn as_u8(&self) -> u8 {
        msg_send!("ns", self, ns_unsignedCharValue)
    }

    #[doc(alias = "shortValue")]
    #[inline]
    pub fn as_i16(&self) -> i16 {
        msg_send!("ns", self, ns_shortValue)
    }

    #[doc(alias = "unsignedShortValue")]
    #[inline]
    pub fn as_u16(&self) -> u16 {
        msg_send!("ns", self, ns_unsignedShortValue)
    }

    #[doc(alias = "intValue")]
    #[doc(alias = "longValue")]
    #[inline]
    pub fn as_i32(&self) -> i32 {
        msg_send!("ns", self, ns_intValue)
    }

    #[doc(alias = "unsignedIntValue")]
    #[doc(alias = "unsignedLongValue")]
    #[inline]
    pub fn as_u32(&self) -> u32 {
        msg_send!("ns", self, ns_unsignedIntValue)
    }

    #[doc(alias = "longLongValue")]
    #[inline]
    pub fn as_i64(&self) -> i64 {
        msg_send!("ns", self, ns_longLongValue)
    }

    #[doc(alias = "unsignedLongLongValue")]
    #[inline]
    pub fn as_u64(&self) -> u64 {
        msg_send!("ns", self, ns_unsignedLongLongValue)
    }

    #[doc(alias = "floatValue")]
    #[inline]
    pub fn as_f32(&self) -> f32 {
        msg_send!("ns", self, ns_floatValue)
    }

    #[doc(alias = "doubleValue")]
    #[inline]
    pub fn as_f64(&self) -> f64 {
        msg_send!("ns", self, ns_doubleValue)
    }

    #[doc(alias = "integerValue")]
    #[inline]
    pub fn as_integer(&self) -> ns::Integer {
        msg_send!("ns", self, ns_integerValue)
    }

    #[doc(alias = "unsignedIntegerValue")]
    #[inline]
    pub fn as_uinteger(&self) -> ns::UInteger {
        msg_send!("ns", self, ns_unsignedIntegerValue)
    }

    #[doc(alias = "stringValue")]
    #[inline]
    pub fn string(&self) -> cf::Retained<cf::String> {
        msg_send!("ns", self, ns_stringValue)
    }

    #[inline]
    pub fn is_equal_to_number(&self, number: &ns::Number) -> bool {
        unsafe { rsel_isEqualToNumber(self, number) }
    }
}

impl From<i8> for cf::Retained<Number> {
    #[inline]
    fn from(value: i8) -> Self {
        Number::with_i8(value)
    }
}

impl From<u8> for cf::Retained<Number> {
    #[inline]
    fn from(value: u8) -> Self {
        Number::with_u8(value)
    }
}

impl From<i16> for cf::Retained<Number> {
    #[inline]
    fn from(value: i16) -> Self {
        Number::with_i16(value)
    }
}

impl From<u16> for cf::Retained<Number> {
    #[inline]
    fn from(value: u16) -> Self {
        Number::with_u16(value)
    }
}

impl From<i32> for cf::Retained<Number> {
    #[inline]
    fn from(value: i32) -> Self {
        Number::with_i32(value)
    }
}

impl From<u32> for cf::Retained<Number> {
    #[inline]
    fn from(value: u32) -> Self {
        Number::with_u32(value)
    }
}

impl From<i64> for cf::Retained<Number> {
    #[inline]
    fn from(value: i64) -> Self {
        Number::with_i64(value)
    }
}

impl From<u64> for cf::Retained<Number> {
    #[inline]
    fn from(value: u64) -> Self {
        Number::with_u64(value)
    }
}

impl From<f32> for cf::Retained<Number> {
    #[inline]
    fn from(value: f32) -> Self {
        Number::with_f32(value)
    }
}

impl From<f64> for cf::Retained<Number> {
    #[inline]
    fn from(value: f64) -> Self {
        Number::with_f64(value)
    }
}

impl From<bool> for cf::Retained<Number> {
    #[inline]
    fn from(value: bool) -> Self {
        Number::with_bool(value)
    }
}

impl From<ns::Integer> for cf::Retained<Number> {
    #[inline]
    fn from(value: ns::Integer) -> Self {
        Number::with_integer(value)
    }
}

impl From<ns::UInteger> for cf::Retained<Number> {
    #[inline]
    fn from(value: ns::UInteger) -> Self {
        Number::with_uinteger(value)
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal_to_number(other)
    }
}

impl Eq for Number {}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSNumber_numberWithChar(value: i8) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithUnsignedChar(value: u8) -> cf::Retained<ns::Number>;

    fn NSNumber_numberWithShort(value: i16) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithUnsignedShort(value: u16) -> cf::Retained<ns::Number>;

    fn NSNumber_numberWithInt(value: i32) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithUnsignedInt(value: u32) -> cf::Retained<ns::Number>;

    fn NSNumber_numberWithLongLong(value: i64) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithUnsignedLongLong(value: u64) -> cf::Retained<ns::Number>;

    fn NSNumber_numberWithFloat(value: f32) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithDouble(value: f64) -> cf::Retained<ns::Number>;

    fn NSNumber_numberWithBool(value: bool) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithInteger(value: ns::Integer) -> cf::Retained<ns::Number>;
    fn NSNumber_numberWithUnsignedInteger(value: ns::UInteger) -> cf::Retained<ns::Number>;

    fn rsel_isEqualToNumber(id: &ns::Id, number: &Number) -> bool;
}
