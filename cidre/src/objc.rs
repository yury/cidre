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

#[derive(Debug)]
#[repr(transparent)]
pub struct ClassInstExtra<T: Obj, I: Sized>(Class<T>, PhantomData<I>);
const NS_OBJECT_SIZE: usize = 8;

pub trait ObjExtra<I>: Obj {
    fn inner(&self) -> &I {
        unsafe {
            let ptr: *const u8 = transmute(self);
            std::mem::transmute(ptr.offset(NS_OBJECT_SIZE as _))
        }
    }
    fn inner_mut(&mut self) -> &I {
        unsafe {
            let ptr: *mut u8 = transmute(self);
            std::mem::transmute(ptr.offset(NS_OBJECT_SIZE as _))
        }
    }
}

impl<T: ObjExtra<T>, I: Sized> ClassInstExtra<T, I> {
    #[inline]
    pub fn alloc_init(&self, var: I) -> Option<arc::R<T>> {
        unsafe {
            let inst = class_createInstance(std::mem::transmute(self), std::mem::size_of::<I>());
            let Some(a) = inst else {
                return None;
            };

            // we may skip init?
            //let a = a.init();

            let ptr: *mut u8 = transmute(a);
            let d_ptr: *mut I = ptr.offset(NS_OBJECT_SIZE as _) as _;
            *d_ptr = var;

            std::mem::transmute(ptr)
        }
    }
}

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

pub trait Obj: Sized + arc::Retain {
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

crate::define_obj_type!(Any(Id));

pub const NONE: Option<&'static Any> = None;

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

    fn class_createInstance(cls: &Class<Id>, extra_bytes: usize) -> Option<arc::A<Id>>;
    // fn objc_msgSend(id: &Id, sel: &Sel, args: ...) -> *const c_void;
    fn objc_autorelease<'a>(id: &mut Id) -> &'a mut Id;
    fn objc_retainAutoreleasedReturnValue<'a>(obj: Option<&Id>) -> Option<&'a Id>;

    pub fn object_getIndexedIvars(obj: *const c_void) -> *mut c_void;
    // pub fn class_createInstance(cls: &Class<Id>, extra_bytes: usize) -> Option<arc::R<Id>>;
    pub fn sel_registerName(str: *const u8) -> &'static Sel;
    pub fn class_addMethod(
        cls: &Class<Id>,
        name: &Sel,
        imp: extern "C" fn(),
        types: *const u8,
    ) -> bool;
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

// #[repr(C)]
// pub struct Delegate<T: Sized> {
//     pub delegate: Box<T>,
//     pub obj: crate::arc::R<Id>,
// }

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

pub use cidre_macros::add_methods;
pub use cidre_macros::cls_msg_send;
pub use cidre_macros::cls_msg_send_debug;
pub use cidre_macros::cls_rar_retain;
pub use cidre_macros::msg_send;
pub use cidre_macros::msg_send_debug;
pub use cidre_macros::obj_trait;
pub use cidre_macros::optional;
pub use cidre_macros::rar_retain;

// global_asm!(
//     "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
//     "    .long    0",
//     "    .long    0",
//     "    .popsection",
// );

#[cfg(test)]
mod tests2 {
    use crate::objc;

    #[objc::obj_trait]
    trait Foo: objc::Obj {
        #[objc::msg_send(count)]
        fn count(&self) -> usize;

        #[objc::optional]
        #[objc::msg_send(count2)]
        fn count2(&self) -> usize;
    }

    #[objc::add_methods]
    impl ObjFoo for objc::Id {
        extern "C" fn count(&self, _cmd: Option<&objc::Sel>) -> usize {
            todo!()
        }
    }

    // struct Foo;

    // #[link(name = "ns", kind = "static")]
    // extern "C" {
    //     static NS_OBJECT: &'static objc::ClassInstExtra<ns::Id, Foo>;
    // }

    // #[test]
    // fn basics() {
    //     let cls: &'static objc::ClassInstExtra<ns::Id, Foo> =
    //         unsafe { std::mem::transmute(NS_OBJECT) };
    //     let f = cls.alloc_init(Foo).unwrap();
    //     println!("{:?}", f.debug_description());
    // }
}
