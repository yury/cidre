use super::{AllocatorRef, HashCode, Index, StringRef, TypeID, TypeRef};
use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// ```
/// use cidre::cf;
/// assert_eq!(cf::dictionary_get_type_id(), 18);
/// ```
#[inline]
pub fn dictionary_get_type_id() -> TypeID {
    unsafe { CFDictionaryGetTypeID() }
}

pub type DictionaryRetainCallBack =
    extern "C" fn(allocator: Option<AllocatorRef>, value: *const c_void);
pub type DictionaryReleaseCallBack =
    extern "C" fn(allocator: Option<AllocatorRef>, value: *const c_void);
pub type DictionaryCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> StringRef;
pub type DictionaryEqualCallBack =
    extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;
pub type DictionaryHashCallBack = extern "C" fn(value: *const c_void) -> HashCode;

#[repr(C)]
pub struct DictionaryKeyCallBacks {
    version: Index,
    retain: DictionaryRetainCallBack,
    release: DictionaryReleaseCallBack,
    copy_description: DictionaryCopyDescriptionCallBack,
    equal: DictionaryEqualCallBack,
    hash: DictionaryHashCallBack,
}

impl DictionaryKeyCallBacks {
    #[inline]
    pub fn default() -> Option<&'static DictionaryKeyCallBacks> {
        unsafe { Some(&kCFTypeDictionaryKeyCallBacks) }
    }

    #[inline]
    pub fn copy_strings() -> Option<&'static DictionaryKeyCallBacks> {
        unsafe { Some(&kCFCopyStringDictionaryKeyCallBacks) }
    }
}

#[repr(C)]
pub struct DictionaryValueCallBacks {
    version: Index,
    retain: DictionaryRetainCallBack,
    release: DictionaryReleaseCallBack,
    copy_description: DictionaryCopyDescriptionCallBack,
    equal: DictionaryEqualCallBack,
}

impl DictionaryValueCallBacks {
    #[inline]
    pub fn default() -> Option<&'static DictionaryValueCallBacks> {
        unsafe { Some(&kCFTypeDictionaryValueCallBacks) }
    }
}

pub type DictionaryApplierFunction =
    extern "C" fn(key: *const c_void, value: *const c_void, context: *mut c_void);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct DictionaryRef(TypeRef);

#[derive(Clone)]
#[repr(transparent)]
pub struct Dictionary(DictionaryRef);

impl DictionaryRef {
    #[inline]
    pub unsafe fn contains_key(&self, key: *const c_void) -> bool {
        CFDictionaryContainsKey(*self, key)
    }

    #[inline]
    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        CFDictionaryContainsValue(*self, value)
    }

    #[inline]
    pub unsafe fn get_value(&self, key: *const c_void) -> Option<NonNull<*const c_void>> {
        CFDictionaryGetValue(*self, key)
    }

    #[inline]
    pub unsafe fn get_value_if_present(
        &self,
        key: *const c_void,
    ) -> Option<Option<NonNull<*const c_void>>> {
            let mut value = Option::None;

            if CFDictionaryGetValueIfPresent(*self, key, &mut value) {
                Some(value)
            } else {
                None
            }
    }

    #[inline]
    pub fn get_count(&self) -> Index {
        unsafe { CFDictionaryGetCount(*self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.get_count() as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
      self.get_count() == 0
    }
}

impl Deref for DictionaryRef {
    type Target = TypeRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DictionaryRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Dictionary {
    type Target = DictionaryRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}

extern "C" {
    static kCFTypeDictionaryKeyCallBacks: DictionaryKeyCallBacks;
    static kCFCopyStringDictionaryKeyCallBacks: DictionaryKeyCallBacks;

    static kCFTypeDictionaryValueCallBacks: DictionaryValueCallBacks;

    fn CFDictionaryGetTypeID() -> TypeID;

    fn CFDictionaryContainsKey(the_dict: DictionaryRef, key: *const c_void) -> bool;
    fn CFDictionaryContainsValue(the_dict: DictionaryRef, value: *const c_void) -> bool;

    fn CFDictionaryGetCount(the_dict: DictionaryRef) -> Index;
    fn CFDictionaryGetValue(
        the_dict: DictionaryRef,
        key: *const c_void,
    ) -> Option<NonNull<*const c_void>>;
    fn CFDictionaryGetValueIfPresent(
        the_dict: DictionaryRef,
        key: *const c_void,
        value: *mut Option<NonNull<*const c_void>>,
    ) -> bool;
}
