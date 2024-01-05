#[cfg(target_arch = "aarch64")]
use std::arch::asm;
use std::{borrow::Cow, ffi::c_void, intrinsics::transmute, marker::PhantomData, ptr::NonNull};

use crate::{arc, cf::Type};

#[derive(Debug)]
#[repr(transparent)]
pub struct Class<T: Obj>(Type, PhantomData<T>);

impl<T: Obj> Class<T> {
    pub unsafe fn method_impl(&self, name: &Sel) -> *const c_void {
        class_getMethodImplementation(std::mem::transmute(self), name)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ClassInstExtra<T: Obj, I: Sized>(Class<T>, PhantomData<I>);
pub const NS_OBJECT_SIZE: usize = 8;

impl<T: Obj, I: Sized> ClassInstExtra<T, I> {
    #[inline]
    pub fn alloc_init(&self, var: I) -> Option<arc::R<T>> {
        unsafe {
            let inst = class_createInstance(std::mem::transmute(self), std::mem::size_of::<I>());
            let Some(a) = inst else {
                return None;
            };

            // we may skip init?
            // let a = a.init();

            let ptr: *mut u8 = transmute(a);
            let d_ptr: *mut std::mem::ManuallyDrop<I> = ptr.add(NS_OBJECT_SIZE) as _;
            *d_ptr = std::mem::ManuallyDrop::new(var);

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
        #[cfg(target_arch = "aarch64")]
        {
            asm!(
                "bl _objc_release_{x}",
                x = in(reg) self,
                clobber_abi("C")
                // system also works
                // clobber_abi("system")
            );
        }

        #[cfg(target_arch = "x86_64")]
        {
            objc_release(transmute(self));
        }
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
        #[cfg(target_arch = "aarch64")]
        {
            let result: *mut Self;
            core::arch::asm!(
                "bl _objc_retain_{obj:x}",
                obj = in(reg) id,
                lateout("x0") result,
                clobber_abi("C"),
            );
            transmute(result)
        }

        #[cfg(target_arch = "x86_64")]
        {
            transmute(objc_retain(transmute(id)))
        }
    }

    #[msg_send(description)]
    fn desc_ar(&self) -> arc::Rar<crate::ns::String>;

    #[rar_retain]
    fn desc(&self) -> arc::R<crate::ns::String>;

    #[msg_send(debugDescription)]
    fn debug_desc_ar(&self) -> arc::Rar<crate::ns::String>;

    #[rar_retain]
    fn debug_desc(&self) -> arc::R<crate::ns::String>;

    #[msg_send(respondsToSelector:)]
    fn responds_to_sel(&self, sel: &Sel) -> bool;

    #[msg_send(class)]
    fn class(&self) -> &crate::objc::Class<Self>;

    #[msg_send(isKindOfClass:)]
    fn is_kind_of_class<T: Obj>(&self, cls: &crate::objc::Class<T>) -> bool;

    #[inline]
    fn try_cast<T: Obj>(&self, cls: &crate::objc::Class<T>) -> Option<&T> {
        if self.is_kind_of_class(cls) {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }

    #[msg_send(isMemberOfClass:)]
    fn is_member_of_class<T: Obj>(&self, cls: &crate::objc::Class<T>) -> bool;

    #[inline]
    fn is_tagged_ptr(&self) -> bool {
        ((self as *const Self as usize) >> 63) == 1
    }

    #[inline]
    fn as_id_ref(&self) -> &Id {
        unsafe { std::mem::transmute(self) }
    }
}

/// Use it as NSObject or id
#[repr(transparent)]
pub struct Id(Type);

impl Id {
    #[inline]
    pub unsafe fn autorelease<'ar>(id: &mut Id) -> &'ar mut Id {
        objc_autorelease(id)
    }

    // #[inline]
    // pub unsafe fn retain_autoreleased_return<'ar>(id: Option<&Id>) -> Option<arc::R<Id>> {
    //     objc_retainAutoreleasedReturnValue(id)
    // }

    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[inline]
    pub fn as_id_ref(&self) -> &Self {
        self
    }

    #[msg_send(isEqual:)]
    pub fn is_equal(&self, other: &Self) -> bool;
}

impl Obj for Id {}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.debug_desc();
        f.write_str(&Cow::from(desc.as_cf()))
        // f.debug_tuple("NS").field(&Cow::from(desc.as_cf())).finish()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Sel(NonNull<c_void>);

pub mod autorelease_pool;
pub mod ns;
pub use autorelease_pool::AutoreleasePoolPage;

pub fn ar_pool<R, F>(f: F) -> R
where
    F: FnOnce() -> R,
    R: Clone, // Autoreleased doesn't implement Clone
{
    let _page = AutoreleasePoolPage::push();
    f()
}

pub unsafe fn sel_reg_name(str: *const u8) -> &'static Sel {
    std::mem::transmute(sel_registerName(str))
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    #[cfg(target_arch = "x86_64")]
    pub fn objc_retain<'a>(obj: &Id) -> &'a Id;
    #[cfg(target_arch = "x86_64")]
    fn objc_release(obj: &mut Id);

    // fn objc_msgSend();

    fn class_createInstance(cls: &Class<Id>, extra_bytes: usize) -> Option<arc::A<Id>>;
    fn class_getMethodImplementation(cls: &Class<Id>, name: &Sel) -> *const c_void;
    fn objc_autorelease<'ar>(id: &mut Id) -> &'ar mut Id;

    pub fn objc_retainAutoreleasedReturnValue<'ar>(obj: Option<&Id>) -> Option<arc::R<Id>>;
    pub fn objc_autoreleaseReturnValue<'ar>(obj: Option<&Id>) -> Option<&'ar Id>;

    pub fn object_getIndexedIvars(obj: *const c_void) -> *mut c_void;
    pub fn sel_registerName(str: *const u8) -> *const std::ffi::c_void;
    pub fn class_addMethod(
        cls: &Class<Id>,
        name: &Sel,
        imp: extern "C" fn(),
        types: *const u8,
    ) -> bool;

    pub fn objc_allocateClassPair(
        super_cls: &Class<Id>,
        name: *const u8,
        extra_bytes: usize,
    ) -> Option<&'static Class<Id>>;
    pub fn objc_registerClassPair(cls: &Class<Id>);
    pub fn objc_getClass(name: *const u8) -> Option<&'static Class<Id>>;
    pub static NS_OBJECT: &'static crate::objc::Class<Id>;
    fn objc_exception_throw(exception: &Id) -> !;
}

/// Same as `define_cls!` but with open `init`
#[macro_export]
macro_rules! define_cls_init {
    ($NewType:ident, $CLS:ident) => {
        impl $crate::arc::A<$NewType> {
            #[$crate::objc::msg_send(init)]
            pub fn init(self) -> arc::R<$NewType>;
        }

        impl $NewType {
            $crate::define_cls!($CLS);

            /// shortcut to `Self::alloc().init()`
            #[inline]
            pub fn new() -> $crate::arc::R<$NewType> {
                Self::alloc().init()
            }
        }
    };
}

/// Defines class
///
/// Use when:
/// + (instancetype)new NS_UNAVAILABLE;
/// - (instancetype)init NS_UNAVAILABLE;
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
    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident $(+ $TraitImpl:path)*, $InnerType:path, $CLS:ident) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis
            $NewType(objc::Id)
        );

