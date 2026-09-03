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

#[derive(Debug)]
#[repr(transparent)]
pub struct Ivar(Type);

#[derive(Debug)]
#[repr(transparent)]
pub struct Method(Type);

impl<T: Obj> Class<T> {
    pub unsafe fn method_impl(&self, name: &Sel) -> *const c_void {
        unsafe { class_getMethodImplementation(std::mem::transmute(self), name) }
    }

    pub unsafe fn add_protocol(&self, protocol: &Protocol) -> bool {
        unsafe { class_addProtocol(std::mem::transmute(self), protocol) }
    }

    /// The instance variable `name` declares, or `None` if it declares none.
    ///
    /// Unlike [`Self::instance_method`] this does not search superclasses.
    #[doc(alias = "class_getInstanceVariable")]
    #[inline]
    pub fn instance_var(&self, name: &std::ffi::CStr) -> Option<&Ivar> {
        unsafe { class_getInstanceVariable(std::mem::transmute(self), name.as_ptr()) }
    }

    /// The instance method `sel`, searching this class and its superclasses.
    #[doc(alias = "class_getInstanceMethod")]
    #[inline]
    pub fn instance_method(&self, sel: &Sel) -> Option<&Method> {
        unsafe { class_getInstanceMethod(std::mem::transmute(self), sel) }
    }

    /// The metaclass, where class methods live.
    #[doc(alias = "object_getClass")]
    #[inline]
    pub fn meta_cls(&self) -> &Class<Id> {
        unsafe { object_getClass(Some(std::mem::transmute(self))).unwrap_unchecked() }
    }
}

impl Ivar {
    #[doc(alias = "ivar_getName")]
    #[inline]
    pub fn name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(ivar_getName(self)) }
    }

    #[doc(alias = "ivar_getTypeEncoding")]
    #[inline]
    pub fn type_encoding(&self) -> Option<&std::ffi::CStr> {
        unsafe {
            let enc = ivar_getTypeEncoding(self);
            (!enc.is_null()).then(|| std::ffi::CStr::from_ptr(enc))
        }
    }

    /// Byte offset of the variable from the start of an instance.
    #[doc(alias = "ivar_getOffset")]
    #[inline]
    pub fn offset(&self) -> isize {
        unsafe { ivar_getOffset(self) }
    }

    /// Where the variable sits inside `obj`.
    ///
    /// # Safety
    /// `obj` must be a live instance of the class this ivar was looked up on,
    /// and `T` must be what the variable actually holds — check
    /// [`Self::type_encoding`] when the layout is not yours.
    #[inline]
    pub unsafe fn value_ptr<T>(&self, obj: *mut c_void) -> *mut T {
        unsafe { obj.byte_offset(self.offset()).cast() }
    }
}

impl Method {
    #[doc(alias = "method_getTypeEncoding")]
    #[inline]
    pub fn type_encoding(&self) -> Option<&std::ffi::CStr> {
        unsafe {
            let enc = method_getTypeEncoding(self);
            (!enc.is_null()).then(|| std::ffi::CStr::from_ptr(enc))
        }
    }

    #[doc(alias = "method_getImplementation")]
    #[inline]
    pub fn imp(&self) -> extern "C" fn() {
        unsafe { method_getImplementation(self) }
    }

    /// Swaps in `imp` and returns the implementation it displaced.
    ///
    /// This replaces the method on whichever class defines it, so an inherited
    /// method is changed for every subclass too — unlike
    /// [`class_replaceMethod`], which would add an override to one class.
    ///
    /// # Safety
    /// `imp` must have the signature the method's [`Self::type_encoding`]
    /// describes, and callers of the original may run concurrently with the
    /// swap.
    #[doc(alias = "method_setImplementation")]
    #[inline]
    pub unsafe fn set_imp(&self, imp: extern "C" fn()) -> extern "C" fn() {
        unsafe { method_setImplementation(self, imp) }
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

#[doc(hidden)]
#[inline]
pub const fn extra_bytes_for_inner<I>() -> usize {
    let max_padding = std::mem::align_of::<I>().saturating_sub(NS_OBJECT_SIZE);
    std::mem::size_of::<I>() + max_padding
}

#[doc(hidden)]
#[inline]
pub unsafe fn inner_ptr<I>(obj: *const u8) -> *const I {
    let ptr = unsafe { obj.add(NS_OBJECT_SIZE) };
    let align = std::mem::align_of::<I>();
    if align <= NS_OBJECT_SIZE {
        return ptr.cast();
    }

    let offset = ptr.align_offset(align);
    unsafe { ptr.add(offset).cast() }
}

#[doc(hidden)]
#[inline]
pub unsafe fn inner_ptr_mut<I>(obj: *mut u8) -> *mut I {
    unsafe { inner_ptr::<I>(obj).cast_mut() }
}

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
                            $crate::objc::inner_ptr_mut(ptr);
                        d_ptr.write(std::mem::ManuallyDrop::new(T::default()));

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
            let inst =
                class_createInstance(std::mem::transmute(self), extra_bytes_for_inner::<I>());

            // we may skip init?
            // let inst = inst.init();

            let ptr: *mut u8 = std::mem::transmute(inst);
            let d_ptr: *mut std::mem::ManuallyDrop<I> = inner_ptr_mut(ptr);
            d_ptr.write(std::mem::ManuallyDrop::new(var));

            std::mem::transmute(ptr)
        }
    }
}

