use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, Index, IndexMut},
};

use crate::{
    arc, define_cls, ns,
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

impl<T: Obj> arc::A<Array<T>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Array<T>>;

    #[objc::msg_send(initWithObjects:count:)]
    pub unsafe fn init_with_objects_count(
        self,
        ptr: *const c_void,
        count: usize,
    ) -> arc::R<Array<T>>;
}

impl<T: Obj> Array<T> {
    define_cls!(NS_ARRAY);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    // use new() with uses alloc_init and it is faster
    // _new() is slower
    #[inline]
    pub fn _new() -> arc::R<Self> {
        unsafe { transmute(NS_ARRAY.new()) }
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe { Self::alloc().init_with_objects_count(objs.as_ptr() as _, objs.len()) }
    }

    #[objc::msg_send(containsObject:)]
    pub fn contains(&self, object: &T) -> bool;

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

impl<T: Obj> arc::A<ArrayMut<T>> {
    #[objc::msg_send(initWithCapacity:)]
    pub fn init_with_capacity(self, capacity: usize) -> arc::R<ArrayMut<T>>;

    #[objc::msg_send(initWithObjects:count:)]
    pub unsafe fn init_with_objects_count(
        &self,
        ptr: *const c_void,
        count: usize,
    ) -> arc::R<ArrayMut<T>>;
}

impl<T: Obj> ArrayMut<T> {
    define_cls!(NS_MUTABLE_ARRAY);

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        Self::alloc().init_with_capacity(capacity)
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe { Self::alloc().init_with_objects_count(objs.as_ptr() as _, objs.len()) }
    }
    #[objc::msg_send(addObject:)]
    pub fn push(&mut self, obj: &T);

    #[objc::msg_send(removeLastObject)]
    pub fn remove_last(&mut self);

    #[objc::msg_send(removeObjectAtIndex:)]
    pub fn remove(&mut self, index: usize);

    #[objc::msg_send(insertObject:atIndex:)]
    pub fn insert_obj(&mut self, obj: &T, at_index: usize);

    #[inline]
    pub fn insert(&mut self, index: usize, element: &T) {
        self.insert_obj(element, index)
    }
}

impl<T: Obj> arc::R<ArrayMut<T>> {
    pub fn freeze(self) -> arc::R<Array<T>> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Obj> ns::FastEnumeration<T> for Array<T> {}
impl<T: Obj> ns::FastEnumeration<T> for ArrayMut<T> {}

impl From<&[&str]> for arc::R<ns::Array<ns::String>> {
    fn from(value: &[&str]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::String::with_str(v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&values[..]) })
    }
}

impl From<&[i8]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i8]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::tagged_i8(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[u8]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u8]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::tagged_u8(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[i16]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i16]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::tagged_i16(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[u16]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u16]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::tagged_u16(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[i32]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i32]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::tagged_i32(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[u32]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u32]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::tagged_u32(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[i64]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i64]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::with_i64(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&values[..]) })
    }
}

impl From<&[u64]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u64]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::with_u64(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&values[..]) })
    }
}

impl From<&[f32]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[f32]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::with_f32(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&values[..]) })
    }
}

impl From<&[f64]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[f64]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value.iter() {
            values.push(ns::Number::with_f64(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&values[..]) })
    }
}

impl<const N: usize> From<[i8; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i8; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_i8(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[u8; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [u8; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_u8(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[i16; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i16; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_i16(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[u16; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [u16; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_u16(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[i32; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i32; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_i32(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[u32; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [u32; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_u32(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

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
