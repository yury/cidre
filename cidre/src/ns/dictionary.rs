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
    }
}