impl<T: Obj, I: Sized + Default> ClassInstExtra<T, I> {
    pub fn new(&self) -> arc::R<T> {
        self.alloc_init(Default::default())
    }
}

/// Class registered at runtime with an explicit superclass.
///
/// The Rust payload `I` is stored in a real instance variable (see [`InnerSlot`]),
/// so it is valid for any superclass and any allocation path.
#[derive(Debug)]
#[repr(transparent)]
pub struct ClassInstIvar<T: Obj, I: Sized>(Class<T>, PhantomData<I>);

impl<T: Obj, I: Sized> std::ops::Deref for ClassInstIvar<T, I> {
    type Target = Class<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj, I: Sized + Default> ClassInstIvar<T, I> {
    /// `[[Cls alloc] init]`. The payload is set by the generated `+allocWithZone:`.
    pub fn new(&self) -> arc::R<T> {
        unsafe { self.0.new() }
    }
}

/// Name of the instance variable holding the Rust payload of a runtime-registered subclass.
#[doc(hidden)]
pub const INNER_IVAR_NAME: &std::ffi::CStr = c"cidre_inner";

/// Storage for the Rust payload of a runtime-registered subclass.
///
/// Lives inside an instance variable declared with [`class_addIvar`]. The ivar is
/// declared with pointer alignment plus padding, and the slot is aligned at runtime,
/// so payloads with alignment greater than the allocator's are supported.
#[doc(hidden)]
#[repr(C)]
pub struct InnerSlot<I> {
    init: bool,
    value: std::mem::MaybeUninit<I>,
}

impl<I> InnerSlot<I> {
    /// Size of the ivar: the slot plus room to align it at runtime.
    pub const IVAR_SIZE: usize = std::mem::size_of::<Self>()
        + std::mem::align_of::<Self>().saturating_sub(std::mem::size_of::<usize>());

    /// Aligned slot inside the object `obj` whose payload ivar starts at `ivar_offset`.
    #[inline]
    pub unsafe fn from_obj(obj: *mut u8, ivar_offset: usize) -> *mut Self {
        let ptr = unsafe { obj.add(ivar_offset) };
        let align = std::mem::align_of::<Self>();
        if align <= std::mem::size_of::<usize>() {
            return ptr.cast();
        }
        let offset = ptr.align_offset(align);
        unsafe { ptr.add(offset).cast() }
    }

    #[inline]
    pub fn is_init(&self) -> bool {
        self.init
    }

    #[track_caller]
    #[inline]
    pub fn get(&self) -> &I {
        assert!(
            self.init,
            "inner is not initialized: create the object with `with`/`alloc_with`, or make the inner type `Default`"
        );
        unsafe { self.value.assume_init_ref() }
    }

    #[track_caller]
    #[inline]
    pub fn get_mut(&mut self) -> &mut I {
        assert!(
            self.init,
            "inner is not initialized: create the object with `with`/`alloc_with`, or make the inner type `Default`"
        );
        unsafe { self.value.assume_init_mut() }
    }

    /// Stores `value`, dropping the previous one if any.
    #[inline]
    pub fn set(&mut self, value: I) {
        if self.init {
            unsafe { self.value.assume_init_drop() };
        }
        self.value.write(value);
        self.init = true;
    }

    /// Stores `I::default()` unless the slot is already initialized.
    #[inline]
    pub fn init_default(&mut self)
    where
        I: Default,
    {
        if !self.init {
            self.value.write(I::default());
            self.init = true;
        }
    }

