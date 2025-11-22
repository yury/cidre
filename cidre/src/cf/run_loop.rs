#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::mach;
use crate::{arc, cf, define_cf_type};

#[doc(alias = "CFRunLoopRunResult")]
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

define_cf_type!(
    #[doc(alias = "CFRunLoop")]
    RunLoop(cf::Type)
);

define_cf_type!(
    #[doc(alias = "CFRunLoopSource")]
    Src(cf::Type)
);

define_cf_type!(
    #[doc(alias = "CFRunLoopObserverRef")]
    Observer(cf::Type)
);

define_cf_type!(
    #[doc(alias = "CFRunLoopTimerRef")]
    Timer(cf::Type)
);

impl RunLoop {
    #[doc(alias = "CFRunLoopRun")]
    #[inline]
    pub fn run() {
        unsafe { CFRunLoopRun() }
    }

    #[doc(alias = "CFRunLoopGetCurrent")]
    #[inline]
    pub fn current() -> &'static RunLoop {
        unsafe { CFRunLoopGetCurrent() }
    }

    #[doc(alias = "CFRunLoopGetMain")]
    #[inline]
    pub fn main() -> &'static RunLoop {
        unsafe { CFRunLoopGetMain() }
    }

    #[doc(alias = "CFRunLoopStop")]
    #[inline]
    pub fn stop(&self) {
        unsafe { CFRunLoopStop(self) }
    }

    #[doc(alias = "CFRunLoopCopyCurrentMode")]
    #[inline]
    pub fn current_mode(&self) -> Option<arc::R<Mode>> {
        unsafe { CFRunLoopCopyCurrentMode(self) }
    }

    #[doc(alias = "CFRunLoopCopyAllModes")]
    #[inline]
    pub fn all_modes(&self) -> arc::R<cf::ArrayOf<Mode>> {
        unsafe { CFRunLoopCopyAllModes(self) }
    }

    #[doc(alias = "CFRunLoopContainsSource")]
    #[inline]
    pub fn contains_src(&self, source: &Src, mode: &Mode) -> bool {
        unsafe { CFRunLoopContainsSource(self, source, mode) }
    }

    #[doc(alias = "CFRunLoopAddSource")]
    #[inline]
    pub fn add_src(&self, source: &Src, mode: &Mode) {
        unsafe { CFRunLoopAddSource(self, source, mode) }
    }

    #[doc(alias = "CFRunLoopRemoveSource")]
    #[inline]
    pub fn remove_src(&self, source: &Src, mode: &Mode) {
        unsafe { CFRunLoopRemoveSource(self, source, mode) }
    }

    #[doc(alias = "CFRunLoopContainsObserver")]
    #[inline]
    pub fn contains_observer(&self, observer: &Observer, mode: &Mode) -> bool {
        unsafe { CFRunLoopContainsObserver(self, observer, mode) }
    }

    #[doc(alias = "CFRunLoopAddObserver")]
    #[inline]
    pub fn add_observer(&self, observer: &Observer, mode: &Mode) {
        unsafe { CFRunLoopAddObserver(self, observer, mode) }
    }

    #[doc(alias = "CFRunLoopRemoveObserver")]
    #[inline]
    pub fn remove_observer(&self, observer: &Observer, mode: &Mode) {
        unsafe { CFRunLoopRemoveObserver(self, observer, mode) }
    }

    #[doc(alias = "CFRunLoopContainsTimer")]
    #[inline]
    pub fn contains_timer(&self, timer: &Timer, mode: &Mode) -> bool {
        unsafe { CFRunLoopContainsTimer(self, timer, mode) }
    }

    #[doc(alias = "CFRunLoopAddTimer")]
    #[inline]
    pub fn add_timer(&self, timer: &Timer, mode: &Mode) {
        unsafe { CFRunLoopAddTimer(self, timer, mode) }
    }

    #[doc(alias = "CFRunLoopRemoveTimer")]
    #[inline]
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
    #[doc(alias = "CFRunLoopRunInMode")]
    #[inline]
    pub fn run_in_mode(
        mode: &Mode,
        seconds: cf::TimeInterval,
        return_after_source_handled: bool,
    ) -> RunResult {
        unsafe { CFRunLoopRunInMode(mode, seconds, return_after_source_handled) }
    }

    #[doc(alias = "CFRunLoopIsWaiting")]
    #[inline]
    pub fn is_waiting(&self) -> bool {
        unsafe { CFRunLoopIsWaiting(self) }
    }

    #[doc(alias = "CFRunLoopWakeUp")]
    #[inline]
    pub fn wakeup(&self) {
        unsafe { CFRunLoopWakeUp(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn CFRunLoopRun();
    fn CFRunLoopStop(rl: &RunLoop);
    fn CFRunLoopGetCurrent() -> &'static RunLoop;
    fn CFRunLoopGetMain() -> &'static RunLoop;
    fn CFRunLoopCopyCurrentMode(rl: &RunLoop) -> Option<arc::R<Mode>>;
    fn CFRunLoopCopyAllModes(rl: &RunLoop) -> arc::R<cf::ArrayOf<Mode>>;
    fn CFRunLoopContainsSource(rl: &RunLoop, source: &Src, mode: &Mode) -> bool;
    fn CFRunLoopAddSource(rl: &RunLoop, source: &Src, mode: &Mode);
    fn CFRunLoopRemoveSource(rl: &RunLoop, source: &Src, mode: &Mode);
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

define_cf_type!(
    #[doc(alias = "CFRunLoopMode")]
    Mode(cf::String)
);

impl Mode {
    pub fn with_string(name: &cf::String) -> &Self {
        unsafe { std::mem::transmute(name) }
    }

    #[doc(alias = "kCFRunLoopDefaultMode")]
    #[inline]
    pub fn default() -> &'static Mode {
        unsafe { kCFRunLoopDefaultMode }
    }

    #[doc(alias = "kCFRunLoopCommonModes")]
    #[inline]
    pub fn common() -> &'static Mode {
        unsafe { kCFRunLoopCommonModes }
    }
}

impl Default for &'static Mode {
    fn default() -> Self {
        Mode::default()
    }
}

