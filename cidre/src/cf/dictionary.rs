#[cfg(feature = "ns")]
use crate::ns;

#[cfg(feature = "objc")]
use crate::objc;

use crate::{arc, cf, define_cf_type};
use cf::{Allocator, HashCode, Index, String, Type, TypeId};

use std::{ffi::c_void, marker, ptr};

pub type RetainCb = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ReleaseCb = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type CopyDescCb = extern "C" fn(value: *const c_void) -> Option<arc::R<String>>;
pub type EqualCb = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;
pub type HashCb = extern "C" fn(value: *const c_void) -> HashCode;

#[repr(C)]
pub struct KeyCbs {
    version: Index,
    retain: RetainCb,
    release: ReleaseCb,
    copy_description: CopyDescCb,
    equal: EqualCb,
    hash: HashCb,
}

impl KeyCbs {
    #[inline]
    pub fn default() -> Option<&'static KeyCbs> {
        unsafe { Some(&kCFTypeDictionaryKeyCallBacks) }
    }

    #[inline]
    pub fn copy_strings() -> Option<&'static KeyCbs> {
        unsafe { Some(&kCFCopyStringDictionaryKeyCallBacks) }
    }
}

#[repr(C)]
pub struct ValueCbs {
    version: Index,
    retain: RetainCb,
    release: ReleaseCb,
    copy_descr: CopyDescCb,
    equal: EqualCb,
}

impl ValueCbs {
    #[inline]
    pub fn default() -> Option<&'static ValueCbs> {
        unsafe { Some(&kCFTypeDictionaryValueCallBacks) }
    }
}

pub type ApplierFn = extern "C" fn(key: *const c_void, value: *const c_void, context: *mut c_void);

define_cf_type!(
    #[doc(alias = "CFDictionary")]
    Dictionary(Type)
);

