use crate::objc;
use std::{
    arch::asm,
    ops::{Deref, DerefMut},
};

pub trait Release {
    unsafe fn release(&mut self);
}

pub trait Retain: Sized + Release {
    fn retained(&self) -> Retained<Self>;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Allocated<T: Release + 'static>(&'static mut T);

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

impl<T: Retain> AsRef<T> for Retained<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.0
    }
}

impl<T: Retain> AsMut<T> for Retained<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.0
    }
}

impl<T: Retain> Retained<T> {
    #[inline]
    pub fn retained(&self) -> Self {
        self.0.retained()
    }

    #[must_use]
    pub fn autoreleased<'ar>(self) -> &'ar mut T
    where
        T: objc::Obj,
    {
        unsafe {
            let res = objc::Id::autorelease(std::mem::transmute(self));
            std::mem::transmute(res)
        }
    }

    #[must_use]
    pub unsafe fn return_ar<'ar>(self) -> &'ar mut T
    where
        T: objc::Obj,
    {
        unsafe {
            let res = objc::objc_autoreleaseReturnValue(std::mem::transmute(self));
            std::mem::transmute(res)
        }
    }

    // /// #Safety
    // /// Use `return_ar` macro
    // #[inline]
    // pub unsafe fn return_ar(self) -> crate::arc::Rar<T>
    // where
    //     T: objc::Obj,
    // {
    //     unsafe { std::mem::transmute(objc::objc_autoreleaseReturnValue(std::mem::transmute(self))) }
    // }
}

impl<T: Release> Drop for Allocated<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.0.release() }
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

#[macro_export]
macro_rules! return_ar {
    ($r:path) => {
        return unsafe { $r.return_ar() }
    };
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

pub type A<T> = Allocated<T>;
pub type R<T> = Retained<T>;

#[inline]
pub fn rar_retain_option<T: objc::Obj>(id: Option<&T>) -> Option<R<T>> {
    unsafe {
        // see comments in rar_retain
        asm!("mov x29, x29");
        std::mem::transmute(objc::objc_retainAutoreleasedReturnValue(
            std::mem::transmute(id),
        ))
    }
}

#[inline]
pub fn rar_retain<T: objc::Obj>(id: &T) -> R<T> {
    unsafe {
        // latest runtimes don't need this marker anymore.
        // see https://developer.apple.com/videos/play/wwdc2022/110363/ at 13:24
        // but mechmarks show that on macos it is not a case yet. Need to check on iOS.
        asm!("mov x29, x29");
        std::mem::transmute(objc::objc_retainAutoreleasedReturnValue(
            std::mem::transmute(id),
        ))
    }
}
