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
    fn retained(&self) -> Retained<Self>;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Retained<T: Release + 'static>(&'static mut T);

impl<T: Retain + PartialEq> PartialEq for Retained<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Retain + PartialOrd> PartialOrd for Retained<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Retain> Retained<T> {
    #[inline]
    pub fn retained(&self) -> Self {
        self.0.retained()
    }
}

impl<T: Release> Drop for Retained<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.0.release() }
    }
}

impl<T: Release> Deref for Retained<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Release> DerefMut for Retained<T> {
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
impl<T: Retain> Clone for Retained<T> {
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
    pub unsafe fn retain<T: Release>(cf: &Type) -> Retained<T> {
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
    fn retained<'a>(&self) -> Retained<Self> {
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
            fn retained(&self) -> crate::cf::runtime::Retained<Self> {
                $NewType::retained(self)
            }
        }

        impl $NewType {
            #[inline]
            pub fn retained(&self) -> crate::cf::runtime::Retained<Self> {
                unsafe { crate::cf::runtime::Type::retain(self) }
            }
        }
    };
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRetain(cf: &Type) -> Retained<Type>;
    fn CFRelease(cf: &mut Type);
    fn CFGetTypeID(cf: &Type) -> TypeId;
}
