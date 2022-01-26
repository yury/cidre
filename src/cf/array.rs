use crate::define_ref;

use super::{AllocatorRef, Index, StringRef, TypeID, TypeRef};
use std::{
    ffi::c_void,
    ptr::NonNull,
};

/// ```
/// use cidre::cf;
/// assert_eq!(cf::array_get_type_id(), 19);
/// ```
#[inline]
pub fn array_get_type_id() -> TypeID {
    unsafe { CFArrayGetTypeID() }
}

pub type ArrayRetainCallBack = extern "C" fn(allocator: Option<AllocatorRef>, value: *const c_void);
pub type ArrayReleaseCallBack =
    extern "C" fn(allocator: Option<AllocatorRef>, value: *const c_void);
pub type ArrayCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> StringRef;
pub type ArrayEqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;

#[repr(C)]
pub struct ArrayCallbacks {
    version: Index,
    retain: ArrayRetainCallBack,
    release: ArrayReleaseCallBack,
    copy_description: ArrayCopyDescriptionCallBack,
    equal: ArrayEqualCallBack,
}

impl ArrayCallbacks {
    #[inline]
    pub fn default() -> Option<&'static ArrayCallbacks> {
        unsafe { Some(&kCFTypeArrayCallBacks) }
    }
}

define_ref!(TypeRef, ArrayRef, Array);
define_ref!(ArrayRef, MutableArrayRef, MutableArray);

impl ArrayRef {

    #[inline]
    pub fn create_copy(&self, allocator: Option<AllocatorRef>) -> Option<Array> {
        unsafe { CFArrayCreateCopy(allocator, *self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::ArrayRef::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn get_count(&self) -> Index {
        unsafe { CFArrayGetCount(*self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::ArrayRef::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.get_count() as _
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::ArrayRef::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn create(
        allocator: Option<AllocatorRef>,
        values: Option<NonNull<*const *const c_void>>,
        num_values: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Array> {
        unsafe { CFArrayCreate(allocator, values, num_values, callbacks) }
    }
}

extern "C" {
    static kCFTypeArrayCallBacks: ArrayCallbacks;

    fn CFArrayGetTypeID() -> TypeID;

    fn CFArrayCreate(
        allocator: Option<AllocatorRef>,
        values: Option<NonNull<*const *const c_void>>,
        num_values: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Array>;
    fn CFArrayCreateCopy(allocator: Option<AllocatorRef>, the_array: ArrayRef) -> Option<Array>;
    fn CFArrayGetCount(the_array: ArrayRef) -> Index;
}
