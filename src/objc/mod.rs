use std::{ffi::c_void, intrinsics::transmute};

use crate::cf::{
    runtime::{Release, Retain},
    Retained, Type,
};

#[repr(transparent)]
struct ObjcClass(c_void);

#[repr(transparent)]
pub struct Class(ObjcClass);

#[repr(transparent)]
pub struct Id(Type);

impl Id {
    #[inline]
    pub unsafe fn retain<'a, T: Release>(id: &Id) -> Retained<'a, T> {
        transmute(objc_retain(id))
    }

    #[inline]
    pub unsafe fn release(id: &mut Id) {
        objc_release(id)
    }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
      &self.0
    }
}

impl Retain for Id {
    #[inline]
    fn retained<'a>(&self) -> Retained<'a, Self> {
        unsafe { Id::retain(self) }
    }
}

impl Release for Id {
    unsafe fn release(&mut self) {
      Id::release(self)
    }
}

#[repr(transparent)]
pub struct Sel(c_void);

pub mod ns;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);
    pub fn objc_msgSend<'a>(id: &Id, op: &Sel, ...) -> Option<&'a Id>;
}
