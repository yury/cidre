use core::{marker::PhantomData, ptr::NonNull};

use super::abi;

#[derive(Debug)]
#[repr(transparent)]
pub struct Object<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Object<T> {}
unsafe impl<T: Sync> Sync for Object<T> {}

impl<T> Object<T> {
    /// Takes ownership of a Swift native class object reference.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid owned Swift class object reference.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        let ptr = NonNull::new(ptr).expect("swift object must not be null");
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[inline]
    pub fn into_raw(self) -> *mut T {
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }
}

impl<T> Clone for Object<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            abi::object_retain(self.as_raw().cast());
            Self::from_raw(self.as_raw())
        }
    }
}

impl<T> Drop for Object<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::object_release(self.as_raw().cast()) }
    }
}
