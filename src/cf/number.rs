use crate::define_cf_type;

use super::{Allocator, ComparisonResult, Index, Retained, Type, TypeID};

use std::{convert::From, ffi::c_void};

define_cf_type!(Boolean(Type));

impl Boolean {
    /// ```
    /// use cidre::cf;
    ///
    /// assert_eq!(cf::Boolean::type_id(), 21);
    /// ```
    #[inline]
    pub fn type_id() -> TypeID {
        unsafe { CFBooleanGetTypeID() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let f = cf::Boolean::value_true();
    /// assert_eq!(true, f.get_value());
    /// ```
    #[inline]
    pub fn get_value(&self) -> bool {
        unsafe { CFBooleanGetValue(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let f = cf::Boolean::value_true();
    /// assert_eq!(true, f.get_value());
    /// ```
    #[inline]
    pub fn value_true() -> &'static Boolean {
        unsafe { kCFBooleanTrue }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let f = cf::Boolean::value_false();
    /// assert_eq!(false, f.get_value());
    /// ```
    #[inline]
    pub fn value_false() -> &'static Boolean {
        unsafe { kCFBooleanFalse }
    }
}

impl std::fmt::Debug for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.get_value()))
    }
}

impl From<Boolean> for bool {
    #[inline]
    fn from(cf: Boolean) -> Self {
        cf.get_value()
    }
}

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
    #[inline]
    pub fn type_id() -> TypeID {
        unsafe { CFNumberGetTypeID() }
    }

    #[inline]
    pub fn get_number_type(&self) -> NumberType {
        unsafe { CFNumberGetType(self) }
    }

    #[inline]
    pub fn get_byte_size(&self) -> Index {
        unsafe { CFNumberGetByteSize(self) }
    }

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
    /// let num = cf::Number::from_i8(-5).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I8);
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
    /// let num = cf::Number::from_i16(-5).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I16);
    /// assert_eq!(num.to_i8().unwrap(), -5i8);
    /// assert_eq!(num.to_i16().unwrap(), -5i16);
    /// assert_eq!(num.to_i32().unwrap(), -5i32);
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
    /// let num = cf::Number::from_i32(-5).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I32);
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
}

impl Number {
    #[inline]
    pub fn positive_inifinity() -> &'static Number {
        unsafe { kCFNumberPositiveInfinity }
    }

    #[inline]
    pub fn negative_inifinity() -> &'static Number {
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
    pub fn nan() -> &'static Number {
        unsafe { kCFNumberNaN }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = unsafe { cf::Number::create(None, cf::NumberType::I8, &5i8 as *const i8 as *const std::ffi::c_void).unwrap() };
    /// assert_eq!(num.get_number_type(), cf::NumberType::I8);
    /// assert_eq!(num.to_i8().unwrap(), 5i8);
    /// ```
    #[inline]
    pub unsafe fn create(
        allocator: Option<&Allocator>,
        the_type: NumberType,
        value_ptr: *const c_void,
    ) -> Option<Retained<Number>> {
        CFNumberCreate(allocator, the_type, value_ptr)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i8(8).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I8);
    /// assert_eq!(1, num.get_byte_size());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i8<'a>(value: i8) -> Option<Retained<'a, Number>> {
        unsafe { Number::create(None, NumberType::I8, &value as *const _ as _) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i16(16).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I16);
    /// assert_eq!(2, num.get_byte_size());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i16<'a>(value: i16) -> Option<Retained<'a, Number>> {
        unsafe { Number::create(None, NumberType::I16, &value as *const _ as _) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(32).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I32);
    /// assert_eq!(4, num.get_byte_size());
    /// assert_eq!(32, num.to_i32().unwrap());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i32<'a>(value: i32) -> Option<Retained<'a, Number>> {
        unsafe { Number::create(None, NumberType::I32, &value as *const _ as _) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i64(64).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I64);
    /// assert_eq!(8, num.get_byte_size());
    /// assert_eq!(64, num.to_i64().unwrap());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i64<'a>(value: i64) -> Option<Retained<'a, Number>> {
        unsafe { Number::create(None, NumberType::I64, &value as *const _ as _) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_f64(64.0).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::F64);
    /// assert_eq!(8, num.get_byte_size());
    /// assert_eq!(64.0, num.to_f64().unwrap());
    /// assert_eq!(true, num.is_float_type());
    /// ```
    pub fn from_f64<'a>(value: f64) -> Option<Retained<'a, Number>> {
        unsafe { Number::create(None, NumberType::F64, &value as *const _ as _) }
    }
}

extern "C" {
    fn CFBooleanGetTypeID() -> TypeID;
    static kCFBooleanTrue: &'static Boolean;
    static kCFBooleanFalse: &'static Boolean;

    fn CFBooleanGetValue(boolean: &Boolean) -> bool;

    fn CFNumberGetTypeID() -> TypeID;

    static kCFNumberPositiveInfinity: &'static Number;
    static kCFNumberNegativeInfinity: &'static Number;
    static kCFNumberNaN: &'static Number;

    fn CFNumberCreate(
        allocator: Option<&Allocator>,
        the_type: NumberType,
        value_ptr: *const c_void,
    ) -> Option<Retained<Number>>;

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
