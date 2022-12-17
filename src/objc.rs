use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull, marker::PhantomData};

use crate::cf::{
    runtime::{Release, Retain},
    Retained, Type,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Class(Type);

impl Class {
    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }
}

/// Use it as NSObject or id
#[repr(transparent)]
pub struct Id(Type);

impl Id {
    #[inline]
    pub unsafe fn retain<T: Release>(id: &Id) -> Retained<T> {
        transmute(objc_retain(id))
    }

    #[inline]
    pub unsafe fn release(id: &mut Id) {
        objc_release(id)
    }

    #[inline]
    pub unsafe fn autorelease<'ar>(id: &Id) -> &mut Id {
        objc_autorelease(id)
    }

    #[inline]
    pub unsafe fn return_ar<'ar>(id: &Id) -> *mut Id {
        objc_autoreleaseReturnValue(id)
    }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[inline]
    pub unsafe fn sel0<R>(&self, selector: &Sel) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector)
    }

    #[inline]
    pub unsafe fn sel1<R, A>(&self, selector: &Sel, a: A) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector, a)
    }

    #[inline]
    pub unsafe fn sel2<R, A, B>(&self, selector: &Sel, a: A, b: B) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b)
    }

    #[inline]
    pub unsafe fn sel3<R, A, B, C>(&self, selector: &Sel, a: A, b: B, c: C) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c)
    }

    #[inline]
    pub unsafe fn sel4<R, A, B, C, D>(&self, selector: &Sel, a: A, b: B, c: C, d: D) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d)
    }

    #[inline]
    pub unsafe fn sel5<R, A, B, C, D, E>(&self, selector: &Sel, a: A, b: B, c: C, d: D, e: E) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D, E) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e)
    }

    #[inline]
    pub unsafe fn sel6<R, A, B, C, D, E, F>(
        &self,
        selector: &Sel,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
    ) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D, E, F) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e, f)
    }
    #[inline]
    pub unsafe fn sel7<R, A, B, C, D, E, F, G>(
        &self,
        selector: &Sel,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
    ) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D, E, F, G) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e, f, g)
    }
}

impl Retain for Id {
    #[inline]
    fn retained(&self) -> Retained<Self> {
        unsafe { Id::retain(self) }
    }
}

impl Release for Id {
    #[inline]
    unsafe fn release(&mut self) {
        Id::release(self)
    }
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self
            .as_type_ref()
            .description()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "no desc".to_string());
        f.debug_tuple("NS").field(&desc).finish()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Sel(NonNull<c_void>);

pub mod autorelease_pool;
pub mod ns;
pub use autorelease_pool::AutoreleasePoolPage;

pub fn autoreleasepool<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
    T: Clone, // Autoreleased doesn't implement Clone
{
    let _page = AutoreleasePoolPage::push();
    f()
}

#[macro_export]
macro_rules! msg_send {
    // TODO: we should pass name and kind
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel7($sel, $a, $b, $c, $d, $e, $f, $g) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel6($sel, $a, $b, $c, $d, $e, $f) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel5($sel, $a, $b, $c, $d, $e) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel4($sel, $a, $b, $c, $d) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr, $c:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel3($sel, $a, $b, $c) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:expr, $b:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel2($sel, $a, $b) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:expr) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel1($sel, $a) }
    }};

    ($lib:literal, $self:ident, $sel:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel0($sel) }
    }};
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);

    fn objc_msgSend(id: &Id, sel: &Sel, args: ...) -> *const c_void;
    fn objc_autorelease<'a>(id: &Id) -> &'a mut Id;

    fn objc_autoreleaseReturnValue<'a>(id: &Id) -> &'a mut Id;
    fn objc_retainAutoreleasedReturnValue(id: *const Id) -> *mut Id;
}

#[macro_export]
macro_rules! define_obj_type {
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
                unsafe { crate::objc::Id::retain(self) }
            }
        }

        impl crate::cf::runtime::Retained<$NewType> {
            #[must_use]
            pub fn autoreleased<'ar>(self) -> &'ar mut $NewType {
                unsafe {
                    let res = crate::objc::Id::autorelease(std::mem::transmute(self));
                    return std::mem::transmute(res);
                }
            }

            /// Do not use it for now
            #[must_use]
            pub unsafe fn return_ar<'ar>(self) -> &'ar crate::objc::AR<$NewType> {
                unsafe {
                    let res = crate::objc::Id::return_ar(std::mem::transmute(self));
                    return std::mem::transmute(res);
                }
            }
        }

        // impl std::fmt::Debug for $NewType {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         let desc = self
        //             .copy_description()
        //             .map(|f| f.to_string())
        //             .unwrap_or_else(|| "no desc".to_string());
        //         f.debug_tuple("cf::Type").field(&desc).finish()
        //     }
        // }
    };
}

#[repr(transparent)]
pub struct AR<T>(Id, PhantomData<T>);

impl<T> AR<T> where T: Retain {
    #[inline(always)]
    pub fn retain(&self) -> Retained<T> {
        unsafe {
            // std::arch::asm!("mov r7, r7", options(raw, nomem));
            transmute(objc_retainAutoreleasedReturnValue(transmute(self)))
        }
    }
}

#[repr(transparent)]
pub struct OptionAR<T>(Id, PhantomData<T>);

impl<T> OptionAR<T> where T: Retain {
    #[inline(always)]
    pub fn retain(&self) -> Option<Retained<T>> {
        unsafe {
            // std::arch::asm!("mov r7, r7");
            transmute(objc_retainAutoreleasedReturnValue(transmute(self)))
        }
    }
}

#[repr(C)]
pub struct Delegate<T: Sized> {
    pub delegate: Box<T>,
    pub obj: crate::cf::Retained<Id>,
}

#[cfg(test)]
mod tests {

    use super::{autoreleasepool, AR};
    use crate::{cf, dispatch};
    use std;

    fn autorelease_example<'ar>() -> &'ar mut dispatch::Queue {
        let q = dispatch::Queue::new();
        q.autoreleased()
    }

    fn autorelease_return_example<'ar>() -> &'ar AR<dispatch::Queue> {
        let q = dispatch::Queue::new();
        // assert_eq!(1, q.as_type_ref().retain_count());
        unsafe { q.return_ar() }
    }

    #[test]
    fn autorelease() {
        let ptr = autoreleasepool(|| {
            let q = autorelease_example();
            assert_eq!(1, q.as_type_ref().retain_count());
            unsafe { q.as_type_ref().as_type_ptr() }
        });

        let ptr: &cf::Type = unsafe { std::mem::transmute(ptr) };
        // expect crash: ptr.show()
    }

    /// this test shows that we can't use autorelease_rt for now
    /// we need `unsafe { std::arch::asm!("mov r7, r7", options(raw)) }; ` to work
    #[test]
    fn autorelease_ar() {
        let ptr = autoreleasepool(|| {
            let q = autorelease_return_example().retain();
            // this should be 1 actually
            assert_eq!(2, q.as_type_ref().retain_count());
            q
        });

        assert_eq!(1, ptr.as_type_ref().retain_count());
    }    
}

