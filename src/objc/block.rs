use std::ffi::c_void;

use super::Class;

#[repr(C)]
pub struct Literal<'a, CD, F> {
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: extern "C" fn(&Literal<'a, CD, F>),
    descriptor: &'a Descriptor<CD>,
    func: F,
}

#[repr(C)]
pub struct Descriptor<CD> {
    pub reserved: usize,
    pub size: usize,
    pub copy_dispose: CD,
}

#[repr(transparent)]
pub struct NoCopyDispose;

pub struct CopyDispose<L> {
    pub copy: extern "C" fn(src: *const L, dest: *const L),
    pub dispose: extern "C" fn(liteal: *mut L),
}

impl<'a, F> Literal<'a, NoCopyDispose, F>
where
    F: Fn(),
{
    extern "C" fn invoke(literal: &Self) {
        (literal.func)()
    }

    const DESCRIPTOR: Descriptor<NoCopyDispose> = Descriptor {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
        copy_dispose: NoCopyDispose {},
    };

    pub fn new(f: F) -> Self {
        let literal = Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: Self::invoke,
            descriptor: &Self::DESCRIPTOR,
            func: f,
        };

        literal
    }
}

#[repr(transparent)]
pub struct Flags(pub i32);

impl Flags {
    pub const NONE: Self = Self(0);
    pub const NOESCAPE: Self = Self(1 << 23);
    pub const HAS_COPY_DISPOSE: Self = Self(1 << 25);
    pub const HAS_CTOR: Self = Self(1 << 26);
    pub const IS_GLOBAL: Self = Self(1 << 28);
    pub const HAS_STRET: Self = Self(1 << 29);
    pub const HAS_SIGNATURE: Self = Self(1 << 30);
}

#[repr(C)]
pub struct CompletionBlock<F>
where
    F: ?Sized,
{
    pub fn_ptr: *const c_void,
    pub f: F,
}

pub trait CompletionHandlerA<A>: FnOnce(A) + Sized + Send {
    fn into_raw(self) -> *const c_void {
        let block = Box::new(CompletionBlock {
            fn_ptr: Self::fn_a as _,
            f: self,
        });

        Box::into_raw(block) as _
    }

    unsafe extern "C" fn fn_a(raw: *mut CompletionBlock<Self>, a: A) {
        let b = Box::from_raw(raw);
        (b.f)(a);
    }
}

impl<F, A> CompletionHandlerA<A> for F where F: FnOnce(A) + Send {}

pub trait CompletionHandlerAB<A, B>: FnOnce(A, B) + Sized + Send {
    fn into_raw(self) -> *const c_void {
        let block = Box::new(CompletionBlock {
            fn_ptr: Self::fn_ab as _,
            f: self,
        });

        Box::into_raw(block) as _
    }

    unsafe extern "C" fn fn_ab(raw: *mut CompletionBlock<Self>, a: A, b: B) {
        let block = Box::from_raw(raw);
        (block.f)(a, b);
    }
}

impl<F, A, B> CompletionHandlerAB<A, B> for F where F: FnOnce(A, B) + Send {}

extern "C" {
    static _NSConcreteGlobalBlock: &'static Class;
    static _NSConcreteStackBlock: &'static Class;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_block() {
        let b = Literal::new(|| {
            println!("nice");
        });

        (b.invoke)(&b);
    }
}