impl Dictionary {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe {
            CFDictionaryCreate(
                allocator,
                std::ptr::null(),
                std::ptr::null(),
                0,
                KeyCbs::default(),
                ValueCbs::default(),
            )
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let type_id = cf::Dictionary::type_id();
    /// assert_eq!(type_id, 18);
    ///
    /// unsafe {
    ///     let type_desc = cf::type_id_desc(type_id).unwrap();
    ///     assert_eq!("CFDictionary", type_desc.to_string());
    /// }
    /// ```
    #[doc(alias = "CFDictionaryGetTypeID")]
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFDictionaryGetTypeID() }
    }

    /// try use contains_key first
    #[doc(alias = "CFDictionaryContainsKey")]
    #[inline]
    pub unsafe fn contains_raw_key(&self, key: *const c_void) -> bool {
        unsafe { CFDictionaryContainsKey(self, key) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// assert!(d.contains_key(&key));
    ///
    /// let key2 = cf::Number::from_i8(12);
    /// assert!(!d.contains_key(&key2));
    /// ```
    #[doc(alias = "CFDictionaryContainsKey")]
    #[inline]
    pub fn contains_key(&self, key: &Type) -> bool {
        unsafe { CFDictionaryContainsKey(self, key.as_type_ptr()) }
    }

    #[doc(alias = "CFDictionaryContainsValue")]
    #[inline]
    pub unsafe fn contains_raw_value(&self, val: *const c_void) -> bool {
        unsafe { CFDictionaryContainsValue(self, val) }
    }

    #[doc(alias = "CFDictionaryGetValue")]
    #[inline]
    pub unsafe fn raw_value(&self, key: *const c_void) -> Option<ptr::NonNull<c_void>> {
        unsafe { CFDictionaryGetValue(self, key) }
    }

    #[doc(alias = "CFDictionaryGetValueIfPresent")]
    #[inline]
    pub unsafe fn raw_value_if_present(&self, key: *const c_void) -> Option<ptr::NonNull<c_void>> {
        let mut value = None;

        if unsafe { CFDictionaryGetValueIfPresent(self, key, &mut value) } {
            value
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
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// let v = d.value(&key).unwrap();
    /// assert!(v.equal(&value));
    /// unsafe {
    ///     assert_eq!(v.as_type_ptr(), value.as_type_ptr());
    /// }
    /// ```
    pub fn value<'a>(&'a self, key: &Type) -> Option<&'a Type> {
        let mut value = Option::None;
        unsafe {
            if CFDictionaryGetValueIfPresent(self, key.as_type_ptr(), &mut value) {
                Some(std::mem::transmute(value))
            } else {
                None
            }
        }
    }

    #[doc(alias = "CFDictionaryGetCount")]
    #[inline]
    pub fn count(&self) -> Index {
        unsafe { CFDictionaryGetCount(self) }
    }

    #[doc(alias = "CFDictionaryGetCount")]
    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// ```
    #[inline]
    pub fn with_keys_values<const N: usize>(
        keys: &[&Type; N],
        values: &[&Type; N],
    ) -> Option<arc::R<Dictionary>> {
        unsafe {
            Self::create_in(
                keys.as_ptr() as _,
                values.as_ptr() as _,
                N as _,
                KeyCbs::default(),
                ValueCbs::default(),
                None,
            )
        }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let dict = unsafe { cf::Dictionary::create_in(std::ptr::null(), std::ptr::null(), 0, None, None, None).unwrap() };
    ///
    /// dict.show();
    /// ```
    #[inline]
    pub unsafe fn create_in(
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&KeyCbs>,
        value_callbacks: Option<&ValueCbs>,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<Dictionary>> {
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

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// let keys = d.keys();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// assert_eq!(1, keys.len());
    /// assert!(key.equal(keys[0]));
    /// ```
    pub fn keys(&self) -> Vec<&Type> {
        let len = self.len();
        let mut keys: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            let keys_ptr = keys.as_ptr() as *mut *const c_void;
            self.keys_and_values(keys_ptr, std::ptr::null_mut());
            keys.set_len(len);
        }
        keys
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let key = cf::Number::from_i8(10);
    /// let value = cf::Number::from_i8(20);
    ///
    /// let d = cf::Dictionary::with_keys_values(&[&key], &[&value]).unwrap();
    ///
    /// let vals = d.values();
    ///
    /// assert!(!d.is_empty());
    /// assert_eq!(1, d.len());
    /// assert_eq!(1, vals.len());
    /// assert!(value.equal(vals[0]));
    /// ```
    pub fn values(&self) -> Vec<&Type> {
        let len = self.len();
        let mut values: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            let values_ptr = values.as_ptr() as *mut *const c_void;
            self.keys_and_values(std::ptr::null_mut(), values_ptr);
            values.set_len(len);
        }
        values
    }

    pub fn keys_with_values(&self) -> (Vec<&Type>, Vec<&Type>) {
        let len = self.len();
        let mut keys: Vec<&Type> = Vec::with_capacity(len);
        let mut values: Vec<&Type> = Vec::with_capacity(len);
        unsafe {
            let keys_ptr = keys.as_ptr() as *mut *const c_void;
            let values_ptr = values.as_ptr() as *mut *const c_void;
            self.keys_and_values(keys_ptr, values_ptr);
            keys.set_len(len);
            values.set_len(len);
        }
        (keys, values)
    }

    #[doc(alias = "CFDictionaryGetKeysAndValues")]
    #[inline]
    pub unsafe fn keys_and_values(&self, keys: *mut *const c_void, values: *mut *const c_void) {
        unsafe { CFDictionaryGetKeysAndValues(self, keys, values) }
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { self.copy_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn copy_in(&self, allocator: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFDictionaryCreateCopy(allocator, self) }
    }
}

impl Default for arc::R<Dictionary> {
    fn default() -> Self {
        Dictionary::new()
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    static kCFTypeDictionaryKeyCallBacks: KeyCbs;
    static kCFCopyStringDictionaryKeyCallBacks: KeyCbs;
    static kCFTypeDictionaryValueCallBacks: ValueCbs;

    fn CFDictionaryGetTypeID() -> TypeId;

    fn CFDictionaryContainsKey(the_dict: &Dictionary, key: *const c_void) -> bool;

    fn CFDictionaryContainsValue(the_dict: &Dictionary, value: *const c_void) -> bool;

    fn CFDictionaryGetCount(the_dict: &Dictionary) -> Index;

    fn CFDictionaryGetValue(
        the_dict: &Dictionary,
        key: *const c_void,
    ) -> Option<ptr::NonNull<c_void>>;

    fn CFDictionaryGetValueIfPresent(
        the_dict: &Dictionary,
        key: *const c_void,
        value: *mut Option<ptr::NonNull<c_void>>,
    ) -> bool;

    fn CFDictionaryCreate(
        allocator: Option<&Allocator>,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: Index,
        key_callbacks: Option<&KeyCbs>,
        value_callbacks: Option<&ValueCbs>,
    ) -> Option<arc::R<Dictionary>>;

    fn CFDictionaryGetKeysAndValues(
        the_dict: &Dictionary,
        keys: *mut *const c_void,
        values: *mut *const c_void,
    );

    fn CFDictionaryCreateCopy(
        allocator: Option<&cf::Allocator>,
        the_dict: &cf::Dictionary,
    ) -> Option<arc::R<cf::Dictionary>>;

}

define_cf_type!(
    #[doc(alias = "CFMutableDictionary")]
    DictionaryMut(Dictionary)
);

impl DictionaryMut {
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        unsafe { Self::with_capacity_in(None, capacity).unwrap_unchecked() }
    }

    pub fn with_capacity_in(
        allocator: Option<&Allocator>,
        capacity: usize,
    ) -> Option<arc::R<Self>> {
        unsafe {
            Self::create(
                allocator,
                capacity as _,
                KeyCbs::default(),
                ValueCbs::default(),
            )
        }
    }

    pub unsafe fn create(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        key_callbacks: Option<&KeyCbs>,
        value_callbacks: Option<&ValueCbs>,
    ) -> Option<arc::R<Self>> {
        unsafe { CFDictionaryCreateMutable(allocator, capacity, key_callbacks, value_callbacks) }
    }

    #[doc(alias = "CFDictionarySetValue")]
    #[inline]
    pub fn insert(&mut self, key: &cf::String, val: &cf::Type) {
        unsafe { CFDictionarySetValue(self, key.as_type_ptr(), val.as_type_ptr()) }
    }

    #[doc(alias = "CFDictionaryRemoveValue")]
    #[inline]
    pub fn remove(&mut self, key: &cf::String) {
        unsafe { CFDictionaryRemoveValue(self, key.as_type_ptr()) }
    }

    #[doc(alias = "CFDictionaryAddValue")]
    #[inline]
    pub unsafe fn add_value(&mut self, key: *const c_void, val: *const c_void) {
        unsafe { CFDictionaryAddValue(self, key, val) }
    }

    #[doc(alias = "CFDictionarySetValue")]
    #[inline]
    pub unsafe fn set_value(&mut self, key: *const c_void, val: *const c_void) {
        unsafe { CFDictionarySetValue(self, key, val) }
    }

    #[doc(alias = "CFDictionaryReplaceValue")]
    #[inline]
    pub unsafe fn replace_value(&mut self, key: *const c_void, val: *const c_void) {
        unsafe { CFDictionaryReplaceValue(self, key, val) }
    }

    #[doc(alias = "CFDictionaryRemoveValue")]
    #[inline]
    pub unsafe fn remove_value(&mut self, key: *const c_void) {
        unsafe { CFDictionaryRemoveValue(self, key) }
    }

    #[doc(alias = "CFDictionaryRemoveAllValues")]
    #[inline]
    pub fn remove_all_values(&mut self) {
        unsafe { CFDictionaryRemoveAllValues(self) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct DictionaryOf<K, V>(Dictionary, marker::PhantomData<(K, V)>)
where
    K: arc::Retain,
    V: arc::Retain;

impl<K, V> DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        unsafe { std::mem::transmute(self.0.value(std::mem::transmute(k))) }
    }

    pub fn new() -> arc::R<Self> {
        unsafe { std::mem::transmute(Dictionary::new()) }
    }

    pub fn retained(&self) -> arc::R<Self> {
        unsafe { std::mem::transmute(self.0.retained()) }
    }
}

impl<K, V> AsRef<DictionaryOf<K, V>> for arc::R<DictionaryOfMut<K, V>>
where
    K: arc::Retain,
    V: arc::Retain,
{
    fn as_ref(&self) -> &DictionaryOf<K, V> {
        self
    }
}

#[cfg(feature = "objc")]
impl<K, V> DictionaryOf<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    /// Toll-Free Bridged
    #[cfg(feature = "ns")]
    pub fn as_ns(&self) -> &ns::Dictionary<K, V> {
        unsafe { std::mem::transmute(self) }
    }
}

impl DictionaryOf<cf::String, cf::Plist> {
    pub fn value<'a>(&'a self, key: &cf::String) -> Option<&'a cf::Plist> {
        let mut result = None;
        unsafe {
            if CFDictionaryGetValueIfPresent(&self.0, key.as_type_ptr(), &mut result) {
                Some(std::mem::transmute(result))
            } else {
                None
            }
        }
    }
}

impl<K, V> std::ops::Deref for DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    type Target = Dictionary;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> arc::Release for DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    #[inline]
    unsafe fn release(&mut self) {
        unsafe { self.0.release() }
    }
}

impl<K, V> arc::Retain for DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    #[inline]
    fn retained(&self) -> arc::R<Self> {
        unsafe { std::mem::transmute(self.0.retained()) }
    }
}

