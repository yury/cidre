use super::{AllocatorRef, ComparisonResult, Index, TypeID, TypeRef};
use std::{
    convert::From,
    ffi::c_void,
    ops::{Deref, DerefMut},
};

///```
/// use cidre::cf;
///
/// assert_eq!(cf::boolean_get_type_id(), 21);
///```
#[inline]
pub fn boolean_get_type_id() -> TypeID {
    unsafe { CFBooleanGetTypeID() }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct BooleanRef(TypeRef);

impl BooleanRef {
    ///```
    /// use cidre::cf;
    ///
    /// let f = cf::BooleanRef::value_true();
    /// assert_eq!(true, f.get_value());
    ///```
    #[inline]
    pub fn get_value(&self) -> bool {
        unsafe { CFBooleanGetValue(*self) }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let f = cf::BooleanRef::value_true();
    /// assert_eq!(true, f.get_value());
    ///```
    #[inline]
    pub fn value_true() -> BooleanRef {
        unsafe { kCFBooleanTrue }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let f = cf::BooleanRef::value_false();
    /// assert_eq!(false, f.get_value());
    ///```
    #[inline]
    pub fn value_false() -> BooleanRef {
        unsafe { kCFBooleanFalse }
    }
}

impl std::fmt::Debug for BooleanRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.get_value()))
    }
}

impl From<bool> for BooleanRef {
    ///```
    /// use cidre::cf;
    /// let f: cf::BooleanRef = true.into();
    /// assert_eq!(cf::BooleanRef::from(true), f);
    ///```
    #[inline]
    fn from(v: bool) -> Self {
        if v {
            Self::value_true()
        } else {
            Self::value_false()
        }
    }
}

impl From<BooleanRef> for bool {
    #[inline]
    fn from(cf: BooleanRef) -> Self {
        cf.get_value()
    }
}

