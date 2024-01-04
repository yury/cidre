use crate::{arc, define_obj_type, dispatch};

pub struct SignalGuard {
    sema: arc::R<Semaphore>,
}

impl SignalGuard {
    pub fn consume(self) {}
}

impl Drop for SignalGuard {
    #[inline]
    fn drop(&mut self) {
        self.sema.signal();
    }
}

define_obj_type!(
    #[doc(alias = "dispatch_semaphore_t")]
    #[doc(alias = "DispatchSemaphore")]
    pub Semaphore(dispatch::Object)
);

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
    #[doc(alias = "dispatch_semaphore_create")]
    #[inline]
    pub fn new(value: isize) -> arc::R<Self> {
        debug_assert!(value >= 0);
        unsafe { dispatch_semaphore_create(value) }
    }

    #[doc(alias = "dispatch_semaphore_wait")]
    #[inline]
    pub fn wait(&self, timeout: dispatch::Time) -> isize {
        unsafe { dispatch_semaphore_wait(self, timeout) }
    }

    #[doc(alias = "dispatch_semaphore_wait")]
    #[inline]
    pub fn wait_forever(&self) -> isize {
        unsafe { dispatch_semaphore_wait(self, dispatch::Time::DISTANT_FUTURE) }
    }

    #[doc(alias = "dispatch_semaphore_signal")]
    #[inline]
    pub fn signal(&self) -> isize {
        unsafe { dispatch_semaphore_signal(self) }
    }

    #[inline]
    pub fn signal_guard(&self) -> SignalGuard {
        SignalGuard {
            sema: self.retained(),
        }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_semaphore_create(value: isize) -> arc::R<Semaphore>;
    fn dispatch_semaphore_wait(sema: &Semaphore, timeout: dispatch::Time) -> isize;
    fn dispatch_semaphore_signal(sema: &Semaphore) -> isize;
}
