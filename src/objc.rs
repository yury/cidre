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
    pub unsafe fn sel0<R>(&self, selector: &Sel) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector)
    }

    #[inline]
    pub unsafe fn sel1<A, R>(&self, selector: &Sel, a: A) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A) -> R = transmute(objc_msgSend as *const c_void);
        imp(self, selector, a)
    }

    #[inline]
    pub unsafe fn sel2<A, B, R>(&self, selector: &Sel, a: A, b: B) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b)
    }

    #[inline]
    pub unsafe fn sel3<A, B, C, R>(&self, selector: &Sel, a: A, b: B, c: C) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c)
    }

    #[inline]
    pub unsafe fn sel4<A, B, C, D, R>(&self, selector: &Sel, a: A, b: B, c: C, d: D) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d)
    }

    #[inline]
    pub unsafe fn sel5<A, B, C, D, E, R>(&self, selector: &Sel, a: A, b: B, c: C, d: D, e: E) -> R {
        let imp: unsafe extern "C" fn(&Id, &Sel, A, B, C, D, E) -> R =
            transmute(objc_msgSend as *const c_void);
        imp(self, selector, a, b, c, d, e)
    }

    #[inline]
    pub unsafe fn sel6<A, B, C, D, E, F, R>(
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
    pub unsafe fn sel7<A, B, C, D, E, F, G, R>(
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
macro_rules! ext_msg_send {
    // TODO: we should pass name and kind
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel7($sel, $a, $b, $c, $d, $e, $f, $g) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel6($sel, $a, $b, $c, $d, $e, $f) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel5($sel, $a, $b, $c, $d, $e) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident, $d:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel4($sel, $a, $b, $c, $d) }
    }};
    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident, $c:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel3($sel, $a, $b, $c) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:ident, $b:ident) => {{
        #[link(name = $lib, kind = "static")]
        extern "C" {
            static $sel: &'static crate::objc::Sel;
        }

        unsafe { $self.sel2($sel, $a, $b) }
    }};

    ($lib:literal, $self:ident, $sel:ident, $a:ident) => {{
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

#[macro_export]
macro_rules! msg_send {
    ($self:ident, $sel:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {{
        unsafe { $self.sel7($sel, $a, $b, $c, $d, $e, $f, $g) }
    }};

    ($self:ident, $sel:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {{
        unsafe { $self.sel6($sel, $a, $b, $c, $d, $e, $f) }
    }};

    ($self:ident, $sel:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        unsafe { $self.sel5($sel, $a, $b, $c, $d, $e) }
    }};

    ($self:ident, $sel:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        unsafe { $self.sel4($sel, $a, $b, $c, $d) }
    }};

    ($self:ident, $sel:expr, $a:expr, $b:expr, $c:expr) => {{
        unsafe { $self.sel3($sel, $a, $b, $c) }
    }};

    ($self:ident, $sel:expr, $a:expr, $b:expr) => {{
        unsafe { $self.sel2($sel, $a, $b) }
    }};

    ($self:ident, $sel:expr, $a:expr) => {{
        unsafe { $self.sel1($sel, $a) }
    }};

    ($self:ident, $sel:expr) => {{
        unsafe { $self.sel0($sel) }
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

// struct ImageInfo {
//     _version: u32,
//     _flags: u32,
// }

// #[link_section = "__DATA,__objc_imageinfo"]
// #[used]
// static IMAGE_INFO: ImageInfo = ImageInfo {
//     _version: 0,
//     _flags: 0,
// };

// pub fn sel_processor_count() -> &'static Sel {
//     #[link_section = "__TEXT,__objc_methname,cstring_literals"]
//     static STR: [u8; 15] = *b"processorCount\0";
//     #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
//     static SEL: &[u8; 15] = &STR;

//     unsafe {
//         let ptr = std::ptr::read_volatile(&SEL);
//         transmute(ptr)
//     }
// }

core::arch::global_asm!(
    "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
    "L_OBJC_IMAGE_INFO:",
    "    .long    0",
    "    .long    0",
    "    .popsection",
);

#[macro_export]
macro_rules! define_sel {

    ($name:ident, $sel:literal) => {

        impl crate::objc::Sel {

            #[inline(always)]
            pub fn $name() -> &'static Self {
                unsafe {
                    let cmd: *const u8;

                    core::arch::asm!(
                        "    .pushsection __TEXT,__objc_methname,cstring_literals",
                        "2:",
                        concat!("    .asciz   \"",  $sel, "\""),
                        "",
                        "    .section     __DATA,__objc_selrefs,literal_pointers,no_dead_strip",
                        "    .p2align 3",
                        "3:",
                        "    .quad    2b",
                        "    .popsection",
                        "adrp	{y}, 3b@PAGE",
                        "ldr	{x}, [{y}, 3b@PAGEOFF]",
                        y = out(reg) _,
                        x = out(reg) cmd,
                        options(nomem, nostack, pure),
                    );

                    transmute(cmd)
                }
            }
        }

    };
}

define_sel!(processor_count, "processorCount");

#[cfg(test)]
mod tests {

    #[test]
    fn basics() {
        let proc = crate::ns::ProcessInfo::current();

        // let count: usize = unsafe { proc.sel0(sel_processor_count()) };

        // println!("count {:?}", count);
    }
}