        impl $NewType {
            #[allow(dead_code)]
            #[inline]
            pub fn inner(&self) -> &$InnerType {
                unsafe {
                    let ptr =  self as *const Self as *const u8;
                    let ptr = ptr.add($crate::objc::NS_OBJECT_SIZE);
                    &*(ptr as *const $InnerType)
                }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn inner_mut(&mut self) -> &mut $InnerType {
                unsafe {
                    let ptr: *mut u8 = self as *mut Self as *mut u8;
                    let ptr = ptr.add($crate::objc::NS_OBJECT_SIZE);
                    &mut *(ptr as *mut $InnerType)
                }
            }

            #[allow(dead_code)]
            pub fn register_cls() -> &'static $crate::objc::ClassInstExtra<Self, $InnerType> {
                let name = concat!(stringify!($CLS), "\0");
                let cls = unsafe { $crate::objc::objc_allocateClassPair($crate::objc::NS_OBJECT, name.as_ptr(), 0) };
                let cls = cls.unwrap();
                $(<Self as $TraitImpl>::cls_add_methods(cls);)*

                if std::mem::needs_drop::<$InnerType>() {
                    extern "C" fn impl_dealloc(s: &mut $NewType, _sel: Option<$crate::objc::Sel>) {
                        let ptr = s.inner_mut() as *mut _;
                        unsafe { std::ptr::drop_in_place(ptr); }
                    }
                    unsafe {
                        let sel = $crate::objc::sel_reg_name(b"dealloc\0".as_ptr());
                        let imp: extern "C" fn() = std::mem::transmute(impl_dealloc as *const u8);
                        $crate::objc::class_addMethod(cls, sel, imp, std::ptr::null());
                    }
                }
                unsafe { $crate::objc::objc_registerClassPair(cls) };
                unsafe { std::mem::transmute(cls) }
            }

