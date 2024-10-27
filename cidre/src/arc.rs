#[cfg(feature = "objc")]
use crate::objc;

#[cfg(feature = "objc")]
use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
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

impl<T: Release + std::error::Error> std::error::Error for Retained<T> {}

impl<T: Release + std::fmt::Display> std::fmt::Display for Retained<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Retain + PartialEq> PartialEq for Retained<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Retain + PartialEq> PartialEq<T> for Retained<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == other
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
}

#[cfg(feature = "objc")]
impl<T: Retain> Retained<T> {
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

impl<T: Release> std::ops::Deref for Retained<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Release> std::ops::DerefMut for Retained<T> {
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

#[cfg(feature = "objc")]
#[repr(transparent)]
pub struct ReturnedAutoReleased<T: objc::Obj>(NonNull<T>);

#[cfg(feature = "objc")]
impl<T: objc::Obj> Deref for ReturnedAutoReleased<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

#[cfg(feature = "objc")]
impl<T: objc::Obj> DerefMut for ReturnedAutoReleased<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

pub type A<T> = Allocated<T>;
pub type R<T> = Retained<T>;
#[cfg(feature = "objc")]
pub type Rar<T> = ReturnedAutoReleased<T>;

#[cfg(target_arch = "aarch64")]
#[cfg(feature = "objc")]
#[inline]
pub fn rar_retain_option<T: objc::Obj>(id: Option<Rar<T>>) -> Option<R<T>> {
    use std::arch::asm;

    unsafe {
        // see comments in rar_retain
        asm!("mov x29, x29");

        std::mem::transmute(objc::objc_retainAutoreleasedReturnValue(
            std::mem::transmute(id),
        ))
    }
}

#[cfg(target_arch = "x86_64")]
#[cfg(feature = "objc")]
#[inline]
pub fn rar_retain_option<T: objc::Obj>(id: Option<Rar<T>>) -> Option<R<T>> {
    // since we can't insert marker right before actual `objc_msgSend` we fallback to retain
    unsafe { std::mem::transmute(objc::objc_retain(std::mem::transmute(id))) }
}

#[cfg(feature = "objc")]
#[cfg(target_arch = "aarch64")]
#[inline]
pub fn rar_retain<T: objc::Obj>(id: Rar<T>) -> R<T> {
    use std::arch::asm;

    unsafe {
        // latest runtimes don't need this marker anymore.
        // see https://developer.apple.com/videos/play/wwdc2022/110363/ at 13:24
        // but benchmarks show that on macos it is not a case yet
        // (see alloc_with_ar_retain bench).
        // Need to check on iOS.
        asm!("mov x29, x29");

        std::mem::transmute(objc::objc_retainAutoreleasedReturnValue(
            std::mem::transmute(id),
        ))
    }
}

#[cfg(feature = "objc")]
#[cfg(target_arch = "x86_64")]
#[inline]
pub fn rar_retain<T: objc::Obj>(id: Rar<T>) -> R<T> {
    // asm!("mov rax, rdi");
    // since we can't insert marker right before actual `objc_msgSend` we fallback to retain
    unsafe { std::mem::transmute(objc::objc_retain(std::mem::transmute(id))) }
}
