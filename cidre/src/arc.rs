#[cfg(feature = "objc")]
use crate::objc;

#[cfg(feature = "objc")]
use std::{
    marker::PhantomData,
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
#[inline]
#[must_use]
pub unsafe fn return_opt_ar<T: objc::Obj>(val: Option<&T>) -> Option<Rar<T>> {
    unsafe {
        let res = objc::objc_autoreleaseReturnValue(std::mem::transmute(val));
        std::mem::transmute(res)
    }
}

#[cfg(feature = "objc")]
#[inline]
#[must_use]
pub unsafe fn return_rar<T: objc::Obj>(val: &T) -> Rar<T> {
    unsafe {
        let res = objc::objc_retainAutoreleaseReturnValue(std::mem::transmute(val));
        std::mem::transmute(res)
    }
}

#[cfg(feature = "objc")]
#[inline]
#[must_use]
pub unsafe fn return_opt_rar<T: objc::Obj>(val: Option<&T>) -> Option<Rar<T>> {
    unsafe {
        let res = objc::objc_retainAutoreleaseReturnValue(std::mem::transmute(val));
        std::mem::transmute(res)
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
    pub unsafe fn return_ar(self) -> Rar<T>
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

/// return objc_autoreleaseReturnValue(objc_retain(value))
#[macro_export]
macro_rules! return_rar {
    ($r:expr) => {
        return unsafe { $crate::arc::return_rar($r) }
    };
}

#[macro_export]
macro_rules! return_opt_ar {
    ($r:expr) => {
        return unsafe { $crate::arc::return_opt_ar($r) }
    };
}

#[macro_export]
macro_rules! return_opt_rar {
    ($r:expr) => {
        return unsafe { $crate::arc::return_opt_rar($r) }
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
#[derive(Debug)]
pub struct Weak<T: objc::Obj> {
    slot: Box<*mut objc::Id>,
    marker: PhantomData<T>,
}

#[cfg(feature = "objc")]
impl<T: objc::Obj> Weak<T> {
    #[inline]
    fn slot_ptr(&self) -> *mut *mut objc::Id {
        self.slot.as_ref() as *const *mut objc::Id as *mut *mut objc::Id
    }

    #[inline]
    pub fn new() -> Self {
        let slot = Box::new(std::ptr::null_mut());
        // unsafe {
        //     objc::objc_storeWeak(&mut *slot, None);
        // }
        Self {
            slot,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_retained(val: &Retained<T>) -> Self {
        let mut slot = Box::new(std::ptr::null_mut());
        unsafe {
            objc::objc_storeWeak(&mut *slot, Some(val.as_id_ref()));
        }
        Self {
            slot,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn upgrade(&self) -> Option<Retained<T>> {
        unsafe { std::mem::transmute(objc::objc_loadWeakRetained(self.slot_ptr())) }
    }
}

#[cfg(feature = "objc")]
impl<T: objc::Obj> Default for Weak<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "objc")]
impl<T: objc::Obj> Clone for Weak<T> {
    #[inline]
    fn clone(&self) -> Self {
        let mut weak = Self {
            slot: Box::new(std::ptr::null_mut()),
            marker: PhantomData,
        };

        unsafe {
            objc::objc_copyWeak(&mut *weak.slot, self.slot_ptr());
        }

        weak
    }
}

#[cfg(feature = "objc")]
impl<T: objc::Obj> Drop for Weak<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            objc::objc_storeWeak(&mut *self.slot, None);
        }
    }
}

#[cfg(feature = "objc")]
unsafe impl<T: objc::Obj + Sync> Sync for Weak<T> {}

#[cfg(feature = "objc")]
unsafe impl<T: objc::Obj + Send> Send for Weak<T> {}

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

#[cfg(feature = "objc")]
#[inline]
pub fn downgrade<T: objc::Obj>(val: &Retained<T>) -> Weak<T> {
    Weak::from_retained(val)
}

impl<T: Release> std::borrow::Borrow<T> for R<T> {
    fn borrow(&self) -> &T {
        &self
    }
}

#[cfg(feature = "objc")]
impl<T: objc::Obj> std::borrow::BorrowMut<T> for R<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

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

/// Accept a value returned through a +0 autoreleasing convention for use at +1,
/// without a NOP in the caller on ARM64.
#[doc(alias = "objc_claimAutoreleasedReturnValue")]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "objc")]
#[inline]
pub fn rar_claim_value<T: objc::Obj>() -> Option<R<T>> {
    unsafe { std::mem::transmute(objc::objc_claimAutoreleasedReturnValue()) }
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

#[cfg(all(test, feature = "objc"))]
mod tests {
    use crate::{arc, objc};
    use std::sync::{
        Arc, Once,
        atomic::{AtomicBool, Ordering},
    };

    struct D(Arc<AtomicBool>);

    impl Drop for D {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    crate::define_obj_type!(WeakTestObj, D, WEAK_TEST_OBJ);

    static INIT: Once = Once::new();

    fn init_cls() {
        INIT.call_once(|| {
            let _ = WeakTestObj::cls();
        });
    }

    #[test]
    fn weak_upgrades_while_strong_exists() {
        init_cls();
        let dropped = Arc::new(AtomicBool::new(false));
        let o = WeakTestObj::with(D(Arc::clone(&dropped)));
        let w = arc::Weak::from_retained(&o);

        let strong = w.upgrade();
        assert!(strong.is_some());
    }

    #[test]
    fn weak_clears_after_drop() {
        init_cls();
        let dropped = Arc::new(AtomicBool::new(false));
        let dropped_in = Arc::clone(&dropped);
        let w = objc::ar_pool(|| {
            let o = WeakTestObj::with(D(dropped_in));
            let w = arc::Weak::from_retained(&o);
            assert!(w.upgrade().is_some());
            w
        });

        assert!(dropped.load(Ordering::SeqCst));
        assert!(w.upgrade().is_none());
    }

    #[test]
    fn weak_clears_after_drop_no_ar_pool() {
        init_cls();
        let dropped = Arc::new(AtomicBool::new(false));
        let dropped_in = Arc::clone(&dropped);
        let w = {
            let o = WeakTestObj::with(D(dropped_in));
            let w = arc::Weak::from_retained(&o);
            assert!(w.upgrade().is_some());
            w
        };

        assert!(dropped.load(Ordering::SeqCst));
        assert!(w.upgrade().is_none());
    }

    #[test]
    fn weak_new_is_empty() {
        init_cls();
        let w: arc::Weak<WeakTestObj> = arc::Weak::new();
        assert!(w.upgrade().is_none());
    }

    #[test]
    fn weak_default_is_empty() {
        init_cls();
        let w: arc::Weak<WeakTestObj> = Default::default();
        assert!(w.upgrade().is_none());
    }

    #[test]
    fn weak_clone_tracks_slot() {
        init_cls();
        let dropped = Arc::new(AtomicBool::new(false));
        let dropped_in = Arc::clone(&dropped);
        let w = objc::ar_pool(|| {
            let o = WeakTestObj::with(D(dropped_in));
            let w = arc::Weak::from_retained(&o);
            let w2 = w.clone();

            assert!(w.upgrade().is_some());
            assert!(w2.upgrade().is_some());

            (w, w2)
        });

        let (w1, w2) = w;
        assert!(dropped.load(Ordering::SeqCst));
        assert!(w1.upgrade().is_none());
        assert!(w2.upgrade().is_none());
    }
}
