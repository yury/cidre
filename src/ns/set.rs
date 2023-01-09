use std::{
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, Index, IndexMut},
};

use crate::{arc, msg_send, ns, objc};

#[derive(Debug)]
#[repr(transparent)]
pub struct Set<T>(ns::Id, PhantomData<T>)
where
    T: objc::Obj;

impl<T> objc::Obj for Set<T> where T: objc::Obj {}

#[derive(Debug)]
#[repr(transparent)]
pub struct SetMut<T>(ns::Set<T>)
where
    T: objc::Obj;

impl<T> objc::Obj for SetMut<T> where T: objc::Obj {}

impl<T> Deref for Set<T>
where
    T: objc::Obj,
{
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for SetMut<T>
where
    T: objc::Obj,
{
    type Target = Set<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Set<T>
where
    T: objc::Obj,
{
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(NSSet_set()) }
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe { transmute(NSSet_withObjs(objs.as_ptr() as _, objs.len())) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        msg_send!("ns", self, ns_count)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FEIterator<Self, T> {
        // ns::FEIterator::new(self, self.len())
        //let _n = self.len();
        ns::FastEnumeration::iter(self)
    }
}

impl<T> ns::FastEnumeration<T> for Set<T> where T: objc::Obj {}
impl<T> ns::FastEnumeration<T> for SetMut<T> where T: objc::Obj {}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSSet_withObjs(objects: *const ns::Id, count: ns::UInteger) -> arc::R<Set<ns::Id>>;
    fn NSSet_set() -> arc::R<Set<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;
    #[test]
    fn basics() {
        let two = ns::Number::with_i32(10);
        let set: &[&ns::Number] = &[&two, &two, &two];
        let set = ns::Set::from_slice(set);
        assert_eq!(1, set.len());
        let sum = set.iter().map(|v| v.as_i32()).sum();
        assert_eq!(10, sum);
    }
}
