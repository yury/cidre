use std::{ffi::c_void, mem::transmute};

use crate::{arc, blocks, dispatch, objc};

/// The work you want to perform, encapsulated in a way that lets
/// you attach a completion handle or execution dependencies.
///
/// A dispatch::WorkItem encapsulates work to be performed on a
/// dispatch queue or within a dispatch group. You can also use
/// a work item as a dispatch::Source event, registration, or
/// cancellation handler.
#[repr(transparent)]
pub struct WorkItem(dispatch::Block<blocks::Sync>);

impl objc::Obj for WorkItem {
    #[inline]
    unsafe fn retain(id: &Self) -> arc::R<Self> {
        std::mem::transmute(_Block_copy(std::mem::transmute(id)))
    }

    #[inline]
    unsafe fn release(id: &mut Self) {
        _Block_release(std::mem::transmute(id))
    }
}

impl WorkItem {
    #[inline]
    pub fn with_flags(flags: dispatch::BlockFlags, block: &mut dispatch::Block) -> arc::R<Self> {
        unsafe { dispatch_block_create(flags, block) }
    }

    #[inline]
    pub fn with_qos(
        flags: dispatch::BlockFlags,
        qos_class: dispatch::QosClass,
        block: &dispatch::Block,
    ) -> Self {
        Self::with_qos_priority(flags, qos_class, 0, block)
    }

    #[inline]
    pub fn with_qos_priority(
        flags: dispatch::BlockFlags,
        qos_class: dispatch::QosClass,
        relative_priority: i32,
        block: &dispatch::Block,
    ) -> Self {
        unsafe {
            transmute(dispatch_block_create_with_qos_class(
                flags,
                qos_class,
                relative_priority,
                block,
            ))
        }
    }

    #[inline]
    pub fn wait_forever(&self) {
        unsafe {
            dispatch_block_wait(self, dispatch::Time::FOREVER);
        }
    }

    #[inline]
    pub fn wait(&self, timeout: dispatch::Time) -> Result<(), ()> {
        unsafe {
            if dispatch_block_wait(self, timeout) == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[inline]
    pub fn wait_wall_timeout(&self, timeout: dispatch::WallTime) -> Result<(), ()> {
        self.wait(timeout.0)
    }

    #[inline]
    #[doc(alias = "dispatch_block_cancel")]
    pub fn cancel(&mut self) {
        unsafe { dispatch_block_cancel(self) }
    }

    /// Tests whether the given dispatch block object has been canceled.
    #[inline]
    #[doc(alias = "dispatch_block_testcancel")]
    #[must_use]
    pub fn is_canceled(&self) -> bool {
        unsafe { dispatch_block_testcancel(self) != 0 }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C-unwind" {
    fn dispatch_block_create<'a>(
        flags: dispatch::BlockFlags,
        block: &dispatch::Block,
    ) -> arc::R<WorkItem>;
    fn dispatch_block_create_with_qos_class(
        flags: dispatch::BlockFlags,
        qos_class: dispatch::QosClass,
        relative_priority: i32,
        block: &dispatch::Block,
    ) -> *mut c_void;
    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
    fn dispatch_block_cancel(block: &WorkItem);
    fn dispatch_block_testcancel(block: &WorkItem) -> isize;
    fn dispatch_block_wait(block: &WorkItem, timeout: dispatch::Time) -> isize;
}

#[cfg(test)]
mod tests {
    use crate::{blocks, dispatch};

    #[test]
    fn basics() {
        let mut b = dispatch::Block::<blocks::Send>::new0(|| println!("nice"));
        let mut item = dispatch::WorkItem::with_flags(Default::default(), &mut b);
        assert!(!item.is_canceled());
        item.cancel();
        assert!(item.is_canceled());
        let res = item.wait(dispatch::Time::NOW);
        assert!(matches!(res, Err(())));
    }
}
