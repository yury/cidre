use std::{ffi::c_void, intrinsics::transmute};

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
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[inline]
    pub unsafe fn sel<R>(&self, selector: &Sel) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector)
    }

    #[inline]
    pub unsafe fn sel_a<R, A>(&self, selector: &Sel, a: A) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector, a)
    }

    #[inline]
    pub unsafe fn sel_ab<R, A, B>(&self, selector: &Sel, a: A, b: B) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b)
    }

    #[inline]
    pub unsafe fn sel_abc<R, A, B, C>(&self, selector: &Sel, a: A, b: B, c: C) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c)
    }

    #[inline]
    pub unsafe fn sel_abcd<R, A, B, C, D>(&self, selector: &Sel, a: A, b: B, c: C, d: D) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d)
    }

    #[inline]
    pub unsafe fn sel_abcde<R, A, B, C, D, E>(
        &self,
        selector: &Sel,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
    ) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D, E) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e)
    }

    #[inline]
    pub unsafe fn sel_abcdef<R, A, B, C, D, E, F>(
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
    pub unsafe fn sel_abcdefg<R, A, B, C, D, E, F, G>(
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
pub struct Sel(*const u8);

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
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_abcdefg($sel, $a, $b, $c, $d, $e, $f, $g) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_abcdef($sel, $a, $b, $c, $d, $e, $f) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_abcde($sel, $a, $b, $c, $d, $e) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_abcd($sel, $a, $b, $c, $d) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_abc($sel, $a, $b, $c) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_ab($sel, $a, $b) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel_a($sel, $a) }
    }};

    ($lib:literal, $self:ident, $sel:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel($sel) }
    }};
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);

    fn objc_msgSend(id: &Id, sel: &Sel, args: ...) -> *const c_void;
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

#[repr(C)]
pub struct Delegate<T: Sized> {
    pub delegate: Box<T>,
    pub obj: crate::cf::Retained<Id>,
}

struct ImageInfo {
    _version: u32,
    _flags: u32,
}

#[link_section = "__DATA,__objc_imageinfo"]
#[used] 
static IMAGE_INFO: ImageInfo = ImageInfo {
    _version: 0,
    _flags: 0
};

// #[link_section="__TEXT,__objc_methname,cstring_literals"]
// static STR_ALLOC : [u8; 6] = *b"alloc\0";
// #[link_section="__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
// static SEL_ALLOC: &[u8; 6] = &STR_ALLOC;

// #[link_section="__TEXT,__objc_methname,cstring_literals"]
// static STR_INIT : [u8; 5] = *b"init\0";
// #[link_section="__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
// static SEL_INIT: Sel = Sel(&STR_INIT as *const _);


pub fn sel_processor_count() -> &'static Sel {
    #[link_section="__TEXT,__objc_methname,cstring_literals"]
    static STR : [u8; 15] = *b"processorCount\0";
    #[link_section="__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
    static SEL: &[u8; 15] = &STR;


    unsafe { 
        let ptr = std::ptr::read_volatile(&SEL);
        transmute(ptr) 
    }
}

#[cfg(test)]
mod tests {
    use super::sel_processor_count;


    #[test]
    fn basics(){
        let proc = crate::ns::ProcessInfo::current();        

        let count: usize = unsafe { proc.sel(sel_processor_count()) };

        println!("count {:?}" , count);
    }
}