unsafe extern "C" {
    static kCFRunLoopDefaultMode: &'static Mode;
    static kCFRunLoopCommonModes: &'static Mode;
}

#[doc(alias = "CFRunLoopSourceContext")]
#[repr(C)]
pub struct SrcCtx<T> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C-unwind" fn(info: *const T) -> *mut T>,
    pub release: Option<extern "C-unwind" fn(info: *const T)>,
    pub desc: Option<extern "C-unwind" fn(info: *const T) -> arc::R<cf::String>>,
    pub equal: Option<extern "C-unwind" fn(info0: *const T, info1: *const T) -> bool>,
    pub hash: Option<extern "C-unwind" fn(info: *const T) -> cf::HashCode>,
    pub schedule:
        Option<extern "C-unwind" fn(info: *mut T, rl: &cf::RunLoop, mode: cf::RunLoopMode)>,
    pub cancel: Option<extern "C-unwind" fn(info: *mut T, rl: &cf::RunLoop, mode: cf::RunLoopMode)>,
    pub perform: Option<extern "C-unwind" fn(info: *mut T)>,
}

impl<T> Default for SrcCtx<T> {
    fn default() -> Self {
        Self {
            version: 0,
            info: std::ptr::null_mut(),
            retain: None,
            release: None,
            desc: None,
            equal: None,
            hash: None,
            schedule: None,
            cancel: None,
            perform: None,
        }
    }
}

#[doc(alias = "CFRunLoopSourceContext1")]
#[repr(C)]
pub struct SrcCtx1<T> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C-unwind" fn(info: *const T) -> *mut T>,
    pub release: Option<extern "C-unwind" fn(info: *const T)>,
    pub desc: Option<extern "C-unwind" fn(info: *const T) -> arc::R<cf::String>>,
    pub equal: Option<extern "C-unwind" fn(info0: *const T, info1: *const T) -> bool>,
    pub hash: Option<extern "C-unwind" fn(info: *const T) -> cf::HashCode>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub port: Option<extern "C-unwind" fn(info: *mut T) -> mach::Port>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub perform: Option<
        extern "C-unwind" fn(
            msg: *mut std::ffi::c_void,
            size: cf::Index,
            allocator: Option<&cf::Allocator>,
            info: *mut T,
        ) -> *mut std::ffi::c_void,
    >,
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    pub port: Option<extern "C-unwind" fn(info: *mut T) -> *mut std::ffi::c_void>,
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    pub perform: Option<extern "C-unwind" fn(info: *mut T)>,
}

impl<T> Default for SrcCtx1<T> {
    fn default() -> Self {
        Self {
            version: 1,
            info: std::ptr::null_mut(),
            retain: None,
            release: None,
            desc: None,
            equal: None,
            hash: None,
            port: None,
            perform: None,
        }
    }
}

