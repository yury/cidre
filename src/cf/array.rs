use crate::define_cf_type;

use super::{Allocator, Index, Retained, String, Type, TypeId};
use std::{ffi::c_void, ptr::NonNull};

pub type ArrayRetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ArrayReleaseCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ArrayCopyDescriptionCallBack =
    extern "C" fn(value: *const c_void) -> Option<Retained<'static, String>>;
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
    pub fn type_id() -> TypeId {
        unsafe { CFArrayGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new().expect("arr");
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
    /// let arr = cf::Array::new().expect("Array::new");
    /// assert_eq!(arr.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new().expect("Array::new");
    /// let arr2 = arr1.create_copy(None).expect("copy");
    /// unsafe {    
    ///     assert_ne!(arr1.as_ptr(), arr2.as_ptr());
    /// }
    /// ```
    #[inline]
    pub fn create_copy<'a>(&self, allocator: Option<&Allocator>) -> Option<Retained<'a, Array>> {
        unsafe { CFArrayCreateCopy(allocator, self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new().expect("Array::new");
    /// let arr2 = arr1.copy().expect("copy");
    /// unsafe {
    ///     assert_ne!(arr1.as_ptr(), arr2.as_ptr());
    /// }
    /// ```
    #[inline]
    pub fn copy<'a>(&self) -> Option<Retained<'a, Array>> {
        Self::create_copy(self, None)
    }

    #[inline]
    pub fn create<'a>(
        allocator: Option<&Allocator>,
        values: Option<NonNull<*const c_void>>,
        num_values: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Retained<'a, Array>> {
        unsafe { CFArrayCreate(allocator, values, num_values, callbacks) }
    }

    #[inline]
    pub fn new<'a>() -> Option<Retained<'a, Array>> {
        Self::create(None, None, 0, None)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10).unwrap();
    /// let arr = cf::Array::from_type_refs(&[&num, &num, &num]).unwrap();
    /// assert_eq!(3, arr.len());
    /// ```
    #[inline]
    pub fn from_type_refs<'a, const N: usize>(values: &[&Type; N]) -> Option<Retained<'a, Array>> {
        let vals = unsafe {
            let ptr = values.as_ptr() as *const *const c_void as _;
            NonNull::new_unchecked(ptr)
        };
        Array::create(None, Some(vals), N as _, None)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10).unwrap();
    ///
    /// let empty_arr = cf::Array::new().unwrap();
    /// let mut mut_arr = empty_arr.create_mutable_copy(None, 0).unwrap();
    ///
    ///
    /// mut_arr.append(&num);
    ///
    /// assert_eq!(1, mut_arr.len());
    /// assert_eq!(0, empty_arr.len());
    ///
    /// ```
    #[inline]
    pub fn create_mutable_copy<'a>(
        &self,
        allocator: Option<&Allocator>,
        capacity: Index,
    ) -> Option<Retained<'a, MutableArray>> {
        unsafe { CFArrayCreateMutableCopy(allocator, capacity, self) }
    }

    #[inline]
    pub fn mutable_copy<'a>(&self) -> Option<Retained<'a, MutableArray>> {
        unsafe { CFArrayCreateMutableCopy(None, self.get_count(), self) }
    }
}

define_cf_type!(MutableArray(Array));

impl MutableArray {
    #[inline]
    pub unsafe fn append_value(&mut self, value: *const c_void) {
        CFArrayAppendValue(self, value)
    }

    #[inline]
    pub fn append(&mut self, value: &Type) {
        unsafe { self.append_value(value.as_ptr()) }
    }

    #[inline]
    pub fn remove_all_values(&mut self) {
        unsafe {
            CFArrayRemoveAllValues(self);
        }
    }

    #[inline]
    pub fn create<'a>(
        allocator: Option<&Allocator>,
        capacity: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Retained<'a, MutableArray>> {
        unsafe { CFArrayCreateMutable(allocator, capacity, callbacks) }
    }

    #[inline]
    pub fn with_capacity<'a>(capacity: Index) -> Option<Retained<'a, MutableArray>> {
        Self::create(None, capacity, None)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let mut arr = cf::MutableArray::new().unwrap();
    /// assert_eq!(0, arr.len());
    ///
    /// let num = cf::Number::from_i32(0).unwrap();
    ///
    /// arr.append(&num);
    /// arr.append(&num);
    /// assert_eq!(2, arr.len());
    ///
    /// arr.remove_all_values();
    /// assert_eq!(0, arr.len());
    /// ```
    #[inline]
    pub fn new<'a>() -> Option<Retained<'a, MutableArray>> {
        Self::with_capacity(0)
    }
}

extern "C" {
    static kCFTypeArrayCallBacks: ArrayCallbacks;

    fn CFArrayGetTypeID() -> TypeId;

    fn CFArrayCreate<'a>(
        allocator: Option<&Allocator>,
        values: Option<NonNull<*const c_void>>,
        num_values: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Retained<'a, Array>>;

    fn CFArrayCreateCopy<'a>(
        allocator: Option<&Allocator>,
        the_array: &Array,
    ) -> Option<Retained<'a, Array>>;

    fn CFArrayGetCount(the_array: &Array) -> Index;

    fn CFArrayCreateMutable<'a>(
        allocator: Option<&Allocator>,
        capacity: Index,
        callbacks: Option<&ArrayCallbacks>,
    ) -> Option<Retained<'a, MutableArray>>;
    fn CFArrayCreateMutableCopy<'a>(
        allocator: Option<&Allocator>,
        capacity: Index,
        the_array: &Array,
    ) -> Option<Retained<'a, MutableArray>>;
    fn CFArrayAppendValue(the_array: &mut MutableArray, value: *const c_void);
    fn CFArrayRemoveAllValues(the_array: &mut MutableArray);
}
