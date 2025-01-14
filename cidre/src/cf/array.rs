use crate::{
    arc::{self, Retain},
    cf, define_cf_type,
};

use super::{Allocator, Index, String, Type, TypeId};
use std::{ffi::c_void, intrinsics::transmute, marker::PhantomData};

pub type RetainCb = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ReleaseCb = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type CopyDescCb = extern "C" fn(value: *const c_void) -> Option<arc::R<String>>;
pub type EqualCb = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;

#[repr(C)]
pub struct Cbs {
    version: Index,
    retain: RetainCb,
    release: ReleaseCb,
    copy_desc: CopyDescCb,
    equal: EqualCb,
}

impl Cbs {
    #[inline]
    pub fn default() -> Option<&'static Cbs> {
        unsafe { Some(&kCFTypeArrayCallBacks) }
    }
}

define_cf_type!(
    #[doc(alias = "CFArray")]
    Array(Type)
);

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayOf<T>(Array, PhantomData<T>);

impl<T> ArrayOf<T> {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(Array::new()) }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { transmute(Array::new_in(allocator)) }
    }

    #[doc(alias = "CFArrayContainsValue")]
    #[inline]
    pub fn contains(&self, val: &cf::Type) -> bool {
        unsafe { CFArrayContainsValue(self, cf::Range::new(0, self.count()), val as *const _ as _) }
    }

    #[inline]
    pub fn iter(&self) -> ArrayOfIterator<T> {
        ArrayOfIterator {
            array: self,
            index: 0,
            len: self.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn copy_mut(&self) -> Option<arc::R<ArrayOfMut<T>>> {
        let copy = self.0.copy_mut();
        unsafe { transmute(copy) }
    }

    #[inline]
    pub fn from_slice(values: &[&T]) -> arc::R<Self>
    where
        T: arc::Retain,
    {
        unsafe {
            let arr = Array::create_in(
                values.as_ptr() as _,
                values.len() as _,
                Cbs::default(),
                None,
            );
            transmute(arr)
        }
    }

    #[inline]
    pub fn from_retained_slice(values: &[arc::R<T>]) -> Option<arc::R<Self>>
    where
        T: arc::Retain,
    {
        unsafe {
            let arr = Array::create_in(
                values.as_ptr() as _,
                values.len() as _,
                Cbs::default(),
                None,
            );
            transmute(arr)
        }
    }
}

impl<T> std::ops::Deref for ArrayOf<T> {
    type Target = Array;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::Index<usize> for ArrayOf<T>
where
    T: arc::Retain,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { transmute::<&Type, &T>(&self.0[index]) }
    }
}

impl<T> std::ops::IndexMut<usize> for ArrayOf<T>
where
    T: arc::Retain,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { transmute::<&mut Type, &mut T>(&mut self.0[index]) }
    }
}