            #[allow(dead_code)]
            pub fn cls() -> &'static $crate::objc::ClassInstExtra<Self, $InnerType> {
                let name = concat!(stringify!($CLS), "\0");
                let cls = unsafe { $crate::objc::objc_getClass(name.as_ptr()) };
                match cls {
                    Some(c) => unsafe { std::mem::transmute(c) }
                    None => Self::register_cls()
                }
            }

            #[allow(dead_code)]
            pub fn with(inner: $InnerType) -> $crate::arc::R<Self> {
                unsafe { Self::cls().alloc_init(inner).unwrap_unchecked() }
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident $(+ $TraitImpl:path)*, (), $CLS:ident) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis
            $NewType(objc::Id)
        );

        impl $NewType {

            #[allow(dead_code)]
            pub fn register_cls() -> &'static $crate::objc::ClassInstExtra<Self, ()> {
                let name = concat!(stringify!($CLS), "\0");
                let cls = unsafe { $crate::objc::objc_allocateClassPair($crate::objc::NS_OBJECT, name.as_ptr(), 0) };
                let cls = cls.unwrap();
                $(<Self as $TraitImpl>::cls_add_methods(cls);)*

                unsafe { $crate::objc::objc_registerClassPair(cls) };
                unsafe { std::mem::transmute(cls) }
            }

            #[allow(dead_code)]
            pub fn cls() -> &'static $crate::objc::ClassInstExtra<Self, ()> {
                let name = concat!(stringify!($CLS), "\0");
                let cls = unsafe { $crate::objc::objc_getClass(name.as_ptr()) };
                match cls {
                    Some(c) => unsafe { std::mem::transmute(c) }
                    None => Self::register_cls()
                }
            }

            #[allow(dead_code)]
            pub fn new() -> $crate::arc::R<Self> {
                unsafe { Self::cls().alloc_init(()).unwrap_unchecked() }
            }
        }
    };

    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident($BaseType:path)
    ) => {
        $(#[$outer])*
        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        $vis struct $NewType($BaseType);

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
            #[allow(dead_code)]
            #[inline]
            pub fn retained(&self) -> $crate::arc::R<Self> {
                unsafe { $crate::objc::Obj::retain(self) }
            }
        }

        impl PartialEq<$crate::arc::R<$NewType>> for $NewType {
            fn eq(&self, other: &$crate::arc::R<$NewType>) -> bool {
                self.0.is_equal(other)
            }
        }

        // impl PartialEq for $NewType {
        //     fn eq(&self, other: &$NewType) -> bool {
        //         self.is_equal(other)
        //     }
        // }

    };
    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident($BaseType:path), $CLS:ident
    ) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis$
            NewType($BaseType)
        );
        $crate::define_cls_init!($NewType, $CLS);
    };
}

