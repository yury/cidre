use std::{
    ffi::{c_char, c_void},
    marker::PhantomData,
    mem::{transmute, ManuallyDrop},
    ops::{Deref, DerefMut},
};

use crate::define_options;

use super::Class;

define_options!(Flags(i32));

impl Flags {
    pub const NONE: Self = Self(0);

    // runtime
    pub const DEALLOCATING: Self = Self(1);

    // runtime
    pub const REFCOUNT_MASK: Self = Self(0xfffei32);

    // compiler
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
}

#[repr(C)]
pub struct Descriptor1 {
    reserved: usize,
    size: usize,
}

#[repr(C)]
pub struct Descriptor2 {
    descriptor1: Descriptor1,
    copy: extern "C" fn(dest: *mut c_void, src: *mut c_void),
    dispose: extern "C" fn(liteal: *mut c_void),
}

#[repr(C)]
pub struct Block<Args, R> {
    /// initialized to &_NSConcreteStackBlock or &_NSConcreteGlobalBlock
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor1,
    args: PhantomData<(Args, R)>,
}

#[repr(C)]
pub struct BlockFn<Args, R, F: Sized> {
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor2,
    func: ManuallyDrop<F>,
    args: PhantomData<(Args, R)>,
}

impl<Args, R, F: Sized> Drop for BlockFn<Args, R, F> {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.func) };
    }
}

#[repr(transparent)]
pub struct RetainedBlockFn<Args, R, F>(&'static mut BlockFn<Args, R, F>)
where
    Args: 'static,
    R: 'static,
    F: 'static;

impl<Args, R, F> RetainedBlockFn<Args, R, F>
where
    Args: 'static,
    R: 'static,
    F: 'static,
{
    #[inline]
    pub fn escape(&mut self) -> &'static mut BlockFn<Args, R, F> {
        unsafe { transmute(self.0 as *mut _) }
    }
}

impl<Args, R, F> Deref for RetainedBlockFn<Args, R, F> {
    type Target = BlockFn<Args, R, F>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<Args, R, F> DerefMut for RetainedBlockFn<Args, R, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<Args, R, F> Drop for RetainedBlockFn<Args, R, F> {
    fn drop(&mut self) {
        unsafe {
            _Block_release(transmute(self.0 as *mut _));
        }
    }
}

impl<Args, R, F> Clone for RetainedBlockFn<Args, R, F> {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = _Block_copy(transmute(self.0 as *const _));
            Self(transmute(ptr))
        }
    }
}

