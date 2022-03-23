use std::ffi::c_void;

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

    unsafe fn fn_a(raw: *mut CompletionBlock<Self>, a: A) {
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

    unsafe fn fn_ab(raw: *mut CompletionBlock<Self>, a: A, b: B) {
        let block = Box::from_raw(raw);
        (block.f)(a, b);
    }
}

impl<F, A, B> CompletionHandlerAB<A, B> for F where F: FnOnce(A, B) + Send {}
