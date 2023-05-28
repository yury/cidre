// TODO:
// 1. Pure fn block is global _NSConcreteGlobalBlock
// 2. Pure fn block with fields - _NSConcreteMallocBlock
// 3. NoEscaping block - _NSConcreteStackBlock

// https://opensource.apple.com/source/libclosure/libclosure-79/BlockImplementation.txt.auto.html
// https://github.com/apple-oss-distributions/libclosure/blob/main/BlockImplementation.txt
// https://developer.apple.com/documentation/swift/calling-objective-c-apis-asynchronously
// https://github.com/apple/swift-corelibs-foundation/blob/main/Sources/BlocksRuntime/runtime.c

use std::{
    ffi::c_void,
    mem::{self, transmute},
    ops,
};

use crate::{arc, define_options, ns, objc::Class};

#[repr(transparent)]
pub struct Block<F>(c_void, std::marker::PhantomData<F>);

impl<F> Block<F> {
    #[inline]
    pub fn as_ptr(&mut self) -> *mut c_void {
        unsafe { transmute(self) }
    }
}

pub fn fn0<R>(f: extern "C" fn(*const c_void) -> R) -> bl<extern "C" fn(*const c_void) -> R> {
    bl::with(f)
}

pub fn fn1<A, R>(
    f: extern "C" fn(*const c_void, a: A) -> R,
) -> bl<extern "C" fn(*const c_void, a: A) -> R> {
    bl::with(f)
}

pub fn fn2<A, B, R>(
    f: extern "C" fn(*const c_void, a: A, b: B) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B) -> R> {
    bl::with(f)
}

pub fn fn3<A, B, C, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C) -> R> {
    bl::with(f)
}

pub fn fn4<A, B, C, D, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C, d: D) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C, d: D) -> R> {
    bl::with(f)
}

pub fn fn5<A, B, C, D, E, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C, d: D, e: E) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C, d: D, e: E) -> R> {
    bl::with(f)
}

pub fn once0<R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce() -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke0 as _, f)
}

pub fn once1<A, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke1 as _, f)
}

pub fn once2<A, B, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke2 as _, f)
}

pub fn once3<A, B, C, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B, C) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke3 as _, f)
}

pub fn once4<A, B, C, D, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B, C, D) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke4 as _, f)
}

pub fn once5<A, B, C, D, E, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B, C, D, E) -> R,
{
    Layout2Once::new(Layout2Once::<F>::invoke5 as _, f)
}

pub fn no_esc0<R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut() -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke0 as _, f)
}

pub fn no_esc1<A, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke1 as _, f)
}

pub fn no_esc2<A, B, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke2 as _, f)
}

pub fn no_esc3<A, B, C, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B, C) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke3 as _, f)
}

pub fn no_esc4<A, B, C, D, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B, C, D) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke4 as _, f)
}

pub fn no_esc5<A, B, C, D, E, R, F>(f: &mut F) -> NoEscBlMut<F>
where
    F: FnMut(A, B, C, D, E) -> R,
{
    Layout1Mut::new(Layout1Mut::<F>::invoke5 as _, f)
}

pub fn mut0<R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut() -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke0 as _, f)
}

pub fn mut1<A, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke1 as _, f)
}

pub fn mut2<A, B, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke2 as _, f)
}

pub fn mut3<A, B, C, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B, C) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke3 as _, f)
}

pub fn mut4<A, B, C, D, R, F>(f: F) -> BlMut<F>
where
    F: FnMut(A, B, C, D) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke4 as _, f)
}

pub fn mut5<A, B, C, D, E, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B, C, D, E) -> R,
{
    Layout2Mut::new(Layout2Mut::<F>::invoke5 as _, f)
}

define_options!(Flags(i32));

impl Flags {
    pub const NONE: Self = Self(0);

    // runtime
    pub const DEALLOCATING: Self = Self(1);

    // runtime
    pub const REFCOUNT_MASK: Self = Self(0xfffei32);

