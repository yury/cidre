use std::marker::PhantomData;

use crate::{arc, ns};

#[derive(Debug)]
#[repr(transparent)]
pub struct Dictionary<K, V>(ns::Id, PhantomData<(K, V)>);

#[derive(Debug)]
#[repr(transparent)]
pub struct DictionaryMut<K, V>(Dictionary<K, V>);

impl<K, V> std::ops::Deref for Dictionary<K, V> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> std::ops::Deref for DictionaryMut<K, V> {
    type Target = Dictionary<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> arc::Release for Dictionary<K, V> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<K, V> arc::Release for DictionaryMut<K, V> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}
