use std::{marker::PhantomData, mem::transmute};

use crate::{arc, msg_send, ns, objc};

#[derive(Debug)]
#[repr(transparent)]
pub struct Dictionary<K, V>(ns::Id, PhantomData<(K, V)>)
where
    K: objc::Obj,
    V: objc::Obj;

impl<K, V> objc::Obj for Dictionary<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
}

impl<K, V> Dictionary<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(NSDictionary_dictionary()) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        msg_send!("ns", self, ns_count)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct DictionaryMut<K, V>(Dictionary<K, V>)
where
    K: objc::Obj,
    V: objc::Obj;

impl<K, V> objc::Obj for DictionaryMut<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
}

impl<K, V> std::ops::Deref for Dictionary<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> std::ops::Deref for DictionaryMut<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    type Target = Dictionary<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> arc::Release for Dictionary<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<K, V> arc::Retain for Dictionary<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    fn retained(&self) -> arc::Retained<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

impl<K, V> arc::Release for DictionaryMut<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<K, V> arc::Retain for DictionaryMut<K, V>
where
    K: objc::Obj,
    V: objc::Obj,
{
    fn retained(&self) -> arc::Retained<Self> {
        unsafe { transmute(self.0.retained()) }
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
