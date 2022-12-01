use parking_lot::Mutex;
use std::sync::Arc;

use crate::cf::{self, runtime::Retain, Retained};

use super::blocks_runtime::BlOnce;

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

fn async_comp0() -> (Comp<()>, BlOnce<impl FnOnce()>) {
    let shared = Shared::new();
    let comp = Comp(shared.clone());
    let block = crate::objc::blocks_runtime::once0(move || {
        shared.lock().ready(());
    });

    (comp, block)
}

pub fn async_ok() -> (
    Comp<Result<(), Retained<cf::Error>>>,
    BlOnce<impl FnOnce(Option<&'static cf::Error>)>,
) {
    let shared = Shared::new();
    (
        Comp(shared.clone()),
        crate::objc::blocks_runtime::once1(move |error: Option<&'static cf::Error>| {
            shared.lock().ready(match error {
                Some(err) => Err(err.retained()),
                None => Ok(()),
            });
        }),
    )
}

pub fn async_result<T: Retain>() -> (
    Comp<Result<Retained<T>, Retained<cf::Error>>>,
    BlOnce<impl FnOnce(Option<&'static T>, Option<&'static cf::Error>)>,
) {
    let shared = Shared::new();
    (
        Comp(shared.clone()),
        crate::objc::blocks_runtime::once2(
            move |value: Option<&'static T>, error: Option<&'static cf::Error>| {
                let res = match error {
                    Some(err) => Err(err.retained()),
                    None => Ok(unsafe { value.unwrap_unchecked().retained() }),
                };

                shared.lock().ready(res);
            },
        ),
    )
}
