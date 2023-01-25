use std::{
    arch::asm,
    borrow::Cow,
    ffi::c_void,
    intrinsics::transmute,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{arc, cf::Type};
#[derive(Debug)]
#[repr(transparent)]
pub struct Class<T: Obj>(Type, PhantomData<T>);

impl<T: Obj> Class<T> {
    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[must_use]
    #[msg_send(alloc)]
    pub fn alloc(&self) -> arc::A<T>;

    // in general alloc_init is faster
    #[msg_send(new)]
    pub unsafe fn new(&self) -> arc::R<T>;
}

impl<T: Obj> Obj for Class<T> {}

impl<T: Obj> arc::Release for T {
    #[inline]
    unsafe fn release(&mut self) {
        objc_release(transmute(self))
    }
}

impl<T: Obj> arc::Retain for T {
    fn retained(&self) -> arc::R<Self> {
        unsafe { Self::retain(self) }
    }
}

pub trait Obj: arc::Retain {
    #[inline]
    unsafe fn retain(id: &Self) -> arc::R<Self> {
        transmute(objc_retain(transmute(id)))
    }

    #[msg_send(description)]
    fn description_ar(&self) -> arc::Rar<crate::ns::String>;

    #[rar_retain()]
    fn description(&self) -> arc::R<crate::ns::String>;

    #[msg_send(debugDescription)]
    fn debug_description_ar(&self) -> arc::Rar<crate::ns::String>;

    #[rar_retain()]
    fn debug_description(&self) -> arc::R<crate::ns::String>;

    #[msg_send(respondsToSelector:)]
    fn responds_to_sel(&self, sel: &Sel) -> bool;

    #[inline]
    fn is_tagged_ptr(&self) -> bool {
        ((self as *const Self as usize) >> 63) == 1
    }
}

/// Use it as NSObject or id
#[repr(transparent)]
pub struct Id(Type);

impl Id {
    #[inline]
    pub unsafe fn autorelease<'ar>(id: &mut Id) -> &mut Id {
        objc_autorelease(id)
    }

    #[inline]
    pub unsafe fn retain_autoreleased<'ar>(id: Option<&Id>) -> Option<&Id> {
        objc_retainAutoreleasedReturnValue(id)
    }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[msg_send(isEqual:)]
    pub fn is_equal(&self, other: &Self) -> bool;

    #[inline]
    pub fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl Obj for Id {}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.description();
        f.debug_tuple("NS").field(&Cow::from(desc.deref())).finish()
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

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_retain<'a>(obj: &Id) -> &'a Id;
    fn objc_release(obj: &mut Id);

    // fn objc_msgSend(id: &Id, sel: &Sel, args: ...) -> *const c_void;
    fn objc_autorelease<'a>(id: &mut Id) -> &'a mut Id;
    fn objc_retainAutoreleasedReturnValue<'a>(obj: Option<&Id>) -> Option<&'a Id>;

    fn object_getIndexedIvars(obj: &Id) -> *mut c_void;
    fn class_createInstance(cls: &Class<Id>, extra_bytes: usize) -> Option<arc::R<Id>>;
    fn sel_registerName(str: *const u8) -> &'static Sel;
}

#[macro_export]
macro_rules! define_cls_init {
    ($NewType:ident, $CLS:ident) => {
        impl $crate::arc::A<$NewType> {
            #[$crate::objc::msg_send(init)]
            pub fn init(self) -> arc::R<$NewType>;
        }

        impl $NewType {
            $crate::define_cls!($CLS);

            pub fn new() -> $crate::arc::R<$NewType> {
                Self::alloc().init()
            }
        }
    };
}

#[macro_export]
macro_rules! define_cls {
    ($CLS:ident) => {
        #[inline]
        pub fn cls() -> &'static $crate::objc::Class<Self> {
            unsafe { std::mem::transmute($CLS) }
        }

        #[inline]
        pub fn alloc() -> $crate::arc::A<Self> {
            Self::cls().alloc()
        }
    };
}

