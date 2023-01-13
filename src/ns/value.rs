use crate::{
    arc, define_obj_type, ns,
    objc::{msg_send, Class, Obj},
};

define_obj_type!(Value(ns::Id));
define_obj_type!(Number(Value));

impl Number {
    #[inline]
    pub fn with_i8(value: i8) -> arc::R<Self> {
        unsafe { NS_NUMBER.call1(number_with_char, value) }
    }

    #[inline]
    pub fn tagged_i8(value: i8) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_char, value) }
    }

    #[inline]
    pub fn with_u8(value: u8) -> arc::R<Self> {
        unsafe { NS_NUMBER.call1(number_with_unsigned_char, value) }
    }

    #[inline]
    pub fn tagged_u8(value: u8) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_unsigned_char, value) }
    }

    #[inline]
    pub fn with_i16(value: i16) -> arc::R<Self> {
        unsafe { NS_NUMBER.call1(number_with_short, value) }
    }

    #[inline]
    pub fn with_u16(value: u16) -> arc::R<Self> {
        unsafe { NS_NUMBER.call1(number_with_unsigned_short, value) }
    }

    #[inline]
    pub fn with_i32(value: i32) -> arc::R<Self> {
        unsafe { NSNumber_numberWithInt(value) }
    }

    #[inline]
    pub fn with_u32(value: u32) -> arc::R<Self> {
        unsafe { NSNumber_numberWithUnsignedInt(value) }
    }

    #[inline]
    pub fn with_i64(value: i64) -> arc::R<Self> {
        unsafe { NSNumber_numberWithLongLong(value) }
    }

    #[inline]
    pub fn with_isize(value: isize) -> arc::R<Self> {
        unsafe { NSNumber_numberWithLongLong(value as _) }
    }

    #[inline]
    pub fn with_u64(value: u64) -> arc::R<Self> {
        unsafe { NSNumber_numberWithUnsignedLongLong(value) }
    }

    #[inline]
    pub fn with_f32(value: f32) -> arc::R<Self> {
        unsafe { NSNumber_numberWithFloat(value) }
    }

    #[inline]
    pub fn with_f64(value: f64) -> arc::R<Self> {
        unsafe { NSNumber_numberWithDouble(value) }
    }

    #[inline]
    pub fn with_bool(value: bool) -> arc::R<Self> {
        unsafe { NSNumber_numberWithBool(value) }
    }

    #[inline]
    pub fn with_integer(value: ns::Integer) -> arc::R<Self> {
        unsafe { NSNumber_numberWithInteger(value) }
    }

    #[inline]
    pub fn with_uinteger(value: ns::UInteger) -> arc::R<Self> {
        unsafe { NSNumber_numberWithUnsignedInteger(value) }
    }

    #[doc(alias = "charValue")]
    #[inline]
    pub fn as_i8(&self) -> i8 {
        unsafe { self.call0(msg_send::char_value) }
    }

    #[doc(alias = "unsignedCharValue")]
    #[inline]
    pub fn as_u8(&self) -> u8 {
        unsafe { self.call0(msg_send::unsigned_char_value) }
    }

    #[doc(alias = "shortValue")]
    #[inline]
    pub fn as_i16(&self) -> i16 {
        unsafe { self.call0(msg_send::short_value) }
    }

    #[doc(alias = "unsignedShortValue")]
    #[inline]
    pub fn as_u16(&self) -> u16 {
        unsafe { self.call0(msg_send::unsigned_short_value) }
    }

    #[doc(alias = "intValue")]
    #[doc(alias = "longValue")]
    #[inline]
    pub fn as_i32(&self) -> i32 {
        unsafe { self.call0(msg_send::int_value) }
    }

    #[doc(alias = "unsignedIntValue")]
    #[doc(alias = "unsignedLongValue")]
    #[inline]
    pub fn as_u32(&self) -> u32 {
        unsafe { self.call0(msg_send::unsingned_int_value) }
    }

    #[doc(alias = "longLongValue")]
    #[inline]
    pub fn as_i64(&self) -> i64 {
        unsafe { self.call0(msg_send::long_long_value) }
    }

    #[doc(alias = "longLongValue")]
    #[inline]
    pub fn as_isize(&self) -> isize {
        self.as_i64() as _
    }

    #[doc(alias = "unsignedLongLongValue")]
    #[inline]
    pub fn as_u64(&self) -> u64 {
        unsafe { self.call0(msg_send::unsigned_long_long_value) }
    }

    #[doc(alias = "floatValue")]
    #[inline]
    pub fn as_f32(&self) -> f32 {
        unsafe { self.call0(msg_send::float_value) }
    }

    #[doc(alias = "doubleValue")]
    #[inline]
    pub fn as_f64(&self) -> f64 {
        unsafe { self.call0(msg_send::double_value) }
    }

    #[doc(alias = "integerValue")]
    #[inline]
    pub fn as_integer(&self) -> ns::Integer {
        unsafe { self.call0(msg_send::integer_value) }
    }

    #[doc(alias = "unsignedIntegerValue")]
    #[inline]
    pub fn as_uinteger(&self) -> ns::UInteger {
        unsafe { self.call0(msg_send::unsigned_integer_value) }
    }

    #[doc(alias = "stringValue")]
    #[inline]
    pub fn string(&self) -> arc::R<ns::String> {
        unsafe { self.call0(msg_send::string_value) }
    }

    #[inline]
    pub fn is_equal_to_number(&self, number: &ns::Number) -> bool {
        unsafe { rsel_isEqualToNumber(self, number) }
    }
}

