use crate::define_cf_type;

use super::{Allocator, HashCode, Index, Retained, String, Type, TypeId};
use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull};

pub type RetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ReleaseCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type CopyDescriptionCallBack =
    extern "C" fn(value: *const c_void) -> Option<Retained<'static, String>>;
pub type EqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;
pub type HashCallBack = extern "C" fn(value: *const c_void) -> HashCode;

#[repr(C)]
pub struct KeyCallBacks {
    version: Index,
    retain: RetainCallBack,
    release: ReleaseCallBack,
    copy_description: CopyDescriptionCallBack,
    equal: EqualCallBack,
    hash: HashCallBack,
}

impl KeyCallBacks {
    #[inline]
    pub fn default() -> &'static KeyCallBacks {
        unsafe { &kCFTypeDictionaryKeyCallBacks }
    }

    #[inline]
    pub fn copy_strings() -> &'static KeyCallBacks {
        unsafe { &kCFCopyStringDictionaryKeyCallBacks }
    }
}

#[repr(C)]
pub struct ValueCallBacks {
    version: Index,
    retain: RetainCallBack,
    release: ReleaseCallBack,
    copy_description: CopyDescriptionCallBack,
    equal: EqualCallBack,
}

impl ValueCallBacks {
    #[inline]
    pub fn default() -> Option<&'static ValueCallBacks> {
        unsafe { Some(&kCFTypeDictionaryValueCallBacks) }
    }
}

pub type ApplierFunction =
    extern "C" fn(key: *const c_void, value: *const c_void, context: *mut c_void);

define_cf_type!(Dictionary(Type));

