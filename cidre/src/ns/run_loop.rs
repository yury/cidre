use crate::{arc, cf, define_obj_type, ns, objc};

define_obj_type!(
    pub RunLoopMode(ns::String)
);

impl RunLoopMode {
    #[inline]
    pub fn default() -> &'static Self {
        unsafe { NSDefaultRunLoopMode }
    }

    #[inline]
    pub fn common() -> &'static Self {
        unsafe { NSRunLoopCommonModes }
    }
}

extern "C" {
    static NSDefaultRunLoopMode: &'static RunLoopMode;
    static NSRunLoopCommonModes: &'static RunLoopMode;
}

define_obj_type!(pub RunLoop(ns::Id), NS_RUN_LOOP);

impl RunLoop {
    #[objc::msg_send2(currentRunLoop)]
    pub fn current() -> &'static Self;

    #[objc::msg_send2(mainRunLoop)]
    pub fn main() -> &'static Self;

    #[objc::msg_send(getCFRunLoop)]
    pub fn as_cf_run_loop(&self) -> &cf::RunLoop;

    //- (void)addTimer:(NSTimer *)timer forMode:(NSRunLoopMode)mode;
    //pub fn add_timer(&self, c    )

    #[objc::msg_send(run)]
    pub fn run(&self);

    #[objc::msg_send(runUntilDate:)]
    pub fn run_until_date(&self, date: &ns::Date);

    #[objc::msg_send(runMode:beforeDate:)]
    pub fn run_mode_until_date(&self, mode: &RunLoopMode, before_date: &ns::Date) -> bool;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_RUN_LOOP: &'static objc::Class<ns::Id>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let current = ns::RunLoop::current();
        let main = ns::RunLoop::main();

        assert!(!current.is_equal(main));
    }
}
