use crate::{
    arc, define_cls, define_obj_type, ns,
    objc::{self, Class},
};

#[cfg(feature = "cf")]
use crate::cf;

define_obj_type!(
    #[doc(alias = "NSValue")]
    pub Value(ns::Id)
);

impl Value {
    define_cls!(NS_VALUE);

    #[objc::msg_send(isEqualToValue:)]
    pub fn eq_to_value(&self, other: &Self) -> bool;
}

define_obj_type!(
    #[doc(alias = "NSNumber")]
    pub Number(Value)
);

// initializers
impl arc::A<Number> {
    #[objc::msg_send(initWithChar:)]
    fn init_with_char(self, value: i8) -> arc::R<Number>;

    #[objc::msg_send(initWithUnsignedChar:)]
    fn init_with_unsigned_char(self, value: u8) -> arc::R<Number>;

    #[objc::msg_send(initWithShort:)]
    fn init_with_short(self, value: i16) -> arc::R<Number>;

    #[objc::msg_send(initWithUnsignedShort:)]
    fn init_with_unsinged_short(self, value: u16) -> arc::R<Number>;

    #[objc::msg_send(initWithInt:)]
    fn init_with_int(self, value: i32) -> arc::R<Number>;

    #[objc::msg_send(initWithUnsignedInt:)]
    fn init_with_unsigned_int(self, value: u32) -> arc::R<Number>;

    #[objc::msg_send(initWithLongLong:)]
    fn init_with_long_long(self, value: i64) -> arc::R<Number>;

    #[objc::msg_send(initWithUnsignedLongLong:)]
    fn init_with_unsigned_long_long(self, value: u64) -> arc::R<Number>;

    #[objc::msg_send(initWithFloat:)]
    fn init_with_float(self, value: f32) -> arc::R<Number>;

    #[objc::msg_send(initWithDouble:)]
    fn init_with_double(self, value: f64) -> arc::R<Number>;

    #[objc::msg_send(initWithBool:)]
    fn init_with_bool(self, value: bool) -> arc::R<Number>;

    #[objc::msg_send(initWithInteger:)]
    fn init_with_integer(self, value: ns::Integer) -> arc::R<Number>;

    #[objc::msg_send(initWithUnsignedInteger:)]
    fn init_with_unsigned_integer(self, value: ns::UInteger) -> arc::R<Number>;
}

impl Number {
    define_cls!(NS_NUMBER);

    #[inline]
    pub fn with_i8(value: i8) -> arc::R<Self> {
        Self::alloc().init_with_char(value)
    }

    #[inline]
    pub fn with_u8(value: u8) -> arc::R<Self> {
        Self::alloc().init_with_unsigned_char(value)
    }

    #[objc::msg_send(numberWithChar:)]
    pub fn tagged_i8(value: i8) -> &'static Self;

    #[objc::msg_send(numberWithUnsignedChar:)]
    pub fn tagged_u8(value: u8) -> &'static Self;

    #[inline]
    pub fn with_i16(value: i16) -> arc::R<Self> {
        Self::alloc().init_with_short(value)
    }

    #[inline]
    pub fn with_u16(value: u16) -> arc::R<Self> {
        Self::alloc().init_with_unsinged_short(value)
    }

    #[objc::msg_send(numberWithShort:)]
    pub fn tagged_i16(value: i16) -> &'static Self;

    #[objc::msg_send(numberWithUnsignedShort:)]
    pub fn tagged_u16(value: u16) -> &'static Self;

    #[inline]
    pub fn with_i32(value: i32) -> arc::R<Self> {
        Self::alloc().init_with_int(value)
    }

    #[inline]
    pub fn with_u32(value: u32) -> arc::R<Self> {
        Self::alloc().init_with_unsigned_int(value)
    }

    #[objc::msg_send(numberWithInt:)]
    pub fn tagged_i32(value: i32) -> &'static Self;

    #[objc::msg_send(numberWithUnsignedInt:)]
    pub fn tagged_u32(value: u32) -> &'static Self;

