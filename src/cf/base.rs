use crate::define_cf_type;

use super::{
    runtime::{Retained, Type},
    String,
};
use std::{cmp::Ordering, ffi::c_void, intrinsics::transmute};

pub type Index = isize;
pub type TypeId = usize;
pub type HashCode = usize;

pub type OptionFlags = usize;

pub type ComparatorFunction = extern "C" fn(
    val1: *const c_void,
    val2: *const c_void,
    context: *mut c_void,
) -> ComparisonResult;

pub const NOT_FOUND: Index = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Range {
    pub location: Index,
    pub length: Index,
}

impl Range {
    #[inline]
    pub fn new(location: Index, length: Index) -> Self {
        Self { location, length }
    }
}

#[repr(isize)]
pub enum ComparisonResult {
    LessThen = -1,
    EqualTo = 0,
    GreaterThen = 1,
}

/// ```
/// use cidre::cf;
///
/// let s = cf::Number::from_i32(10);//from_static_string("hello").unwrap();
/// let t = s.get_type_id();
/// unsafe {
///     let desc = cf::copy_type_id_description(t).unwrap();
///     assert_eq!("CFNumber", desc.to_string());
/// }
/// ```
pub unsafe fn copy_type_id_description<'a>(type_id: TypeId) -> Option<Retained<'a, String>> {
    CFCopyTypeIDDescription(type_id)
}

impl From<ComparisonResult> for Ordering {
    /// ```
    /// use cidre::cf::ComparisonResult as CR;
    /// use core::cmp::Ordering as O;
    ///
    /// let less: O = CR::LessThen.into();
    /// let equal: O = CR::EqualTo.into();
    /// let greater: O = CR::GreaterThen.into();
    ///
    /// assert_eq!(less, O::Less);
    /// assert_eq!(equal, O::Equal);
    /// assert_eq!(greater, O::Greater);
    /// ```
    #[inline]
    fn from(o: ComparisonResult) -> Self {
        unsafe { transmute(o as i8) }
    }
}

define_cf_type!(Null(Type));

impl Type {
    #[inline]
    pub fn show(&self) {
        unsafe { CFShow(Some(self)) }
    }

    #[inline]
    pub fn allocator(&self) -> Option<&Allocator> {
        unsafe { CFGetAllocator(self) }
    }

    #[inline]
    pub fn retain_count(&self) -> isize {
        unsafe { CFGetRetainCount(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let n1 = cf::Number::from_i8(4);
    /// let n2 = cf::Number::from_i32(4);
    /// let n3 = cf::Number::from_f64(3.0).unwrap();
    ///
    /// assert!(n1.equal(&n2));
    /// assert_eq!(false, n1.equal(&n3));
    /// ```
    #[inline]
    pub fn equal(&self, other: &Type) -> bool {
        unsafe { CFEqual(self, other) }
    }

    #[inline]
    pub fn hash(&self) -> usize {
        unsafe { CFHash(self) }
    }

    #[inline]
    pub fn copy_description(&self) -> Option<Retained<String>> {
        unsafe { CFCopyDescription(Some(self)) }
    }
}

impl Null {
    #[inline]
    pub fn value() -> &'static Null {
        unsafe { kCFNull }
    }
}

define_cf_type!(Allocator(Type));

impl Allocator {
    #[inline]
    pub fn default() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorDefault }
    }

    #[inline]
    pub fn system_default() -> &'static Allocator {
        unsafe { kCFAllocatorSystemDefault }
    }

    #[inline]
    pub fn null() -> &'static Allocator {
        unsafe { kCFAllocatorNull }
    }
}

extern "C" {
    fn CFCopyTypeIDDescription<'a>(type_id: TypeId) -> Option<Retained<'a, String>>;

    static kCFNull: &'static Null;

    static kCFAllocatorDefault: Option<&'static Allocator>;
    static kCFAllocatorSystemDefault: &'static Allocator;
    static kCFAllocatorNull: &'static Allocator;

    fn CFGetAllocator<'a>(cf: &Type) -> Option<&'a Allocator>;
    fn CFShow(cf: Option<&Type>);
    fn CFGetRetainCount(cf: &Type) -> Index;

    fn CFHash(cf: &Type) -> usize;

    fn CFEqual(cf1: &Type, cf2: &Type) -> bool;
    fn CFCopyDescription<'a>(cf: Option<&Type>) -> Option<Retained<'a, String>>;
}

define_cf_type!(PropertyList(Type));