    /// Drops the payload if it was initialized.
    #[inline]
    pub unsafe fn drop_in_place(&mut self) {
        if self.init {
            self.init = false;
            unsafe { self.value.assume_init_drop() };
        }
    }
}

/// Fallback for runtime-registered classes without an `#[objc::add_methods] impl Type`
/// block. That block generates an inherent `cls_add_own_methods`, which shadows this one.
#[doc(hidden)]
pub trait OwnMethods: Obj {
    fn cls_add_own_methods(_cls: &Class<Id>) {}
}

impl<T: Obj> OwnMethods for T {}

/// Same specialization hack as `init_with_default!`: emits a `+allocWithZone:` IMP
/// that stores `InnerType::default()` into the payload ivar when `InnerType: Default`,
/// otherwise `None`.
#[doc(hidden)]
#[macro_export]
macro_rules! default_inner_alloc {
    ($NewType:ty, $InnerType:ty) => {{
        trait A {
            fn alloc_fn(&self) -> Option<extern "C" fn()>;
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
            fn alloc_fn(&self) -> Option<extern "C" fn()> {
                extern "C" fn impl_alloc_with_zone<T: Default>(
                    cls: *const std::ffi::c_void,
                    sel: *const std::ffi::c_void,
                    zone: *const std::ffi::c_void,
                ) -> *mut $NewType {
                    unsafe extern "C" {
                        #[link_name = "objc_msgSendSuper"]
                        fn msg_send_super();
                    }
                    unsafe {
                        let sup = $crate::objc::Super {
                            receiver: cls as *mut $crate::objc::Id,
                            super_class: <$NewType>::super_cls().meta_cls(),
                        };
                        let sig: extern "C" fn(
                            *const $crate::objc::Super,
                            *const std::ffi::c_void,
                            *const std::ffi::c_void,
                        ) -> *mut $NewType =
                            std::mem::transmute(msg_send_super as *const std::ffi::c_void);
                        let obj = sig(&sup, sel, zone);
                        if !obj.is_null() {
                            let slot = $crate::objc::InnerSlot::<T>::from_obj(
                                obj.cast(),
                                <$NewType>::inner_offset(),
                            );
                            (*slot).init_default();
                        }
                        obj
                    }
                }

                let ptr = unsafe { std::mem::transmute(impl_alloc_with_zone::<T> as *const u8) };
                Some(ptr)
            }
        }

        impl A for () {
            fn alloc_fn(&self) -> Option<extern "C" fn()> {
                None
            }
        }

        B::<$InnerType>(core::marker::PhantomData).alloc_fn()
    }};
}

impl<T: Obj> arc::A<T> {
    /// `[obj init]` without checking that `T` supports plain `init`.
    #[objc::msg_send(init)]
    pub unsafe fn init_unchecked(self) -> arc::R<T>;
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
    unsafe fn release(ptr: NonNull<Self>) {
        unsafe { <T as Obj>::release(ptr) }
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
                    out("x16") _,
                    out("x17") _,
                    out("x30") _,
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
    unsafe fn release(id: NonNull<Self>) {
        unsafe {
            #[cfg(all(
                target_arch = "aarch64",
                target_pointer_width = "64",
                not(feature = "classic-objc-retain-release")
            ))]
            {
                asm!(
                    "bl _objc_release_{x}",
                    x = in(reg) id.as_ptr(),
                    out("x16") _,
                    out("x17") _,
                    out("x30") _,
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
                objc_release(id.cast().as_ptr());
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
    /// `NSObject` class
    #[inline]
    pub fn cls() -> &'static Class<Id> {
        unsafe { NS_OBJECT }
    }

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

    #[objc::msg_send(hash)]
    pub fn hash(&self) -> ns::UInteger;

    pub fn as_ptr(&self) -> *const Self {
        self as *const Self
    }
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

/// Replaces meta class method implementation and returns original
pub unsafe fn cls_meta_replace_method(
    cls: &std::ffi::CStr,
    sel: &std::ffi::CStr,
    sig: &std::ffi::CStr,
    imp: extern "C" fn(),
) -> Option<extern "C" fn()> {
    unsafe {
        let cls = objc::objc_getClass(cls.as_ptr().cast())?;
        let meta = objc::object_getClass(Some(std::mem::transmute(cls)))?;
        let sel = objc::sel_reg_name(sel.as_ptr());
        objc::class_replaceMethod(meta, sel, imp, sig.as_ptr().cast())
    }
}

pub unsafe fn cls_replace_method(
    cls: &std::ffi::CStr,
    sel: &std::ffi::CStr,
    sig: &std::ffi::CStr,
    imp: extern "C" fn(),
) -> Option<extern "C" fn()> {
    unsafe {
        let cls = objc::objc_getClass(cls.as_ptr().cast())?;
        let sel = objc::sel_reg_name(sel.as_ptr());
        objc::class_replaceMethod(cls, sel, imp, sig.as_ptr().cast())
    }
}

#[doc(alias = "objc_super")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Super {
    pub receiver: *mut Id,
    pub super_class: *const Class<Id>,
}

/// `objc_msgSendSuper` for a selector that takes no arguments and returns nothing.
#[doc(hidden)]
#[inline]
pub unsafe fn msg_send_super_void(sup: &Super, sel: &Sel) {
    unsafe extern "C" {
        #[link_name = "objc_msgSendSuper"]
        fn msg_send_super();
    }
    unsafe {
        let sig: extern "C" fn(&Super, &Sel) = std::mem::transmute(msg_send_super as *const c_void);
        sig(sup, sel)
    }
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
    fn objc_release(obj: *mut Id);

    // fn objc_msgSend();

    pub fn class_createInstance(cls: &Class<Id>, extra_bytes: usize) -> arc::A<Id>;
    fn class_getMethodImplementation(cls: &Class<Id>, name: &Sel) -> *const c_void;
    fn class_addProtocol(cls: &Class<Id>, protocol: &Protocol) -> bool;
    fn objc_autorelease<'ar>(id: &mut Id) -> &'ar mut Id;

    pub fn objc_retainAutoreleasedReturnValue<'ar>(obj: Option<&Id>) -> Option<arc::R<Id>>;
    pub fn objc_retainAutoreleaseReturnValue<'ar>(obj: Option<&Id>) -> Option<&'ar Id>;
    #[cfg(any(
        all(target_os = "macos", feature = "macos_13_0"),
        all(target_os = "ios", feature = "ios_16_0"),
        all(target_os = "tvos", feature = "tvos_16_0"),
        all(target_os = "watchos", feature = "watchos_9_0"),
        all(target_os = "visionos", feature = "visionos_1_0"),
    ))]
    pub fn objc_claimAutoreleasedReturnValue(obj: Option<&Id>) -> Option<arc::R<Id>>;
    pub fn objc_autoreleaseReturnValue<'ar>(obj: Option<&Id>) -> Option<&'ar Id>;

    pub fn objc_copyWeak<'ar>(dest: *mut *mut Id, src: *mut *mut Id) -> Option<&'ar Id>;
    pub fn objc_destroyWeak(location: *mut *mut Id);
    pub fn objc_storeWeak<'ar>(location: *mut *mut Id, value: Option<&Id>) -> Option<&'ar Id>;
    pub fn objc_loadWeakRetained(location: *mut *mut Id) -> Option<arc::R<Id>>;

    pub fn object_getIndexedIvars(obj: *const c_void) -> *mut c_void;

    fn class_getInstanceVariable<'a>(cls: &Class<Id>, name: *const i8) -> Option<&'a Ivar>;
    fn ivar_getName(ivar: &Ivar) -> *const i8;
    fn ivar_getTypeEncoding(ivar: &Ivar) -> *const i8;
    fn ivar_getOffset(ivar: &Ivar) -> isize;

    fn class_getInstanceMethod<'a>(cls: &Class<Id>, sel: &Sel) -> Option<&'a Method>;
    fn method_getTypeEncoding(m: &Method) -> *const i8;
    fn method_getImplementation(m: &Method) -> extern "C" fn();
    fn method_setImplementation(m: &Method, imp: extern "C" fn()) -> extern "C" fn();
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
    /// `alignment` is log2 of the variable's alignment. Only valid before `objc_registerClassPair`.
    pub fn class_addIvar(
        cls: &Class<Id>,
        name: *const i8,
        size: usize,
        alignment: u8,
        types: *const i8,
    ) -> bool;
    pub fn object_getClass(obj: Option<&Id>) -> Option<&Class<Id>>;
    pub fn objc_registerClassPair(cls: &Class<Id>);
    pub fn objc_getClass(name: *const u8) -> Option<&'static Class<Id>>;
    pub fn class_respondsToSelector(cls: &Class<Id>, sel: &Sel) -> bool;
    pub fn objc_getProtocol(name: *const i8) -> Option<&'static Protocol>;
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

