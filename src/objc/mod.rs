use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull};

use crate::cf::{
    runtime::{Release, Retain},
    Retained, Type,
};

#[repr(transparent)]
pub struct Class(Type);

impl Class {
    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }
}

#[repr(transparent)]
pub struct Id(Type);

impl Id {
    #[inline]
    pub unsafe fn retain<'a, T: Release>(id: &Id) -> Retained<'a, T> {
        transmute(objc_retain(id))
    }

    #[inline]
    pub unsafe fn release(id: &mut Id) {
        objc_release(id)
    }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[inline]
    pub unsafe fn rsel<R>(&self, selector: &Sel) -> R {
        let imp: unsafe extern fn(&Id, &Sel) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector)
    }

    #[inline]
    pub unsafe fn wsel(&self, selector: &Sel) {
        let imp: unsafe extern fn(&Id, &Sel) = transmute(objc_msgSend as *const c_void);
        imp(self, selector)
    }

    #[inline]
    pub unsafe fn wsel_a<A>(&self, selector: &Sel, a: A) {
        let imp: unsafe extern fn(&Id, &Sel, A) = transmute(objc_msgSend as *const c_void);
        imp(self, selector, a)
    }

    #[inline]
    pub unsafe fn wsel_ab<A, B>(&self, selector: &Sel, a: A, b: B) {
        let imp: unsafe extern fn(&Id, &Sel, A, B) = transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b)
    }

    #[inline]
    pub unsafe fn wsel_abc<A, B, C>(&self, selector: &Sel, a: A, b: B, c: C) {
        let imp: unsafe extern fn(&Id, &Sel, A, B, C) = transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c)
    }
}

impl Retain for Id {
    #[inline]
    fn retained<'a>(&self) -> Retained<'a, Self> {
        unsafe { Id::retain(self) }
    }
}

impl Release for Id {
    #[inline]
    unsafe fn release(&mut self) {
        Id::release(self)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Sel(NonNull<c_void>);

pub mod autorelease_pool;
pub mod block;
pub mod ns;
pub use autorelease_pool::AutoreleasePoolPage;

pub fn autoreleasepool<'a, T, F>(f: F) -> T
where
    F: FnOnce() -> T,
    T: Clone,
{
    let _page = AutoreleasePoolPage::push();
    f()
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);

    // static objc_msgSend: *const c_void;

    // #[link_name = "objc_msgSend"]
    fn objc_msgSend(id: &Id, args:...) -> *const c_void;
}

#[macro_export]
macro_rules! define_obj_type {
    ($NewType:ident($BaseType:path)) => {
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
                unsafe { crate::objc::Id::retain(self) }
            }
        }
    };
}
