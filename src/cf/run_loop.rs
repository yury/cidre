use std::intrinsics::transmute;

use crate::{cf, define_cf_type};

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum RunResult {
    /// The run loop mode mode has no sources or timers.
    Finished = 1,

    /// The run loop was stopped with CFRunLoopStop.
    Stopped = 2,

    /// The time interval seconds passed.
    TimedOut = 3,

    /// A source was processed. This exit condition only applies when `return_after_source_handled` is true.
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

    pub fn current_mode(&self) -> Option<cf::Retained<Mode>> {
        unsafe { CFRunLoopCopyCurrentMode(self) }
    }

    pub fn all_modes(&self) -> cf::Retained<cf::ArrayOf<Mode>> {
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

    /// Run loops can be run recursively. You can call `Mode` from within any run loop
    /// callout and create nested run loop activations on the current thread’s call stack.
    /// You are not restricted in which modes you can run from within a callout.
    /// You can create another run loop activation running in any available run loop mode,
    /// including any modes already running higher in the call stack.
    ///
    /// The run loop exits with the following return values under the indicated conditions:
    ///     - Result::Finished. The run loop mode mode has no sources or timers.
    ///     - Result::Stopped. The run loop was stopped with CFRunLoopStop.
    ///     - Result::TimedOut. The time interval seconds passed.
    ///     - Result::HandledSource. A source was processed.
    /// This exit condition only applies when returnAfterSourceHandled is true.
    ///
    /// You must not specify the kCFRunLoopCommonModes constant for the mode parameter.
    /// Run loops always run in a specific mode. You specify the common modes only when
    /// configuring a run-loop observer and only in situations where you want that
    /// observer to run in more than one mode.
    #[inline]
    pub fn run_in_mode(
        mode: &Mode,
        seconds: cf::TimeInterval,
        return_after_source_handled: bool,
    ) -> RunResult {
        unsafe { CFRunLoopRunInMode(mode, seconds, return_after_source_handled) }
    }

    pub fn is_waiting(&self) -> bool {
        unsafe { CFRunLoopIsWaiting(self) }
    }

    pub fn wakeup(&self) {
        unsafe { CFRunLoopWakeUp(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRunLoopRun();
    fn CFRunLoopStop(rl: &RunLoop);
    fn CFRunLoopGetCurrent() -> &'static RunLoop;
    fn CFRunLoopGetMain() -> &'static RunLoop;
    fn CFRunLoopCopyCurrentMode(rl: &RunLoop) -> Option<cf::Retained<Mode>>;
    fn CFRunLoopCopyAllModes(rl: &RunLoop) -> cf::Retained<cf::ArrayOf<Mode>>;
    fn CFRunLoopContainsSource(rl: &RunLoop, source: &Source, mode: &Mode) -> bool;
    fn CFRunLoopAddSource(rl: &RunLoop, source: &Source, mode: &Mode);
    fn CFRunLoopRemoveSource(rl: &RunLoop, source: &Source, mode: &Mode);
    fn CFRunLoopContainsObserver(rl: &RunLoop, observer: &Observer, mode: &Mode) -> bool;
    fn CFRunLoopAddObserver(rl: &RunLoop, observer: &Observer, mode: &Mode);
    fn CFRunLoopRemoveObserver(rl: &RunLoop, observer: &Observer, mode: &Mode);
    fn CFRunLoopContainsTimer(rl: &RunLoop, timer: &Timer, mode: &Mode) -> bool;
    fn CFRunLoopAddTimer(rl: &RunLoop, timer: &Timer, mode: &Mode);
    fn CFRunLoopRemoveTimer(rl: &RunLoop, timer: &Timer, mode: &Mode);
    fn CFRunLoopIsWaiting(rl: &RunLoop) -> bool;
    fn CFRunLoopWakeUp(rl: &RunLoop);

    fn CFRunLoopRunInMode(
        mode: &Mode,
        seconds: cf::TimeInterval,
        return_after_source_handled: bool,
    ) -> RunResult;
}

define_cf_type!(Mode(cf::String));

impl Mode {
    pub fn new(name: &cf::String) -> &Self {
        unsafe { transmute(name) }
    }

    #[inline]
    pub fn default() -> &'static Mode {
        unsafe { kCFRunLoopDefaultMode }
    }

    #[inline]
    pub fn common() -> &'static Mode {
        unsafe { kCFRunLoopCommonModes }
    }
}

extern "C" {
    static kCFRunLoopDefaultMode: &'static Mode;
    static kCFRunLoopCommonModes: &'static Mode;
}

impl Source {
    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFRunLoopSourceInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFRunLoopSourceIsValid(self) }
    }

    #[inline]
    pub fn order(&self) -> cf::Index {
        unsafe { CFRunLoopSourceGetOrder(self) }
    }

    #[inline]
    pub fn signal(&self) {
        unsafe { CFRunLoopSourceSignal(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRunLoopSourceInvalidate(source: &Source);
    fn CFRunLoopSourceIsValid(source: &Source) -> bool;
    fn CFRunLoopSourceGetOrder(source: &Source) -> cf::Index;
    fn CFRunLoopSourceSignal(source: &Source);
}

impl Timer {
    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFRunLoopTimerInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFRunLoopTimerIsValid(self) }
    }

    #[inline]
    pub fn does_repeat(&self) -> bool {
        unsafe { CFRunLoopTimerDoesRepeat(self) }
    }

    #[inline]
    pub fn tolerance(&self) -> cf::TimeInterval {
        unsafe { CFRunLoopTimerGetTolerance(self) }
    }

    #[inline]
    pub fn set_tolerance(&self, value: cf::TimeInterval) {
        unsafe { CFRunLoopTimerSetTolerance(self, value) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRunLoopTimerInvalidate(timer: &Timer);
    fn CFRunLoopTimerIsValid(timer: &Timer) -> bool;
    fn CFRunLoopTimerDoesRepeat(timer: &Timer) -> bool;
    fn CFRunLoopTimerGetTolerance(timer: &Timer) -> cf::TimeInterval;
    fn CFRunLoopTimerSetTolerance(timer: &Timer, value: cf::TimeInterval);
}

impl Observer {
    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFRunLoopObserverInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFRunLoopObserverIsValid(self) }
    }

    #[inline]
    pub fn does_repeat(&self) -> bool {
        unsafe { CFRunLoopObserverDoesRepeat(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRunLoopObserverInvalidate(timer: &Observer);
    fn CFRunLoopObserverIsValid(timer: &Observer) -> bool;
    fn CFRunLoopObserverDoesRepeat(timer: &Observer) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn runloop_main() {
        assert!(!cf::RunLoop::current().all_modes().is_empty());
    }
}