impl<Args, R, F: Sized> BlockFn<Args, R, F> {
    const DESCRIPTOR_1: Descriptor1 = Descriptor1 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
    };

    const DESCRIPTOR_2_NO_COPY_DISPOSE: Descriptor2 = Descriptor2 {
        descriptor1: Self::DESCRIPTOR_1,
        copy: Self::no_copy,
        dispose: Self::no_dispose,
    };

    const DESCRIPTOR_2: Descriptor2 = Descriptor2 {
        descriptor1: Self::DESCRIPTOR_1,
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn no_copy(_dest: *mut c_void, _src: *mut c_void) {
        panic!("should not be called");
    }

    extern "C" fn no_dispose(_dest: *mut c_void) {
        panic!("should not be called");
    }

    extern "C" fn copy(_dest: *mut c_void, _src: *mut c_void) {
        panic!("should not be called");
    }

    extern "C" fn dispose(block: *mut c_void) {
        unsafe {
            let block: &mut Self = transmute(block);
            ManuallyDrop::drop(&mut block.func);
        }
    }

    extern "C" fn invoke(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.func)()
    }

    extern "C" fn invoke_a<A>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.func)(a)
    }

    extern "C" fn invoke_ab<A, B>(&mut self, a: A, b: B) -> R
    where
        F: FnMut(A, B) -> R,
    {
        (self.func)(a, b)
    }

    extern "C" fn invoke_abc<A, B, C>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnMut(A, B, C) -> R,
    {
        (self.func)(a, b, c)
    }

    fn with_invoke(invoke: *const c_void, f: F) -> Self {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::IS_NOESCAPE,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2_NO_COPY_DISPOSE,
            func: ManuallyDrop::new(f),
            args: PhantomData,
        }
    }

    fn new_with_invoke(invoke: *const c_void, f: F) -> Box<Self> {
        Box::new(Self {
            isa: unsafe { _NSConcreteMallocBlock },
            flags: Flags::HAS_COPY_DISPOSE | Flags::NEEDS_FREE | Flags(2), // logical rain count 1
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            func: ManuallyDrop::new(f),
            args: PhantomData,
        })
    }

    pub fn with(f: F) -> BlockFn<(), R, F>
    where
        F: FnMut() -> R,
    {
        BlockFn::with_invoke(Self::invoke as _, f)
    }

    pub fn new(f: F) -> RetainedBlockFn<(), R, F>
    where
        F: FnMut() -> R,
    {
        let b = BlockFn::new_with_invoke(BlockFn::<(), R, F>::invoke as _, f);
        RetainedBlockFn(Box::leak(b))
    }

    pub fn with_a<A>(f: F) -> BlockFn<A, R, F>
    where
        F: Fn(A) -> R,
    {
        BlockFn::with_invoke(Self::invoke_a as _, f)
    }

    pub fn with_a_mut<A>(f: F) -> BlockFn<A, R, F>
    where
        F: FnMut(A) -> R,
    {
        BlockFn::with_invoke(Self::invoke_a as _, f)
    }

    pub fn with_ab<A, B>(f: F) -> BlockFn<(A, B), R, F>
    where
        F: Fn(A, B) -> R,
    {
        BlockFn::with_invoke(Self::invoke_ab as _, f)
    }

    pub fn with_ab_mut<A, B>(f: F) -> BlockFn<(A, B), R, F>
    where
        F: FnMut(A, B) -> R,
    {
        BlockFn::with_invoke(Self::invoke_ab as _, f)
    }

    pub fn with_abc<A, B, C>(f: F) -> BlockFn<(A, B, C), R, F>
    where
        F: Fn(A, B, C) -> R,
    {
        BlockFn::with_invoke(Self::invoke_abc as _, f)
    }

    pub fn with_abc_mut<A, B, C>(f: F) -> BlockFn<(A, B, C), R, F>
    where
        F: FnMut(A, B, C) -> R,
    {
        BlockFn::with_invoke(Self::invoke_abc as _, f)
    }
}

impl<Args, R> Block<Args, R> {
    const DESCRIPTOR: Descriptor1 = Descriptor1 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
    };

    fn with(f: extern "C" fn(&Self) -> R) -> Self {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(f) },
            descriptor: &Self::DESCRIPTOR,
            args: PhantomData,
        }
    }

    pub fn with_a<A>(f: extern "C" fn(&Self, A) -> R) -> Block<A, R> {
        Block::with(unsafe { transmute(f) })
    }

    pub fn with_ab<A, B>(f: extern "C" fn(&Self, A, B) -> R) -> Block<(A, B), R> {
        Block::with(unsafe { transmute(f) })
    }

    pub fn with_abc<A, B, C>(f: extern "C" fn(&Self, A, B, C) -> R) -> Block<(A, B, C), R> {
        Block::with(unsafe { transmute(f) })
    }

    pub fn with_abcd<A, B, C, D>(
        f: extern "C" fn(&Self, A, B, C, D) -> R,
    ) -> Block<(A, B, C, D), R> {
        Block::with(unsafe { transmute(f) })
    }
}

pub trait B0Mut<R> {
    fn invoke_mut(&mut self) -> R;
}

pub trait B0<R>: B0Mut<R> {
    fn invoke(&self) -> R;
}

pub trait B1Mut<A, R> {
    fn invoke_mut(&mut self, a: A) -> R;
}

pub trait B1<A, R>: B1Mut<A, R> {
    fn invoke(&self, a: A) -> R;
}

pub trait B2Mut<A, B, R> {
    fn invoke_mut(&mut self, a: A, b: B) -> R;
}

pub trait B2<A, B, R>: B2Mut<A, B, R> {
    fn invoke(&self, a: A, b: B) -> R;
}