#[inline]
pub fn number_get_type_id() -> TypeID {
    unsafe { CFNumberGetTypeID() }
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

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct NumberRef(TypeRef);

impl NumberRef {
    #[inline]
    pub fn retained(&self) -> Number {
        unsafe { Number(self.retain()) }
    }

    #[inline]
    pub unsafe fn retain(&self) -> NumberRef {
        NumberRef(self.0.retain())
    }

    #[inline]
    pub fn get_number_type(&self) -> NumberType {
        unsafe { CFNumberGetType(*self) }
    }

    #[inline]
    pub fn get_byte_size(&self) -> Index {
        unsafe { CFNumberGetByteSize(*self) }
    }

    #[inline]
    pub fn is_float_type(&self) -> bool {
        unsafe { CFNumberIsFloatType(*self) }
    }

    #[inline]
    pub fn compare(&self, other_number: &Self, context: &mut c_void) -> ComparisonResult {
        unsafe { CFNumberCompare(*self, *other_number, context) }
    }

    #[inline]
    pub fn positive_inifinity() -> NumberRef {
        unsafe { kCFNumberPositiveInfinity }
    }

    #[inline]
    pub fn negative_inifinity() -> NumberRef {
        unsafe { kCFNumberNegativeInfinity }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let nan1 = cf::NumberRef::nan();
    /// let nan2 = cf::NumberRef::nan();
    /// assert_eq!(nan1.equal(&nan2), true);
    ///```
    #[inline]
    pub fn nan() -> NumberRef {
        unsafe { kCFNumberNaN }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let num = cf::NumberRef::create(None, cf::NumberType::I8, &5i8 as *const i8 as *const std::ffi::c_void).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I8);
    /// assert_eq!(num.to_i8().unwrap(), 5i8);
    /// ```
    #[inline]
    pub fn create(
        allocator: Option<AllocatorRef>,
        the_type: NumberType,
        value_ptr: *const c_void,
    ) -> Option<Number> {
        unsafe { CFNumberCreate(allocator, the_type, value_ptr) }
    }

    #[inline]
    pub fn to_i8(&self) -> Option<i8> {
        unsafe {
            let mut value: i8 = 0;
            if CFNumberGetValue(*self, NumberType::I8, &mut value as *mut _ as *mut c_void) {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn to_i16(&self) -> Option<i16> {
        unsafe {
            let mut value: i16 = 0;
            if CFNumberGetValue(*self, NumberType::I16, &mut value as *mut _ as *mut c_void) {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        unsafe {
            let mut value: i32 = 0;
            if CFNumberGetValue(*self, NumberType::I32, &mut value as *mut _ as *mut c_void) {
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
            if CFNumberGetValue(*self, NumberType::I64, &mut value as *mut _ as *mut c_void) {
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
            if CFNumberGetValue(*self, NumberType::F64, &mut value as *mut _ as *mut c_void) {
                Some(value)
            } else {
                None
            }
        }
    }
}

#[repr(transparent)]
pub struct Number(NumberRef);

impl Number {
    ///```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i8(8).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I8);
    /// assert_eq!(1, num.get_byte_size());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i8(value: i8) -> Option<Number> {
        NumberRef::create(None, NumberType::I8, &value as *const _ as _)
    }

    ///```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i16(16).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I16);
    /// assert_eq!(2, num.get_byte_size());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i16(value: i16) -> Option<Number> {
        NumberRef::create(None, NumberType::I16, &value as *const _ as _)
    }

    ///```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(32).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I32);
    /// assert_eq!(4, num.get_byte_size());
    /// assert_eq!(32, num.to_i32().unwrap());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i32(value: i32) -> Option<Number> {
        NumberRef::create(None, NumberType::I32, &value as *const _ as _)
    }

    ///```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i64(64).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::I64);
    /// assert_eq!(8, num.get_byte_size());
    /// assert_eq!(64, num.to_i64().unwrap());
    /// assert_eq!(false, num.is_float_type());
    /// ```
    pub fn from_i64(value: i64) -> Option<Number> {
        NumberRef::create(None, NumberType::I64, &value as *const _ as _)
    }

    ///```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_f64(64.0).unwrap();
    /// assert_eq!(num.get_number_type(), cf::NumberType::F64);
    /// assert_eq!(8, num.get_byte_size());
    /// assert_eq!(64.0, num.to_f64().unwrap());
    /// assert_eq!(true, num.is_float_type());
    /// ```
    pub fn from_f64(value: f64) -> Option<Number> {
        NumberRef::create(None, NumberType::F64, &value as *const _ as _)
    }
}

impl Drop for Number {
    #[inline]
    fn drop(&mut self) {
        self.release()
    }
}

impl Deref for NumberRef {
    type Target = TypeRef;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NumberRef {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Number {
    type Target = NumberRef;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Number {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

extern "C" {
    fn CFBooleanGetTypeID() -> TypeID;
    static kCFBooleanTrue: BooleanRef;
    static kCFBooleanFalse: BooleanRef;

    fn CFBooleanGetValue(boolean: BooleanRef) -> bool;

    fn CFNumberGetTypeID() -> TypeID;

    static kCFNumberPositiveInfinity: NumberRef;
    static kCFNumberNegativeInfinity: NumberRef;
    static kCFNumberNaN: NumberRef;

    fn CFNumberCreate(
        allocator: Option<AllocatorRef>,
        the_type: NumberType,
        value_ptr: *const c_void,
    ) -> Option<Number>;

    fn CFNumberGetType(number: NumberRef) -> NumberType;
    fn CFNumberGetByteSize(number: NumberRef) -> Index;
    fn CFNumberIsFloatType(number: NumberRef) -> bool;
    fn CFNumberCompare(
        number: NumberRef,
        other_number: NumberRef,
        context: *mut c_void,
    ) -> ComparisonResult;
    fn CFNumberGetValue(number: NumberRef, the_type: NumberType, value_ptr: *mut c_void) -> bool;
}