impl Src {
    #[doc(alias = "CFRunLoopSourceGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFRunLoopSourceGetTypeID() }
    }

    #[doc(alias = "CFRunLoopSourceCreate")]
    pub fn with_ctx_in<T>(
        order: cf::Index,
        ctx: &mut SrcCtx<T>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        debug_assert_eq!(ctx.version, 0);
        unsafe { CFRunLoopSourceCreate(allocator, order, std::mem::transmute(ctx)) }
    }

    #[doc(alias = "CFRunLoopSourceCreate")]
    pub fn with_ctx_v1_in<T>(
        order: cf::Index,
        ctx: &mut SrcCtx1<T>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        debug_assert_eq!(ctx.version, 1);
        unsafe { CFRunLoopSourceCreate(allocator, order, std::mem::transmute(ctx)) }
    }

    #[doc(alias = "CFRunLoopSourceCreate")]
    pub fn with_ctx<T>(order: cf::Index, ctx: &mut SrcCtx<T>) -> Option<arc::R<Self>> {
        Self::with_ctx_in(order, ctx, None)
    }

    #[doc(alias = "CFRunLoopSourceCreate")]
    pub fn with_ctx_v1<T>(order: cf::Index, ctx: &mut SrcCtx1<T>) -> Option<arc::R<Self>> {
        Self::with_ctx_v1_in(order, ctx, None)
    }

    #[doc(alias = "CFRunLoopSourceInvalidate")]
    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFRunLoopSourceInvalidate(self) }
    }

    #[doc(alias = "CFRunLoopSourceIsValid")]
    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFRunLoopSourceIsValid(self) }
    }

    #[doc(alias = "CFRunLoopSourceGetOrder")]
    #[inline]
    pub fn order(&self) -> cf::Index {
        unsafe { CFRunLoopSourceGetOrder(self) }
    }

    #[doc(alias = "CFRunLoopSourceSignal")]
    #[inline]
    pub fn signal(&self) {
        unsafe { CFRunLoopSourceSignal(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn CFRunLoopSourceGetTypeID() -> cf::TypeId;
    fn CFRunLoopSourceCreate(
        allocator: Option<&cf::Allocator>,
        order: cf::Index,
        ctx: *mut SrcCtx<std::ffi::c_void>,
    ) -> Option<arc::R<Src>>;
    fn CFRunLoopSourceInvalidate(source: &Src);
    fn CFRunLoopSourceIsValid(source: &Src) -> bool;
    fn CFRunLoopSourceGetOrder(source: &Src) -> cf::Index;
    fn CFRunLoopSourceSignal(source: &Src);
}

#[doc(alias = "CFRunLoopTimerContext")]
#[repr(C)]
pub struct TimerCtx<T: Send = std::ffi::c_void> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C-unwind" fn(info: *mut T) -> *mut T>,
    pub release: Option<extern "C-unwind" fn(info: *mut T)>,
    pub desc: Option<extern "C-unwind" fn(info: *const T) -> arc::R<cf::String>>,
}

pub type TimerCb<T = std::ffi::c_void> = extern "C-unwind" fn(timer: &mut Timer, info: *mut T);

impl Timer {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFRunLoopTimerGetTypeID() }
    }

    #[inline]
    pub fn invalidate(&mut self) {
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

    #[inline]
    pub fn next_fire_date(&self) -> cf::AbsTime {
        unsafe { CFRunLoopTimerGetNextFireDate(self) }
    }

    #[inline]
    pub fn set_next_fire_date(&mut self, val: cf::AbsTime) {
        unsafe {
            CFRunLoopTimerSetNextFireDate(self, val);
        }
    }

    #[inline]
    pub fn interval(&self) -> cf::TimeInterval {
        unsafe { CFRunLoopTimerGetInterval(self) }
    }

    #[inline]
    pub fn order(&self) -> cf::Index {
        unsafe { CFRunLoopTimerGetOrder(self) }
    }

    #[inline]
    pub unsafe fn create_in(
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        cb: TimerCb,
        ctx: *mut TimerCtx,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { CFRunLoopTimerCreate(allocator, fire_date, interval, flags, order, cb, ctx) }
    }

    #[inline]
    pub unsafe fn new<T: Send>(
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        cb: TimerCb<T>,
        ctx: *mut TimerCtx<T>,
    ) -> arc::R<Self> {
        unsafe {
            Self::create_in(
                fire_date,
                interval,
                flags,
                order,
                std::mem::transmute(cb),
                std::mem::transmute(ctx),
                None,
            )
            .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn new_without_ctx(
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        cb: TimerCb,
    ) -> arc::R<Self> {
        unsafe { Self::new(fire_date, interval, flags, order, cb, std::ptr::null_mut()) }
    }
}

#[cfg(feature = "ns")]
use crate::ns;

impl Timer {
    #[cfg(feature = "ns")]
    pub fn as_ns(&self) -> &ns::Timer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "blocks")]
impl Timer {
    pub fn with_handler_block_in(
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        block: &mut blocks::SyncBlock<fn(timer: &mut Timer)>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CFRunLoopTimerCreateWithHandler(allocator, fire_date, interval, flags, order, block)
        }
    }
    pub fn with_handler_block(
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        block: &mut blocks::SyncBlock<fn(timer: &mut Timer)>,
    ) -> arc::R<Self> {
        unsafe {
            Self::with_handler_block_in(fire_date, interval, flags, order, block, None)
                .unwrap_unchecked()
        }
    }
    pub fn with_handler(
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        block: impl FnMut(&mut Timer) + 'static + Sync,
    ) -> arc::R<Self> {
        let mut block = blocks::SyncBlock::new1(block);
        Self::with_handler_block(fire_date, interval, flags, order, &mut block)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn CFRunLoopTimerGetTypeID() -> cf::TypeId;
    fn CFRunLoopTimerCreate(
        allocator: Option<&cf::Allocator>,
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        cb: TimerCb,
        ctx: *mut TimerCtx,
    ) -> Option<arc::R<Timer>>;

    #[cfg(feature = "blocks")]
    fn CFRunLoopTimerCreateWithHandler(
        allocator: Option<&cf::Allocator>,
        fire_date: cf::AbsTime,
        interval: cf::TimeInterval,
        flags: cf::OptionFlags,
        order: cf::Index,
        block: &mut blocks::SyncBlock<fn(timer: &mut Timer)>,
    ) -> Option<arc::R<Timer>>;
    fn CFRunLoopTimerInvalidate(timer: &mut Timer);
    fn CFRunLoopTimerIsValid(timer: &Timer) -> bool;
    fn CFRunLoopTimerDoesRepeat(timer: &Timer) -> bool;
    fn CFRunLoopTimerGetTolerance(timer: &Timer) -> cf::TimeInterval;
    fn CFRunLoopTimerSetTolerance(timer: &Timer, value: cf::TimeInterval);
    fn CFRunLoopTimerGetNextFireDate(timer: &Timer) -> cf::AbsTime;
    fn CFRunLoopTimerSetNextFireDate(timer: &mut Timer, val: cf::AbsTime);
    fn CFRunLoopTimerGetInterval(timer: &Timer) -> cf::TimeInterval;
    fn CFRunLoopTimerGetOrder(timer: &Timer) -> cf::Index;
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
unsafe extern "C-unwind" {
    fn CFRunLoopObserverInvalidate(timer: &Observer);
    fn CFRunLoopObserverIsValid(timer: &Observer) -> bool;
    fn CFRunLoopObserverDoesRepeat(timer: &Observer) -> bool;
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU16, Ordering};

    use crate::cf;

    #[test]
    fn runloop_main() {
        assert!(!cf::RunLoop::current().all_modes().is_empty());
    }

    #[test]
    fn timer() {
        extern "C-unwind" fn callback(_timer: &mut cf::RunLoopTimer, _info: *mut std::ffi::c_void) {
        }
        let timer = unsafe {
            cf::RunLoopTimer::new(
                0.0,
                0.0,
                Default::default(),
                0,
                callback,
                std::ptr::null_mut(),
            )
        };

        assert_eq!(cf::RunLoopTimer::type_id(), timer.get_type_id());

        let timer = cf::RunLoopTimer::with_handler(0.0, 0.0, Default::default(), 1, |_timer| {});

        assert_eq!(cf::RunLoopTimer::type_id(), timer.get_type_id());
    }

    #[test]
    fn src() {
        static COUNTER: AtomicU16 = AtomicU16::new(0);

        extern "C-unwind" fn perform(_info: *mut std::ffi::c_void) {
            COUNTER.fetch_add(1, Ordering::SeqCst);
            cf::RunLoop::current().stop();
        }

        let mut ctx = cf::RunLoopSrcCtx {
            perform: Some(perform),
            ..Default::default()
        };

        let src = cf::RunLoopSrc::with_ctx(1, &mut ctx).unwrap();
        assert!(src.is_valid());
        assert_eq!(src.order(), 1);

        let rl = cf::RunLoop::current();
        rl.add_src(&src, &cf::RunLoopMode::common());
        assert!(rl.contains_src(&src, &cf::RunLoopMode::common()));

        src.signal();
        cf::RunLoop::run();

        assert_eq!(1, COUNTER.load(Ordering::SeqCst));

        src.signal();
        cf::RunLoop::run();

        assert_eq!(2, COUNTER.load(Ordering::SeqCst));

        src.invalidate();
        assert!(!src.is_valid());
        assert!(!rl.contains_src(&src, &cf::RunLoopMode::common()));
    }
}
