use std::{
    cell::UnsafeCell,
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    ptr::NonNull,
};

use super::TypeId;

use crate::{arc, cf};

/// Opaque Core Foundation object.
///
/// `&Type` *is* the object pointer, so this struct is deliberately zero-sized
/// and 1-aligned: it must not claim any bytes (`dereferenceable`) or alignment
/// for the pointee, otherwise references to tagged pointers would be UB.
///
/// The `UnsafeCell` field makes the type `!Freeze`, so `&Type` is not lowered
/// as `readonly` and the runtime may mutate the object (refcounts, caches)
/// behind a shared reference.
///
/// `!Send`, `!Sync` and `!Unpin` like the `NonNull` it replaced.
#[repr(C)]
pub struct Type {
    _priv: [u8; 0],
    _marker: UnsafeCell<PhantomData<(*const c_void, PhantomPinned)>>,
}

impl Type {
    #[inline]
    pub unsafe fn retain<T: arc::Release>(cf: &Type) -> arc::R<T> {
        unsafe { std::mem::transmute(CFRetain(cf)) }
    }

    /// Releases one ownership reference for a Core Foundation object.
    ///
    /// # Safety
    ///
    /// `cf` must carry a live ownership reference that has not already been
    /// released.
    #[inline]
    pub unsafe fn release(cf: NonNull<Type>) {
        unsafe { CFRelease(cf.as_ptr()) }
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
    unsafe fn release(ptr: NonNull<Self>) {
        unsafe { Type::release(ptr) }
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
        // `repr(C)`, not `repr(transparent)`: the base is zero-sized, and the
        // `improper_ctypes` lint rejects transparent wrappers over ZSTs.
        #[repr(C)]
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
            unsafe fn release(ptr: std::ptr::NonNull<Self>) {
                unsafe {
                    <$BaseType as $crate::arc::Release>::release(ptr.cast())
                }
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

        impl AsRef<$crate::cf::Type> for $NewType {
            fn as_ref(&self) -> &$crate::cf::Type {
                self
            }
        }
    };
}

impl Type {
    pub fn try_as_number(&self) -> Option<&cf::Number> {
        if self.get_type_id() == cf::Number::type_id() {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }

    pub fn try_as_string(&self) -> Option<&cf::String> {
        if self.get_type_id() == cf::String::type_id() {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }
}

unsafe extern "C-unwind" {
    fn CFRetain(cf: &Type) -> arc::R<Type>;
    fn CFRelease(cf: *const Type);
    fn CFGetTypeID(cf: &Type) -> TypeId;
}

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn opaque_layout() {
        // `&Type` is the object pointer itself; the struct must claim no bytes
        // and no alignment so tagged pointers are valid references.
        assert_eq!(std::mem::size_of::<Type>(), 0);
        assert_eq!(std::mem::align_of::<Type>(), 1);
    }
}
