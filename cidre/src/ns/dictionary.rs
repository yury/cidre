use std::marker::PhantomData;

use crate::{
    arc, define_cls, ns,
    objc::{self, Obj},
};

use super::Class;

#[derive(Debug)]
#[repr(transparent)]
pub struct Dictionary<K: Obj, V: Obj>(ns::Id, PhantomData<(K, V)>);

impl<K: Obj, V: Obj> Obj for Dictionary<K, V> {}

impl<K: Obj, V: Obj> arc::A<Dictionary<K, V>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Dictionary<K, V>>;

    #[objc::msg_send(initWithObjects:forKeys:count:)]
    pub unsafe fn init_with_objects_for_keys_count_throws(
        self,
        objects: *const V,
        keys: *const K,
        count: usize,
    ) -> arc::R<Dictionary<K, V>>;
}

impl<K: Obj, V: Obj> Dictionary<K, V> {
    define_cls!(NS_DICTIONARY);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(count)]
    pub fn len(&self) -> usize;

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(objectForKey:)]
    pub fn obj_for_key<'a>(&'a self, key: &K) -> Option<&'a V>;

    #[objc::msg_send(copy)]
    pub fn copy_ar(&self) -> arc::Rar<Self>;

    #[objc::rar_retain]
    pub fn copy(&self) -> arc::R<Self>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut_ar(&self) -> arc::Rar<DictionaryMut<K, V>>;

    #[objc::rar_retain]
    pub fn copy_mut(&self) -> arc::R<DictionaryMut<K, V>>;

    pub fn with_keys_values<const N: usize>(keys: &[&K; N], objects: &[&V; N]) -> arc::R<Self> {
        unsafe {
            Self::alloc().init_with_objects_for_keys_count_throws(
                std::mem::transmute(objects.as_ptr()),
                std::mem::transmute(keys.as_ptr()),
                N,
            )
        }
    }
}

impl<K: Obj, V: Obj> std::ops::Index<&K> for Dictionary<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.obj_for_key(index).expect("no entry found for key")
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct DictionaryMut<K: Obj, V: Obj>(Dictionary<K, V>);

impl<K: Obj, V: Obj> Obj for DictionaryMut<K, V> {}

impl<K: Obj, V: Obj> std::ops::Deref for Dictionary<K, V> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Obj, V: Obj> std::ops::Deref for DictionaryMut<K, V> {
    type Target = Dictionary<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Obj, V: Obj> arc::A<DictionaryMut<K, V>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<DictionaryMut<K, V>>;

    #[objc::msg_send(initWithCapacity:)]
    pub fn init_with_capacity(self, capacity: usize) -> arc::R<DictionaryMut<K, V>>;
}

impl<K: Obj, V: Obj> DictionaryMut<K, V> {
    define_cls!(NS_MUTABLE_DICTIONARY);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        Self::alloc().init_with_capacity(capacity)
    }

    #[objc::msg_send(removeObjectForKey:)]
    pub fn remove_obj_for_key(&mut self, key: &K);

    #[objc::msg_send(setObject:forKey:)]
    pub fn set_obj_for_key(&mut self, obj: &V, key: &K);

    #[objc::msg_send(removeAllObjects)]
    pub fn remove_all_objs(&mut self);

    #[objc::msg_send(removeObjectsForKeys:)]
    pub fn remove_objs_for_keys(&mut self, keys: &ns::Array<K>);

    #[objc::msg_send(setDictionary:)]
    pub fn set_dictionary(&mut self, other: &Self);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_DICTIONARY: &'static Class<Dictionary<ns::Id, ns::Id>>;
    static NS_MUTABLE_DICTIONARY: &'static Class<DictionaryMut<ns::Id, ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::{arc, ns};
    #[test]
    fn basics() {
        let dict: arc::R<ns::Dictionary<ns::String, ns::Id>> = ns::Dictionary::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        let dict: arc::R<ns::DictionaryMut<ns::String, ns::Id>> =
            ns::DictionaryMut::with_capacity(10);
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        let dict = ns::Dictionary::with_keys_values(
            &[ns::Number::with_i8(10).as_ref()],
            &[ns::Number::with_i16(20).as_ref()],
        );

        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
    }
}
