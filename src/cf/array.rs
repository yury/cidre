use crate::define_cf_type;

use super::{Allocator, Index, Retained, String, Type, TypeID};
use std::{ffi::c_void, ptr::NonNull};

pub type ArrayRetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ArrayReleaseCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ArrayCopyDescriptionCallBack =
    extern "C" fn(value: *const c_void) -> Option<Retained<String>>;
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

define_cf_type!(Array(Type));

impl Array {
    /// ```
    /// use cidre::cf;
    /// assert_eq!(cf::Array::type_id(), 19);
    /// ```
    #[inline]
    pub fn type_id() -> TypeID {
        unsafe { CFArrayGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn get_count(&self) -> Index {
        unsafe { CFArrayGetCount(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.get_count() as _
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

define_cf_type!(MutableArray(Array));

impl Array {
    #[inline]
    pub fn create_copy(&self, allocator: Option<&Allocator>) -> Option<Retained<Array>> {
        unsafe { CFArrayCreateCopy(allocator, self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let n = cf::Number::from_i32(10).unwrap();
    ///
    /// let arr = cf::Array::with_type_refs(&[&n, &n, &n]).unwrap();
    ///
    /// assert_eq!(3, arr.len());
    /// ```
    #[inline]
    pub fn with_type_refs(values: &[&Type]) -> Option<Retained<Array>> {
        let vals = unsafe {
            let ptr = values.as_ptr() as *const *const c_void as _;
            NonNull::new_unchecked(ptr)
        };
        Self::create(None, Some(vals), values.len() as _, None)
    }

    #[inline]
    pub fn create(
        allocator: Option<&Allocator>,
        values: Option<NonNull<*const c_void>>,
        num_values: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Retained<Array>> {
        unsafe { CFArrayCreate(allocator, values, num_values, callbacks) }
    }
}

extern "C" {
    static kCFTypeArrayCallBacks: ArrayCallbacks;

    fn CFArrayGetTypeID() -> TypeID;

    fn CFArrayCreate(
        allocator: Option<&Allocator>,
        values: Option<NonNull<*const c_void>>,
        num_values: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Retained<Array>>;
    fn CFArrayCreateCopy(
        allocator: Option<&Allocator>,
        the_array: &Array,
    ) -> Option<Retained<Array>>;
    fn CFArrayGetCount(the_array: &Array) -> Index;
}