impl Dictionary {
    /// ```
    /// use cidre::cf;
    ///
    /// let type_id = cf::Dictionary::type_id();
    /// assert_eq!(type_id, 18);
    ///
    /// unsafe {
    ///     let type_desc = cf::copy_type_id_description(type_id).unwrap();
    ///     assert_eq!("CFDictionary", type_desc.to_string());
    /// }
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFDictionaryGetTypeID() }
    }

    #[inline]
    pub unsafe fn contains_key(&self, key: *const c_void) -> bool {
        CFDictionaryContainsKey(self, key)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::from_pairs(&[&key], &[&value]).unwrap();
    ///
    /// assert!(d.contains_type_ref_key(&key));
    ///
    /// let key2 = cf::Number::from_i8(12);
    /// assert!(!d.contains_type_ref_key(&key2));
    /// ```
    #[inline]
    pub fn contains_type_ref_key(&self, key: &Type) -> bool {
        unsafe { CFDictionaryContainsKey(self, key.as_ptr()) }
    }

    #[inline]
    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        CFDictionaryContainsValue(self, value)
    }

    #[inline]
    pub unsafe fn get_value(&self, key: *const c_void) -> Option<NonNull<c_void>> {
        CFDictionaryGetValue(self, key)
    }

    #[inline]
    pub unsafe fn get_value_if_present(
        &self,
        key: *const c_void,
    ) -> Option<Option<NonNull<c_void>>> {
        let mut value = Option::None;

        if CFDictionaryGetValueIfPresent(self, key, &mut value) {
            Some(value)
        } else {
            None
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::from_pairs(&[&key], &[&value]).unwrap();
    ///
    /// let v = d.value_by_type_ref_key(&key).unwrap();
    /// assert!(v.equal(&value));
    /// unsafe {
    ///     assert_eq!(v.as_ptr(), value.as_ptr());
    /// }
    /// ```
    pub fn value_by_type_ref_key<'a>(&'a self, key: &Type) -> Option<&'a Type> {
        unsafe {
            let mut value = Option::None;
            if CFDictionaryGetValueIfPresent(self, key.as_ptr(), &mut value) {
                Some(transmute(value))
            } else {
                None
            }
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
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::from_pairs(&[&key], &[&value]).unwrap();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// ```
    #[inline]
    pub fn from_pairs<'a, const N: usize>(
        keys: &[&Type; N],
        values: &[&Type; N],
    ) -> Option<Retained<'a, Dictionary>> {
        unsafe {
            Self::create(
                None,
                keys.as_ptr() as _,
                values.as_ptr() as _,
                N as _,
                None,
                None,
            )
        }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let dict = unsafe { cf::Dictionary::create(None, std::ptr::null(), std::ptr::null(), 0, None, None).unwrap() };
    ///
    /// dict.show();
    /// ```
    #[inline]
    pub unsafe fn create<'a>(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&KeyCallBacks>,
        value_callbacks: Option<&ValueCallBacks>,
    ) -> Option<Retained<'a, Dictionary>> {
        CFDictionaryCreate(
            allocator,
            keys,
            values,
            num_values,
            key_callbacks,
            value_callbacks,
        )
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::from_pairs(&[&key], &[&value]).unwrap();
    ///
    /// let keys = d.get_keys();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// assert_eq!(1, keys.len());
    /// assert!(key.equal(keys[0]));
    /// ```
    pub fn get_keys(&self) -> Vec<&Type> {
        let len = self.len();
        let mut keys: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            keys.set_len(len);
            let keys = keys.as_ptr() as *const *const c_void;
            self.get_keys_and_values(keys, std::ptr::null());
        }
        keys
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::from_pairs(&[&key], &[&value]).unwrap();
    ///
    /// let vals = d.get_values();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// assert_eq!(1, vals.len());
    /// assert!(value.equal(vals[0]));
    /// ```
    pub fn get_values(&self) -> Vec<&Type> {
        let len = self.len();
        let mut values: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            values.set_len(len);
            let values = values.as_ptr() as *const *const c_void;
            self.get_keys_and_values(std::ptr::null(), values);
        }
        values
    }

    pub fn get_keys_with_values(&self) -> (Vec<&Type>, Vec<&Type>) {
        let len = self.len();
        let mut keys: Vec<&Type> = Vec::with_capacity(len);
        let mut values: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            keys.set_len(len);
            values.set_len(len);
            let keys = keys.as_ptr() as *const *const c_void;
            let values = values.as_ptr() as *const *const c_void;
            self.get_keys_and_values(keys, values);
        }
        (keys, values)
    }

    #[inline]
    pub unsafe fn get_keys_and_values(
        &self,
        keys: *const *const c_void,
        values: *const *const c_void,
    ) {
        CFDictionaryGetKeysAndValues(self, keys, values)
    }
}

define_cf_type!(MutableDictionary(Dictionary));

impl MutableDictionary {}

extern "C" {
    static kCFTypeDictionaryKeyCallBacks: KeyCallBacks;
    static kCFCopyStringDictionaryKeyCallBacks: KeyCallBacks;

    static kCFTypeDictionaryValueCallBacks: ValueCallBacks;

    fn CFDictionaryGetTypeID() -> TypeId;

    fn CFDictionaryContainsKey(the_dict: &Dictionary, key: *const c_void) -> bool;
    fn CFDictionaryContainsValue(the_dict: &Dictionary, value: *const c_void) -> bool;

    fn CFDictionaryGetCount(the_dict: &Dictionary) -> Index;
    fn CFDictionaryGetValue(the_dict: &Dictionary, key: *const c_void) -> Option<NonNull<c_void>>;
    fn CFDictionaryGetValueIfPresent(
        the_dict: &Dictionary,
        key: *const c_void,
        value: *mut Option<NonNull<c_void>>,
    ) -> bool;

    fn CFDictionaryCreate<'a>(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&KeyCallBacks>,
        value_callbacks: Option<&ValueCallBacks>,
    ) -> Option<Retained<'a, Dictionary>>;

    fn CFDictionaryGetKeysAndValues(
        the_dict: &Dictionary,
        keys: *const *const c_void,
        values: *const *const c_void,
    );

}