    #[inline]
    pub fn with_i64(value: i64) -> arc::R<Self> {
        Self::alloc().init_with_long_long(value)
    }

    // for benches
    #[inline]
    pub fn with_i64_ar_retain(value: i64) -> arc::R<Self> {
        arc::rar_retain(Self::with_i64_ar(value))
    }

    #[objc::msg_send(numberWithLongLong:)]
    pub fn with_i64_ar(value: i64) -> arc::Rar<Self>;

    #[inline]
    pub fn with_isize(value: isize) -> arc::R<Self> {
        Self::with_i64(value as _)
    }

    #[inline]
    pub fn with_u64(value: u64) -> arc::R<Self> {
        Self::alloc().init_with_unsigned_long_long(value)
    }

    #[inline]
    pub fn with_f32(value: f32) -> arc::R<Self> {
        Self::alloc().init_with_float(value)
    }

    #[inline]
    pub fn with_f64(value: f64) -> arc::R<Self> {
        Self::alloc().init_with_double(value)
    }

    #[inline]
    pub fn with_bool(value: bool) -> arc::R<Self> {
        Self::alloc().init_with_bool(value)
    }

    #[inline]
    pub fn with_integer(value: ns::Integer) -> arc::R<Self> {
        Self::alloc().init_with_integer(value)
    }

    #[must_use]
    #[inline]
    pub fn with_uinteger(value: ns::UInteger) -> arc::R<Self> {
        Self::alloc().init_with_unsigned_integer(value)
    }

    #[objc::msg_send(charValue)]
    pub fn as_i8(&self) -> i8;

    #[objc::msg_send(unsignedCharValue)]
    pub fn as_u8(&self) -> u8;

    #[objc::msg_send(shortValue)]
    pub fn as_i16(&self) -> i16;

    #[objc::msg_send(unsignedShortValue)]
    pub fn as_u16(&self) -> u16;

    #[objc::msg_send(intValue)]
    pub fn as_i32(&self) -> i32;

    #[objc::msg_send(unsignedIntValue)]
    pub fn as_u32(&self) -> u32;

    #[objc::msg_send(longLongValue)]
    pub fn as_i64(&self) -> i64;

    #[objc::msg_send(longLongValue)]
    pub fn as_isize(&self) -> isize;

    #[objc::msg_send(unsignedLongLongValue)]
    pub fn as_u64(&self) -> u64;

    #[objc::msg_send(unsignedLongLongValue)]
    pub fn as_usize(&self) -> usize;

    #[objc::msg_send(floatValue)]
    pub fn as_f32(&self) -> f32;

    #[objc::msg_send(doubleValue)]
    pub fn as_f64(&self) -> f64;

    #[objc::msg_send(integerValue)]
    pub fn as_integer(&self) -> ns::Integer;

    #[objc::msg_send(unsignedIntegerValue)]
    pub fn as_uinteger(&self) -> ns::UInteger;

    #[objc::msg_send(stringValue)]
    pub fn string(&self) -> arc::R<ns::String>;

    #[objc::msg_send(isEqualToNumber:)]
    pub fn eq_to_number(&self, number: &ns::Number) -> bool;

    #[cfg(feature = "cf")]
    #[inline]
    pub fn as_cf(&self) -> &cf::Number {
        unsafe { std::mem::transmute(self) }
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

impl Eq for Number {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_VALUE: &'static Class<ns::Value>;
    static NS_NUMBER: &'static Class<ns::Number>;
}

#[cfg(test)]
mod tests {
    use crate::{
        ns,
        objc::{ar_pool, Obj},
    };

    #[test]
    fn tagged_pointers() {
        ar_pool(|| {
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
        ar_pool(|| {
            let s = ar_pool(|| ns::Number::with_i32(10).string());
            println!("{:?}", s);
        });
        //        let foo = autoreleasepool(|| ns::Number::with_i64_ar(10));
    }

    #[test]
    fn basics() {
        let f = ns::Number::with_i8(10);

        assert_eq!(f.as_usize(), 10usize);
    }
}
