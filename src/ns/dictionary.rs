use std::{marker::PhantomData, mem::transmute};

use crate::{
    arc, ns,
    objc::{msg_send, Obj},
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Dictionary<K: Obj, V: Obj>(ns::Id, PhantomData<(K, V)>);

impl<K: Obj, V: Obj> Obj for Dictionary<K, V> {}

impl<K: Obj, V: Obj> Dictionary<K, V> {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(NSDictionary_dictionary()) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.call0(msg_send::count) }
    }

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

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSDictionary_dictionary() -> arc::R<Dictionary<ns::Id, ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::{arc, ns};
    #[test]
    fn basics() {
        let dict: arc::R<ns::Dictionary<ns::String, ns::Id>> = ns::Dictionary::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);
    }
}
