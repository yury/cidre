use crate::{define_cf_type, define_options};

use super::{
    runtime::{Retained, Type},
    String,
};
use std::{cmp::Ordering, ffi::c_void, fmt::Debug, intrinsics::transmute};

pub type Index = isize;
pub type TypeId = usize;
pub type HashCode = usize;

define_options!(OptionFlags(usize));

impl OptionFlags {
    pub const NONE: Self = Self(0);
}


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

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self
            .copy_description()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "no desc".to_string());
        f.debug_tuple("cf::Type").field(&desc).finish()
    }
}

impl Null {
    #[inline]
    pub fn value() -> &'static Null {
        unsafe { kCFNull }
    }
}

define_cf_type!(Allocator(Type));

/// Most of the time when specifying an allocator to Create functions, the None
/// argument indicates "use the default"; this is the same as using Allocator::default()
/// or the return value from CFAllocatorGetDefault().  This assures that you will use
/// the allocator in effect at that time.
impl Allocator {
    /// This is a synonym for NULL, if you'd rather use a named constant.
    #[inline]
    pub fn default() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorDefault }
    }

    /// Default system allocator; you rarely need to use this.
    #[inline]
    pub fn system_default() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorSystemDefault }
    }

    /// Null allocator which does nothing and allocates no memory. This allocator
    /// is useful as the `bytes_deallocator` in [`cf::Data`] or `contents_deallocator`
    /// in cf::String where the memory should not be freed.
    #[inline]
    pub fn null() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorNull }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFCopyTypeIDDescription<'a>(type_id: TypeId) -> Option<Retained<'a, String>>;

    static kCFNull: &'static Null;

    static kCFAllocatorDefault: Option<&'static Allocator>;
    static kCFAllocatorSystemDefault: Option<&'static Allocator>;
    static kCFAllocatorNull: Option<&'static Allocator>;

    fn CFGetAllocator<'a>(cf: &Type) -> Option<&'a Allocator>;
    fn CFShow(cf: Option<&Type>);
    fn CFGetRetainCount(cf: &Type) -> Index;

    fn CFHash(cf: &Type) -> usize;

    fn CFEqual(cf1: &Type, cf2: &Type) -> bool;
    fn CFCopyDescription<'a>(cf: Option<&Type>) -> Option<Retained<'a, String>>;
}

define_cf_type!(PropertyList(Type));

/// Type to mean any instance of a property list type;
/// currently, cf::String, cf::Data, cf::Number, cf::Boolean, cf::Date,
/// cf::Array, and cf::Dictionary.
impl PropertyList {
    pub fn try_as_string(&self) -> Option<&crate::cf::String> {
        if self.get_type_id() == crate::cf::String::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_string(&self) -> &crate::cf::String {
        assert!(self.get_type_id() == crate::cf::String::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_data(&self) -> Option<&crate::cf::Data> {
        if self.get_type_id() == crate::cf::Data::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_data(&self) -> &crate::cf::Data {
        assert!(self.get_type_id() == crate::cf::Data::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_number(&self) -> Option<&crate::cf::Number> {
        if self.get_type_id() == crate::cf::Number::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_number(&self) -> &crate::cf::Number {
        assert!(self.get_type_id() == crate::cf::Number::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_boolean(&self) -> Option<&crate::cf::Boolean> {
        if self.get_type_id() == crate::cf::Boolean::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> &crate::cf::Boolean {
        assert!(self.get_type_id() == crate::cf::Boolean::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_date(&self) -> Option<&crate::cf::Date> {
        if self.get_type_id() == crate::cf::Date::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_date(&self) -> &crate::cf::Date {
        assert!(self.get_type_id() == crate::cf::Date::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_array(&self) -> Option<&crate::cf::Array> {
        if self.get_type_id() == crate::cf::Date::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_array(&self) -> &crate::cf::Array {
        assert!(self.get_type_id() == crate::cf::Array::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_dictionary(&self) -> Option<&crate::cf::Dictionary> {
        if self.get_type_id() == crate::cf::Dictionary::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_dictionary(&self) -> &crate::cf::Dictionary {
        assert!(self.get_type_id() == crate::cf::Dictionary::type_id());
        unsafe { transmute(self) }
    }
}
