use std::{ffi::c_void, intrinsics::transmute, marker::PhantomData, ops::Deref};

use crate::{cf, define_cf_type};

use super::{
    runtime::{Release, Retain},
    Retained,
};

define_cf_type!(Set(cf::Type));
define_cf_type!(MutableSet(Set));

impl Set {
    #[inline]
    pub fn count(&self) -> cf::Index {
        unsafe { CFSetGetCount(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    #[inline]
    pub fn is_emtpy(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        CFSetContainsValue(self, value)
    }

    #[inline]
    pub unsafe fn get_values(&self, values: *mut *const c_void) {
        CFSetGetValues(self, values)
    }
}

#[repr(transparent)]
pub struct SetOf<T: Retain + Release>(Set, PhantomData<T>);

impl<T> SetOf<T>
where
    T: Retain + Release,
{
    pub fn values<'a>(&self) -> Vec<Retained<'a, T>> {
        let len = self.len();
        let mut vec: Vec<Retained<T>> = Vec::with_capacity(len);
        unsafe {
            vec.set_len(len);
            self.get_values(vec.as_mut_ptr() as _);
        };

        vec
    }
}

impl<T> Deref for SetOf<T>
where
    T: Retain + Release,
{
    type Target = Set;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> Release for SetOf<T>
where
    T: Release + Retain,
{
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> Retain for SetOf<T>
where
    T: Release + Retain,
{
    fn retained<'a>(&self) -> Retained<'a, Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFSetGetCount(set: &Set) -> cf::Index;
    fn CFSetContainsValue(set: &Set, value: *const c_void) -> bool;
    fn CFSetGetValues(set: &Set, values: *mut *const c_void);
}
