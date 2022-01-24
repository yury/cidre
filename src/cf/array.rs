use super::{AllocatorRef, Index, StringRef, TypeID, TypeRef};
use std::{ffi::c_void, ptr::NonNull, ops::{Deref, DerefMut}};

///'''
/// use cider::cf;
/// assert_eq!(cf::array_get_type_id(), 21);
///```
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

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ArrayRef(TypeRef);

#[derive(Clone)]
#[repr(transparent)]
pub struct Array(ArrayRef);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MutableArrayRef(ArrayRef);

impl ArrayRef {
    #[inline]
    pub fn retained(&self) -> Array {
        unsafe { Array(self.retain()) }
    }
    
    #[inline]
    pub unsafe fn retain(&self) -> ArrayRef {
        ArrayRef(self.0.retain())
    }

    #[inline]
    pub fn create_copy(&self, allocator: Option<AllocatorRef>) -> Option<Array> {
        unsafe { CFArrayCreateCopy(allocator, *self) }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let arr = cf::ArrayRef::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn get_count(&self) -> Index {
        unsafe { CFArrayGetCount(*self) }
    }

    ///```
    /// use cidre::cf;
    ///
    /// let arr = cf::ArrayRef::create(None, None, 0, None).expect("arr");
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.get_count() as _
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

impl Array {}

impl Drop for Array {
    #[inline]
    fn drop(&mut self) {
        self.release()
    }
}

impl Deref for ArrayRef {
    type Target = TypeRef;

    #[inline]
    fn deref(&self) -> &TypeRef {
        &self.0
    }
}

impl Deref for Array {
    type Target = ArrayRef;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Array {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for ArrayRef {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
    fn CFArrayGetCount(theArray: ArrayRef) -> Index;
}