impl PartialEq for Id {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

/// Can throw any object. You may need ns::Exception::raise.
/// [read more](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Tasks/HandlingExceptions.html)
#[inline]
pub fn throw(obj: &Id) -> ! {
    // latest sonoma (4.2 Beta (23C5030f) crash on tagged ptr exception for unknown reason :(
    // TODO: investigate on release
    debug_assert!(!obj.is_tagged_ptr());
    unsafe { objc_exception_throw(obj) }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn cidre_try_catch<'ar>(
        during: extern "C" fn(ctx: *mut c_void),
        ctx: *mut c_void,
    ) -> Option<&'ar Id>;
}

pub fn try_catch<'ar, F, R>(f: F) -> Result<R, &'ar Id>
where
    F: FnOnce() -> R,
{
    let mut result = None;
    let mut wrapper = Some(|| {
        result = Some(f());
    });

    let f = type_helper(&wrapper);
    let ctx = &mut wrapper as *mut _ as *mut c_void;

    unsafe {
        match cidre_try_catch(transmute(f), ctx) {
            None => Ok(result.unwrap_unchecked()),
            Some(e) => Err(e),
        }
    }
}

#[inline]
fn type_helper<F>(_t: &Option<F>) -> extern "C" fn(t: &mut Option<F>)
where
    F: FnOnce(),
{
    extern "C" fn during<F>(f: &mut Option<F>)
    where
        F: FnOnce(),
    {
        unsafe { f.take().unwrap_unchecked()() };
    }
    during
}

#[cfg(target_arch = "aarch64")]
#[cfg(test)]
mod tests {

    use super::ar_pool;
    use crate::{cf, dispatch, return_ar};
    use std;

    fn autorelease_example_ar<'ar>() -> &'ar dispatch::Queue {
        let q = dispatch::Queue::new();
        return_ar!(q)
    }

    #[test]
    fn autorelease() {
        let ptr = ar_pool(|| {
            let q = autorelease_example_ar().retained();
            assert_eq!(2, q.as_type_ref().retain_count());
            unsafe { q.as_type_ref().as_type_ptr() }
        });

        let _ptr: &cf::Type = unsafe { std::mem::transmute(ptr) };
    }
}
pub use cidre_macros::add_methods;
pub use cidre_macros::obj_trait;
pub use cidre_macros::optional;

#[cfg(target_arch = "aarch64")]
pub use cidre_macros::cls_msg_send;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::cls_msg_send_debug;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::cls_msg_send_debug_x86_64 as cls_msg_send_debug;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::cls_msg_send_x86_64 as cls_msg_send;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::cls_rar_retain;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::cls_rar_retain_x86_64 as cls_rar_retain;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send_debug;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::msg_send_x86_64 as msg_send;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::rar_retain;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::rar_retain_x86_64 as rar_retain;

#[cfg(test)]
mod tests2 {

    use crate::{objc, objc::Obj};

    #[objc::obj_trait]
    trait Foo: objc::Obj {
        #[objc::msg_send(count)]
        fn count(&self) -> usize;

        #[objc::optional]
        #[objc::msg_send(count2)]
        fn count2(&self) -> usize;
    }

    static mut DROP_CALLED: bool = false;

    pub struct D;

    impl Drop for D {
        fn drop(&mut self) {
            unsafe {
                DROP_CALLED = true;
            }
        }
    }

    define_obj_type!(Bla + FooImpl, D, BLA_USIZE);

    impl Foo for Bla {}

    #[objc::add_methods]
    impl FooImpl for Bla {
        extern "C" fn impl_count(&self, _cmd: Option<&objc::Sel>) -> usize {
            0
        }
    }

    #[test]
    fn basics() {
        unsafe {
            DROP_CALLED = false;
        }
        {
            let d = Bla::with(D);
            let _r = d.retained();
            let desc = d.desc();
            assert!(desc.to_string().starts_with("<BLA_USIZE: "));
        }
        assert!(unsafe { DROP_CALLED });
    }
}
