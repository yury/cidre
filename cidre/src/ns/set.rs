use std::{ffi::c_void, marker::PhantomData, mem::transmute, ops::Deref};

use crate::{
    arc, ns,
    objc::{self, Class, Obj},
};

#[cfg(feature = "cf")]
use crate::cf;

#[derive(Debug)]
#[repr(transparent)]
pub struct Set<T: Obj>(ns::Id, PhantomData<T>);

impl<T: Obj> Obj for Set<T> {}

#[derive(Debug)]
#[repr(transparent)]
pub struct SetMut<T: Obj>(ns::Set<T>);

impl<T: Obj> Obj for SetMut<T> {}

impl<T: Obj> Deref for Set<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> Deref for SetMut<T> {
    type Target = Set<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> arc::A<Set<T>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Set<T>>;

    #[objc::msg_send(initWithObjects:count:)]
    pub fn init_with_objs_count(self, ptr: *const c_void, count: usize) -> arc::R<Set<T>>;
}

impl<T: Obj> Set<T> {
    #[inline]
    pub fn cls() -> &'static Class<Self> {
        unsafe { transmute(NS_SET) }
    }

    #[inline]
    pub fn alloc() -> arc::A<Self> {
        Self::cls().alloc()
    }

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        Self::alloc().init_with_objs_count(objs.as_ptr() as _, objs.len())
    }

    #[objc::msg_send(count)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FEIterator<Self, T> {
        ns::FastEnumeration::iter(self)
    }

    #[cfg(feature = "cf")]
    #[inline]
    pub fn as_cf(&self) -> &cf::Set {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> ns::FastEnumeration<T> for Set<T> where T: Obj {}
impl<T> ns::FastEnumeration<T> for SetMut<T> where T: Obj {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_SET: &'static Class<ns::Set<ns::Id>>;
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
