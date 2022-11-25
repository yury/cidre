use std::{
    ffi::{c_char, c_void},
    intrinsics::transmute,
    marker::PhantomData,
    mem::ManuallyDrop, ops::{Deref, DerefMut},
};

use parking_lot::Mutex;

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

#[derive(Debug)]
#[repr(C)]
pub struct Descriptor1 {
    reserved: usize,
    size: usize,
}


#[derive(Debug)]
#[repr(C)]
pub struct Descriptor2 {
    reserved: usize,
    size: usize,
    copy: extern "C" fn(dest: *mut c_void, src: *mut c_void),
    dispose: extern "C" fn(liteal: *mut c_void),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
        println!("drop for blockfn");
        unsafe { ManuallyDrop::drop(&mut self.func) };
    }
}


#[derive(Debug)]
#[repr(transparent)]
pub struct RetainedBlockFn<Args, R, F>(*mut BlockFn<Args, R, F>)
where Args:'static, R:'static, F: 'static;

impl<Args, R, F> RetainedBlockFn<Args, R, F>
where Args:'static, R:'static, F: 'static {
    pub fn escape(&mut self) -> &'static mut BlockFn<Args, R, F> {
        unsafe {
            let ptr = self.0 as *mut BlockFn<Args, R, F>;
            transmute(ptr)
        }
    }
}

impl<Args, R, F> Deref for RetainedBlockFn<Args, R, F> {
    type Target = BlockFn<Args, R, F>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<Args, R, F> DerefMut for RetainedBlockFn<Args, R, F>  {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<Args, R, F> Drop for RetainedBlockFn<Args, R, F> {
    fn drop(&mut self) {
        unsafe {
            println!("drop for RetainedBlockFn {:#032b}", self.flags );
            let ptr = self.0 as *mut BlockFn<Args, R, F>;
            _Block_release(transmute(ptr));
            println!("dropped  RetainedBlockFn {:#032b}", self.flags );
        }
    }
}

impl<Args, R, F> Clone for RetainedBlockFn<Args, R, F> {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = _Block_copy(self.0 as *const BlockFn<Args, R, F> as *const c_void);
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
        reserved: 0,
        size: std::mem::size_of::<Self>(),
        copy: Self::no_copy,
        dispose: Self::no_dispose,
    };

    const DESCRIPTOR_2: Descriptor2 = Descriptor2 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
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
        println!("dispose!");
        panic!("should not be called");
    }

