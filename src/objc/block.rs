use std::ffi::c_void;

#[repr(C)]
pub struct CompletionBlock<T>
where
    T: ?Sized,
{
    pub fn_ptr: *const c_void,
    pub f: T,
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

impl<T, A> CompletionHandlerA<A> for T where T: FnOnce(A) + Send {}

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

impl<T, A, B> CompletionHandlerAB<A, B> for T where T: FnOnce(A, B) + Send {}

#[cfg(test)]
mod tests {
    use std::{
        mem,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    use crate::objc::block;
    #[test]
    fn foo() {

        // let val = Arc::new(AtomicUsize::new(5));

        // let n = 10;

        // let cb = || {

        // };
        // let b = block::Block::new(cb);

        // let v = val.load(Ordering::SeqCst);
        // println!("x {}", v);

        // let ptr = {
        //   Arc::into_raw(b.clone())
        // };

        // (b.call_fn)(ptr);

        // let v = val.load(Ordering::SeqCst);
        // println!("x {}", v);

        // let pt = 1u64;
        // let si = AtomicUsize::new(0);
        // println!("size {}", mem::size_of_val(&si));
        // println!("size {}", mem::size_of_val(&b));
        // println!("size {}", mem::size_of_val(&pt));

        // assert_eq!(mem::size_of_val(&pt) * 3, mem::size_of_val(&b));

        // (b.drop_fn)(ptr);
    }
}
