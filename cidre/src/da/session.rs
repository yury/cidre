use crate::{arc, cf, define_cf_type, dispatch};

define_cf_type!(
    #[doc(alias = "DASession")]
    #[doc(alias = "DASessionRef")]
    Session(cf::Type)
);

impl Session {
    #[inline]
    pub fn get_type_id() -> cf::TypeId {
        unsafe { DASessionGetTypeID() }
    }

    #[inline]
    pub fn create_in(alloc: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { DASessionCreate(alloc) }
    }

    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { std::mem::transmute(DASessionCreate(None)) }
    }

    #[inline]
    pub fn schedule_with_run_loop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode) {
        unsafe { DASessionScheduleWithRunLoop(self, run_loop, mode) }
    }

    #[inline]
    pub fn unschedule_from_run_loop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode) {
        unsafe { DASessionUnscheduleFromRunLoop(self, run_loop, mode) }
    }

    #[inline]
    pub fn set_dispatch_queue(&mut self, val: Option<&dispatch::Queue>) {
        unsafe { DASessionSetDispatchQueue(self, val) }
    }
}

#[link(name = "DiskArbitration", kind = "framework")]
extern "C-unwind" {
    fn DASessionGetTypeID() -> cf::TypeId;
    fn DASessionCreate(allocator: Option<&cf::Allocator>) -> Option<arc::R<Session>>;
    fn DASessionScheduleWithRunLoop(
        session: &Session,
        run_loop: &cf::RunLoop,
        mode: &cf::RunLoopMode,
    );

    fn DASessionUnscheduleFromRunLoop(
        session: &Session,
        run_loop: &cf::RunLoop,
        mode: &cf::RunLoopMode,
    );

    fn DASessionSetDispatchQueue(session: &mut Session, val: Option<&dispatch::Queue>);
}

#[cfg(test)]
mod tests {
    use crate::{da, dispatch};

    #[test]
    fn basics() {
        let queue = dispatch::Queue::new();
        let mut session = da::Session::new();
        session.set_dispatch_queue(Some(&queue));
        session.show();
        session.set_dispatch_queue(None);
    }
}