impl<T> arc::Release for ArrayOf<T> {
    #[inline]
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> arc::Retain for ArrayOf<T> {
    #[inline]
    fn retained(&self) -> arc::R<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

impl<T: arc::Retain> AsRef<Self> for arc::R<ArrayOf<T>> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: arc::Retain> AsRef<arc::R<ArrayOf<T>>> for arc::R<ArrayOfMut<T>> {
    fn as_ref(&self) -> &arc::R<ArrayOf<T>> {
        unsafe { std::mem::transmute(self) }
    }
}

pub struct ArrayOfIterator<'a, T> {
    array: &'a Array,
    index: usize,
    len: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ArrayOfIterator<'a, T>
where
    T: arc::Retain,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let res = unsafe { transmute::<&Type, &'a T>(&self.array[self.index]) };
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for ArrayOfIterator<'a, T>
where
    T: arc::Retain,
{
    #[inline]
    fn len(&self) -> usize {
        self.array.len() - self.index
    }
}

#[repr(transparent)]
pub struct ArrayOfMut<T>(ArrayMut, PhantomData<T>);

impl<T: Retain> ArrayOfMut<T> {
    #[inline]
    pub fn new() -> arc::R<ArrayOfMut<T>> {
        Self::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        unsafe { Self::with_capacity_in(capacity, None).unwrap_unchecked() }
    }

    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: Option<&Allocator>) -> Option<arc::R<Self>> {
        let arr = ArrayMut::create_in(capacity as _, Cbs::default(), alloc);
        unsafe { transmute(arr) }
    }

    #[inline]
    pub fn iter(&self) -> ArrayOfIterator<T> {
        ArrayOfIterator {
            array: self,
            index: 0,
            len: self.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn push(&mut self, val: &T) {
        self.0.push(unsafe { transmute(val) });
    }

    #[inline]
    pub fn copy(&self) -> Option<arc::R<ArrayOf<T>>> {
        unsafe { std::mem::transmute(self.0.copy()) }
    }
}

impl<T> std::ops::Deref for ArrayOfMut<T> {
    type Target = ArrayOf<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl<T> std::ops::Index<usize> for ArrayOfMut<T>
where
    T: arc::Retain,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { transmute::<&Type, &T>(&self.0[index]) }
    }
}

impl<T> std::ops::IndexMut<usize> for ArrayOfMut<T>
where
    T: arc::Retain,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { transmute::<&mut Type, &mut T>(&mut self.0[index]) }
    }
}

impl<T> arc::Release for ArrayOfMut<T> {
    #[inline]
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> arc::Retain for ArrayOfMut<T> {
    #[inline]
    fn retained(&self) -> arc::R<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

impl Array {
    /// ```
    /// use cidre::cf;
    ///
    /// let type_id = cf::Array::type_id();
    /// assert_eq!(type_id, 19);
    ///
    /// unsafe {
    ///     let type_desc = cf::copy_type_id_desc(type_id).unwrap();
    ///     assert_eq!("CFArray", type_desc.to_string());
    /// }
    /// ```
    #[doc(alias = "CFArrayGetTypeID")]
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFArrayGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new();
    /// assert_eq!(arr.count(), 0);
    /// ```
    #[doc(alias = "CFArrayGetCount")]
    #[inline]
    pub fn count(&self) -> Index {
        unsafe { CFArrayGetCount(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = unsafe { cf::Array::create_in(std::ptr::null(), 0, None, None).expect("arr") };
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[doc(alias = "CFArrayGetCount")]
    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new();
    /// assert_eq!(arr.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new();
    /// let arr2 = arr1.copy_in(None).expect("copy");
    ///
    /// ```
    #[inline]
    pub fn copy_in(&self, allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFArrayCreateCopy(allocator, self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new();
    /// let arr2 = arr1.copy().expect("copy");
    /// ```
    #[inline]
    pub fn copy(&self) -> Option<arc::R<Array>> {
        self.copy_in(None)
    }

    #[inline]
    pub unsafe fn create_in(
        values: *const c_void,
        num_values: Index,
        callbacks: Option<&Cbs>,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<Self>> {
        CFArrayCreate(allocator, values, num_values, callbacks)
    }

    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<arc::R<Self>> {
        unsafe { Self::create_in(std::ptr::null(), 0, Cbs::default(), allocator) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10);
    /// let arr = cf::Array::from_type_refs(&[&num, &num, &num]).unwrap();
    /// assert_eq!(3, arr.len());
    /// ```
    #[inline]
    pub fn from_type_refs<const N: usize>(values: &[&Type; N]) -> Option<arc::R<Self>> {
        unsafe { Array::create_in(values.as_ptr() as _, N as _, Cbs::default(), None) }
    }

    #[inline]
    pub fn from_slice<T>(values: &[&T]) -> Option<arc::R<Array>>
    where
        T: arc::Retain,
    {
        unsafe {
            Array::create_in(
                values.as_ptr() as _,
                values.len() as _,
                Cbs::default(),
                None,
            )
        }
    }

    #[inline]
    pub fn from_copyable<const N: usize, T>(values: &[T; N]) -> Option<arc::R<Self>>
    where
        T: Copy,
    {
        unsafe { Array::create_in(values.as_ptr() as _, N as _, None, None) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10);
    ///
    /// let empty_arr = cf::Array::new();
    /// let mut mut_arr = empty_arr.copy_mut_in(0, None).unwrap();
    ///
    ///
    /// mut_arr.push(&num);
    ///
    /// assert_eq!(1, mut_arr.len());
    /// assert_eq!(0, empty_arr.len());
    ///
    /// ```
    #[inline]
    pub fn copy_mut_in(
        &self,
        capacity: Index,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<ArrayMut>> {
        unsafe { CFArrayCreateMutableCopy(allocator, capacity, self) }
    }

    #[inline]
    pub fn copy_mut(&self) -> Option<arc::R<ArrayMut>> {
        unsafe { CFArrayCreateMutableCopy(None, 0, self) }
    }

    #[inline]
    pub fn copy_mut_with_capacity(&self, capacity: usize) -> Option<arc::R<ArrayMut>> {
        self.copy_mut_in(capacity as _, None)
    }
}

impl std::ops::Index<usize> for Array {
    type Output = Type;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { CFArrayGetValueAtIndex(self, index as _) }
    }
}

impl std::ops::IndexMut<usize> for Array {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { CFArrayGetValueAtIndex(self, index as _) }
    }
}

define_cf_type!(ArrayMut(Array));

impl ArrayMut {
    #[doc(alias = "CFArrayAppendValue")]
    #[inline]
    pub unsafe fn append_value(&mut self, val: *const c_void) {
        CFArrayAppendValue(self, val)
    }

    #[doc(alias = "CFArrayAppendValue")]
    #[inline]
    pub fn push(&mut self, val: &Type) {
        unsafe { self.append_value(val.as_type_ptr()) }
    }

    #[doc(alias = "CFArrayRemoveAllValues")]
    #[inline]
    pub fn remove_all_values(&mut self) {
        unsafe {
            CFArrayRemoveAllValues(self);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.remove_all_values();
    }

    #[doc(alias = "CFArrayCreateMutable")]
    #[inline]
    pub fn create_in(
        capacity: Index,
        callbacks: Option<&Cbs>,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<ArrayMut>> {
        unsafe { CFArrayCreateMutable(allocator, capacity, callbacks) }
    }

    #[inline]
    pub fn with_capacity(capacity: Index) -> arc::R<Self> {
        unsafe { Self::create_in(capacity, Cbs::default(), None).unwrap_unchecked() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let mut arr = cf::ArrayMut::new();
    /// assert_eq!(0, arr.len());
    ///
    /// let num = cf::Number::from_i32(0);
    ///
    /// arr.push(&num);
    /// arr.push(&num);
    /// assert_eq!(2, arr.len());
    ///
    /// arr.remove_all_values();
    /// assert_eq!(0, arr.len());
    /// ```
    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::with_capacity(0)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    static kCFTypeArrayCallBacks: Cbs;

    fn CFArrayGetTypeID() -> TypeId;

    fn CFArrayGetValueAtIndex(array: &Array, idx: Index) -> &mut Type;

    fn CFArrayCreate(
        allocator: Option<&Allocator>,
        values: *const c_void,
        num_values: Index,
        callbacks: Option<&Cbs>,
    ) -> Option<arc::R<Array>>;

    fn CFArrayCreateCopy(allocator: Option<&Allocator>, array: &Array) -> Option<arc::R<Array>>;

    fn CFArrayGetCount(array: &Array) -> Index;

    fn CFArrayCreateMutable(
        allocator: Option<&Allocator>,
        capacity: Index,
        callbacks: Option<&Cbs>,
    ) -> Option<arc::R<ArrayMut>>;

    fn CFArrayCreateMutableCopy(
        allocator: Option<&Allocator>,
        capacity: Index,
        array: &Array,
    ) -> Option<arc::R<ArrayMut>>;

    fn CFArrayAppendValue(array: &mut ArrayMut, value: *const c_void);

    fn CFArrayRemoveAllValues(array: &mut ArrayMut);

    fn CFArrayContainsValue(array: &Array, range: cf::Range, value: *const c_void) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    pub fn empty_arrays_are_same() {
        let arr1 = cf::Array::new();
        let arr2 = arr1.copy().expect("copy");
        let arr3 = cf::Array::new();
        let arr4 = arr2.copy_mut().expect("copy");
        unsafe {
            assert_eq!(arr1.as_type_ptr(), arr2.as_type_ptr());
            assert_eq!(arr3.as_type_ptr(), arr2.as_type_ptr());
            assert_ne!(arr1.as_type_ptr(), arr4.as_type_ptr());
        }
    }
}