    // compiler
    // Set to true on blocks that have captures (and thus are not true
    // global blocks) but are known not to escape for various other
    // reasons. For backward compatibility with old runtimes, whenever
    // IS_NOESCAPE is set, IS_GLOBAL is set too. Copying a
    // non-escaping block returns the original block and releasing such a
    // block is a no-op, which is exactly how global blocks are handled.
    pub const IS_NOESCAPE: Self = Self(1 << 23);

    // runtime
    pub const NEEDS_FREE: Self = Self(1 << 24);
    // compiler
    pub const HAS_COPY_DISPOSE: Self = Self(1 << 25);
    pub const HAS_CTOR: Self = Self(1 << 26);
    pub const IS_GC: Self = Self(1 << 27);
    pub const IS_GLOBAL: Self = Self(1 << 28);
    pub const USE_STRET: Self = Self(1 << 29);
    pub const HAS_SIGNATURE: Self = Self(1 << 30);
    pub const HAS_EXTENDED_LAYOUT: Self = Self(1 << 31);

    const RETAINED_NEEDS_FREE: Self = Self(2 | Self::NEEDS_FREE.0);
    const RETAINED_NEEDS_DROP: Self = Self(2 | Self::NEEDS_FREE.0 | Self::HAS_COPY_DISPOSE.0);
}

#[repr(C)]
pub struct Layout1Mut<'a, F> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor1,
    closure: &'a mut F,
}

pub type NoEscBlMut<'a, F> = Layout1Mut<'a, F>;

impl<'a, F> std::ops::Deref for Layout1Mut<'a, F> {
    type Target = Block<F>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, F> std::ops::DerefMut for Layout1Mut<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, F> Layout1Mut<'a, F> {
    const DESCRIPTOR_1: Descriptor1 = Descriptor1 {
        reserved: 0,
        size: std::mem::size_of::<&'static Class<ns::Id>>()
            + std::mem::size_of::<Flags>()
            + std::mem::size_of::<i32>()
            + std::mem::size_of::<*const c_void>()
            + std::mem::size_of::<&'static Descriptor1>()
            + std::mem::size_of::<&'static c_void>(), // emulating &mut F
    };

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.closure)()
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.closure)(a)
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnMut(A, B) -> R,
    {
        (self.closure)(a, b)
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnMut(A, B, C) -> R,
    {
        (self.closure)(a, b, c)
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnMut(A, B, C, D) -> R,
    {
        (self.closure)(a, b, c, d)
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnMut(A, B, C, D, E) -> R,
    {
        (self.closure)(a, b, c, d, e)
    }

    fn new(invoke: *const c_void, f: &'a mut F) -> Layout1Mut<'a, F> {
        Self {
            isa: unsafe { &_NSConcreteStackBlock },
            flags: Default::default(),
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_1,
            closure: f,
        }
    }
}

#[repr(C)]
pub struct Descriptor1 {
    reserved: usize,
    size: usize,
}

#[repr(C)]
pub struct Descriptor2<T: Sized> {
    descriptor1: Descriptor1,
    copy: extern "C" fn(dest: &mut T, src: &mut T),
    dispose: extern "C" fn(liteal: &mut T),
}

// for completion handlers
#[repr(C)]
struct Layout2Once<F: Sized + 'static> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor2<Self>,
    closure: Option<F>,
}

#[repr(C)]
struct Layout2Mut<F: Sized + 'static> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor2<Self>,
    closure: mem::ManuallyDrop<F>,
}

/// block with static fn
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct bl<F: Sized> {
    isa: &'static Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: F,
    descriptor: &'static Descriptor1,
}

impl<F: Sized> bl<F> {
    #[inline]
    pub fn escape<'a>(&mut self) -> &'a mut Block<F> {
        unsafe { mem::transmute(self) }
    }
}

#[repr(transparent)]
pub struct BlOnce<F: 'static>(&'static mut Block<F>);

impl<F> BlOnce<F> {
    #[inline]
    pub fn escape<'a>(self) -> &'a mut Block<F> {
        unsafe { transmute(self) }
    }
}

#[repr(transparent)]
pub struct BlMut<F: 'static>(&'static mut Block<F>);