    extern "C" fn dispose(block: *mut c_void) {
        unsafe {
            println!("dispose!!!!");
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

    fn heap_with_invoke(invoke: *const c_void, f: F) -> Self {
        let retain_count = 0; // ???
        println!(
            "heap flags \n{:#032b}\n{:#032b}\n{:#032b}\n{:#032b}\n{:#032b}",
            (Flags::HAS_COPY_DISPOSE | Flags::NEEDS_FREE | Flags(retain_count << 1)).0,
            (Flags::HAS_COPY_DISPOSE | Flags::NEEDS_FREE | Flags(2)).0,
            Flags::REFCOUNT_MASK.0,
            1 << 25,
            1 << 24,
        );
        Self {
            isa: unsafe { _NSConcreteMallocBlock },
            flags: Flags::HAS_COPY_DISPOSE | Flags::NEEDS_FREE | Flags(2), // logical rain count 1
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            func: ManuallyDrop::new(f),
            args: PhantomData,
        }
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
        let b = Box::new(BlockFn::heap_with_invoke(BlockFn::<(), R, F>::invoke as _, f));
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

fn foo() {
    extern "C" fn bla(c: &Block<i32, ()>, _a: i32) {}

    let mut l = Block::with_a(bla);

    let mut x: i32 = 0;
    let mutex: Mutex<i32> = Default::default();
    let mut bl = BlockFn::<i32, _, _>::with_a_mut(|a: i32| {
        x = a;
        println!("nice {a} {mutex:?}");
    });

    let mut bl2 = BlockFn::<i32, _, _>::with_a(|a: i32| {
        println!("nice {a}");
    });

    foo2(&mut bl);
    foo2(&mut bl2);
    foo2(&mut l);
}

fn foo2<B: B1Mut<i32, ()>>(block: &mut B) {
    block.invoke_mut(1);
}

// impl<R> Drop for Block<R> {
//     fn drop(&mut self) {
//         unsafe { _Block_release(self as *mut _ as _) }
//         std::mem::forget(self)
//     }
// }

// #[derive(Debug)]
// #[repr(C)]
// pub struct Descriptor2 {
    // copy: extern "C" fn(dest: *mut c_void, src: *mut c_void),
    // dispose: extern "C" fn(liteal: *mut c_void),
// }

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

/// https://opensource.apple.com/source/libclosure/libclosure-79/BlockImplementation.txt.auto.html
/// https://github.com/apple-oss-distributions/libclosure/blob/main/BlockImplementation.txt
#[repr(C)]
pub struct Literal<CD: 'static, F: Sized, R, D: Sized> {
    /// initialized to &_NSConcreteStackBlock or &_NSConcreteGlobalBlock
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: extern "C" fn(&mut Literal<CD, F, R, D>, args: ...) -> R,
    descriptor: &'static Descriptor<CD>,

    // optional fields
    func: F,
    pub fields: D,
}

#[repr(C)]
pub struct Descriptor<CD> {
    reserved: usize,
    size: usize,
    copy_dispose: CD,
}

#[repr(transparent)]
pub struct NoCopyDispose;

#[repr(transparent)]
pub struct NoFunc;

#[repr(C)]
pub struct CopyDispose {
    copy: Option<extern "C" fn(dest: *mut c_void, src: *mut c_void)>,
    dispose: Option<extern "C" fn(liteal: *mut c_void)>,
}

impl<R, D> Literal<NoCopyDispose, NoFunc, R, D> {
    pub fn with_fn(fields: D, f: extern "C" fn(&mut Self) -> R) -> Self {
        Self {
            isa: unsafe { _NSConcreteGlobalBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(f) },
            descriptor: &Self::DESCRIPTOR_1,
            func: NoFunc,
            fields,
        }
    }

    pub fn call_fn(&mut self) -> R {
        (self.invoke)(self)
    }

    pub fn with_fn_a<A>(fields: D, f: extern "C" fn(&mut Self, a: A) -> R) -> Self {
        Self {
            isa: unsafe { _NSConcreteGlobalBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(f) },
            descriptor: &Self::DESCRIPTOR_1,
            func: NoFunc,
            fields,
        }
    }

    pub fn call_fn_a<A>(&mut self, a: A) -> R {
        (self.invoke)(self, a)
    }

    const DESCRIPTOR_1: Descriptor<NoCopyDispose> = Descriptor {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
        copy_dispose: NoCopyDispose {},
    };
}

impl<F, R> Literal<NoCopyDispose, F, R, ()> {
    const DESCRIPTOR: Descriptor<NoCopyDispose> = Descriptor {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
        copy_dispose: NoCopyDispose {},
    };

    extern "C" fn invoke(literal: &mut Self) -> R
    where
        F: FnMut() -> R,
    {
        (literal.func)()
    }

    extern "C" fn invoke_a<A>(literal: &mut Self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (literal.func)(a)
    }

    extern "C" fn _invoke_abcde<A, B, C, D, E>(
        literal: &mut Self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
    ) -> R
    where
        F: FnMut(A, B, C, D, E) -> R,
    {
        (literal.func)(a, b, c, d, e)
    }

    pub fn call_mut(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.func)()
    }

    pub fn call(&self) -> R
    where
        F: Fn() -> R,
    {
        (self.func)()
    }

    pub fn call_a<A>(&self, a: A) -> R
    where
        F: Fn(A) -> R,
    {
        (self.func)(a)
    }

    pub fn call_mut_a<A>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.func)(a)
    }

    pub fn with(func: F) -> Self
    where
        F: Fn() -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::IS_NOESCAPE | Flags::IS_GLOBAL,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func,
            fields: (),
        }
    }

    pub fn with_mut(func: F) -> Self
    where
        F: FnMut() -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::IS_NOESCAPE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func,
            fields: (),
        }
    }

    pub fn with_a<A>(func: F) -> Self
    where
        F: Fn(A) -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke_a as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func,
            fields: (),
        }
    }

    pub fn with_mut_a<A>(func: F) -> Self
    where
        F: FnMut(A) -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke_a as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func,
            fields: (),
        }
    }
}

