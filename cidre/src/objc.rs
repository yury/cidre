#[cfg(all(
    target_arch = "aarch64",
    not(target_pointer_width = "32"),
    not(feature = "classic-objc-retain-release")
))]
use std::arch::asm;
use std::{borrow::Cow, ffi::c_void, marker::PhantomData, ptr::NonNull};

use crate::{arc, cf::Type, objc};

#[derive(Debug)]
#[repr(transparent)]
pub struct Class<T: Obj>(Type, PhantomData<T>);

#[derive(Debug)]
#[repr(transparent)]
pub struct Protocol(Type);

impl<T: Obj> Class<T> {
    pub unsafe fn method_impl(&self, name: &Sel) -> *const c_void {
        unsafe { class_getMethodImplementation(std::mem::transmute(self), name) }
    }

    pub unsafe fn add_protocol(&self, protocol: &Protocol) -> bool {
        unsafe { class_addProtocol(std::mem::transmute(self), protocol) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ClassInstExtra<T: Obj, I: Sized>(Class<T>, PhantomData<I>);

impl<T: Obj, I: Sized> std::ops::Deref for ClassInstExtra<T, I> {
    type Target = Class<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// class_getInstanceSize([NSObject class]);
pub const NS_OBJECT_SIZE: usize = std::mem::size_of::<usize>();

#[macro_export]
macro_rules! init_with_default {
    ($NewType:ty, $InnerType:ty) => {{
        trait A {
            fn init_fn(&self) -> Option<extern "C" fn()>;
        }

        struct B<T: ?Sized>(core::marker::PhantomData<T>);

        impl<T: ?Sized> core::ops::Deref for B<T> {
            type Target = ();
            fn deref(&self) -> &Self::Target {
                &()
            }
        }

        impl<T: ?Sized> A for B<T>
        where
            T: Default,
        {
            fn init_fn(&self) -> Option<extern "C" fn()> {
                extern "C" fn impl_init<T: Default>(
                    s: *mut $NewType,
                    _sel: Option<$crate::objc::Sel>,
                ) -> $crate::arc::R<$NewType> {
                    unsafe {
                        let ptr: *mut u8 = s.cast();
                        let d_ptr: *mut std::mem::ManuallyDrop<T> =
                            ptr.add($crate::objc::NS_OBJECT_SIZE) as _;
                        *d_ptr = std::mem::ManuallyDrop::new(T::default());

                        std::mem::transmute(ptr)
                    }
                }

                let ptr = unsafe { std::mem::transmute(impl_init::<T> as *const u8) };
                Some(ptr)
            }
        }

        impl A for () {
            fn init_fn(&self) -> Option<extern "C" fn()> {
                None
            }
        }

        B::<$InnerType>(core::marker::PhantomData).init_fn()
    }};
}

impl<T: Obj, I: Sized> ClassInstExtra<T, I> {
    #[inline]
    pub fn alloc_init(&self, var: I) -> arc::R<T> {
        unsafe {
            let inst = class_createInstance(std::mem::transmute(self), std::mem::size_of::<I>());

            // we may skip init?
            // let inst = inst.init();

            let ptr: *mut u8 = std::mem::transmute(inst);
            let d_ptr: *mut std::mem::ManuallyDrop<I> = ptr.add(NS_OBJECT_SIZE) as _;
            *d_ptr = std::mem::ManuallyDrop::new(var);

            std::mem::transmute(ptr)
        }
    }
}

impl<T: Obj, I: Sized + Default> ClassInstExtra<T, I> {
    pub fn new(&self) -> arc::R<T> {
        self.alloc_init(Default::default())
    }
}

impl<T: Obj> Class<T> {
    #[inline]
    pub fn as_type_ref(&self) -> &Type {
        &self.0
    }

    #[must_use]
    #[objc::msg_send(alloc)]
    pub fn alloc(&self) -> arc::A<T>;

    // in general alloc_init is faster
    #[objc::msg_send(new)]
    pub unsafe fn new(&self) -> arc::Retained<T>;
}

impl<T: Obj> Obj for Class<T> {}

impl<T: Obj> arc::Release for T {
    #[inline]
    unsafe fn release(&mut self) {
        unsafe { <T as Obj>::release(self) }
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
        unsafe {
            #[cfg(all(target_arch = "aarch64", not(feature = "classic-objc-retain-release")))]
            {
                let result: *mut Self;
                core::arch::asm!(
                    "bl _objc_retain_{obj:x}",
                    obj = in(reg) id,
                    lateout("x0") result,
                    clobber_abi("C"),
                );
                std::mem::transmute(result)
            }

            #[cfg(any(target_arch = "x86_64", feature = "classic-objc-retain-release"))]
            {
                std::mem::transmute(objc_retain(std::mem::transmute(id)))
            }
        }
    }

    #[inline]
    unsafe fn release(id: &mut Self) {
        unsafe {
            #[cfg(all(
                target_arch = "aarch64",
                target_pointer_width = "64",
                not(feature = "classic-objc-retain-release")
            ))]
            {
                asm!(
                    "bl _objc_release_{x}",
                    x = in(reg) id,
                    clobber_abi("C")
                    // system also works
                    // clobber_abi("system")
                );
            }

            #[cfg(any(
                target_arch = "x86_64",
                target_pointer_width = "32",
                feature = "classic-objc-retain-release"
            ))]
            {
                objc_release(std::mem::transmute(id));
            }
        }
    }

    #[objc::msg_send(description)]
    fn desc(&self) -> arc::R<crate::ns::String>;

    #[objc::msg_send(debugDescription)]
    fn debug_desc(&self) -> arc::R<crate::ns::String>;

    #[objc::msg_send(respondsToSelector:)]
    fn responds_to_sel(&self, sel: &Sel) -> bool;

    #[objc::msg_send(class)]
    fn class(&self) -> &crate::objc::Class<Self>;

    #[objc::msg_send(isKindOfClass:)]
    fn is_kind_of_class<T: Obj>(&self, cls: &crate::objc::Class<T>) -> bool;

    #[inline]
    fn try_cast<T: Obj>(&self, cls: &crate::objc::Class<T>) -> Option<&T> {
        if self.is_kind_of_class(cls) {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }

    #[inline]
    fn try_cast_mut<T: Obj>(&mut self, cls: &crate::objc::Class<T>) -> Option<&mut T> {
        if self.is_kind_of_class(cls) {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }

    #[objc::msg_send(isMemberOfClass:)]
    fn is_member_of_class<T: Obj>(&self, cls: &crate::objc::Class<T>) -> bool;

    #[cfg(not(target_os = "watchos"))]
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

unsafe impl Send for Id {}

impl Id {
    #[inline]
    pub unsafe fn autorelease<'ar>(id: &mut Id) -> &'ar mut Id {
        unsafe { objc_autorelease(id) }
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

    #[objc::msg_send(isEqual:)]
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

pub unsafe fn sel_reg_name(str: *const i8) -> &'static Sel {
    unsafe { std::mem::transmute(sel_registerName(str)) }
}

#[doc(alias = "objc_super")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Super {
    pub receiver: *mut Id,
    pub super_class: *const Class<Id>,
}

#[link(name = "objc", kind = "dylib")]
unsafe extern "C-unwind" {
    #[cfg(any(target_arch = "x86_64", feature = "classic-objc-retain-release"))]
    pub fn objc_retain<'a>(obj: &Id) -> &'a Id;
    #[cfg(any(
        target_arch = "x86_64",
        target_pointer_width = "32",
        feature = "classic-objc-retain-release"
    ))]
    fn objc_release(obj: &mut Id);

    // fn objc_msgSend();

    pub fn class_createInstance(cls: &Class<Id>, extra_bytes: usize) -> arc::A<Id>;
    fn class_getMethodImplementation(cls: &Class<Id>, name: &Sel) -> *const c_void;
    fn class_addProtocol(cls: &Class<Id>, protocol: &Protocol) -> bool;
    fn objc_autorelease<'ar>(id: &mut Id) -> &'ar mut Id;

    pub fn objc_retainAutoreleasedReturnValue<'ar>(obj: Option<&Id>) -> Option<arc::R<Id>>;
    pub fn objc_retainAutoreleaseReturnValue<'ar>(obj: Option<&Id>) -> Option<&'ar Id>;
    pub fn objc_claimAutoreleasedReturnValue() -> Option<arc::R<Id>>;
    pub fn objc_autoreleaseReturnValue<'ar>(obj: Option<&Id>) -> Option<&'ar Id>;

    pub fn object_getIndexedIvars(obj: *const c_void) -> *mut c_void;
    pub fn sel_registerName(str: *const i8) -> *const std::ffi::c_void;
    pub fn class_addMethod(
        cls: &Class<Id>,
        name: &Sel,
        imp: extern "C" fn(),
        types: *const u8,
    ) -> bool;

    pub fn class_replaceMethod(
        cls: &Class<Id>,
        name: &Sel,
        imp: extern "C" fn(),
        types: *const u8,
    ) -> Option<extern "C" fn()>;

    pub fn objc_allocateClassPair(
        super_cls: &Class<Id>,
        name: *const u8,
        extra_bytes: usize,
    ) -> Option<&'static Class<Id>>;
    pub fn object_getClass(obj: Option<&Id>) -> Option<&Class<Id>>;
    pub fn objc_registerClassPair(cls: &Class<Id>);
    pub fn objc_getClass(name: *const u8) -> Option<&'static Class<Id>>;
    pub fn objc_getProtocol(name: *const i8) -> Option<&'static Protocol>;
    pub fn objc_msgSendSuper(s: &Super, sel: &Sel);
    pub static NS_OBJECT: &'static crate::objc::Class<Id>;
    fn objc_exception_throw(exception: &Id) -> !;
}