impl<F> BlMut<F> {
    #[inline]
    pub fn escape(&mut self) -> &'static mut Block<F> {
        let ptr = self.0 as *mut Block<F>;
        unsafe { &mut *ptr }
    }
}

impl<F> Drop for BlOnce<F> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            _Block_release(self.0 as *mut _ as *const _);
        };
    }
}

impl<F> Drop for BlMut<F> {
    #[inline]
    fn drop(&mut self) {
        unsafe { _Block_release(self.0 as *mut _ as *const _) };
    }
}

impl<F> Clone for BlMut<F> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let ptr = _Block_copy(mem::transmute(self));
            Self(mem::transmute(ptr))
        }
    }
}

impl<F> ops::Deref for BlOnce<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<F> ops::Deref for BlMut<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<F> ops::DerefMut for BlMut<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<F> ops::Deref for bl<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<F> ops::DerefMut for bl<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<F: Sized> Layout2Once<F> {
    const DESCRIPTOR_2: Descriptor2<Self> = Descriptor2 {
        descriptor1: Descriptor1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn copy(_dest: &mut Self, _src: &mut Self) {
        panic!("copy should not be called");
    }

    extern "C" fn dispose(block: &mut Self) {
        debug_assert!(mem::needs_drop::<F>());
        block.closure.take();
    }

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnOnce() -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)()
        } else {
            panic!()
        }
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnOnce(A) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnOnce(A, B) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnOnce(A, B, C) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnOnce(A, B, C, D) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c, d)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnOnce(A, B, C, D, E) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c, d, e)
        } else {
            panic!()
        }
    }

    fn new(invoke: *const c_void, f: F) -> BlOnce<F> {
        let flags = if mem::needs_drop::<F>() {
            Flags::RETAINED_NEEDS_DROP
        } else {
            Flags::RETAINED_NEEDS_FREE
        };

        let block = Box::new(Self {
            isa: unsafe { &_NSConcreteMallocBlock },
            flags,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            closure: Some(f),
        });
        let block_ref = Box::leak(block);
        BlOnce(unsafe { mem::transmute(block_ref) })
    }
}

