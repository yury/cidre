use std::{
    ffi::c_void,
    intrinsics::transmute,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use super::TypeID;

pub trait Release {
    unsafe fn release(&self);
}

pub trait Retain: Sized + Release {
    fn retained(&self) -> Option<Retained<Self>>;
}

#[repr(transparent)]
pub struct Retained<T: Release>(NonNull<T>);

impl<T: Retain> Retained<T> {
    #[inline]
    pub fn retained(&self) -> Option<Self> {
        unsafe { self.0.as_ref().retained() }
    }
}

impl<T: Release> Drop for Retained<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}

impl<T: Release> Deref for Retained<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Release> DerefMut for Retained<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

#[repr(transparent)]
pub struct Ltb<'a, T>(T, PhantomData<&'a T>);

impl<'a, T> Deref for Ltb<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Release + Retain> Ltb<'a, Retained<T>> {
    #[inline]
    pub fn retained(&self) -> Option<Ltb<Retained<T>>> {
        unsafe { transmute(self.0.retained()) }
    }
}

#[repr(transparent)]
pub struct Type(c_void);

impl Type {
    #[inline]
    pub unsafe fn retain<T: Release>(cf: &Type) -> Option<Retained<T>> {
        transmute(CFRetain(cf))
    }

    #[inline]
    pub unsafe fn release(cf: &Type) {
        CFRelease(cf)
    }

    #[inline]
    pub fn get_type_id(&self) -> TypeID {
        unsafe { CFGetTypeID(self) }
    }

    pub unsafe fn as_ptr(&self) -> *const c_void {
        &self.0
    }
}

impl Retain for Type {
    #[inline]
    fn retained(&self) -> Option<Retained<Self>> {
        unsafe { Type::retain(self) }
    }
}

impl Release for Type {
    #[inline]
    unsafe fn release(&self) {
        Type::release(self)
    }
}

#[macro_export]
macro_rules! define_cf_type {
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
            unsafe fn release(&self) {
                self.0.release()
            }
        }

        impl crate::cf::runtime::Retain for $NewType {
            #[inline]
            fn retained(&self) -> Option<Retained<Self>> {
                $NewType::retained(self)
            }
        }

        impl $NewType {
            #[inline]
            pub fn retained(&self) -> Option<Retained<Self>> {
                unsafe { Type::retain(self) }
            }
        }
    };
}

extern "C" {
    fn CFRetain(cf: &Type) -> Option<Retained<Type>>;
    fn CFRelease(cf: &Type);
    fn CFGetTypeID(cf: &Type) -> TypeID;
}
