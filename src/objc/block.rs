use std::{ffi::c_void, sync::Arc};

#[repr(C)]
pub struct CompletionBlock<T>
where
    T: ?Sized,
{
    pub call_fn: *const c_void,
    pub f: T,
}

pub trait CompletionHandlerA<A>: Fn(A) + Sized {
    fn into_raw(self) -> *const c_void {
        let arc = Arc::new(CompletionBlock {
            call_fn: Self::call_a as _,
            f: self,
        });

        Arc::into_raw(arc) as _
    }

    unsafe fn call_a(raw: *const CompletionBlock<Self>, a: A) {
        let arc = Arc::from_raw(raw);
        (arc.f)(a);
    }
}

impl<T, A> CompletionHandlerA<A> for T where T: Fn(A) {}

pub trait CompletionHandlerAB<A, B>: Fn(A, B) + Sized {
    fn into_raw(self) -> *const c_void {
        let arc = Arc::new(CompletionBlock {
            call_fn: Self::call_ab as _,
            f: self,
        });

        Arc::into_raw(arc) as _
    }

    unsafe fn call_ab(raw: *const CompletionBlock<Self>, a: A, b: B) {
        let arc = Arc::from_raw(raw);
        (arc.f)(a, b);
    }
}

impl<T, A, B> CompletionHandlerAB<A, B> for T where T: Fn(A, B) {}

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
