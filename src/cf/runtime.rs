use std::{
    ffi::c_void,
    intrinsics::transmute,
    ops::{Deref, DerefMut},
};

use super::TypeId;

pub trait Release {
    unsafe fn release(&mut self);
}

pub trait Retain: Sized + Release {
    fn retained<'a>(&self) -> Retained<'a, Self>;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Retained<'a, T: Release>(&'a mut T);

impl<'a, T: Retain> Retained<'a, T> {
    #[inline]
    pub fn retained(&self) -> Self {
        self.0.retained()
    }
}

impl<'a, T: Release> Drop for Retained<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.0.release() }
    }
}

impl<'a, T: Release> Deref for Retained<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: Release> DerefMut for Retained<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// ```
/// use cidre::cf;
///
/// let n = cf::Number::from_i8(10);
///
/// let f = {
///     n.clone()
/// };
///
/// assert!(f.equal(&n));
/// ```
impl<'a, T: Retain> Clone for Retained<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        self.retained()
    }
}

// #[derive(Debug)]
#[repr(transparent)]
pub struct Type(c_void);

impl Type {
    #[inline]
    pub unsafe fn retain<'a, T: Release>(cf: &Type) -> Retained<'a, T> {
        transmute(CFRetain(cf))
    }

    #[inline]
    pub unsafe fn release(cf: &mut Type) {
        CFRelease(cf)
    }

    #[inline]
    pub fn get_type_id(&self) -> TypeId {
        unsafe { CFGetTypeID(self) }
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const c_void {
        self as *const Type as _
    }
}

impl Retain for Type {
    #[inline]
    fn retained<'a>(&self) -> Retained<'a, Self> {
        unsafe { Type::retain(self) }
    }
}

impl Release for Type {
    #[inline]
    unsafe fn release(&mut self) {
        Type::release(self)
    }
}

#[macro_export]
macro_rules! define_cf_type {
    ($NewType:ident($BaseType:path)) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $NewType($BaseType);

        impl std::ops::Deref for $NewType {
            type Target = $BaseType;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $NewType {
            #[inline]
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
                unsafe { crate::cf::runtime::Type::retain(self) }
            }
        }
    };
}

extern "C" {
    fn CFRetain(cf: &Type) -> Retained<Type>;
    fn CFRelease(cf: &mut Type);
    fn CFGetTypeID(cf: &Type) -> TypeId;
}
