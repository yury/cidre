use std::{ffi::c_void, sync::Arc};

use parking_lot::Mutex;

use crate::cf::{self, runtime::Retain, Retained};

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

// https://developer.apple.com/documentation/swift/calling_objective-c_apis_asynchronously

pub struct Completion<T> {
    block: Arc<Block<T>>,
}

impl<T> Completion<T> {
    pub fn ok_or_error() -> (Self, *const c_void) {
        let block = Arc::new(Block {
            _fn_ptr: Block::ok_or_error_fn as _,
            state: State::mutex(),
        });

        let ptr = Arc::into_raw(block.clone()) as _;
        (Self { block }, ptr)
    }
}

type RetainedResult<R> = Result<Retained<R>, Retained<cf::Error>>;

impl<R> Completion<RetainedResult<R>>
where
    R: Retain,
{
    pub fn result_or_error() -> (Self, *const c_void) {
        let block = Arc::new(Block {
            _fn_ptr: Block::<RetainedResult<R>>::result_or_error_fn as _,
            state: State::mutex(),
        });

        let ptr = Arc::into_raw(block.clone()) as _;
        (Self { block }, ptr)
    }
}

#[repr(C)]
struct Block<T> {
    _fn_ptr: *const c_void,
    state: Mutex<State<T>>,
}

impl<T> Block<T> {
    #[inline]
    fn ready(raw: *mut Self, ready: T) {
        let block = unsafe { Arc::from_raw(raw) };
        let mut state = block.state.lock();
        state.ready = Some(ready);

        if let Some(w) = state.pending.take() {
            w.wake()
        }
    }
}

impl Block<Result<(), Retained<cf::Error>>> {
    unsafe extern "C" fn ok_or_error_fn(raw: *mut Self, err: Option<&cf::Error>) {
        let result = if let Some(err) = err {
            Err(err.retained())
        } else {
            Ok(())
        };

        Block::ready(raw, result);
    }
}

impl<R> Block<RetainedResult<R>>
where
    R: Retain,
{
    unsafe extern "C" fn result_or_error_fn(
        raw: *mut Self,
        res: Option<&R>,
        err: Option<&cf::Error>,
    ) {
        let result = if let Some(err) = err {
            Err(err.retained())
        } else if let Some(res) = res {
            Ok(res.retained())
        } else {
            panic!()
        };

        Block::ready(raw, result);
    }
}

struct State<T> {
    ready: Option<T>,
    pending: Option<std::task::Waker>,
}

impl<T> State<T> {
    fn mutex() -> Mutex<Self> {
        Mutex::new(Self {
            ready: None,
            pending: None,
        })
    }
}

impl<T> std::future::Future for Completion<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut result = self.block.state.lock();

        if let Some(r) = result.ready.take() {
            std::task::Poll::Ready(r)
        } else {
            result.pending = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}