/// Defines an Objective-C object wrapper, or registers a new class at runtime.
///
/// Runtime-registered classes come in two flavours:
///
/// - `define_obj_type!(Name + Trait.., Inner, CLS)` — a direct `NSObject` subclass,
///   optimized for delegates: the payload lives in the object's extra bytes.
/// - `define_obj_type!(Name(Base) + Trait.., Inner, CLS)` — a subclass of `Base`
///   (e.g. `ns::View`), where `Base::cls()` must exist. The payload is a real instance
///   variable. Methods are overridden and initializers declared in an
///   `#[objc::add_methods] impl Name { .. }` block with `#[objc::overrides(sel)]` and
///   `#[objc::init(sel)]`; every override gets a `super_*` twin. Instances are created
///   with `Name::alloc_with(inner)` followed by an initializer, or `Name::with(inner)`
///   for plain `init`. If `Inner: Default`, objects created from Objective-C (nibs,
///   `[[Name alloc] init]`) get `Inner::default()`; otherwise `inner()` panics on them.
///
/// A class from the `NSObject` form can itself be used as `Base` only when its
/// payload is `()`: its extra-bytes payload is not part of the instance size.
///
/// Overridden methods are added without type encodings, so `NSInvocation`-based
/// dispatch to them (e.g. `performSelector:withObject:afterDelay:`) is not supported.
#[macro_export]
macro_rules! define_obj_type {
    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident($BaseType:path) $(+ $TraitImpl:path)*, $InnerType:path, $CLS:ident) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis
            $NewType($BaseType)
        );


        impl $NewType {
            #[allow(dead_code)]
            #[inline]
            pub fn super_cls() -> &'static $crate::objc::Class<$BaseType> {
                <$BaseType>::cls()
            }

            #[doc(hidden)]
            pub fn inner_offset() -> usize {
                static OFFSET: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                *OFFSET.get_or_init(|| {
                    let cls: &$crate::objc::Class<Self> = Self::cls();
                    let ivar = cls
                        .instance_var($crate::objc::INNER_IVAR_NAME)
                        .expect(concat!("class ", stringify!($CLS), " has no cidre inner ivar"));
                    ivar.offset() as usize
                })
            }

            #[allow(dead_code)]
            #[inline]
            pub fn inner(&self) -> &$InnerType {
                unsafe {
                    let ptr = self as *const Self as *mut u8;
                    let slot = $crate::objc::InnerSlot::<$InnerType>::from_obj(ptr, Self::inner_offset());
                    (*slot).get()
                }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn inner_mut(&mut self) -> &mut $InnerType {
                unsafe {
                    let ptr = self as *mut Self as *mut u8;
                    let slot = $crate::objc::InnerSlot::<$InnerType>::from_obj(ptr, Self::inner_offset());
                    (*slot).get_mut()
                }
            }

            #[allow(dead_code)]
            pub fn register_cls() -> &'static $crate::objc::ClassInstIvar<Self, $InnerType> {
                let name = concat!(stringify!($CLS), "\0");
                let super_cls: &'static $crate::objc::Class<$crate::objc::Id> =
                    unsafe { std::mem::transmute(Self::super_cls()) };
                let cls = unsafe { $crate::objc::objc_allocateClassPair(super_cls, name.as_ptr(), 0) };
                let cls = cls.expect(concat!("can't allocate class pair ", stringify!($CLS)));
                let added = unsafe {
                    $crate::objc::class_addIvar(
                        cls,
                        $crate::objc::INNER_IVAR_NAME.as_ptr(),
                        $crate::objc::InnerSlot::<$InnerType>::IVAR_SIZE,
                        std::mem::size_of::<usize>().trailing_zeros() as u8,
                        c"?".as_ptr(),
                    )
                };
                assert!(added, concat!("can't add inner ivar to ", stringify!($CLS)));
                $(<Self as $TraitImpl>::cls_add_methods(cls);)*
                $(<Self as $TraitImpl>::cls_add_protocol(cls);)*
                {
                    #[allow(unused_imports)]
                    use $crate::objc::OwnMethods as _;
                    Self::cls_add_own_methods(cls);
                }

                if let Some(imp) = $crate::default_inner_alloc!($NewType, $InnerType) {
                    unsafe {
                        let sel = $crate::objc::sel_reg_name(c"allocWithZone:".as_ptr() as _);
                        $crate::objc::class_addMethod(cls.meta_cls(), sel, imp, std::ptr::null());
                    }
                }

                if std::mem::needs_drop::<$InnerType>() {
                    extern "C" fn impl_dealloc(s: &mut $NewType, sel: &$crate::objc::Sel) {
                        unsafe {
                            let slot = $crate::objc::InnerSlot::<$InnerType>::from_obj(
                                s as *mut $NewType as *mut u8,
                                <$NewType>::inner_offset(),
                            );
                            (*slot).drop_in_place();
                            let sup = $crate::objc::Super {
                                receiver: s as *mut $NewType as *mut $crate::objc::Id,
                                super_class: <$NewType>::super_cls() as *const _ as *const $crate::objc::Class<$crate::objc::Id>,
                            };
                            $crate::objc::msg_send_super_void(&sup, sel);
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
            #[inline]
            pub fn cls() -> &'static $crate::objc::ClassInstIvar<Self, $InnerType> {
                static CLS: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                let ptr = *CLS.get_or_init(|| {
                    let name = concat!(stringify!($CLS), "\0");
                    match unsafe { $crate::objc::objc_getClass(name.as_ptr()) } {
                        Some(c) => c as *const _ as usize,
                        None => Self::register_cls() as *const _ as usize,
                    }
                });
                unsafe { &*(ptr as *const $crate::objc::ClassInstIvar<Self, $InnerType>) }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn cls_ptr() -> *const std::ffi::c_void {
                Self::cls() as *const $crate::objc::ClassInstIvar<Self, $InnerType> as *const std::ffi::c_void
            }

            /// Allocates an instance and stores `inner`; call an initializer next.
            #[allow(dead_code)]
            pub fn alloc_with(inner: $InnerType) -> $crate::arc::A<Self> {
                let obj = Self::cls().alloc();
                unsafe {
                    let slot = $crate::objc::InnerSlot::<$InnerType>::from_obj(
                        obj.as_ptr() as *mut u8,
                        Self::inner_offset(),
                    );
                    (*slot).set(inner);
                }
                obj
            }

            /// `[[Self alloc] init]` with `inner` as the payload. Other initializers are
            /// declared with `#[objc::init(initWith..:)]` in an `#[objc::add_methods]` block.
            #[allow(dead_code)]
            pub fn with(inner: $InnerType) -> $crate::arc::R<Self> {
                unsafe { Self::alloc_with(inner).init_unchecked() }
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident($BaseType:path) $(+ $TraitImpl:path)*, (), $CLS:ident) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis
            $NewType($BaseType)
        );


        impl $NewType {
            #[allow(dead_code)]
            #[inline]
            pub fn super_cls() -> &'static $crate::objc::Class<$BaseType> {
                <$BaseType>::cls()
            }

            #[allow(dead_code)]
            pub fn register_cls() -> &'static $crate::objc::ClassInstIvar<Self, ()> {
                let name = concat!(stringify!($CLS), "\0");
                let super_cls: &'static $crate::objc::Class<$crate::objc::Id> =
                    unsafe { std::mem::transmute(Self::super_cls()) };
                let cls = unsafe { $crate::objc::objc_allocateClassPair(super_cls, name.as_ptr(), 0) };
                let cls = cls.expect(concat!("can't allocate class pair ", stringify!($CLS)));
                $(<Self as $TraitImpl>::cls_add_methods(cls);)*
                $(<Self as $TraitImpl>::cls_add_protocol(cls);)*
                {
                    #[allow(unused_imports)]
                    use $crate::objc::OwnMethods as _;
                    Self::cls_add_own_methods(cls);
                }

                unsafe { $crate::objc::objc_registerClassPair(cls) };
                unsafe { std::mem::transmute(cls) }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn cls() -> &'static $crate::objc::ClassInstIvar<Self, ()> {
                static CLS: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                let ptr = *CLS.get_or_init(|| {
                    let name = concat!(stringify!($CLS), "\0");
                    match unsafe { $crate::objc::objc_getClass(name.as_ptr()) } {
                        Some(c) => c as *const _ as usize,
                        None => Self::register_cls() as *const _ as usize,
                    }
                });
                unsafe { &*(ptr as *const $crate::objc::ClassInstIvar<Self, ()>) }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn cls_ptr() -> *const std::ffi::c_void {
                Self::cls() as *const $crate::objc::ClassInstIvar<Self, ()> as *const std::ffi::c_void
            }

            #[allow(dead_code)]
            #[inline]
            pub fn alloc() -> $crate::arc::A<Self> {
                Self::cls().alloc()
            }

            #[allow(dead_code)]
            pub fn new() -> $crate::arc::R<Self> {
                unsafe { Self::alloc().init_unchecked() }
            }
        }
    };
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
            pub fn super_cls() -> &'static $crate::objc::Class<$crate::objc::Id> {
                $crate::objc::Id::cls()
            }

            #[allow(dead_code)]
            #[inline]
            pub fn inner(&self) -> &$InnerType {
                unsafe {
                    let ptr = self as *const Self as *const u8;
                    &*$crate::objc::inner_ptr(ptr)
                }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn inner_mut(&mut self) -> &mut $InnerType {
                unsafe {
                    let ptr = self as *mut Self as *mut u8;
                    &mut *$crate::objc::inner_ptr_mut(ptr)
                }
            }

            #[allow(dead_code)]
            pub fn register_cls() -> &'static $crate::objc::ClassInstExtra<Self, $InnerType> {
                let name = concat!(stringify!($CLS), "\0");
                let cls = unsafe { $crate::objc::objc_allocateClassPair($crate::objc::NS_OBJECT, name.as_ptr(), 0) };
                let cls = cls.unwrap();
                $(<Self as $TraitImpl>::cls_add_methods(cls);)*
                $(<Self as $TraitImpl>::cls_add_protocol(cls);)*
                {
                    #[allow(unused_imports)]
                    use $crate::objc::OwnMethods as _;
                    Self::cls_add_own_methods(cls);
                }

                if let Some(init_fn_ptr) = $crate::init_with_default!($NewType, $InnerType) {
                    unsafe {
                        let sel = $crate::objc::sel_reg_name(c"init".as_ptr() as _);
                        let imp: extern "C" fn() = init_fn_ptr;
                        $crate::objc::class_addMethod(cls, sel, imp, std::ptr::null());

                        let sel = $crate::objc::sel_reg_name(c"alloc".as_ptr() as _);
                        let meta_cls = $crate::objc::object_getClass(Some(std::mem::transmute(cls))).unwrap();

                        extern "C" fn alloc_impl(cls: &$crate::objc::Class<$crate::ns::Id>) -> $crate::arc::A<$NewType> {
                            unsafe {
                                let inst = $crate::objc::class_createInstance(
                                    cls,
                                    $crate::objc::extra_bytes_for_inner::<$InnerType>(),
                                );
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
                            $crate::objc::msg_send_super_void(&sup, sel);
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
            #[inline]
            pub fn super_cls() -> &'static $crate::objc::Class<$crate::objc::Id> {
                $crate::objc::Id::cls()
            }

            #[allow(dead_code)]
            pub fn register_cls() -> &'static $crate::objc::ClassInstExtra<Self, ()> {
                let name = concat!(stringify!($CLS), "\0");
                let cls = unsafe { $crate::objc::objc_allocateClassPair($crate::objc::NS_OBJECT, name.as_ptr(), 0) };
                let cls = cls.unwrap();
                $(<Self as $TraitImpl>::cls_add_methods(cls);)*
                $(<Self as $TraitImpl>::cls_add_protocol(cls);)*
                {
                    #[allow(unused_imports)]
                    use $crate::objc::OwnMethods as _;
                    Self::cls_add_own_methods(cls);
                }

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

impl Eq for Id {}
impl std::hash::Hash for Id {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

impl Eq for arc::R<Id> {}
impl std::hash::Hash for arc::R<Id> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash().hash(state);
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
mod is_getter_tests {
    use std::ffi::CStr;

    use crate::objc;

    fn responds(cls: &CStr, sel: &CStr) -> Option<bool> {
        // SAFETY: both are NUL-terminated literals; a missing class is `None`.
        unsafe {
            let cls = objc::objc_getClass(cls.as_ptr().cast())?;
            Some(objc::class_respondsToSelector(
                cls,
                objc::sel_reg_name(sel.as_ptr()),
            ))
        }
    }

    /// A property declared `getter=isX` answers `isX`, not `x`. Binding the
    /// bare name sends a selector the class does not implement, which aborts
    /// the process the first time it is called — so these have to be right.
    #[test]
    fn bindings_use_the_declared_getter() {
        let cases: &[(&CStr, &CStr, &CStr)] = &[
            (c"SCContentSharingPicker", c"active", c"isActive"),
            (
                c"AVCaptureDeviceFormat",
                c"globalToneMappingSupported",
                c"isGlobalToneMappingSupported",
            ),
            (c"NSSplitViewItem", c"collapsed", c"isCollapsed"),
            (c"NSWindow", c"opaque", c"isOpaque"),
        ];
        for (cls, bare, getter) in cases {
            let Some(has_getter) = responds(cls, getter) else {
                continue; // framework not present on this platform
            };
            assert!(has_getter, "{cls:?} should respond to {getter:?}");
            assert_eq!(
                responds(cls, bare),
                Some(false),
                "{cls:?} must not respond to the bare {bare:?} — cidre would be \
                 sending an unrecognized selector"
            );
        }
    }
}

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
pub use cidre_macros::init;
pub use cidre_macros::optional;
pub use cidre_macros::overrides;
pub use cidre_macros::protocol;

/// Docs
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send_debug;
#[cfg(target_arch = "aarch64")]
pub use cidre_macros::msg_send_super;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::msg_send_super_x86_64 as msg_send_super;
#[cfg(target_arch = "x86_64")]
pub use cidre_macros::msg_send_x86_64 as msg_send;

#[cfg(test)]
mod tests2 {

    use std::collections::HashMap;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    use crate::{
        arc::{self, Retain},
        ns,
        objc::{self, Obj},
        return_rar,
    };

    #[objc::protocol(Foo)]
    trait Foo: objc::Obj {
        #[objc::msg_send(count)]
        fn count(&self) -> usize;

        #[objc::msg_send(newObj)]
        fn new_obj(&self) -> arc::R<ns::String>;

        #[objc::msg_send(prop)]
        fn prop(&self) -> arc::R<ns::String>;

        #[objc::optional]
        #[objc::msg_send(count2)]
        fn count2(&self) -> usize;

        fn direct_fn(&self);
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

    #[repr(align(128))]
    struct Aligned128 {
        value: usize,
        dropped: Arc<AtomicBool>,
    }

    impl Drop for Aligned128 {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    define_obj_type!(Aligned128Obj, Aligned128, ALIGNED_128_OBJ);

    #[derive(Default)]
    #[repr(align(128))]
    struct DefaultAligned128 {
        value: usize,
    }

    define_obj_type!(
        DefaultAligned128Obj,
        DefaultAligned128,
        DEFAULT_ALIGNED_128_OBJ
    );

    impl Foo for Bla {
        fn direct_fn(&self) {}
    }

    #[objc::add_methods]
    impl FooImpl for Bla {
        extern "C" fn impl_count(&self, _cmd: Option<&objc::Sel>) -> usize {
            0
        }

        extern "C" fn impl_new_obj(&self, _cmd: Option<&objc::Sel>) -> arc::R<ns::String> {
            ns::String::new()
        }

        extern "C" fn impl_prop_ar(&self, _cmd: Option<&objc::Sel>) -> arc::Rar<ns::String> {
            let s = ns::str!(c"test");
            return_rar!(s)
        }
    }

    #[test]
    fn basics() {
        unsafe {
            DROP_CALLED = false;
        }
        {
            let d = Bla::with(D);
            assert_eq!(d.prop().to_string(), "test");
            let _r = d.retained();
            let desc = d.desc();
            assert!(desc.to_string().starts_with("<BLA_USIZE: "));
        }
        assert!(unsafe { DROP_CALLED });
    }

    #[test]
    fn aligned_128_inner() {
        assert_eq!(
            objc::extra_bytes_for_inner::<Aligned128>(),
            std::mem::size_of::<Aligned128>() + 120
        );

        let dropped = Arc::new(AtomicBool::new(false));
        {
            let mut obj = Aligned128Obj::with(Aligned128 {
                value: 42,
                dropped: Arc::clone(&dropped),
            });
            assert_eq!(obj.inner() as *const Aligned128 as usize % 128, 0);
            assert_eq!(obj.inner().value, 42);

            obj.inner_mut().value = 84;
            assert_eq!(obj.inner().value, 84);
        }
        assert!(dropped.load(Ordering::SeqCst));
    }

    #[test]
    fn aligned_128_inner_with_objc_new() {
        let cls: &objc::Class<DefaultAligned128Obj> = DefaultAligned128Obj::cls();
        let obj = unsafe { cls.new() };
        assert_eq!(obj.inner() as *const DefaultAligned128 as usize % 128, 0);
        assert_eq!(obj.inner().value, 0);
    }

    #[test]
    fn hash() {
        fn foo() -> HashMap<arc::R<ns::Id>, arc::R<ns::String>> {
            let a = ns::String::new();
            let b = ns::String::new();
            let mut map = HashMap::new();
            let _v = map.insert(a.as_id_ref().retained(), b);
            map
        }

        foo();
    }

    // --- subclassing with an explicit base class ---

    struct SubInner {
        tag: usize,
        dropped: Arc<AtomicBool>,
    }

    impl Drop for SubInner {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    define_obj_type!(SubId(ns::Id), SubInner, CIDRE_TEST_SUB_ID);

    #[objc::add_methods]
    impl SubId {
        #[objc::init(init)]
        fn init(self) -> arc::R<Self>;

        #[objc::overrides(hash)]
        fn hash(&self) -> usize {
            self.super_hash().wrapping_add(self.inner().tag)
        }

        #[objc::overrides(description)]
        fn desc(&self) -> arc::R<ns::String> {
            let sup = self.super_desc();
            ns::String::with_str(&format!("sub:{}", sup))
        }

        // not registered: a plain method next to overrides
        fn tag(&self) -> usize {
            self.inner().tag
        }
    }

    // second level: SubSubId -> SubId -> NSObject, with its own payload
    define_obj_type!(SubSubId(SubId), u32, CIDRE_TEST_SUB_SUB_ID);

    #[objc::add_methods]
    impl SubSubId {
        #[objc::init(init)]
        pub fn init(self) -> arc::R<Self>;

        #[objc::overrides(hash)]
        fn hash(&self) -> usize {
            self.super_hash().wrapping_add(*self.inner() as usize)
        }
    }

    #[test]
    fn subclass_overrides_and_super() {
        let dropped = Arc::new(AtomicBool::new(false));
        {
            let obj = SubId::alloc_with(SubInner {
                tag: 7,
                dropped: Arc::clone(&dropped),
            })
            .init();
            let base_hash = &*obj as *const SubId as usize; // NSObject hash is the address
            assert_eq!(ns::Id::hash(&obj), base_hash.wrapping_add(7));
            assert_eq!(obj.tag(), 7);
            let desc = <ns::Id as Obj>::desc(&obj).to_string();
            assert!(desc.starts_with("sub:<CIDRE_TEST_SUB_ID: "), "{desc}");
            assert!(obj.is_kind_of_class(ns::Id::cls()));
            assert!(obj.is_kind_of_class(SubId::cls()));
            assert!(!dropped.load(Ordering::SeqCst));
        }
        assert!(dropped.load(Ordering::SeqCst));
    }

    #[test]
    fn subclass_of_subclass() {
        let dropped = Arc::new(AtomicBool::new(false));
        {
            // SubSubId payload is Default, so `new()` works
            let mut obj = SubSubId::cls().new();
            *obj.inner_mut() = 100;
            // inner of the base class is not initialized: base override must not touch it
            let base: &SubId = &obj;
            let base_hash = base as *const SubId as usize;
            let slot_uninit = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = base.inner();
            }))
            .is_err();
            assert!(slot_uninit);

            let obj = SubSubId::alloc_with(5).init();
            let sub: &SubId = &obj;
            unsafe {
                let slot = objc::InnerSlot::<SubInner>::from_obj(
                    sub as *const SubId as *mut u8,
                    SubId::inner_offset(),
                );
                (*slot).set(SubInner {
                    tag: 1,
                    dropped: Arc::clone(&dropped),
                });
            }
            let addr = &*obj as *const SubSubId as usize;
            // SubSubId: super(SubId: super(NSObject) + 1) + 5
            assert_eq!(ns::Id::hash(&obj), addr.wrapping_add(6));
            assert_ne!(base_hash, addr);
            assert!(obj.is_kind_of_class(SubId::cls()));
            assert!(
                <ns::Id as Obj>::desc(&obj)
                    .to_string()
                    .starts_with("sub:<CIDRE_TEST_SUB_SUB_ID: ")
            );
        }
        assert!(dropped.load(Ordering::SeqCst));
    }

    define_obj_type!(
        SubAligned128(ns::Id),
        Aligned128,
        CIDRE_TEST_SUB_ALIGNED_128
    );
    define_obj_type!(
        SubDefaultAligned128(ns::Id),
        DefaultAligned128,
        CIDRE_TEST_SUB_DEFAULT_ALIGNED_128
    );

    #[test]
    fn subclass_aligned_128_inner() {
        let dropped = Arc::new(AtomicBool::new(false));
        {
            let mut obj = SubAligned128::with(Aligned128 {
                value: 42,
                dropped: Arc::clone(&dropped),
            });
            assert_eq!(obj.inner() as *const Aligned128 as usize % 128, 0);
            assert_eq!(obj.inner().value, 42);
            obj.inner_mut().value = 84;
            assert_eq!(obj.inner().value, 84);
        }
        assert!(dropped.load(Ordering::SeqCst));
    }

    #[test]
    fn subclass_aligned_128_inner_with_objc_new() {
        let obj = SubDefaultAligned128::cls().new();
        assert_eq!(obj.inner() as *const DefaultAligned128 as usize % 128, 0);
        assert_eq!(obj.inner().value, 0);

        // `alloc_with` replaces the default payload
        let obj = SubDefaultAligned128::with(DefaultAligned128 { value: 3 });
        assert_eq!(obj.inner().value, 3);
    }
}
