// https://opensource.apple.com/source/libclosure/libclosure-79/BlockImplementation.txt.auto.html
// https://github.com/apple-oss-distributions/libclosure/blob/main/BlockImplementation.txt
// https://developer.apple.com/documentation/swift/calling-objective-c-apis-asynchronously
// https://github.com/apple/swift-corelibs-foundation/blob/main/Sources/BlocksRuntime/runtime.c

use std::{
    ffi::c_void, marker::PhantomData, marker::Send as MarkerSend, marker::Sync as MarkerSync, mem,
};

use crate::{arc, define_opts, ns, objc};

// block attributes

pub struct NoEsc;
pub struct Esc;
pub struct Send;
pub struct Sync;

// attributted blocks

pub type NoEscBlock<F> = Block<F, NoEsc>;
pub type EscBlock<F> = Block<F, Esc>;
pub type SendBlock<F> = Block<F, Send>;
pub type SyncBlock<F> = Block<F, Sync>;

pub type CompletionBlock = EscBlock<fn()>;
pub type WorkBlock<Attr = Sync> = Block<fn(), Attr>;

pub type ErrCompletionHandler<E = ns::Error> = EscBlock<fn(Option<&E>)>;
pub type ResultCompletionHandler<T> = EscBlock<fn(Option<&T>, Option<&ns::Error>)>;

#[repr(transparent)]
pub struct Block<Sig, Attr = NoEsc>(ns::Id, PhantomData<(Sig, Attr)>);

#[repr(transparent)]
pub struct StackBlock<'a, Closure, Sig>(Layout1Mut<'a, Closure>, PhantomData<Sig>);

#[repr(transparent)]
pub struct StaticBlock<Sig>(Layout1, PhantomData<Sig>);

impl<Sig> std::ops::Deref for Block<Sig, NoEsc> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<Sig, Attr> objc::Obj for Block<Sig, Attr> {
    #[inline]
    unsafe fn retain(id: &Self) -> arc::R<Self> {
        std::mem::transmute(_Block_copy(std::mem::transmute(id)))
    }

    #[inline]
    unsafe fn release(id: &mut Self) {
        _Block_release(std::mem::transmute(id))
    }
}

impl<'a, Closure, Sig> std::ops::Deref for StackBlock<'a, Closure, Sig> {
    type Target = Block<Sig, NoEsc>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, Closure, Sig> std::ops::DerefMut for StackBlock<'a, Closure, Sig> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

macro_rules! call {
    ($($a:ident: $t:ident),*) => {
        impl<$($t,)* R, Attr> Block<fn($($t,)*) -> R, Attr> {
            pub fn call(&mut self, $($a: $t),*) -> R {
                let layout: &Layout1 = unsafe { std::mem::transmute(&self.0) };
                let f: extern "C" fn(literal: &mut Self $(, $t)*) -> R = unsafe { std::mem::transmute(layout.invoke) };
                f(self $(, $a)*)
            }
        }
    };
}

macro_rules! invoke {
    ($name:ident: $($a:ident: $t:ident),*) => {
        extern "C" fn $name<$($t,)* R>(&mut self, $($a: $t),*) -> R
        where
            Closure: FnMut($($t,)*) -> R,
        {
            (self.closure)($($a,)*)
        }
    };
}

macro_rules! new {
    ($name:ident, $invoke:ident: $($t:ident),* $(+ $l:lifetime)* $(+ $trait:ident)*) => {
        pub fn $name<$($t,)* R, Closure>(closure: Closure) -> arc::R<Self>
        where
            Sig: Fn($($t,)*) -> R, // guard for Block Sig
            for<'c> Closure: FnMut($($t,)*) -> R $(+ $l)* $(+ $trait)*,
        {
            let res = Layout2Mut::new(Layout2Mut::<Closure>::$invoke as _, closure);
            unsafe { std::mem::transmute(res) }
        }
    };
}

macro_rules! with_fn {
    ($name:ident: $($t:ident),* ) => {
        pub fn $name<$($t,)* R>(func: extern "C" fn (*const c_void, $($t,)*) -> R) -> StaticBlock<Sig>
        {
            let res = Layout1::with(func as _);
            StaticBlock(res, PhantomData)
        }
    };
}

macro_rules! stack {
    ($name:ident, $invoke:ident: $($t:ident),*) => {
        #[inline]
        pub unsafe fn $name<$($t,)* R, Closure>(closure: &mut Closure) -> StackBlock<Closure, Sig>
        where
            Sig: Fn($($t,)*) -> R, // guard for Block Sig
            for<'c> Closure: FnMut($($t,)*) -> R
        {
            let layout = Layout1Mut::new(Layout1Mut::<Closure>::$invoke as _, closure);
            StackBlock(layout, PhantomData)
        }
    };
}

impl<Sig, Attr> Block<Sig, Attr> {
    with_fn!(with_fn0:);
    with_fn!(with_fn1: A);
    with_fn!(with_fn2: A, B);
    with_fn!(with_fn3: A, B, C);
    with_fn!(with_fn4: A, B, C, D);
    with_fn!(with_fn5: A, B, C, D, E);
    with_fn!(with_fn6: A, B, C, D, E, F);
}

impl<Sig> Block<Sig, NoEsc> {
    stack!(stack0, invoke0:);
    stack!(stack1, invoke1: A);
    stack!(stack2, invoke2: A, B);
    stack!(stack3, invoke3: A, B, C);
    stack!(stack4, invoke4: A, B, C, D);
    stack!(stack5, invoke5: A, B, C, D, E);
    stack!(stack6, invoke6: A, B, C, D, E, F);

