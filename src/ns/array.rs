use std::{
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
};

use crate::{
    arc, ns,
    objc::{msg_send, Class, Obj},
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Array<T: Obj>(ns::Id, PhantomData<T>);

impl<T: Obj> Obj for Array<T> where T: Obj {}

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayMut<T: Obj>(ns::Array<T>);

impl<T: Obj> Obj for ArrayMut<T> {}

impl<T: Obj> Deref for Array<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> Deref for ArrayMut<T> {
    type Target = Array<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> Array<T> {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { NS_ARRAY.alloc::<Self>().call0(msg_send::init) }
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe {
            NS_ARRAY.alloc::<Self>().call2(
                msg_send::init_with_objects_count,
                objs.as_ptr(),
                objs.len(),
            )
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.call0(msg_send::count) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FEIterator<Self, T> {
        ns::FastEnumeration::iter(self)
    }
}

impl<T: Obj> Index<usize> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.call1(msg_send::object_at_index, index) }
    }
}

impl<T: Obj> IndexMut<usize> for Array<T>
where
    T: Obj,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.call1(msg_send::object_at_index, index) }
    }
}
impl<T: Obj> ArrayMut<T> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        unsafe {
            NS_MUTABLE_ARRAY
                .alloc::<Self>()
                .call1(msg_send::init_with_capacity, capacity)
        }
    }
}

impl<T: Obj> ns::FastEnumeration<T> for Array<T> {}
impl<T: Obj> ns::FastEnumeration<T> for ArrayMut<T> {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_ARRAY: &'static Class;
    static NS_MUTABLE_ARRAY: &'static Class;
}

#[cfg(test)]
mod tests {
    use crate::{ns, objc::Obj};

    #[test]
    fn empty() {
        let empty = ns::Array::<ns::Number>::new();
        assert!(empty.is_empty());
        assert!(!empty.is_tagged_ptr());

        let empty = ns::ArrayMut::<ns::Number>::with_capacity(10);
        assert!(empty.is_empty());
        assert!(!empty.is_tagged_ptr());
    }

    #[test]
    fn basics() {
        let one = ns::Number::with_i32(5);
        let arr: &[&ns::Number] = &[&one];
        let arr = ns::Array::from_slice(arr);
        assert_eq!(1, arr.len());
        assert_eq!(5, arr[0].as_i32());

        let mut k = 0;
        for i in arr.iter() {
            k += 1;
            println!("{:?}", i);
        }

        assert_eq!(1, k);
    }
}