#[repr(transparent)]
pub struct DictionaryOfMut<K, V>(DictionaryMut, marker::PhantomData<(K, V)>);

impl<K, V> std::ops::Deref for DictionaryOfMut<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    type Target = DictionaryOf<K, V>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<K, V> DictionaryOfMut<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    #[doc(alias = "CFDictionarySetValue")]
    #[inline]
    pub fn insert(&mut self, key: &K, val: &V) {
        unsafe { CFDictionarySetValue(&mut self.0, key as *const _ as _, val as *const _ as _) }
    }

    #[doc(alias = "CFDictionaryRemoveValue")]
    #[inline]
    pub fn remove(&mut self, key: &K) {
        unsafe { CFDictionaryRemoveValue(&mut self.0, key as *const _ as _) }
    }
}

impl<K, V> DictionaryOf<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    #[inline]
    pub fn with_keys_values<const N: usize>(
        keys: &[&K; N],
        values: &[&V; N],
    ) -> arc::R<DictionaryOf<K, V>> {
        unsafe {
            let dict = Dictionary::create_in(
                keys.as_ptr() as _,
                values.as_ptr() as _,
                N as _,
                KeyCbs::default(),
                ValueCbs::default(),
                None,
            );

            std::mem::transmute(dict)
        }
    }

    pub fn copy_mut(&self) -> Option<arc::R<DictionaryOfMut<K, V>>> {
        unsafe { std::mem::transmute(CFDictionaryCreateMutableCopy(None, 0, &self.0)) }
    }
}

