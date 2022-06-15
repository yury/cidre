use std::{ffi::c_void, intrinsics::transmute};

use crate::define_options;

use super::Class;

// #[repr(transparent)]
// pub struct Flags(pub i32);
define_options!(Flags(i32));

impl Flags {
    pub const NONE: Self = Self(0);

    // runtime
    pub const DEALLOCATING: Self = Self(1);
    // runtime
    pub const REFCOUNT_MASK: Self = Self(0xfffei32);
    // compiler
    pub const INLINE_LAYOUT_STRING: Self = Self(1 << 21);

    // compiler
    pub const IS_NOESCAPE: Self = Self(1 << 23);
    // compiler
    pub const HAS_COPY_DISPOSE: Self = Self(1 << 25);
    pub const HAS_CTOR: Self = Self(1 << 26);
    pub const IS_GLOBAL: Self = Self(1 << 28);
    pub const HAS_STRET: Self = Self(1 << 29);
    pub const HAS_SIGNATURE: Self = Self(1 << 30);
    pub const HAS_EXTENDED_LAYOUT: Self = Self(1 << 31);
}

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
    pub reserved: usize,
    pub size: usize,
    pub copy_dispose: CD,
}

#[repr(transparent)]
pub struct NoCopyDispose;

#[repr(transparent)]
pub struct NoFunc;

#[repr(C)]
pub struct CopyDispose {
    pub copy: Option<extern "C" fn(dest: *mut c_void, src: *mut c_void)>,
    pub dispose: Option<extern "C" fn(liteal: *mut c_void)>,
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
            fields
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
            fields
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

    pub fn with(f: F) -> Self
    where
        F: Fn() -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::IS_NOESCAPE | Flags::IS_GLOBAL,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func: f,
            fields: ()
        }
    }

    pub fn with_mut(f: F) -> Self
    where
        F: FnMut() -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::IS_NOESCAPE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func: f,
            fields: ()
        }
    }

    pub fn with_a<A>(f: F) -> Self
    where
        F: Fn(A) -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke_a as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func: f,
            fields: ()
        }
    }

    pub fn with_mut_a<A>(f: F) -> Self
    where
        F: FnMut(A) -> R,
    {
        Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke_a as *const c_void) },
            descriptor: &Self::DESCRIPTOR,
            func: f,
            fields: ()
        }
    }
}

impl<F, R> Literal<CopyDispose, F, R, ()> {
    extern "C" fn dispose(literal: *mut c_void) {
        unsafe { _ = Box::from_raw(literal) };
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

    pub fn new(f: F) -> Box<Self>
    where
        F: FnMut() -> R,
    {
        Box::new(Self {
            isa: unsafe { _NSConcreteStackBlock },
            flags: Flags::HAS_COPY_DISPOSE,
            reserved: 0,
            invoke: unsafe { transmute(Self::invoke_box as *const c_void) },
            descriptor: &Self::DESCRIPTOR_F,
            func: f,
            fields: ()
        })
    }
}

extern "C" {
    static _NSConcreteGlobalBlock: &'static Class;
    static _NSConcreteStackBlock: &'static Class;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_block() {
        let mut x = 10;
        let mut b = Literal::with_mut_a(|a: i32| {
            x += 5 + a;
            println!("nice {:?}", x);
            x
        });

        b.call_mut_a(10);
        b.call_mut_a(10);

        extern "C" fn invoke(b: &mut Literal<NoCopyDispose, NoFunc, (), i32>) {
            b.fields += 1;
            println!("nice {:}", b.fields);
        }

        let mut x = Literal::with_fn(10, invoke);
        x.call_fn();
        x.call_fn();

        let b = Literal::new(|| {

        });
    }
}