pub trait B3Mut<A, B, C, R> {
    fn invoke_mut(&mut self, a: A, b: B, c: C) -> R;
}

pub trait B3<A, B, C, R>: B3Mut<A, B, C, R> {
    fn invoke(&self, a: A, b: B, c: C) -> R;
}

pub trait B4Mut<A, B, C, D, R> {
    fn invoke_mut(&mut self, a: A, b: B, c: C, d: D) -> R;
}

pub trait B4<A, B, C, D, R>: B4Mut<A, B, C, D, R> {
    fn invoke(&self, a: A, b: B, c: C, d: D) -> R;
}

impl<R> B0Mut<R> for Block<(), R> {
    fn invoke_mut(&mut self) -> R {
        let f: extern "C" fn(&Self) -> R = unsafe { transmute(self.invoke) };
        f(self)
    }
}

impl<R> B0<R> for Block<(), R> {
    fn invoke(&self) -> R {
        let f: extern "C" fn(&Self) -> R = unsafe { transmute(self.invoke) };
        f(self)
    }
}

impl<A, R> B1Mut<A, R> for Block<A, R> {
    fn invoke_mut(&mut self, a: A) -> R {
        let f: extern "C" fn(&Self, a: A) -> R = unsafe { transmute(self.invoke) };
        f(self, a)
    }
}

impl<A, R> B1<A, R> for Block<A, R> {
    fn invoke(&self, a: A) -> R {
        let f: extern "C" fn(&Self, a: A) -> R = unsafe { transmute(self.invoke) };
        f(self, a)
    }
}

impl<A, B, R> B2Mut<A, B, R> for Block<(A, B), R> {
    fn invoke_mut(&mut self, a: A, b: B) -> R {
        let f: extern "C" fn(&Self, a: A, b: B) -> R = unsafe { transmute(self.invoke) };
        f(self, a, b)
    }
}

impl<A, B, R> B2<A, B, R> for Block<(A, B), R> {
    fn invoke(&self, a: A, b: B) -> R {
        let f: extern "C" fn(&Self, a: A, b: B) -> R = unsafe { transmute(self.invoke) };
        f(self, a, b)
    }
}

impl<A, B, C, R> B3Mut<A, B, C, R> for Block<(A, B, C), R> {
    fn invoke_mut(&mut self, a: A, b: B, c: C) -> R {
        let f: extern "C" fn(&Self, a: A, b: B, c: C) -> R = unsafe { transmute(self.invoke) };
        f(self, a, b, c)
    }
}

impl<A, B, C, R> B3<A, B, C, R> for Block<(A, B, C), R> {
    fn invoke(&self, a: A, b: B, c: C) -> R {
        let f: extern "C" fn(&Self, a: A, b: B, c: C) -> R = unsafe { transmute(self.invoke) };
        f(self, a, b, c)
    }
}

impl<A, B, C, D, R> B4Mut<A, B, C, D, R> for Block<(A, B, C, D), R> {
    fn invoke_mut(&mut self, a: A, b: B, c: C, d: D) -> R {
        let f: extern "C" fn(&Self, a: A, b: B, c: C, d: D) -> R =
            unsafe { transmute(self.invoke) };
        f(self, a, b, c, d)
    }
}

impl<A, B, C, D, R> B4<A, B, C, D, R> for Block<(A, B, C, D), R> {
    fn invoke(&self, a: A, b: B, c: C, d: D) -> R {
        let f: extern "C" fn(&Self, a: A, b: B, c: C, d: D) -> R =
            unsafe { transmute(self.invoke) };
        f(self, a, b, c, d)
    }
}

impl<R, F> B0Mut<R> for RetainedBlockFn<(), R, F>
where
    F: FnMut() -> R,
{
    fn invoke_mut(&mut self) -> R {
        (self.func)()
    }
}

impl<R, F> B0Mut<R> for BlockFn<(), R, F>
where
    F: FnMut() -> R,
{
    fn invoke_mut(&mut self) -> R {
        (self.func)()
    }
}

impl<R, F> B0<R> for BlockFn<(), R, F>
where
    F: Fn() -> R,
{
    fn invoke(&self) -> R {
        (self.func)()
    }
}