#[macro_export]
macro_rules! define_obj_type {
    ($NewType:ident($BaseType:path)) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $NewType($BaseType);

        impl $crate::objc::Obj for $NewType {}

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

        impl $NewType {
            #[inline]
            pub fn retained(&self) -> $crate::arc::R<Self> {
                unsafe { $crate::objc::Obj::retain(self) }
            }
        }

        impl $crate::arc::R<$NewType> {
            #[must_use]
            pub fn autoreleased<'ar>(self) -> &'ar mut $NewType {
                unsafe {
                    let res = $crate::objc::Id::autorelease(std::mem::transmute(self));
                    return std::mem::transmute(res);
                }
            }
        }
    };
    ($NewType:ident($BaseType:path), $CLS:ident) => {
        $crate::define_obj_type!($NewType($BaseType));
        $crate::define_cls_init!($NewType, $CLS);
    };
}

impl PartialEq for Id {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

// how we should model delegates
trait Protocol: Sized {}

trait SCStreamOut: Protocol {
    // #[prl_msg_send(stream:didOutputSampleBuffer:ofType:)]
    fn stream_did_output_sample_buffer_of_type(&self, _t: u32);
    extern "C" fn iml_stream_did_output_sample_buffer_of_type(id: &Id, _cmd: &Sel, t: u32) {
        unsafe {
            // TODO: can be optimized here if we know size of object.
            // NSObject is 8 bytes
            let slf: &mut Self = transmute(object_getIndexedIvars(id));
            slf.stream_did_output_sample_buffer_of_type(t)
        }
    }
    fn sel_stream_did_output_sample_buffer_of_type() -> &'static Sel {
        unsafe { sel_registerName(b"stream:DidOutputSampleBuffer:ofType:\0".as_ptr()) }
    }
    // #[msg_send(stream:didOutputSampleBuffer:ofType:)]
    fn opt_stream_did_output_sample(&self, _t: u32) {
        unimplemented!();
    }

    fn sel_opt_stream_did_output_sample_buffer_of_type() -> &'static Sel {
        unsafe { sel_registerName(b"stream:DidOutputSampleBuffer:ofType:\0".as_ptr()) }
    }
}

struct Foo;

impl Protocol for Foo {}

impl SCStreamOut for Foo {
    fn stream_did_output_sample_buffer_of_type(&self, _t: u32) {
        todo!()
    }

    fn opt_stream_did_output_sample(&self, _t: u32) {
        todo!()
    }
}

#[repr(C)]
pub struct Delegate<T: Sized> {
    pub delegate: Box<T>,
    pub obj: crate::arc::R<Id>,
}

#[cfg(test)]
mod tests {

    use super::autoreleasepool;
    use crate::{cf, dispatch};
    use std;

    fn autorelease_example<'ar>() -> &'ar mut dispatch::Queue {
        let q = dispatch::Queue::new();
        q.autoreleased()
    }

    #[test]
    fn autorelease() {
        let ptr = autoreleasepool(|| {
            let q = autorelease_example();
            assert_eq!(1, q.as_type_ref().retain_count());
            unsafe { q.as_type_ref().as_type_ptr() }
        });

        let _ptr: &cf::Type = unsafe { std::mem::transmute(ptr) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ReturnedAutoReleased<T: Obj + 'static>(&'static mut T);

impl<T: Obj> ReturnedAutoReleased<T> {
    #[inline]
    pub fn retain(self) -> arc::R<T> {
        unsafe {
            asm!("mov x29, x29");
            transmute(Id::retain_autoreleased(transmute(self)))
        }
    }

    #[inline]
    pub fn option_retain(value: Option<Self>) -> Option<arc::R<T>> {
        unsafe {
            asm!("mov x29, x29");
            transmute(Id::retain_autoreleased(transmute(value)))
        }
    }
}

impl<T: Obj> Deref for ReturnedAutoReleased<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Obj> DerefMut for ReturnedAutoReleased<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub use cidre_macros::cls_msg_send;
pub use cidre_macros::cls_msg_send_debug;
pub use cidre_macros::cls_rar_retain;
pub use cidre_macros::msg_send;
pub use cidre_macros::msg_send_debug;
pub use cidre_macros::rar_retain;

// global_asm!(
//     "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
//     "    .long    0",
//     "    .long    0",
//     "    .popsection",
// );
