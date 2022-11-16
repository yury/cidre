use crate::cf::Retained;
use crate::define_obj_type;
use crate::dispatch::Object;

pub struct SignalGuard {
    sema: Retained<Semaphore>,
}
impl SignalGuard {
    pub fn consume(&self) {}
}

impl Drop for SignalGuard {
    #[inline]
    fn drop(&mut self) {
        self.sema.signal();
    }
}

define_obj_type!(Semaphore(Object));

impl Semaphore {
    #[inline]
    pub fn new(value: isize) -> Retained<Semaphore> {
        debug_assert!(value >= 0);
        unsafe { dispatch_semaphore_create(value) }
    }

    #[inline]
    pub fn wait(&self, timeout: super::Time) -> isize {
        unsafe { dispatch_semaphore_wait(self, timeout) }
    }

    #[inline]
    pub fn wait_forever(&self) -> isize {
        unsafe { dispatch_semaphore_wait(self, super::Time::FOREVER) }
    }

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
    fn dispatch_semaphore_create(value: isize) -> Retained<Semaphore>;
    fn dispatch_semaphore_wait(sema: &Semaphore, timeout: super::Time) -> isize;
    fn dispatch_semaphore_signal(sema: &Semaphore) -> isize;
}
