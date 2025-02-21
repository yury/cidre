use crate::{arc, define_obj_type, dispatch};

pub struct SignalGuard {
    sema: arc::R<Semaphore>,
}

impl SignalGuard {
    pub fn consume(&mut self) {}
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
    pub fn new(n: usize) -> arc::R<Self> {
        unsafe { dispatch_semaphore_create(n as _) }
    }

    #[doc(alias = "dispatch_semaphore_wait")]
    #[inline]
    pub fn wait(&self, timeout: dispatch::Time) -> isize {
        unsafe { dispatch_semaphore_wait(self, timeout) }
    }

    #[doc(alias = "dispatch_semaphore_wait")]
    #[inline]
    pub fn wait_forever(&self) -> isize {
        unsafe { dispatch_semaphore_wait(self, dispatch::Time::FOREVER) }
    }

    #[doc(alias = "dispatch_semaphore_signal")]
    #[inline]
    pub fn signal(&self) -> isize {
        unsafe { dispatch_semaphore_signal(self) }
    }

    #[inline]
    pub fn guard(&self) -> SignalGuard {
        SignalGuard {
            sema: self.retained(),
        }
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    fn dispatch_semaphore_create(value: isize) -> arc::R<Semaphore>;
    fn dispatch_semaphore_wait(sema: &Semaphore, timeout: dispatch::Time) -> isize;
    fn dispatch_semaphore_signal(sema: &Semaphore) -> isize;
}

#[cfg(test)]
mod tests {
    use crate::dispatch;

    #[test]
    fn basics() {
        let _sema_0 = dispatch::Semaphore::new(0);
        let _sema_1 = dispatch::Semaphore::new(1);
    }
}
