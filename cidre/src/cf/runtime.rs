use std::{ffi::c_void, ptr::NonNull};

use super::TypeId;

use crate::arc;

// #[derive(Debug)]
#[repr(transparent)]
pub struct Type(NonNull<c_void>);

impl Type {
    #[inline]
    pub unsafe fn retain<T: arc::Release>(cf: &Type) -> arc::R<T> {
        std::mem::transmute(CFRetain(cf))
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
    pub unsafe fn as_type_ptr(&self) -> *const c_void {
        self as *const Type as _
    }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        self
    }

    #[cfg(not(target_os = "watchos"))]
    #[inline]
    pub fn is_tagged_ptr(&self) -> bool {
        ((self as *const Self as usize) >> 63) == 1
    }
}

impl arc::Retain for Type {
    #[inline]
    fn retained<'a>(&self) -> arc::R<Self> {
        unsafe { Type::retain(self) }
    }
}

impl arc::Release for Type {
    #[inline]
    unsafe fn release(&mut self) {
        Type::release(self)
    }
}

#[macro_export]
macro_rules! define_cf_type {
    (
        $(#[$outer:meta])*
        $NewType:ident($BaseType:path)
    ) => {
        $(#[$outer])*
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

        impl $crate::arc::Release for $NewType {
            #[inline]
            unsafe fn release(&mut self) {
                self.0.release()
            }
        }

        impl $crate::arc::Retain for $NewType {
            #[inline]
            fn retained(&self) -> crate::arc::R<Self> {
                $NewType::retained(self)
            }
        }

        impl $NewType {
            #[inline]
            pub fn retained(&self) -> crate::arc::R<Self> {
                unsafe { crate::cf::Type::retain(self) }
            }
        }
    };
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFRetain(cf: &Type) -> arc::R<Type>;
    fn CFRelease(cf: &mut Type);
    fn CFGetTypeID(cf: &Type) -> TypeId;
}