impl From<i8> for arc::R<Number> {
    #[inline]
    fn from(value: i8) -> Self {
        Number::with_i8(value)
    }
}

impl From<u8> for arc::R<Number> {
    #[inline]
    fn from(value: u8) -> Self {
        Number::with_u8(value)
    }
}

impl From<i16> for arc::R<Number> {
    #[inline]
    fn from(value: i16) -> Self {
        Number::with_i16(value)
    }
}

impl From<u16> for arc::R<Number> {
    #[inline]
    fn from(value: u16) -> Self {
        Number::with_u16(value)
    }
}

impl From<i32> for arc::R<Number> {
    #[inline]
    fn from(value: i32) -> Self {
        Number::with_i32(value)
    }
}

impl From<u32> for arc::R<Number> {
    #[inline]
    fn from(value: u32) -> Self {
        Number::with_u32(value)
    }
}

impl From<i64> for arc::R<Number> {
    #[inline]
    fn from(value: i64) -> Self {
        Number::with_i64(value)
    }
}

impl From<u64> for arc::R<Number> {
    #[inline]
    fn from(value: u64) -> Self {
        Number::with_u64(value)
    }
}

impl From<f32> for arc::R<Number> {
    #[inline]
    fn from(value: f32) -> Self {
        Number::with_f32(value)
    }
}

impl From<f64> for arc::R<Number> {
    #[inline]
    fn from(value: f64) -> Self {
        Number::with_f64(value)
    }
}

impl From<bool> for arc::R<Number> {
    #[inline]
    fn from(value: bool) -> Self {
        Number::with_bool(value)
    }
}

impl From<ns::Integer> for arc::R<Number> {
    #[inline]
    fn from(value: ns::Integer) -> Self {
        Number::with_integer(value)
    }
}

impl From<ns::UInteger> for arc::R<Number> {
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

    static NS_NUMBER: &'static Class;

    fn NSNumber_numberWithInt(value: i32) -> arc::R<ns::Number>;
    fn NSNumber_numberWithUnsignedInt(value: u32) -> arc::R<ns::Number>;

    fn NSNumber_numberWithLongLong(value: i64) -> arc::R<ns::Number>;
    fn NSNumber_numberWithUnsignedLongLong(value: u64) -> arc::R<ns::Number>;

    fn NSNumber_numberWithFloat(value: f32) -> arc::R<ns::Number>;
    fn NSNumber_numberWithDouble(value: f64) -> arc::R<ns::Number>;

    fn NSNumber_numberWithBool(value: bool) -> arc::R<ns::Number>;
    fn NSNumber_numberWithInteger(value: ns::Integer) -> arc::R<ns::Number>;
    fn NSNumber_numberWithUnsignedInteger(value: ns::UInteger) -> arc::R<ns::Number>;

    fn rsel_isEqualToNumber(id: &ns::Id, number: &Number) -> bool;
}

extern "C" {
    #[link_name = "objc_msgSend$numberWithChar:"]
    fn number_with_char();

    #[link_name = "objc_msgSend$numberWithUnsignedChar:"]
    fn number_with_unsigned_char();

    #[link_name = "objc_msgSend$numberWithShort:"]
    fn number_with_short();

    #[link_name = "objc_msgSend$numberWithUnsignedShort:"]
    fn number_with_unsigned_short();

}

#[cfg(test)]
mod tests {
    use crate::{ns, objc::Obj};
    #[test]
    fn basics() {
        let i8 = ns::Number::tagged_i8(3);
        let u8 = ns::Number::tagged_u8(3);
        assert!(i8.is_tagged_pointer());
        assert!(u8.is_tagged_pointer());
    }
}
