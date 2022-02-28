use std::{ffi::c_void, intrinsics::transmute};

use crate::cf::{
    runtime::{Release, Retain},
    Retained, Type,
};

#[repr(transparent)]
pub struct Class(Type);

impl Class {
    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }
}

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

#[derive(Debug)]
#[repr(transparent)]
pub struct Sel(c_void);

pub mod ns;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);
}

#[macro_export]
macro_rules! define_obj_type {
    ($NewType:ident($BaseType:path)) => {
        #[repr(transparent)]
        pub struct $NewType($BaseType);

        impl std::ops::Deref for $NewType {
            type Target = $BaseType;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $NewType {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl crate::cf::runtime::Release for $NewType {
            #[inline]
            unsafe fn release(&mut self) {
                self.0.release()
            }
        }

        impl crate::cf::runtime::Retain for $NewType {
            #[inline]
            fn retained<'a>(&self) -> crate::cf::runtime::Retained<'a, Self> {
                $NewType::retained(self)
            }
        }

        impl $NewType {
            #[inline]
            pub fn retained<'a>(&self) -> crate::cf::runtime::Retained<'a, Self> {
                unsafe { crate::objc::Id::retain(self) }
            }
        }
    };
}
