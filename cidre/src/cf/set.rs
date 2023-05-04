use std::{ffi::c_void, intrinsics::transmute, marker::PhantomData, ops::Deref};

use crate::{arc, cf, define_cf_type};

#[cfg(freature = "ns")]
use crate::ns;

#[cfg(freature = "ns")]
use crate::objc::Obj;

define_cf_type!(Set(cf::Type));
define_cf_type!(SetMut(Set));

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
    pub fn is_empty(&self) -> bool {
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

#[derive(Debug)]
#[repr(transparent)]
pub struct SetOf<T: arc::Retain + arc::Release>(Set, PhantomData<T>);

impl<T> SetOf<T>
where
    T: arc::Retain + arc::Release,
{
    pub fn values(&self) -> Vec<arc::R<T>> {
        let len = self.len();
        let mut vec: Vec<arc::R<T>> = Vec::with_capacity(len);
        unsafe {
            vec.set_len(len);
            self.get_values(vec.as_mut_ptr() as _);
        };

        vec
    }
}

#[cfg(freature = "ns")]
impl<T: Obj> SetOf<T> {
    pub fn as_ns(&self) -> &ns::Set<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Deref for SetOf<T>
where
    T: arc::Retain + arc::Release,
{
    type Target = Set;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> arc::Release for SetOf<T>
where
    T: arc::Release + arc::Retain,
{
    unsafe fn release(&mut self) {
        self.0.release()
    }
}

impl<T> arc::Retain for SetOf<T>
where
    T: arc::Release + arc::Retain,
{
    fn retained(&self) -> arc::R<Self> {
        unsafe { transmute(self.0.retained()) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFSetGetCount(set: &Set) -> cf::Index;
    fn CFSetContainsValue(set: &Set, value: *const c_void) -> bool;
    fn CFSetGetValues(set: &Set, values: *mut *const c_void);
}