    new!(new0, invoke0:);
    new!(new1, invoke1: A);
    new!(new2, invoke2: A, B);
    new!(new3, invoke3: A, B, C);
    new!(new4, invoke4: A, B, C, D);
    new!(new5, invoke5: A, B, C, D, E);
    new!(new6, invoke6: A, B, C, D, E, F);
}

impl<Sig> Block<Sig, Esc> {
    new!(new0, invoke0: + 'static);
    new!(new1, invoke1: A + 'static);
    new!(new2, invoke2: A, B + 'static);
    new!(new3, invoke3: A, B, C + 'static);
    new!(new4, invoke4: A, B, C, D + 'static);
    new!(new5, invoke5: A, B, C, D, E + 'static);
    new!(new6, invoke6: A, B, C, D, E, F + 'static);

    pub fn as_noesc_mut(&mut self) -> &mut Block<Sig, NoEsc> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<Sig> Block<Sig, Send> {
    new!(new0, invoke0: + 'static + MarkerSend);
    new!(new1, invoke1: A + 'static + MarkerSend);
    new!(new2, invoke2: A, B + 'static + MarkerSend);
    new!(new3, invoke3: A, B, C + 'static + MarkerSend);
    new!(new4, invoke4: A, B, C, D + 'static + MarkerSend);
    new!(new5, invoke5: A, B, C, D, E + 'static + MarkerSend);
    new!(new6, invoke6: A, B, C, D, E, F + 'static + MarkerSend);

    pub fn as_esc_mut(&mut self) -> &mut Block<Sig, Esc> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_noesc_mut(&mut self) -> &mut Block<Sig, NoEsc> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<Sig> Block<Sig, Sync> {
    new!(new0, invoke0: + 'static + MarkerSync);
    new!(new1, invoke1: A + 'static + MarkerSync);
    new!(new2, invoke2: A, B + 'static + MarkerSync);
    new!(new3, invoke3: A, B, C + 'static + MarkerSync);
    new!(new4, invoke4: A, B, C, D + 'static + MarkerSync);
    new!(new5, invoke5: A, B, C, D, E + 'static + MarkerSync);
    new!(new6, invoke6: A, B, C, D, E, F + 'static + MarkerSync);

    pub fn as_send_mut(&mut self) -> &mut Block<Sig, Send> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_esc_mut(&mut self) -> &mut Block<Sig, Esc> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_noesc_mut(&mut self) -> &mut Block<Sig, NoEsc> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<Sig> StaticBlock<Sig> {
    with_fn!(new0:);
    with_fn!(new1: A);
    with_fn!(new2: A, B);
    with_fn!(new3: A, B, C);
    with_fn!(new4: A, B, C, D);
    with_fn!(new5: A, B, C, D, E);
    with_fn!(new6: A, B, C, D, E, F);

    pub fn as_sync_mut(&mut self) -> &mut Block<Sig, Sync> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_send_mut(&mut self) -> &mut Block<Sig, Send> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_esc_mut(&mut self) -> &mut Block<Sig, Esc> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_noesc_mut(&mut self) -> &mut Block<Sig, NoEsc> {
        unsafe { std::mem::transmute(self) }
    }
}

call!();
call!(a:A);
call!(a:A, b: B);
call!(a:A, b: B, c: C);
call!(a:A, b: B, c: C, d: D);
call!(a:A, b: B, c: C, d: D, e: E);
call!(a:A, b: B, c: C, d: D, e: E, f: F);
call!(a:A, b: B, c: C, d: D, e: E, f: F, g: G);
call!(a:A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
call!(a:A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
call!(a:A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
call!(a:A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);

//         // TODO: revisit
//         unsafe impl<'a, $($t,)* R> Send for $bl_name<'a$(, $t)*, R> {}
//         unsafe impl<'a, $($t,)* R> Sync for $bl_name<'a$(, $t)*, R> {}

define_opts!(pub Flags(i32));

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
    // pub const IS_NOESCAPE: Self = Self(1 << 23);

    // runtime
    pub const NEEDS_FREE: Self = Self(1 << 24);
    // compiler
    pub const HAS_COPY_DISPOSE: Self = Self(1 << 25);
    // pub const HAS_CTOR: Self = Self(1 << 26);
    // pub const IS_GC: Self = Self(1 << 27);
    // pub const IS_GLOBAL: Self = Self(1 << 28);
    // pub const USE_STRET: Self = Self(1 << 29);
    // pub const HAS_SIGNATURE: Self = Self(1 << 30);
    // pub const HAS_EXTENDED_LAYOUT: Self = Self(1 << 31);

    const RETAINED_NEEDS_FREE: Self = Self(2 | Self::NEEDS_FREE.0);
    const RETAINED_NEEDS_DROP: Self = Self(2 | Self::NEEDS_FREE.0 | Self::HAS_COPY_DISPOSE.0);
}

#[repr(C)]
pub struct Desc1 {
    reserved: usize,
    size: usize,
}

#[repr(C)]
pub struct Desc2<T: Sized> {
    descriptor1: Desc1,
    copy: extern "C" fn(dest: *mut c_void, src: *mut c_void),
    dispose: extern "C" fn(literal: &mut T),
}

#[repr(C)]
pub struct Layout1 {
    isa: &'static objc::Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Desc1,
}

#[repr(C)]
pub struct Layout1Mut<'a, Closure> {
    isa: &'static objc::Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'a Desc1,
    closure: &'a mut Closure,
}

#[repr(C)]
struct Layout2Mut<'a, F: Sized + 'a> {
    isa: &'static objc::Class<ns::Id>,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'a Desc2<Self>,
    closure: mem::ManuallyDrop<F>,
}

impl Layout1 {
    const DESCRIPTOR: Desc1 = Desc1 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
    };

    pub fn with(invoke: *const c_void) -> Self {
        Self {
            isa: unsafe { &_NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR,
        }
    }
}

impl<'a, Closure> Layout1Mut<'a, Closure> {
    const DESCRIPTOR_1: Desc1 = Desc1 {
        reserved: 0,
        size: std::mem::size_of::<&'static objc::Class<ns::Id>>()
            + std::mem::size_of::<Flags>()
            + std::mem::size_of::<i32>()
            + std::mem::size_of::<*const c_void>()
            + std::mem::size_of::<&'static Desc1>()
            + std::mem::size_of::<&'static c_void>(), // emulating &mut F
    };

    invoke! {invoke0: }
    invoke! {invoke1: a: A}
    invoke! {invoke2: a: A, b: B}
    invoke! {invoke3: a: A, b: B, c: C}
    invoke! {invoke4: a: A, b: B, c: C, d: D}
    invoke! {invoke5: a: A, b: B, c: C, d: D, e: E}
    invoke! {invoke6: a: A, b: B, c: C, d: D, e: E, f: F}

    fn new(invoke: *const c_void, f: &'a mut Closure) -> Self {
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

extern "C" fn no_copy(_dest: *mut c_void, _src: *mut c_void) {
    panic!("copy should not be called");
}

impl<'a, Closure: 'a + Sized> Layout2Mut<'a, Closure> {
    const DESCRIPTOR_2: Desc2<Self> = Desc2 {
        descriptor1: Desc1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: no_copy,
        dispose: Self::dispose,
    };

    extern "C" fn dispose(block: &mut Self) {
        debug_assert!(mem::needs_drop::<Closure>());
        unsafe {
            mem::ManuallyDrop::drop(&mut block.closure);
        }
    }

    invoke! {invoke0: }
    invoke! {invoke1: a: A}
    invoke! {invoke2: a: A, b: B}
    invoke! {invoke3: a: A, b: B, c: C}
    invoke! {invoke4: a: A, b: B, c: C, d: D}
    invoke! {invoke5: a: A, b: B, c: C, d: D, e: E}
    invoke! {invoke6: a: A, b: B, c: C, d: D, e: E, f: F}

    fn new(invoke: *const c_void, closure: Closure) -> &'a mut Self {
        let flags = if mem::needs_drop::<Closure>() {
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
            closure: mem::ManuallyDrop::new(closure),
        });
        Box::leak(block)
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    // static _NSConcreteGlobalBlock: objc::Class<ns::Id>;
    static _NSConcreteStackBlock: objc::Class<ns::Id>;
    static _NSConcreteMallocBlock: objc::Class<ns::Id>;

    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
}

#[cfg(test)]
mod tests {

    use crate::{blocks, dispatch};

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped foo");
        }
    }

    #[test]
    fn simple_block() {
        let foo = Foo;
        // let rc = Rc::new(10);
        let mut b = dispatch::Block::<blocks::Send>::new0(move || println!("nice {foo:?}"));

        let q = dispatch::Queue::new();
        q.async_b(&mut b);
        q.async_b(&mut b);
        q.async_mut(|| println!("nice"));
        q.sync_mut(|| println!("fuck"));
        // q.async_once(move || println!("nice {rc:?}"));

        println!("finished");
    }
}

#[cfg(feature = "async")]
use parking_lot::Mutex;

#[cfg(feature = "async")]
use std::sync::Arc;

#[cfg(feature = "async")]
struct Shared<T> {
    ready: Option<T>,
    pending: Option<std::task::Waker>,
}

#[cfg(feature = "async")]
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

#[cfg(feature = "async")]
pub struct Completion<R>(Arc<Mutex<Shared<R>>>);

#[cfg(feature = "async")]
impl<T> std::future::Future for Completion<T> {
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

#[cfg(feature = "async")]
pub fn comp0() -> (Completion<()>, arc::R<CompletionBlock>) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        CompletionBlock::new0(move || shared.lock().ready(())),
    )
}

#[cfg(feature = "async")]
pub fn comp1<R: std::marker::Send>() -> (Completion<R>, arc::R<Block<fn(R), Send>>) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        SendBlock::new1(move |v: R| shared.lock().ready(v)),
    )
}

#[cfg(feature = "async")]
pub fn ok<'a>() -> (
    Completion<Result<(), arc::R<ns::Error>>>,
    arc::R<ErrCompletionHandler>,
) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        ErrCompletionHandler::new1(move |error: Option<&ns::Error>| {
            shared.lock().ready(match error {
                None => Ok(()),
                Some(err) => Err(err.retained()),
            });
        }),
    )
}

#[cfg(feature = "async")]
pub fn result<T: arc::Retain + std::marker::Send>() -> (
    Completion<Result<arc::R<T>, arc::R<ns::Error>>>,
    arc::R<ResultCompletionHandler<T>>,
) {
    let shared = Shared::new();
    (
        Completion(shared.clone()),
        ResultCompletionHandler::<T>::new2(move |value: Option<&T>, error: Option<&ns::Error>| {
            let res = match error {
                None => Ok(unsafe { value.unwrap_unchecked().retained() }),
                Some(err) => Err(err.retained()),
            };

            shared.lock().ready(res);
        }),
    )
}
