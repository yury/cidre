use crate::{arc, cf, define_cf_type, FourCharCode};

#[cfg(feature = "ns")]
use crate::ns;

use super::{Allocator, ComparisonResult, Index, Type, TypeId};

use std::{convert::From, ffi::c_void, time::Duration};

define_cf_type!(Boolean(Type));

impl Boolean {
    /// ```
    /// use cidre::cf;
    ///
    /// assert_eq!(cf::Boolean::type_id(), cf::Boolean::value_true().get_type_id());
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFBooleanGetTypeID() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let f = cf::Boolean::value_true();
    /// assert_eq!(true, f.value());
    /// ```
    #[inline]
    pub fn value(&self) -> bool {
        unsafe { CFBooleanGetValue(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let f = cf::Boolean::value_true();
    /// assert_eq!(true, f.value());
    /// ```
    #[inline]
    pub fn value_true() -> &'static Boolean {
        unsafe { kCFBooleanTrue }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let f = cf::Boolean::value_false();
    /// assert_eq!(false, f.value());
    /// ```
    #[inline]
    pub fn value_false() -> &'static Boolean {
        unsafe { kCFBooleanFalse }
    }

    #[inline]
    pub fn as_prop_list(&self) -> &cf::PropertyList {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<Boolean> for bool {
    #[inline]
    fn from(cf: Boolean) -> Self {
        cf.value()
    }
}

impl From<bool> for &'static Boolean {
    #[inline]
    fn from(value: bool) -> Self {
        if value {
            Boolean::value_true()
        } else {
            Boolean::value_false()
        }
    }
}

impl PartialEq<bool> for &Boolean {
    /// ```
    /// use cidre::cf;
    ///
    /// assert!(cf::Boolean::value_true() == true);
    /// assert!(cf::Boolean::value_false() == false);
    /// ```
    fn eq(&self, other: &bool) -> bool {
        self.value().eq(other)
    }
}

#[doc(alias = "CFNumberType")]
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NumberType(Index);

impl NumberType {
    pub const I8: Self = Self(1);
    pub const I16: Self = Self(2);
    pub const I32: Self = Self(3);
    pub const I64: Self = Self(4);
    pub const F32: Self = Self(5);
    pub const F64: Self = Self(6);
    pub const CHAR: Self = Self(7);
    pub const SHORT: Self = Self(8);
    pub const INT: Self = Self(9);
    pub const LONG: Self = Self(10);
    pub const LONG_LONG: Self = Self(11);
    pub const FLOAT: Self = Self(12);
    pub const DOUBLE: Self = Self(13);
    pub const CF_INDEX: Self = Self(14);
    pub const NS_INTEGER: Self = Self(15);
    pub const CG_FLOAT: Self = Self(16);
    pub const MAX: Self = Self(16);
}

define_cf_type!(Number(Type));

impl Number {
    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i16(10);
    ///
    /// assert_eq!(num.get_type_id(), cf::Number::type_id());
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFNumberGetTypeID() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(-5);
    /// assert_eq!(num.number_type(), cf::NumberType::I32);
    /// ```
    #[inline]
    pub fn number_type(&self) -> NumberType {
        unsafe { CFNumberGetType(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(-5);
    /// assert_eq!(4, num.byte_size());
    /// ```
    #[inline]
    pub fn byte_size(&self) -> Index {
        unsafe { CFNumberGetByteSize(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(-5);
    /// assert_eq!(false, num.is_float_type());
    /// ```
    #[inline]
    pub fn is_float_type(&self) -> bool {
        unsafe { CFNumberIsFloatType(self) }
    }

    #[inline]
    pub fn compare(&self, other_number: &Self, context: &mut c_void) -> ComparisonResult {
        unsafe { CFNumberCompare(self, other_number, context) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i8(-5);
    /// assert_eq!(num.number_type(), cf::NumberType::I8);
    /// assert_eq!(num.to_i8().unwrap(), -5i8);
    /// assert_eq!(num.to_i16().unwrap(), -5i16);
    /// assert_eq!(num.to_i32().unwrap(), -5i32);
    /// ```
    #[inline]
    pub fn to_i8(&self) -> Option<i8> {
        unsafe {
            let mut value: i8 = 0;
            if CFNumberGetValue(self, NumberType::I8, &mut value as *mut _ as *mut _) {
                Some(value)
            } else {
                None
            }
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i16(-5);
    /// assert_eq!(num.number_type(), cf::NumberType::I16);
    /// assert_eq!(num.to_i8().unwrap(), -5i8);
    /// assert_eq!(num.to_i16().unwrap(), -5i16);
    /// assert_eq!(num.to_i32().unwrap(), -5i32);
    /// assert_eq!(num.to_i64().unwrap(), -5i64);
    /// ```
    #[inline]
    pub fn to_i16(&self) -> Option<i16> {
        unsafe {
            let mut value: i16 = 0;
            if CFNumberGetValue(self, NumberType::I16, &mut value as *mut _ as *mut _) {
                Some(value)
            } else {
                None
            }
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(-5);
    /// assert_eq!(num.number_type(), cf::NumberType::I32);
    /// assert_eq!(num.to_i8().unwrap(), -5i8);
    /// assert_eq!(num.to_i16().unwrap(), -5i16);
    /// assert_eq!(num.to_i32().unwrap(), -5i32);
    /// ```
    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        unsafe {
            let mut value: i32 = 0;
            if CFNumberGetValue(self, NumberType::I32, &mut value as *mut _ as *mut _) {
                Some(value)
            } else {
                None
            }
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(-5);
    /// assert_eq!(num.number_type(), cf::NumberType::I32);
    /// assert_eq!(num.to_i64().unwrap(), -5i64);
    /// ```
    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        unsafe {
            let mut value: i64 = 0;
            if CFNumberGetValue(self, NumberType::I64, &mut value as *mut _ as *mut _) {
                Some(value)
            } else {
                None
            }
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_f64(-5.0f64);
    /// assert_eq!(num.number_type(), cf::NumberType::F64);
    /// assert_eq!(num.to_f64().unwrap(), -5f64);
    /// assert_eq!(true, num.is_float_type());
    /// ```
    #[inline]
    pub fn to_f64(&self) -> Option<f64> {
        unsafe {
            let mut value: f64 = 0.0;
            if CFNumberGetValue(self, NumberType::F64, &mut value as *mut _ as *mut _) {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn positive_infinity() -> &'static Self {
        unsafe { kCFNumberPositiveInfinity }
    }

    #[inline]
    pub fn negative_infinity() -> &'static Self {
        unsafe { kCFNumberNegativeInfinity }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let nan1 = cf::Number::nan();
    /// let nan2 = cf::Number::nan();
    /// assert_eq!(nan1.equal(&nan2), true);
    /// ```
    #[inline]
    pub fn nan() -> &'static Self {
        unsafe { kCFNumberNaN }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = unsafe { cf::Number::create_in(cf::NumberType::I8, &5i8 as *const i8 as *const std::ffi::c_void, None).unwrap() };
    /// assert_eq!(num.number_type(), cf::NumberType::I8);
    /// assert_eq!(num.to_i8().unwrap(), 5i8);
    /// ```
    #[inline]
    pub unsafe fn create_in(
        the_type: NumberType,
        value_ptr: *const c_void,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<Self>> {
        CFNumberCreate(allocator, the_type, value_ptr)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i8(8);
    /// assert_eq!(num.number_type(), cf::NumberType::I8);
    /// assert_eq!(1, num.byte_size());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    /// Will return tagged
    pub fn from_i8(value: i8) -> arc::R<Self> {
        unsafe { Self::create_in(NumberType::I8, &value as *const _ as _, None).unwrap_unchecked() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i16(16);
    /// assert_eq!(num.number_type(), cf::NumberType::I16);
    /// assert_eq!(2, num.byte_size());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    /// Will return tagged
    pub fn from_i16(value: i16) -> arc::R<Self> {
        unsafe {
            Self::create_in(NumberType::I16, &value as *const _ as _, None).unwrap_unchecked()
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(32);
    /// assert_eq!(num.number_type(), cf::NumberType::I32);
    /// assert_eq!(4, num.byte_size());
    /// assert_eq!(32, num.to_i32().unwrap());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    /// Will return tagged: see <https://opensource.apple.com/source/CF/CF-635/CFNumber.c.auto.html>
    #[inline]
    pub fn from_i32(value: i32) -> arc::R<Self> {
        unsafe {
            Self::create_in(NumberType::I32, &value as *const _ as _, None).unwrap_unchecked()
        }
    }

    #[inline]
    pub fn tagged_i8(value: i8) -> &'static Self {
        unsafe {
            std::mem::transmute(Self::create_in(
                NumberType::I8,
                &value as *const _ as _,
                None,
            ))
        }
    }

    #[inline]
    pub fn tagged_i16(value: i16) -> &'static Self {
        unsafe {
            std::mem::transmute(Self::create_in(
                NumberType::I16,
                &value as *const _ as _,
                None,
            ))
        }
    }

    #[inline]
    pub fn tagged_i32(value: i32) -> &'static Self {
        unsafe {
            std::mem::transmute(Self::create_in(
                NumberType::I32,
                &value as *const _ as _,
                None,
            ))
        }
    }

    #[inline]
    pub fn from_four_char_code(value: FourCharCode) -> &'static Self {
        Self::tagged_i32(value as _)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i64(64);
    /// assert_eq!(num.number_type(), cf::NumberType::I64);
    /// assert_eq!(8, num.byte_size());
    /// assert_eq!(64, num.to_i64().unwrap());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    #[inline]
    pub fn from_i64(value: i64) -> arc::R<Self> {
        unsafe {
            Self::create_in(NumberType::I64, &value as *const _ as _, None).unwrap_unchecked()
        }
    }

    #[inline]
    pub fn from_usize(value: usize) -> arc::R<Self> {
        unsafe {
            Self::create_in(NumberType::I64, &value as *const _ as _, None).unwrap_unchecked()
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_f64(64.0);
    /// assert_eq!(num.number_type(), cf::NumberType::F64);
    /// assert_eq!(8, num.byte_size());
    /// assert_eq!(64.0, num.to_f64().unwrap());
    /// assert_eq!(true, num.is_float_type());
    /// ```
    #[inline]
    pub fn from_f64(value: f64) -> arc::R<Self> {
        unsafe {
            Self::create_in(NumberType::F64, &value as *const _ as _, None).unwrap_unchecked()
        }
    }

    /// ```
    /// use cidre::cf;
    /// use std::time::Duration;
    ///
    /// let num = cf::Number::from_duration(Duration::from_secs(1));
    /// assert_eq!(num.number_type(), cf::NumberType::F64);
    /// assert_eq!(8, num.byte_size());
    /// assert_eq!(1f64, num.to_f64().unwrap());
    /// ```
    #[inline]
    pub fn from_duration(value: Duration) -> arc::R<Self> {
        unsafe {
            Self::create_in(NumberType::F64, &value.as_secs_f64() as *const _ as _, None)
                .unwrap_unchecked()
        }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::Number {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<i8> for arc::R<Number> {
    #[inline]
    fn from(value: i8) -> Self {
        Number::from_i8(value)
    }
}

impl From<i16> for arc::R<Number> {
    #[inline]
    fn from(value: i16) -> Self {
        Number::from_i16(value)
    }
}

impl From<i32> for arc::R<Number> {
    #[inline]
    fn from(value: i32) -> Self {
        Number::from_i32(value)
    }
}

impl From<i64> for arc::R<Number> {
    #[inline]
    fn from(value: i64) -> Self {
        Number::from_i64(value)
    }
}

impl From<Duration> for arc::R<Number> {
    #[inline]
    fn from(value: Duration) -> Self {
        Number::from_duration(value)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFBooleanGetTypeID() -> TypeId;
    static kCFBooleanTrue: &'static Boolean;
    static kCFBooleanFalse: &'static Boolean;

    fn CFBooleanGetValue(boolean: &Boolean) -> bool;

    fn CFNumberGetTypeID() -> TypeId;

    static kCFNumberPositiveInfinity: &'static Number;
    static kCFNumberNegativeInfinity: &'static Number;
    static kCFNumberNaN: &'static Number;

    fn CFNumberCreate(
        allocator: Option<&Allocator>,
        the_type: NumberType,
        value_ptr: *const c_void,
    ) -> Option<arc::R<Number>>;

    fn CFNumberGetType(number: &Number) -> NumberType;
    fn CFNumberGetByteSize(number: &Number) -> Index;
    fn CFNumberIsFloatType(number: &Number) -> bool;
    fn CFNumberCompare(
        number: &Number,
        other_number: &Number,
        context: *mut c_void,
    ) -> ComparisonResult;
    fn CFNumberGetValue(number: &Number, the_type: NumberType, value_ptr: *mut c_void) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn tagged() {
        assert!(cf::Number::tagged_i8(10).is_tagged_ptr());
        assert!(cf::Number::tagged_i16(10).is_tagged_ptr());
        assert!(cf::Number::tagged_i32(10).is_tagged_ptr());
    }
}