impl<F: 'static, R> Literal<CopyDispose, ManuallyDrop<F>, R, ()> {
    extern "C" fn dispose(literal: *mut c_void) {
        let ptr = literal as *mut Self;
        println!("dropping: {:?}", ptr);
        let b = unsafe { Box::from_raw(ptr) };
        println!("dropping: {:b}", b.flags.0);
        let mut b = ManuallyDrop::new(b);

        unsafe { ManuallyDrop::drop(&mut b.func) };
    }

    const DESCRIPTOR_F: Descriptor<CopyDispose> = Descriptor {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
        copy_dispose: Self::COPY_DISPOSE,
    };

    const COPY_DISPOSE: CopyDispose = CopyDispose {
        copy: None,
        dispose: Some(Self::dispose),
    };

    extern "C" fn invoke_box(literal: &mut Self) -> R
    where
        F: FnMut() -> R,
    {
        (literal.func)()
    }

    pub fn new(func: F) -> Box<Self>
    where
        F: FnMut() -> R,
    {
        let retain_count = 0; // ???
        println!(
            "flags :{:b}",
            (Flags::HAS_COPY_DISPOSE | Flags::NEEDS_FREE | Flags(retain_count << 1)).0
        );
        Box::new(Self {
            isa: unsafe { _NSConcreteMallocBlock },
            flags: Flags::HAS_COPY_DISPOSE | Flags::NEEDS_FREE | Flags(retain_count << 1),
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke_box as *const c_void) },
            descriptor: &Self::DESCRIPTOR_F,
            func: ManuallyDrop::new(func),
            fields: (),
        })
    }
}

impl<F: 'static, R> Literal<NoCopyDispose, F, R, ()> {
    const DESCRIPTOR_F: Descriptor<NoCopyDispose> = Descriptor {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
        copy_dispose: NoCopyDispose,
    };

    pub fn stack(func: F) -> Self
    where
        F: FnMut() -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::IS_NOESCAPE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke as *const c_void) },
            descriptor: &Self::DESCRIPTOR_F,
            func,
            fields: (),
        }
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
    static _NSConcreteGlobalBlock: &'static Class;
    static _NSConcreteStackBlock: &'static Class;
    static _NSConcreteMallocBlock: &'static Class;

    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
}

pub trait DispatchB: B0Mut<()> {}

impl DispatchB for Block<(), ()> {}
impl<F: FnMut() -> ()> DispatchB for BlockFn<(), (), F> {}
impl<F: FnMut() -> ()> DispatchB for RetainedBlockFn<(), (), F> {}

pub type DispatchBlock<F, CD> = Literal<CD, F, (), ()>;

#[cfg(test)]
mod tests {

    use std::{time::Duration, thread::sleep};

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

        autoreleasepool(||{
            let mut b = BlockFn::<(), (), _>::new(|| println!("nice"));

            let mut b2 = b.clone();
            

            let q = crate::dispatch::Queue::new();
            q.async_bb(b.escape());
            // q.async_with(|| println!("fuck"));
    
            println!("finished");
    
    
            sleep(Duration::from_secs(1));
        });

        sleep(Duration::from_secs(1));

        println!("done");
        

        // let _f = DispatchBlock::new(|| println!("nice"));

        // let mut x = 10;
        // let mut b = Literal::with_mut_a(|a: i32| {
        //     x += 5 + a;
        //     println!("nice {:?}", x);
        //     x
        // });

        // b.call_mut_a(10);
        // b.call_mut_a(10);

        // extern "C" fn invoke(_b: &mut Literal<NoCopyDispose, NoFunc, (), i32>) {
        //     // b.fields += 1;
        //     // println!("nice {:}", b.fields);
        // }

        // let mut x = Literal::with_fn(10, invoke);
        // x.call_fn();
        // x.call_fn();

        // let _b = Literal::new(|| {});
    }
}
