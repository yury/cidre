use std::marker::PhantomData;

use crate::dispatch;

#[repr(transparent)]
pub struct WorkItem<F, T: dispatch::Block<F> + 'static>(&'static mut T, PhantomData<F>);

impl<F, T> WorkItem<F, T>
where
    T: dispatch::Block<F> + 'static,
{
    pub fn wait() {
        todo!();
    }

    pub fn wait_timeout(timeout: dispatch::Time) {
        todo!();
    }

    pub fn with_wall_timeout(timeout: dispatch::WallTime) {
        todo!()
    }

    pub fn cancel() {
        todo!();
    }

    pub fn is_canceled() -> bool {
        todo!();
    }
}

impl<F, T> Drop for WorkItem<F, T>
where
    T: dispatch::Block<F>,
{
    fn drop(&mut self) {
        todo!()
    }
}