/// Same as `define_cls!` but with open `init`
#[macro_export]
macro_rules! define_cls_init {
    ($NewType:ident, $CLS:ident) => {
        impl $crate::arc::A<$NewType> {
            #[$crate::objc::msg_send(init)]
            pub fn init(self) -> arc::Retained<$NewType>;
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

#[macro_export]
macro_rules! define_weak_cls_init {
    ($NewType:ident, $CLS:ident) => {
        impl $crate::arc::A<$NewType> {
            #[$crate::objc::msg_send(init)]
            pub fn init(self) -> arc::Retained<$NewType>;
        }

        impl $NewType {
            $crate::define_weak_cls!($CLS);

            /// shortcut to `Self::alloc().init()`
            #[inline]
            pub fn new() -> Option<$crate::arc::R<$NewType>> {
                Some(Self::alloc()?.init())
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
        pub fn cls_ptr() -> *const std::ffi::c_void {
            unsafe { std::mem::transmute($CLS) }
        }

        #[inline]
        pub fn alloc() -> $crate::arc::A<Self> {
            Self::cls().alloc()
        }
    };
}

#[macro_export]
macro_rules! define_weak_cls {
    ($CLS:ident) => {
        #[inline]
        pub fn cls() -> Option<&'static $crate::objc::Class<Self>> {
            unsafe { std::mem::transmute($CLS) }
        }

        #[inline]
        pub fn cls_ptr() -> *const std::ffi::c_void {
            unsafe { std::mem::transmute($CLS) }
        }

        #[inline]
        pub fn alloc() -> Option<$crate::arc::A<Self>> {
            Some(Self::cls()?.alloc())
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
                $(<Self as $TraitImpl>::cls_add_protocol(cls);)*

                if let Some(init_fn_ptr) = $crate::init_with_default!($NewType, $InnerType) {
                    unsafe {
                        let sel = $crate::objc::sel_reg_name(c"init".as_ptr() as _);
                        let imp: extern "C" fn() = init_fn_ptr;
                        $crate::objc::class_addMethod(cls, sel, imp, std::ptr::null());

                        let sel = $crate::objc::sel_reg_name(c"alloc".as_ptr() as _);
                        let meta_cls = $crate::objc::object_getClass(Some(std::mem::transmute(cls))).unwrap();

                        extern "C" fn alloc_impl(cls: &$crate::objc::Class<$crate::ns::Id>) -> $crate::arc::A<$NewType> {
                            unsafe {
                                let inst = $crate::objc::class_createInstance(cls, std::mem::size_of::<$InnerType>());
                                std::mem::transmute(inst)
                            }

                        }


                        $crate::objc::class_addMethod(meta_cls, sel, std::mem::transmute(alloc_impl as *const u8), std::ptr::null());

                    }
                }

                if std::mem::needs_drop::<$InnerType>() {
                    extern "C" fn impl_dealloc(s: &mut $NewType, sel: &$crate::objc::Sel) {
                        let ptr = s.inner_mut() as *mut _;
                        unsafe {
                            std::ptr::drop_in_place(ptr);
                            let sup = $crate::objc::Super {
                                receiver: std::mem::transmute(s),
                                super_class: $crate::objc::NS_OBJECT
                            };
                            $crate::objc::objc_msgSendSuper(&sup, sel);
                        }
                    }
                    unsafe {
                        let sel = $crate::objc::sel_reg_name(c"dealloc".as_ptr() as _);
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
            #[inline]
            pub fn cls_ptr() -> *const std::ffi::c_void {
                Self::cls() as *const $crate::objc::ClassInstExtra<Self, $InnerType> as *const std::ffi::c_void
            }

            #[allow(dead_code)]
            pub fn with(inner: $InnerType) -> $crate::arc::R<Self> {
                Self::cls().alloc_init(inner)
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
                $(<Self as $TraitImpl>::cls_add_protocol(cls);)*

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
                unsafe { Self::cls().new() }
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

        impl AsRef<$crate::ns::Id> for $NewType {
            fn as_ref(&self) -> &$crate::ns::Id {
                self
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
        $(, #[$api_available:meta])?
    ) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis$
            NewType($BaseType)
        );
        $(#[$api_available])?
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
    #[cfg(not(target_os = "watchos"))]
    debug_assert!(!obj.is_tagged_ptr());
    unsafe { objc_exception_throw(obj) }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C-unwind" {
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
        match cidre_try_catch(std::mem::transmute(f), ctx) {
            None => Ok(result.unwrap_unchecked()),
            Some(e) => Err(e),
        }
    }
}

#[inline]
fn type_helper<F>(_t: &Option<F>) -> extern "C-unwind" fn(t: &mut Option<F>)
where
    F: FnOnce(),
{
    extern "C-unwind" fn during<F>(f: &mut Option<F>)
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
    use crate::{arc, cf, dispatch, return_ar};
    use std;

    fn autorelease_example_ar() -> arc::Rar<dispatch::Queue> {
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
pub use cidre_macros::api_available as available;
pub use cidre_macros::optional;
pub use cidre_macros::protocol;

/// Docs
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send_debug;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::msg_send_x86_64 as msg_send;

#[cfg(test)]
mod tests2 {

    use crate::objc::{self, Obj};

    #[objc::protocol(Foo)]
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
