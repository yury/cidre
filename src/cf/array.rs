use crate::define_cf_type;

use super::{
    runtime::{Release, Retain},
    Allocator, Index, Retained, String, Type, TypeId,
};
use std::{ffi::c_void, intrinsics::transmute, marker::PhantomData};

pub type RetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type ReleaseCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);
pub type CopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> Option<Retained<String>>;
pub type EqualCallBack = extern "C" fn(value1: *const c_void, value2: *const c_void) -> bool;

#[repr(C)]
pub struct Callbacks {
    version: Index,
    retain: RetainCallBack,
    release: ReleaseCallBack,
    copy_description: CopyDescriptionCallBack,
    equal: EqualCallBack,
}

impl Callbacks {
    #[inline]
    pub fn default() -> Option<&'static Callbacks> {
        unsafe { Some(&kCFTypeArrayCallBacks) }
    }
}

define_cf_type!(Array(Type));

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayOf<T>(Array, PhantomData<T>);

impl<T> ArrayOf<T> {
    #[inline]
    pub fn new() -> Option<Retained<ArrayOf<T>>> {
        unsafe { transmute(Array::new()) }
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<Retained<ArrayOf<T>>> {
        unsafe { transmute(Array::new_in(allocator)) }
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
    pub fn mutable_copy(&self) -> Option<Retained<MutArrayOf<T>>> {
        let copy = self.0.mutable_copy();
        unsafe { transmute(copy) }
    }

    #[inline]
    pub fn from_slice(values: &[&T]) -> Option<Retained<Self>>
    where
        T: Retain,
    {
        unsafe {
            let arr = Array::create_in(
                None,
                values.as_ptr() as _,
                values.len() as _,
                Callbacks::default(),
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
    T: Retain + Release,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { transmute::<&Type, &T>(&self.0[index]) }
    }
}

impl<T> Release for ArrayOf<T> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> Retain for ArrayOf<T> {
    fn retained(&self) -> Retained<Self> {
        unsafe { transmute(self.0.retained()) }
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
    T: Retain,
{
    type Item = &'a T;

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
    T: Retain,
{
    fn len(&self) -> usize {
        self.array.len() - self.index
    }
}

#[repr(transparent)]
pub struct MutArrayOf<T>(MutableArray, PhantomData<T>);

impl<T> MutArrayOf<T> {
    #[inline]
    pub fn new() -> Option<Retained<MutArrayOf<T>>> {
        Self::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Option<Retained<MutArrayOf<T>>> {
        Self::with_capacity_in(capacity, None)
    }

    #[inline]
    pub fn with_capacity_in(
        capacity: usize,
        alloc: Option<&Allocator>,
    ) -> Option<Retained<MutArrayOf<T>>> {
        let arr = MutableArray::create_in(alloc, capacity as _, Callbacks::default());
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
    pub fn push(&mut self, value: &T) {
        self.0.append(unsafe { transmute(value) });
    }
}

impl<T> std::ops::Deref for MutArrayOf<T> {
    type Target = ArrayOf<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl<T> std::ops::Index<usize> for MutArrayOf<T>
where
    T: Retain,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { transmute::<&Type, &T>(&self.0[index]) }
    }
}

impl<T> Release for MutArrayOf<T> {
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> Retain for MutArrayOf<T> {
    fn retained(&self) -> Retained<Self> {
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
    ///     let type_desc = cf::copy_type_id_description(type_id).unwrap();
    ///     assert_eq!("CFArray", type_desc.to_string());
    /// }
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
        unsafe { CFArrayGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new().expect("arr");
    /// assert_eq!(arr.count(), 0);
    /// ```
    #[inline]
    pub fn count(&self) -> Index {
        unsafe { CFArrayGetCount(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = unsafe { cf::Array::create_in(None, std::ptr::null(), 0, None).expect("arr") };
    /// assert_eq!(arr.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr = cf::Array::new().expect("Array::new");
    /// assert_eq!(arr.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new().expect("Array::new");
    /// let arr2 = arr1.copy_in(None).expect("copy");
    ///
    /// ```
    #[inline]
    pub fn copy_in(&self, allocator: Option<&Allocator>) -> Option<Retained<Array>> {
        unsafe { CFArrayCreateCopy(allocator, self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let arr1 = cf::Array::new().expect("Array::new");
    /// let arr2 = arr1.copy().expect("copy");
    /// ```
    #[inline]
    pub fn copy(&self) -> Option<Retained<Array>> {
        self.copy_in(None)
    }

    #[inline]
    pub unsafe fn create_in(
        allocator: Option<&Allocator>,
        values: *const c_void,
        num_values: Index,
        callbacks: Option<&Callbacks>,
    ) -> Option<Retained<Array>> {
        CFArrayCreate(allocator, values, num_values, callbacks)
    }

    #[inline]
    pub fn new() -> Option<Retained<Array>> {
        Self::new_in(None)
    }

    #[inline]
    pub fn new_in(allocator: Option<&Allocator>) -> Option<Retained<Array>> {
        unsafe { Self::create_in(allocator, std::ptr::null(), 0, Callbacks::default()) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10);
    /// let arr = cf::Array::from_type_refs(&[&num, &num, &num]).unwrap();
    /// assert_eq!(3, arr.len());
    /// ```
    #[inline]
    pub fn from_type_refs<const N: usize>(values: &[&Type; N]) -> Option<Retained<Array>> {
        unsafe { Array::create_in(None, values.as_ptr() as _, N as _, Callbacks::default()) }
    }

    #[inline]
    pub fn from_slice<T>(values: &[&T]) -> Option<Retained<Array>>
    where
        T: Retain + Release,
    {
        unsafe {
            Array::create_in(
                None,
                values.as_ptr() as _,
                values.len() as _,
                Callbacks::default(),
            )
        }
    }

    #[inline]
    pub fn from_copyable<const N: usize, T>(values: &[T; N]) -> Option<Retained<Array>>
    where
        T: Copy,
    {
        unsafe { Array::create_in(None, values.as_ptr() as _, N as _, None) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let num = cf::Number::from_i32(10);
    ///
    /// let empty_arr = cf::Array::new().unwrap();
    /// let mut mut_arr = empty_arr.mutable_copy_in(None, 0).unwrap();
    ///
    ///
    /// mut_arr.append(&num);
    ///
    /// assert_eq!(1, mut_arr.len());
    /// assert_eq!(0, empty_arr.len());
    ///
    /// ```
    #[inline]
    pub fn mutable_copy_in(
        &self,
        allocator: Option<&Allocator>,
        capacity: Index,
    ) -> Option<Retained<MutableArray>> {
        unsafe { CFArrayCreateMutableCopy(allocator, capacity, self) }
    }

    #[inline]
    pub fn mutable_copy(&self) -> Option<Retained<MutableArray>> {
        unsafe { CFArrayCreateMutableCopy(None, 0, self) }
    }

    #[inline]
    pub fn mutable_copy_with_capacity(&self, capacity: usize) -> Option<Retained<MutableArray>> {
        self.mutable_copy_in(None, capacity as _)
    }
}

impl std::ops::Index<usize> for Array {
    type Output = Type;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { CFArrayGetValueAtIndex(self, index as _) }
    }
}

define_cf_type!(MutableArray(Array));

impl MutableArray {
    #[inline]
    pub unsafe fn append_value(&mut self, value: *const c_void) {
        CFArrayAppendValue(self, value)
    }

    #[inline]
    pub fn append(&mut self, value: &Type) {
        unsafe { self.append_value(value.as_ptr()) }
    }

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

    #[inline]
    pub fn create_in(
        allocator: Option<&Allocator>,
        capacity: Index,
        callbacks: Option<&Callbacks>,
    ) -> Option<Retained<MutableArray>> {
        unsafe { CFArrayCreateMutable(allocator, capacity, callbacks) }
    }

    #[inline]
    pub fn with_capacity(capacity: Index) -> Option<Retained<MutableArray>> {
        Self::create_in(None, capacity, Callbacks::default())
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let mut arr = cf::MutableArray::new().unwrap();
    /// assert_eq!(0, arr.len());
    ///
    /// let num = cf::Number::from_i32(0);
    ///
    /// arr.append(&num);
    /// arr.append(&num);
    /// assert_eq!(2, arr.len());
    ///
    /// arr.remove_all_values();
    /// assert_eq!(0, arr.len());
    /// ```
    #[inline]
    pub fn new() -> Option<Retained<MutableArray>> {
        Self::with_capacity(0)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFTypeArrayCallBacks: Callbacks;

    fn CFArrayGetTypeID() -> TypeId;

    fn CFArrayGetValueAtIndex(array: &Array, idx: Index) -> &Type;

    fn CFArrayCreate(
        allocator: Option<&Allocator>,
        values: *const c_void,
        num_values: Index,
        callbacks: Option<&Callbacks>,
    ) -> Option<Retained<Array>>;

    fn CFArrayCreateCopy(allocator: Option<&Allocator>, array: &Array) -> Option<Retained<Array>>;

    fn CFArrayGetCount(array: &Array) -> Index;

    fn CFArrayCreateMutable(
        allocator: Option<&Allocator>,
        capacity: Index,
        callbacks: Option<&Callbacks>,
    ) -> Option<Retained<MutableArray>>;

    fn CFArrayCreateMutableCopy(
        allocator: Option<&Allocator>,
        capacity: Index,
        array: &Array,
    ) -> Option<Retained<MutableArray>>;

    fn CFArrayAppendValue(array: &mut MutableArray, value: *const c_void);

    fn CFArrayRemoveAllValues(array: &mut MutableArray);
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    pub fn empty_arrays_are_same() {
        let arr1 = cf::Array::new().expect("Array::new");
        let arr2 = arr1.copy().expect("copy");
        let arr3 = cf::Array::new().expect("Array::new");
        let arr4 = arr2.mutable_copy().expect("copy");
        unsafe {
            assert_eq!(arr1.as_ptr(), arr2.as_ptr());
            assert_eq!(arr3.as_ptr(), arr2.as_ptr());
            assert_ne!(arr1.as_ptr(), arr4.as_ptr());
        }
    }
}
