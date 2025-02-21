use std::{
    marker::PhantomData,
    mem::{MaybeUninit, transmute},
    ops::Deref,
};

use crate::{arc, define_cls, ns, objc};

#[doc(alias = "NSArray")]
#[derive(Debug)]
#[repr(transparent)]
pub struct Array<T: objc::Obj>(ns::Id, PhantomData<T>);

unsafe impl<T: objc::Obj> Send for Array<T> where T: Send {}

impl<T: objc::Obj> objc::Obj for Array<T> where T: objc::Obj {}

#[doc(alias = "NSMutableArray")]
#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayMut<T: objc::Obj>(ns::Array<T>);

impl<T: objc::Obj> objc::Obj for ArrayMut<T> {}

impl<T: objc::Obj> Deref for Array<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: objc::Obj> Deref for ArrayMut<T> {
    type Target = Array<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: objc::Obj> std::ops::DerefMut for ArrayMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: objc::Obj> arc::A<Array<T>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Array<T>>;

    #[objc::msg_send(initWithObjects:count:)]
    pub unsafe fn init_with_objs(self, ptr: *const &T, count: usize) -> arc::R<Array<T>>;
}

impl<T: objc::Obj> Array<T> {
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
        unsafe { Self::alloc().init_with_objs(objs.as_ptr(), objs.len()) }
    }

    #[inline]
    pub fn from_slice_retained(objs: &[arc::R<T>]) -> arc::R<Self> {
        unsafe { Self::alloc().init_with_objs(objs.as_ptr() as _, objs.len()) }
    }

    #[objc::msg_send(containsObject:)]
    pub fn contains(&self, object: &T) -> bool;

    #[objc::msg_send(count)]
    pub fn len(&self) -> usize;

    #[objc::msg_send(firstObject)]
    pub fn first(&self) -> Option<&T>;

    #[objc::msg_send(lastObject)]
    pub fn last(&self) -> Option<&T>;

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<Self>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::R<ArrayMut<T>>;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FEIterator<Self, T> {
        ns::FastEnumeration::iter(self)
    }

    #[objc::msg_send(objectAtIndex:)]
    pub unsafe fn get_throws(&self, index: usize) -> arc::R<T>;

    pub fn get<'ear>(&self, index: usize) -> ns::ExResult<arc::R<T>> {
        unsafe { ns::try_catch(|| self.get_throws(index)) }
    }
}

impl<T: objc::Obj> arc::A<ArrayMut<T>> {
    #[objc::msg_send(initWithCapacity:)]
    pub fn init_with_capacity(self, capacity: usize) -> arc::R<ArrayMut<T>>;

    #[objc::msg_send(initWithObjects:count:)]
    pub unsafe fn init_with_objs(&self, ptr: *const &T, count: usize) -> arc::R<ArrayMut<T>>;
}

impl<T: objc::Obj> ArrayMut<T> {
    define_cls!(NS_MUTABLE_ARRAY);

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        Self::alloc().init_with_capacity(capacity)
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe { Self::alloc().init_with_objs(objs.as_ptr(), objs.len()) }
    }

    #[inline]
    pub fn from_slice_retained(objs: &[arc::R<T>]) -> arc::R<Self> {
        unsafe { Self::alloc().init_with_objs(objs.as_ptr() as _, objs.len()) }
    }

    #[objc::msg_send(addObject:)]
    pub fn push(&mut self, obj: &T);

    #[objc::msg_send(removeLastObject)]
    pub fn remove_last(&mut self);

    #[objc::msg_send(removeObjectAtIndex:)]
    pub unsafe fn remove_throws(&mut self, index: usize);

    #[inline]
    pub fn remove(&mut self, index: usize) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.remove_throws(index) })
    }

    #[objc::msg_send(removeAllObjects)]
    pub fn clear(&mut self);

    #[objc::msg_send(insertObject:atIndex:)]
    pub unsafe fn insert_obj_throws(&mut self, obj: &T, at_index: usize);

    #[inline]
    pub fn insert(&mut self, index: usize, element: &T) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.insert_obj_throws(element, index) })
    }
}

