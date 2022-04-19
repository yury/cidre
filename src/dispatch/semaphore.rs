use crate::cf::Retained;
use crate::define_obj_type;
use crate::dispatch::Object;

pub struct SignalGuard {
    sema: Retained<'static, Semaphore>
}
impl SignalGuard {
    pub fn consume(&self) { }
}

impl<'a> Drop for SignalGuard {
    fn drop(&mut self) {
        self.sema.signal();
    }
}

define_obj_type!(Semaphore(Object));

impl Semaphore {
    pub fn new<'a>(value: isize) -> Retained<'a, Semaphore> {
        debug_assert!(value >= 0);
        unsafe { dispatch_semaphore_create(value) }
    }

    pub fn wait(&self, timeout: super::Time) -> isize {
        unsafe { dispatch_semaphore_wait(self, timeout) }
    }

    pub fn wait_forever(&self) -> isize {
        unsafe { dispatch_semaphore_wait(self, super::Time::FOREVER) }
    }

    pub fn signal(&self) -> isize {
        unsafe { dispatch_semaphore_signal(self) }
    }

    pub fn signal_guard(&self) -> SignalGuard {
        SignalGuard {
            sema: self.retained()
        }
    }
}


#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_semaphore_create<'a>(value: isize) -> Retained<'a, Semaphore>;
    fn dispatch_semaphore_wait(sema: &Semaphore, timeout: super::Time) -> isize;
    fn dispatch_semaphore_signal(sema: &Semaphore) -> isize;
}