impl<A, R, F> B1Mut<A, R> for BlockFn<A, R, F>
where
    F: FnMut(A) -> R,
{
    fn invoke_mut(&mut self, a: A) -> R {
        (self.func)(a)
    }
}

impl<A, R, F> B1<A, R> for BlockFn<A, R, F>
where
    F: Fn(A) -> R,
{
    fn invoke(&self, a: A) -> R {
        (self.func)(a)
    }
}

impl<A, B, R, F> B2Mut<A, B, R> for BlockFn<(A, B), R, F>
where
    F: FnMut(A, B) -> R,
{
    fn invoke_mut(&mut self, a: A, b: B) -> R {
        (self.func)(a, b)
    }
}

impl<A, B, R, F> B2<A, B, R> for BlockFn<(A, B), R, F>
where
    F: Fn(A, B) -> R,
{
    fn invoke(&self, a: A, b: B) -> R {
        (self.func)(a, b)
    }
}

impl<A, B, C, R, F> B3Mut<A, B, C, R> for BlockFn<(A, B, C), R, F>
where
    F: FnMut(A, B, C) -> R,
{
    fn invoke_mut(&mut self, a: A, b: B, c: C) -> R {
        (self.func)(a, b, c)
    }
}

impl<A, B, C, R, F> B3<A, B, C, R> for BlockFn<(A, B, C), R, F>
where
    F: Fn(A, B, C) -> R,
{
    fn invoke(&self, a: A, b: B, c: C) -> R {
        (self.func)(a, b, c)
    }
}

impl<A, B, C, D, R, F> B4Mut<A, B, C, D, R> for BlockFn<(A, B, C, D), R, F>
where
    F: FnMut(A, B, C, D) -> R,
{
    fn invoke_mut(&mut self, a: A, b: B, c: C, d: D) -> R {
        (self.func)(a, b, c, d)
    }
}

impl<A, B, C, D, R, F> B4<A, B, C, D, R> for BlockFn<(A, B, C, D), R, F>
where
    F: Fn(A, B, C, D) -> R,
{
    fn invoke(&self, a: A, b: B, c: C, d: D) -> R {
        (self.func)(a, b, c, d)
    }
}

unsafe impl<A, R> Send for Block<A, R> {}
unsafe impl<A, R> Sync for Block<A, R> {}
unsafe impl<A, R, F> Send for BlockFn<A, R, F> where F: Send {}
unsafe impl<A, R, F> Sync for BlockFn<A, R, F> where F: Sync {}

#[repr(C)]
pub struct Layout2<R> {
    /// initialized to &_NSConcreteStackBlock or &_NSConcreteGlobalBlock
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: extern "C" fn(&Self, args: ...) -> R,
    descriptor1: &'static Descriptor1,
    descriptor2: &'static Descriptor2,
}

#[repr(C)]
pub struct Descriptor3 {
    signature: *const c_char,
    layout: *const c_char,
}
// TODO:
// 1. Pure fn block is global _NSConcreteGlobalBlock
// 2. Pure fn block with fields - _NSConcreteMallocBlock
// 3. NoEscaping block - _NSConcreteStackBlock

// https://opensource.apple.com/source/libclosure/libclosure-79/BlockImplementation.txt.auto.html
// https://github.com/apple-oss-distributions/libclosure/blob/main/BlockImplementation.txt

#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link(name = "System", kind = "dylib")
)]
#[cfg_attr(
    not(any(target_os = "macos", target_os = "ios")),
    link(name = "BlocksRuntime", kind = "dylib")
)]
extern "C" {
    static _NSConcreteGlobalBlock: &'static Class;
    static _NSConcreteStackBlock: &'static Class;
    static _NSConcreteMallocBlock: &'static Class;

    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
}

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

    use crate::objc::autoreleasepool;

    use super::*;

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped");
        }
    }

    #[test]
    fn test_simple_block() {
        autoreleasepool(|| {
            let mut b = BlockFn::<(), (), _>::new(|| println!("nice"));

            let q = crate::dispatch::Queue::new();
            q.async_b(b.escape());
            q.sync_with(|| println!("fuck"));

            println!("finished");
        });

        println!("done");
    }
}