impl<T: objc::Obj> arc::R<ArrayMut<T>> {
    pub fn freeze(self) -> arc::R<Array<T>> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: objc::Obj> ns::FastEnumeration<T> for Array<T> {}
impl<T: objc::Obj> ns::FastEnumeration<T> for ArrayMut<T> {}

impl<T: objc::Obj> From<&[&T]> for arc::R<Array<T>> {
    fn from(value: &[&T]) -> Self {
        Array::from_slice(value)
    }
}

impl<T: objc::Obj> From<&[arc::R<T>]> for arc::R<Array<T>> {
    fn from(value: &[arc::R<T>]) -> Self {
        Array::from_slice_retained(value)
    }
}

impl From<&[&str]> for arc::R<Array<ns::String>> {
    fn from(value: &[&str]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::String::with_str(v));
        }
        ns::Array::from_slice_retained(&values[..])
    }
}

impl From<&[i8]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i8]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::tagged_i8(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[u8]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u8]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::tagged_u8(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[i16]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i16]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::tagged_i16(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[u16]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u16]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::tagged_u16(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[i32]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i32]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::tagged_i32(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[u32]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u32]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::tagged_u32(*v));
        }
        ns::Array::from_slice(&values)
    }
}

impl From<&[i64]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[i64]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::with_i64(*v));
        }
        ns::Array::from_slice_retained(&values[..])
    }
}

impl From<&[u64]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[u64]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::with_u64(*v));
        }
        ns::Array::from_slice_retained(&values[..])
    }
}

impl From<&[f32]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[f32]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::with_f32(*v));
        }
        ns::Array::from_slice_retained(&values[..])
    }
}

impl From<&[f64]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: &[f64]) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for v in value {
            values.push(ns::Number::with_f64(*v));
        }
        ns::Array::from_slice_retained(&values[..])
    }
}

impl<const N: usize> From<[i8; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i8; N]) -> Self {
        let mut vals: [MaybeUninit<&'static ns::Number>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, v) in value.iter().enumerate() {
            vals[i].write(ns::Number::tagged_i8(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[u8; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [u8; N]) -> Self {
        let mut vals: [MaybeUninit<&'static ns::Number>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, v) in value.iter().enumerate() {
            vals[i].write(ns::Number::tagged_u8(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[i16; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i16; N]) -> Self {
        let mut vals: [MaybeUninit<&'static ns::Number>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, v) in value.iter().enumerate() {
            vals[i].write(ns::Number::tagged_i16(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[u16; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [u16; N]) -> Self {
        let mut vals: [MaybeUninit<&'static ns::Number>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, v) in value.iter().enumerate() {
            vals[i].write(ns::Number::tagged_u16(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[i32; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i32; N]) -> Self {
        let mut vals: [MaybeUninit<&'static ns::Number>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, v) in value.iter().enumerate() {
            vals[i].write(ns::Number::tagged_i32(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

impl<const N: usize> From<[u32; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [u32; N]) -> Self {
        let mut vals: [MaybeUninit<&'static ns::Number>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (i, v) in value.iter().enumerate() {
            vals[i].write(ns::Number::tagged_u32(*v));
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_ARRAY: &'static objc::Class<ns::Array<ns::Id>>;
    static NS_MUTABLE_ARRAY: &'static objc::Class<ns::ArrayMut<ns::Id>>;
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
        assert_eq!(5, arr.first().unwrap().as_i32());

        let mut k = 0;
        for i in arr.iter() {
            k += 1;
            println!("{:?}", i);
        }

        assert_eq!(1, k);
    }

    #[test]
    fn copy() {
        let arr = ns::Array::<ns::Number>::new();
        let mut mut_copy = arr.copy_mut();
        assert!(mut_copy.is_empty());
        mut_copy.insert(0, &ns::Number::tagged_i8(1)).unwrap();
        assert_eq!(1, mut_copy.len());
        assert!(arr.is_empty());
        assert!(!mut_copy.is_empty());

        mut_copy.remove(10).expect_err("should be exception");
        mut_copy.clear();
        assert!(mut_copy.is_empty());
    }

    #[test]
    fn exception() {
        let arr = ns::Array::<ns::Number>::new();
        arr.get(0).expect_err("Should be exception");
    }
}