impl<K, V> arc::Release for DictionaryOfMut<K, V>
where
    K: arc::Release,
    V: arc::Release,
{
    #[inline]
    unsafe fn release(&mut self) {
        unsafe { self.0.release() }
    }
}

impl<K, V> arc::Retain for DictionaryOfMut<K, V>
where
    K: arc::Retain,
    V: arc::Retain,
{
    #[inline]
    fn retained(&self) -> arc::Retained<Self> {
        unsafe { std::mem::transmute(self.0.retained()) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn CFDictionaryCreateMutable(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        key_callbacks: Option<&KeyCbs>,
        value_callbacks: Option<&ValueCbs>,
    ) -> Option<arc::R<DictionaryMut>>;

    fn CFDictionaryCreateMutableCopy(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        the_dict: &Dictionary,
    ) -> Option<arc::R<DictionaryMut>>;

    fn CFDictionaryAddValue(the_dict: &mut DictionaryMut, key: *const c_void, value: *const c_void);
    fn CFDictionarySetValue(the_dict: &mut DictionaryMut, key: *const c_void, value: *const c_void);
    fn CFDictionaryReplaceValue(
        the_dict: &mut DictionaryMut,
        key: *const c_void,
        value: *const c_void,
    );
    fn CFDictionaryRemoveValue(the_dict: &mut DictionaryMut, key: *const c_void);
    fn CFDictionaryRemoveAllValues(the_dict: &mut DictionaryMut);

}
