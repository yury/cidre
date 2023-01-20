use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, Index, IndexMut},
};

use crate::{
    arc, ns,
    objc::{self, Class, Obj},
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
        unsafe { transmute(NS_ARRAY.alloc().init()) }
    }

    #[objc::msg_send(init)]
    fn init(&self) -> arc::R<Self>;

    // use new() with uses alloc_init and it is faster
    // _new() is slower
    #[inline]
    pub fn _new() -> arc::R<Self> {
        unsafe { transmute(NS_ARRAY.new()) }
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe {
            transmute(
                NS_ARRAY
                    .alloc()
                    .init_with_objects_count(objs.as_ptr() as _, objs.len()),
            )
        }
    }

    #[objc::msg_send(initWithObjects:count:)]
    fn init_with_objects_count(&self, ptr: *const c_void, count: usize) -> arc::R<Self>;

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
}

impl<T: Obj> Index<usize> for Array<T> {
    type Output = T;

    #[objc::msg_send(objectAtIndex:)]
    fn index(&self, index: usize) -> &Self::Output;
}

impl<T: Obj> IndexMut<usize> for Array<T>
where
    T: Obj,
{
    #[objc::msg_send(objectAtIndex:)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output;
}

impl<T: Obj> ArrayMut<T> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        unsafe { transmute(NS_MUTABLE_ARRAY.alloc().init_with_capacity(capacity)) }
    }

    #[objc::msg_send(initWithCapacity:)]
    fn init_with_capacity(&self, capacity: usize) -> arc::R<Self>;
}

impl<T: Obj> ns::FastEnumeration<T> for Array<T> {}
impl<T: Obj> ns::FastEnumeration<T> for ArrayMut<T> {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_ARRAY: &'static Class<ns::Array<ns::Id>>;
    static NS_MUTABLE_ARRAY: &'static Class<ns::ArrayMut<ns::Id>>;
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
