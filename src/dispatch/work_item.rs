use std::{ffi::c_void, marker::PhantomData, mem::transmute};

use crate::dispatch;

#[repr(transparent)]
pub struct WorkItem<F, T: dispatch::Block<F> + 'static>(&'static mut T, PhantomData<F>);

impl<F, B> WorkItem<F, B>
where
    B: dispatch::Block<F> + 'static,
{
    #[inline]
    pub fn with_flags(flags: dispatch::block::Flags, block: &'static mut B) -> Self {
        unsafe { transmute(dispatch_block_create(flags, block.ptr())) }
    }

    #[inline]
    pub fn with_qos_class(
        flags: dispatch::block::Flags,
        qos_class: dispatch::QOSClass,
        block: &'static mut B,
    ) -> Self {
        unsafe {
            transmute(dispatch_block_create_with_qos_class(
                flags,
                qos_class,
                0,
                block.ptr(),
            ))
        }
    }

    #[inline]
    pub fn with_qos_class_priority(
        flags: dispatch::block::Flags,
        qos_class: dispatch::QOSClass,
        relative_priority: i32,
        block: &'static mut B,
    ) -> Self {
        unsafe {
            transmute(dispatch_block_create_with_qos_class(
                flags,
                qos_class,
                relative_priority,
                block.ptr(),
            ))
        }
    }

    #[inline]
    pub fn wait() {
        todo!();
    }

    #[inline]
    pub fn wait_timeout(timeout: dispatch::Time) {
        todo!();
    }

    #[inline]
    pub fn wait_wall_timeout(timeout: dispatch::WallTime) {
        todo!()
    }

    #[inline]
    #[doc(alias = "dispatch_block_cancel")]
    pub fn cancel(&mut self) {
        unsafe { dispatch_block_cancel(self.0 as *mut B as _) }
    }

    /// Tests whether the given dispatch block object has been canceled.
    #[inline]
    #[doc(alias = "dispatch_block_testcancel")]
    #[must_use]
    pub fn is_canceled(&self) -> bool {
        unsafe { dispatch_block_testcancel(self.0 as *const B as _) != 0 }
    }
}

impl<B, F> dispatch::Block<F> for WorkItem<F, B>
where
    B: dispatch::Block<F>,
{
    unsafe fn ptr(&mut self) -> *mut c_void {
        self.0 as *mut B as _
    }
}

impl<F, B> Drop for WorkItem<F, B>
where
    B: dispatch::Block<F>,
{
    fn drop(&mut self) {
        unsafe { _Block_release(self.0 as *mut B as _) }
    }
}

impl<F, B> Clone for WorkItem<F, B>
where
    B: dispatch::Block<F>,
    F: FnMut() -> (),
{
    fn clone(&self) -> Self {
        unsafe { transmute(_Block_copy(self.0 as *const B as _)) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_block_create(flags: dispatch::block::Flags, block: *mut c_void) -> *mut c_void;
    fn dispatch_block_create_with_qos_class(
        flags: dispatch::BlockFlags,
        qos_class: dispatch::QOSClass,
        relative_priority: i32,
        block: *mut c_void,
    ) -> *mut c_void;
    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
    fn dispatch_block_cancel(block: *mut c_void);
    fn dispatch_block_testcancel(block: *const c_void) -> isize;
}

#[cfg(test)]
mod tests {
    use crate::{blocks, dispatch};

    #[test]
    fn basics() {
        let b = blocks::once0(|| println!("nice"));
        let mut item = dispatch::WorkItem::with_flags(Default::default(), b.escape());
        assert!(!item.is_canceled());
        item.cancel();
        assert!(item.is_canceled());
    }
}
