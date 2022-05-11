use crate::{cf, define_cf_type};

use super::Retained;

#[repr(i32)]
pub enum RunResult {
    Finished = 1,
    Stopped = 2,
    TimedOut = 3,
    HandledSource = 4,
}

define_cf_type!(RunLoop(cf::Type));
define_cf_type!(Source(cf::Type));
define_cf_type!(Observer(cf::Type));
define_cf_type!(Timer(cf::Type));

impl RunLoop {
    pub fn run() {
        unsafe { CFRunLoopRun() }
    }

    pub fn current() -> &'static RunLoop {
        unsafe { CFRunLoopGetCurrent() }
    }

    pub fn main() -> &'static RunLoop {
        unsafe { CFRunLoopGetMain() }
    }

    pub fn stop(&self) {
        unsafe { CFRunLoopStop(self) }
    }

    pub fn current_mode<'a>(&self) -> Option<Retained<'a, Mode>> {
        unsafe { CFRunLoopCopyCurrentMode(self) }
    }

    pub fn all_modes<'a>(&self) -> Retained<'a, cf::ArrayOf<Mode>> {
        unsafe { CFRunLoopCopyAllModes(self) }
    }

    pub fn contains_source(&self, source: &Source, mode: &Mode) -> bool {
        unsafe { CFRunLoopContainsSource(self, source, mode) }
    }

    pub fn add_source(&self, source: &Source, mode: &Mode) {
        unsafe { CFRunLoopAddSource(self, source, mode) }
    }

    pub fn remove_source(&self, source: &Source, mode: &Mode) {
        unsafe { CFRunLoopRemoveSource(self, source, mode) }
    }

    pub fn contains_observer(&self, observer: &Observer, mode: &Mode) -> bool {
        unsafe { CFRunLoopContainsObserver(self, observer, mode) }
    }

    pub fn add_observer(&self, observer: &Observer, mode: &Mode) {
        unsafe { CFRunLoopAddObserver(self, observer, mode) }
    }

    pub fn remove_observer(&self, observer: &Observer, mode: &Mode) {
        unsafe { CFRunLoopRemoveObserver(self, observer, mode) }
    }

    pub fn contains_timer(&self, timer: &Timer, mode: &Mode) -> bool {
        unsafe { CFRunLoopContainsTimer(self, timer, mode) }
    }

    pub fn add_timer(&self, timer: &Timer, mode: &Mode) {
        unsafe { CFRunLoopAddTimer(self, timer, mode) }
    }

    pub fn remove_timer(&self, timer: &Timer, mode: &Mode) {
        unsafe { CFRunLoopRemoveTimer(self, timer, mode) }
    }
}

extern "C" {
    fn CFRunLoopRun();
    fn CFRunLoopStop(rl: &RunLoop);
    fn CFRunLoopGetCurrent() -> &'static RunLoop;
    fn CFRunLoopGetMain() -> &'static RunLoop;
    fn CFRunLoopCopyCurrentMode<'a>(rl: &RunLoop) -> Option<Retained<'a, Mode>>;
    fn CFRunLoopCopyAllModes<'a>(rl: &RunLoop) -> Retained<'a, cf::ArrayOf<Mode>>;
    fn CFRunLoopContainsSource(rl: &RunLoop, source: &Source, mode: &Mode) -> bool;
    fn CFRunLoopAddSource(rl: &RunLoop, source: &Source, mode: &Mode);
    fn CFRunLoopRemoveSource(rl: &RunLoop, source: &Source, mode: &Mode);
    fn CFRunLoopContainsObserver(rl: &RunLoop, observer: &Observer, mode: &Mode) -> bool;
    fn CFRunLoopAddObserver(rl: &RunLoop, observer: &Observer, mode: &Mode);
    fn CFRunLoopRemoveObserver(rl: &RunLoop, observer: &Observer, mode: &Mode);
    fn CFRunLoopContainsTimer(rl: &RunLoop, timer: &Timer, mode: &Mode) -> bool;
    fn CFRunLoopAddTimer(rl: &RunLoop, timer: &Timer, mode: &Mode);
    fn CFRunLoopRemoveTimer(rl: &RunLoop, timer: &Timer, mode: &Mode);
}

define_cf_type!(Mode(cf::String));

impl Mode {
    pub fn default() -> &'static Mode {
        unsafe { kCFRunLoopDefaultMode }
    }

    pub fn common() -> &'static Mode {
        unsafe { kCFRunLoopCommonModes }
    }
}

extern "C" {
    static kCFRunLoopDefaultMode: &'static Mode;
    static kCFRunLoopCommonModes: &'static Mode;
}

#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn runloop_main() {
        assert!(!cf::RunLoop::current().all_modes().is_empty());
    }
}
