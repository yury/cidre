use std::{arch::asm, mem::transmute};

use crate::{
    arc, define_obj_type, ns,
    objc::{msg_send, Class, Obj},
};

define_obj_type!(Value(ns::Id));
define_obj_type!(Number(Value));

impl Number {
    #[inline]
    pub fn with_i8(value: i8) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_char, value) }
    }

    #[inline]
    pub fn with_u8(value: u8) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_unsigned_char, value) }
    }

    #[inline]
    pub fn tagged_i8(value: i8) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_char, value) }
    }

    #[inline]
    pub fn tagged_u8(value: u8) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_unsigned_char, value) }
    }

    #[inline]
    pub fn with_i16(value: i16) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_short, value) }
    }

    #[inline]
    pub fn with_u16(value: u16) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_unsigned_short, value) }
    }

    #[inline]
    pub fn tagged_i16(value: i16) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_short, value) }
    }

    #[inline]
    pub fn tagged_u16(value: u16) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_unsigned_short, value) }
    }

    #[inline]
    pub fn with_i32(value: i32) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_int, value) }
    }

    #[inline]
    pub fn with_u32(value: u32) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_unsigned_int, value) }
    }

    #[inline]
    pub fn tagged_i32(value: i32) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_int, value) }
    }

    #[inline]
    pub fn tagged_i32_alloc(value: i32) -> &'static Self {
        unsafe { NS_NUMBER.alloc().call1(init_with_int, value) }
    }

    #[inline]
    pub fn tagged_u32(value: u32) -> &'static Self {
        unsafe { NS_NUMBER.call1(number_with_unsigned_int, value) }
    }

    #[inline]
    pub fn with_i64(value: i64) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_integer, value) }
    }

    #[inline]
    pub fn with_i64_ar_retain(value: i64) -> arc::R<Self> {
        Self::with_i64_ar(value).retain()
    }

    #[inline]
    pub fn with_i64_ar(value: i64) -> arc::Rar<Self> {
        unsafe { NS_NUMBER.call1(number_with_integer, value) }
    }

    // just for benchmarks
    #[inline]
    pub fn with_i64_call(value: i64) -> arc::R<Self> {
        unsafe { NSNumber_numberWithInteger(value) }
    }

    #[inline]
    pub fn with_isize(value: isize) -> arc::R<Self> {
        Self::with_i64(value as _)
    }

    #[inline]
    pub fn with_u64(value: u64) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_unsigned_integer, value) }
    }

    #[inline]
    pub fn with_f32(value: f32) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_float, value) }
    }

    // #[inline]
    // pub fn tagged_f32(value: f32) -> &'static Self {
    //     unsafe { NS_NUMBER.call1(number_with_float, value) }
    // }

    #[inline]
    pub fn with_f64(value: f64) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_double, value) }
    }

    #[inline]
    pub fn with_bool(value: bool) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_bool, value) }
    }

    // #[inline]
    // pub fn tagged_bool(value: bool) -> &'static Self {
    //     unsafe { NS_NUMBER.call1(number_with_bool, value) }
    // }

    #[inline]
    pub fn with_integer(value: ns::Integer) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_integer, value) }
    }

    #[inline]
    pub fn with_uinteger(value: ns::UInteger) -> arc::R<Self> {
        unsafe { NS_NUMBER.alloc().call1(init_with_unsigned_integer, value) }
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
        unsafe { self.call1(is_equal_to_number, number) }
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

    static NS_NUMBER: &'static Class<ns::Number>;

    // just for benchmarks
    fn NSNumber_numberWithInteger(value: i64) -> arc::R<ns::Number>;
}

extern "C" {
    #[link_name = "objc_msgSend$numberWithChar:"]
    fn number_with_char();

    #[link_name = "objc_msgSend$initWithChar:"]
    fn init_with_char();

    #[link_name = "objc_msgSend$numberWithUnsignedChar:"]
    fn number_with_unsigned_char();

    #[link_name = "objc_msgSend$initWithUnsignedChar:"]
    fn init_with_unsigned_char();

    #[link_name = "objc_msgSend$numberWithShort:"]
    fn number_with_short();

    #[link_name = "objc_msgSend$initWithShort:"]
    fn init_with_short();

    #[link_name = "objc_msgSend$numberWithUnsignedShort:"]
    fn number_with_unsigned_short();

    #[link_name = "objc_msgSend$initWithUnsignedShort:"]
    fn init_with_unsigned_short();

    #[link_name = "objc_msgSend$numberWithInt:"]
    fn number_with_int();

    #[link_name = "objc_msgSend$initWithInt:"]
    fn init_with_int();

    #[link_name = "objc_msgSend$numberWithUnsignedInt:"]
    fn number_with_unsigned_int();

    #[link_name = "objc_msgSend$initWithUnsignedInt:"]
    fn init_with_unsigned_int();

    // #[link_name = "objc_msgSend$numberWithFloat:"]
    // fn number_with_float();

    #[link_name = "objc_msgSend$initWithFloat:"]
    fn init_with_float();

    #[link_name = "objc_msgSend$initWithDouble:"]
    fn init_with_double();

    // #[link_name = "objc_msgSend$numberWithBool:"]
    // fn number_with_bool();

    #[link_name = "objc_msgSend$initWithBool:"]
    fn init_with_bool();

    #[link_name = "objc_msgSend$numberWithInteger:"]
    fn number_with_integer();

    #[link_name = "objc_msgSend$initWithInteger:"]
    fn init_with_integer();

    #[link_name = "objc_msgSend$initWithUnsignedInteger:"]
    fn init_with_unsigned_integer();

    #[link_name = "objc_msgSend$isEqualToNumber:"]
    fn is_equal_to_number();
}

#[cfg(test)]
mod tests {
    use crate::{
        ns,
        objc::{autoreleasepool, Obj},
    };

    #[test]
    fn tagged_pointers() {
        autoreleasepool(|| {
            // let b = ns::Number::tagged_bool(true);
            let i8 = ns::Number::tagged_i8(i8::MAX - 1);
            let u8 = ns::Number::tagged_u8(u8::MAX - 1);
            let i16 = ns::Number::tagged_i16(i16::MAX - 1);
            let u16 = ns::Number::tagged_u16(u16::MAX - 1);
            let i32 = ns::Number::tagged_i32(i32::MAX - 1);
            let u32 = ns::Number::tagged_u32(u32::MAX - 1);
            // let f32 = ns::Number::tagged_f32(f32::MAX - 1f32);
            let i64 = ns::Number::with_i64(i64::MAX - 1);
            let i64_tagged = ns::Number::with_i64(1);

            // println!("{:?}", b.debug_description());
            // assert!(b.is_tagged_ptr());
            assert!(i8.is_tagged_ptr());
            assert!(u8.is_tagged_ptr());
            assert!(i16.is_tagged_ptr());
            assert!(u16.is_tagged_ptr());
            assert!(i32.is_tagged_ptr());
            assert!(u32.is_tagged_ptr());
            // assert!(f32.is_tagged_ptr());
            assert!(!i64.is_tagged_ptr());
            assert!(i64_tagged.is_tagged_ptr());

            let _s = i8.string();
            let _s = u8.string();
            let _s = i16.string();
        });
    }

    #[test]
    fn rar() {
        //        let foo = autoreleasepool(|| ns::Number::with_i64_ar(10));
    }
}