impl<F: Sized> Layout2Mut<F> {
    const DESCRIPTOR_2: Descriptor2<Self> = Descriptor2 {
        descriptor1: Descriptor1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn copy(_dest: &mut Self, _src: &mut Self) {
        panic!("copy should not be called");
    }

    extern "C" fn dispose(block: &mut Self) {
        debug_assert!(mem::needs_drop::<F>());
        unsafe {
            mem::ManuallyDrop::drop(&mut block.closure);
        }
    }

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.closure)()
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.closure)(a)
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnMut(A, B) -> R,
    {
        (self.closure)(a, b)
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnMut(A, B, C) -> R,
    {
        (self.closure)(a, b, c)
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnMut(A, B, C, D) -> R,
    {
        (self.closure)(a, b, c, d)
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnMut(A, B, C, D, E) -> R,
    {
        (self.closure)(a, b, c, d, e)
    }

    fn new(invoke: *const c_void, f: F) -> BlMut<F> {
        let flags = if mem::needs_drop::<F>() {
            Flags::RETAINED_NEEDS_DROP
        } else {
            Flags::RETAINED_NEEDS_FREE
        };

        let block = Box::new(Self {
            isa: unsafe { &_NSConcreteMallocBlock },
            flags,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            closure: mem::ManuallyDrop::new(f),
        });
        let block_ref = Box::leak(block);
        BlMut(unsafe { mem::transmute(block_ref) })
    }
}

impl<F> bl<F> {
    const DESCRIPTOR: Descriptor1 = Descriptor1 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
    };

    pub fn with(invoke: F) -> bl<F> {
        bl {
            isa: unsafe { &_NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR,
        }
    }
}

impl<R> bl<extern "C" fn(b: *const c_void) -> R> {
    pub fn call(&self) -> R {
        (self.invoke)(self as *const Self as _)
    }
}

impl<A, R> bl<extern "C" fn(b: *const c_void, a: A) -> R> {
    pub fn call(&self, a: A) -> R {
        (self.invoke)(self as *const Self as _, a)
    }
}

impl<A, B, R> bl<extern "C" fn(b: *const c_void, a: A, b: B) -> R> {
    pub fn call(&self, a: A, b: B) -> R {
        (self.invoke)(self as *const Self as _, a, b)
    }
}

impl<A, B, C, R> bl<extern "C" fn(b: *const c_void, a: A, b: B, c: C) -> R> {
    pub fn call(&self, a: A, b: B, c: C) -> R {
        (self.invoke)(self as *const Self as _, a, b, c)
    }
}

impl<A, B, C, D, R> bl<extern "C" fn(b: *const c_void, a: A, b: B, c: C, d: D) -> R> {
    pub fn call(&self, a: A, b: B, c: C, d: D) -> R {
        (self.invoke)(self as *const Self as _, a, b, c, d)
    }
}

impl<A, B, C, D, E, R> bl<extern "C" fn(b: *const c_void, a: A, b: B, c: C, d: D, e: E) -> R> {
    pub fn call(&self, a: A, b: B, c: C, d: D, e: E) -> R {
        (self.invoke)(self as *const Self as _, a, b, c, d, e)
    }
}

#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link(name = "System", kind = "dylib")
)]
#[cfg_attr(
    not(any(target_os = "macos", target_os = "ios")),
    link(name = "BlocksRuntime", kind = "dylib")
)]
extern "C" {
    static _NSConcreteGlobalBlock: Class<ns::Id>;
    static _NSConcreteStackBlock: Class<ns::Id>;
    static _NSConcreteMallocBlock: Class<ns::Id>;

    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::blocks;

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped foo");
        }
    }

    #[test]
    fn test_simple_block() {
        let foo = Foo;
        let _rc = Rc::new(10);
        let mut b = blocks::mut0(move || println!("nice {foo:?}"));

        let q = crate::dispatch::Queue::new();
        q.async_b(b.escape());
        q.async_b(b.escape());
        q.async_mut(|| println!("nice"));
        q.sync_mut(|| println!("fuck"));
        // q.async_once(move || println!("nice {rc:?}"));

        println!("finished");
    }
}

use parking_lot::Mutex;
use std::sync::Arc;

struct Shared<T> {
    ready: Option<T>,
    pending: Option<std::task::Waker>,
}

impl<T> Shared<T> {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            ready: None,
            pending: None,
        }))
    }

    pub fn ready(&mut self, result: T) {
        self.ready = Some(result);

        if let Some(waker) = self.pending.take() {
            waker.wake();
        }
    }
}

pub struct Comp<R>(Arc<Mutex<Shared<R>>>);

impl<T> std::future::Future for Comp<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut lock = self.0.lock();

        if let Some(r) = lock.ready.take() {
            std::task::Poll::Ready(r)
        } else {
            lock.pending = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

pub fn comp0() -> (Comp<()>, BlOnce<impl FnOnce()>) {
    let shared = Shared::new();
    (Comp(shared.clone()), once0(move || shared.lock().ready(())))
}

pub fn ok() -> (
    Comp<Result<(), arc::R<ns::Error>>>,
    BlOnce<impl FnOnce(Option<&'static ns::Error>)>,
) {
    let shared = Shared::new();
    (
        Comp(shared.clone()),
        once1(move |error: Option<&'static ns::Error>| {
            shared.lock().ready(match error {
                None => Ok(()),
                Some(err) => Err(err.retained()),
            });
        }),
    )
}

pub fn result<T: arc::Retain>() -> (
    Comp<Result<arc::R<T>, arc::R<ns::Error>>>,
    BlOnce<impl FnOnce(Option<&'static T>, Option<&'static ns::Error>)>,
) {
    let shared = Shared::new();
    (
        Comp(shared.clone()),
        once2(
            move |value: Option<&'static T>, error: Option<&'static ns::Error>| {
                let res = match error {
                    None => Ok(unsafe { value.unwrap_unchecked().retained() }),
                    Some(err) => Err(err.retained()),
                };

                shared.lock().ready(res);
            },
        ),
    )
}
