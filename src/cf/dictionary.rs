use crate::define_cf_type;

use super::{Allocator, HashCode, Index, Retained, String, Type, TypeID};
use std::{ffi::c_void, ptr::NonNull};

pub type DictionaryRetainCallBack =
    extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type DictionaryReleaseCallBack =
    extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type DictionaryCopyDescriptionCallBack =
    extern "C" fn(value: *const c_void) -> Option<Retained<String>>;
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
    pub fn default() -> &'static DictionaryKeyCallBacks {
        unsafe { &kCFTypeDictionaryKeyCallBacks }
    }

    #[inline]
    pub fn copy_strings() -> &'static DictionaryKeyCallBacks {
        unsafe { &kCFCopyStringDictionaryKeyCallBacks }
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

define_cf_type!(Dictionary(Type));

impl Dictionary {
    /// ```
    /// use cidre::cf;
    /// assert_eq!(cf::Dictionary::type_id(), 18);
    /// ```
    #[inline]
    pub fn type_id() -> TypeID {
        unsafe { CFDictionaryGetTypeID() }
    }

    #[inline]
    pub unsafe fn contains_key(&self, key: *const c_void) -> bool {
        CFDictionaryContainsKey(self, key)
    }

    #[inline]
    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        CFDictionaryContainsValue(self, value)
    }

    #[inline]
    pub unsafe fn get_value(&self, key: *const c_void) -> Option<NonNull<*const c_void>> {
        CFDictionaryGetValue(self, key)
    }

    #[inline]
    pub unsafe fn get_value_if_present(
        &self,
        key: *const c_void,
    ) -> Option<Option<NonNull<*const c_void>>> {
        let mut value = Option::None;

        if CFDictionaryGetValueIfPresent(self, key, &mut value) {
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_count(&self) -> Index {
        unsafe { CFDictionaryGetCount(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.get_count() as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.get_count() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10).unwrap();
    /// let value = cf::Number::from_i8(20).unwrap();
    ///
    /// let d = cf::Dictionary::create_with_type_refs(&[&key], &[&value]).unwrap();
    ///
    /// assert!(!d.is_empty());
    ///
    pub fn create_with_type_refs(keys: &[&Type], values: &[&Type]) -> Option<Retained<Dictionary>> {
        Self::create(
            None,
            keys.as_ptr() as _,
            values.as_ptr() as _,
            std::cmp::min(keys.len(), values.len()) as _,
            None,
            None,
        )
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let dict = cf::Dictionary::create(None, std::ptr::null(), std::ptr::null(), 0, None, None).unwrap();
    ///
    /// dict.show();
    /// ```
    #[inline]
    pub fn create(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&DictionaryKeyCallBacks>,
        value_callbacks: Option<&DictionaryValueCallBacks>,
    ) -> Option<Retained<Dictionary>> {
        unsafe {
            CFDictionaryCreate(
                allocator,
                keys,
                values,
                num_values,
                key_callbacks,
                value_callbacks,
            )
        }
    }
}

extern "C" {
    static kCFTypeDictionaryKeyCallBacks: DictionaryKeyCallBacks;
    static kCFCopyStringDictionaryKeyCallBacks: DictionaryKeyCallBacks;

    static kCFTypeDictionaryValueCallBacks: DictionaryValueCallBacks;

    fn CFDictionaryGetTypeID() -> TypeID;

    fn CFDictionaryContainsKey(the_dict: &Dictionary, key: *const c_void) -> bool;
    fn CFDictionaryContainsValue(the_dict: &Dictionary, value: *const c_void) -> bool;

    fn CFDictionaryGetCount(the_dict: &Dictionary) -> Index;
    fn CFDictionaryGetValue(
        the_dict: &Dictionary,
        key: *const c_void,
    ) -> Option<NonNull<*const c_void>>;
    fn CFDictionaryGetValueIfPresent(
        the_dict: &Dictionary,
        key: *const c_void,
        value: *mut Option<NonNull<*const c_void>>,
    ) -> bool;

    fn CFDictionaryCreate(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&DictionaryKeyCallBacks>,
        value_callbacks: Option<&DictionaryValueCallBacks>,
    ) -> Option<Retained<Dictionary>>;

}
