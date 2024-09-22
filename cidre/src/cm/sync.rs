use crate::{arc, cf, cm, define_cf_type, dispatch, os};

define_cf_type!(
    /// A timing source object.
    ///
    /// A clock represents a source of time information: generally, a piece of hardware that measures the passage of time.
    /// One example of a clock is the host time clock, accessible via cm::Clock::host_time_clock().
    /// It measures time using the CPU system clock, which on Mac OS X is mach_absolute_time().
    /// Every audio device can also be considered a clock since the audio samples that it outputs or inputs each have a
    /// defined duration (eg, 1/48000 of a second for 48 kHz audio).
    ///
    /// cm::Clocks are read-only: they cannot be stopped or started, and the current time cannot be set.
    /// A CMClock has one primary function, cm::Clock::get_time, which tells what time it is now.
    /// Additionally, the cm::Sync infrastructure monitors relative drift between cm::Clocks.
    Clock(cf::Type)
);
define_cf_type!(
    /// Models a timeline under application control.
    ///
    /// A timebase represents a timeline that clients can control by setting the rate and time.
    /// Each timebase has either a source clock or a source timebase (previously referred to as a master clock or master timebase).
    /// The rate of the timebase is expressed relative to its source.
    /// When a timebase has rate 0.0, its time is fixed and does not change as its source's time changes.
    /// When a timebase has rate 1.0, its time increases one second as its source's time increases by one second.
    /// When a timebase has rate 2.0, its time increases two seconds as its source's time increases by one second.
    /// When a timebase has rate -1.0, its time decreases one second as its source's time increases by one second.
    ///
    /// If a timebase has a source timebase, the source timebase's rate is a factor in determining the timebase's effective rate.
    /// In fact, a timebase's effective rate is defined as the product of its rate, its source timebase's rate,
    /// its source timebase's source timebase's rate, and so on up to the ultimate source clock.  This is the rate at which
    /// the timebase's time changes relative to the ultimate source clock.
    Timebase(cf::Type)
);
define_cf_type!(ClockOrTimebase(cf::Type));

unsafe impl Send for Clock {}
unsafe impl Send for Timebase {}
unsafe impl Send for ClockOrTimebase {}

impl Clock {
    #[doc(alias = "CMClockGetTypeID")]
    pub fn get_type_id() -> cf::TypeId {
        unsafe { CMClockGetTypeID() }
    }

    /// Returns a reference to the singleton clock logically identified with host time.
    #[inline]
    #[doc(alias = "CMClockGetHostTimeClock")]
    pub fn host_time_clock() -> &'static Clock {
        unsafe { CMClockGetHostTimeClock() }
    }

    /// Use `audio_clock`
    #[doc(alias = "CMAudioClockCreate")]
    #[inline]
    pub unsafe fn audio_clock_create_in(
        allocator: Option<&cf::Allocator>,
        clock_out: *mut Option<arc::R<Clock>>,
    ) -> os::Status {
        CMAudioClockCreate(allocator, clock_out)
    }

    /// Creates a clock that advances at the same rate as audio output.
    ///
    /// This clock will not drift from audio output, but may drift from cm::Clock::host_time_clock().
    /// When audio output is completely stopped, the clock continues to advance, tracking cm::Clock::host_time_clock()
    /// until audio output starts up again.
    /// This clock is suitable for use as av::Player.master_clock when synchronizing video-only playback
    /// with audio played through other APIs or objects.
    #[inline]
    pub fn audio_clock() -> Result<arc::R<Clock>, os::Status> {
        let mut clock_out = None;
        unsafe {
            let res = Self::audio_clock_create_in(None, &mut clock_out);
            res.to_result_unchecked(clock_out)
        }
    }

    #[doc(alias = "CMClockGetTime")]
    #[inline]
    pub fn time(&self) -> cm::Time {
        unsafe { CMClockGetTime(self) }
    }

    /// Converts a host time from CMTime to the host time's native units.
    ///
    /// This function performs a scale conversion, not a clock conversion.
    /// It can be more accurate than CMTimeConvertScale because the system units may
    /// have a non-integer timescale.
    /// On Mac OS X, this function converts to the units of mach_absolute_time.
    #[doc(alias = "CMClockConvertHostTimeToSystemUnits")]
    #[inline]
    pub fn convert_host_time_to_sys_units(host_time: cm::Time) -> u64 {
        unsafe { CMClockConvertHostTimeToSystemUnits(host_time) }
    }

    /// Converts a host time from native units to cm::Time.
    #[doc(alias = "CMClockMakeHostTimeFromSystemUnits")]
    #[inline]
    pub fn make_host_time_from_sys_units(host_time: u64) -> cm::Time {
        unsafe { CMClockMakeHostTimeFromSystemUnits(host_time) }
    }

    #[doc(alias = "CMClockGetAnchorTime")]
    #[inline]
    pub fn anchor_time(
        &self,
        clock_time_out: &mut cm::Time,
        ref_clock_time_out: &mut cm::Time,
    ) -> Result<(), os::Status> {
        unsafe { CMClockGetAnchorTime(self, clock_time_out, ref_clock_time_out).result() }
    }

    /// Indicates whether it is possible for two clocks to drift relative to each other.
    #[doc(alias = "CMClockMightDrift")]
    #[inline]
    pub fn might_dirft(&self, other: &Clock) -> bool {
        unsafe { CMClockMightDrift(self, other) }
    }

    /// Makes the clock stop functioning.
    ///
    /// After invalidation, the clock will return errors from all APIs.
    /// This should only be called by the "owner" of the clock, who knows (for example)
    /// that some piece of hardware has gone away, and the clock will no longer work (and might even crash).
    #[doc(alias = "CMClockInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { CMClockInvalidate(self) }
    }
}

/// cm::Clock error codes
pub mod clock_err {
    use crate::os;

    #[doc(alias = "kCMClockError_MissingRequiredParameter")]
    pub const MISSING_REQUIRED_PARAMETER: os::Status = os::Status(-12745);

    #[doc(alias = "kCMClockError_InvalidParameter")]
    pub const INVALID_PARAMETER: os::Status = os::Status(-12746);

    #[doc(alias = "kCMClockError_AllocationFailed")]
    pub const ALLOCATION_FAILED: os::Status = os::Status(-12747);

    #[doc(alias = "kCMClockError_UnsupportedOperation")]
    pub const UNSUPPORTED_OPERATION: os::Status = os::Status(-12756);
}

/// cm::Timebase error codes
pub mod timebase_err {
    use crate::os;

    #[doc(alias = "kCMTimebaseError_MissingRequiredParameter")]
    pub const MISSING_REQUIRED_PARAMETER: os::Status = os::Status(-12748);

    #[doc(alias = "kCMTimebaseError_InvalidParameter")]
    pub const INVALID_PARAMETER: os::Status = os::Status(-12749);

    #[doc(alias = "kCMTimebaseError_AllocationFailed")]
    pub const ALLOCATION_FAILED: os::Status = os::Status(-12750);

    #[doc(alias = "kCMTimebaseError_TimerIntervalTooShort")]
    pub const TIMER_INTERVAL_TOO_SHORT: os::Status = os::Status(-12751);

    #[doc(alias = "kCMTimebaseError_ReadOnly")]
    pub const READ_ONLY: os::Status = os::Status(-12757);
}

pub mod sync_err {
    use crate::os;

    #[doc(alias = "kCMSyncError_MissingRequiredParameter")]
    pub const MISSING_REQUIRED_PARAMETER: os::Status = os::Status(-12752);

    #[doc(alias = "kCMSyncError_InvalidParameter")]
    pub const INVALID_PARAMETER: os::Status = os::Status(-12753);

    #[doc(alias = "kCMSyncError_AllocationFailed")]
    pub const ALLOCATION_FAILED: os::Status = os::Status(-12754);

    #[doc(alias = "kCMSyncError_RateMustBeNonZero")]
    pub const RATE_MUST_BE_NON_ZERO: os::Status = os::Status(-12755);
}

#[link(name = "CoreMedia", kind = "framework")]
extern "C-unwind" {
    fn CMClockGetTypeID() -> cf::TypeId;
    fn CMClockGetHostTimeClock() -> &'static Clock;
    fn CMAudioClockCreate(
        allocator: Option<&cf::Allocator>,
        clock_out: *mut Option<arc::R<Clock>>,
    ) -> os::Status;

    fn CMClockGetTime(clock: &Clock) -> cm::Time;
    fn CMClockGetAnchorTime(
        clock: &Clock,
        clock_time_out: &mut cm::Time,
        ref_clock_time_out: &mut cm::Time,
    ) -> os::Status;
    fn CMClockMightDrift(clock: &Clock, other_clock: &Clock) -> bool;
    fn CMClockInvalidate(clock: &mut Clock);
    fn CMClockConvertHostTimeToSystemUnits(host_time: cm::Time) -> u64;
    fn CMClockMakeHostTimeFromSystemUnits(host_time: u64) -> cm::Time;
}

impl Timebase {
    #[doc(alias = "CMTimebaseGetTypeID")]
    pub fn get_type_id() -> cf::TypeId {
        unsafe { CMTimebaseGetTypeID() }
    }

    #[doc(alias = "CMTimebaseCreateWithSourceClock")]
    pub fn with_src_clock_in(
        src_clock: &cm::Clock,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut res = None;
        unsafe {
            CMTimebaseCreateWithSourceClock(allocator, src_clock, &mut res).to_result_unchecked(res)
        }
    }

    /// Creates a timebase driven by the given clock.
    ///
    /// The timebase will initially have rate zero and time zero.
    /// Pass cm::Clock::host_time_clock() for source_clock to have the host time clock drive
    /// this timebase.
    #[doc(alias = "CMTimebaseCreateWithSourceClock")]
    pub fn with_src_clock(src_clock: &cm::Clock) -> Result<arc::R<Self>, os::Status> {
        Self::with_src_clock_in(src_clock, None)
    }

    #[doc(alias = "CMTimebaseCreateWithSourceTimebase")]
    pub fn with_src_timebase_in(
        src_timebase: &cm::Timebase,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut res = None;
        unsafe {
            CMTimebaseCreateWithSourceTimebase(allocator, src_timebase, &mut res)
                .to_result_unchecked(res)
        }
    }

    /// Creates a timebase driven by the given source timebase.
    ///
    /// The timebase will initially have rate zero and time zero.
    #[doc(alias = "CMTimebaseCreateWithSourceTimebase")]
    #[inline]
    pub fn with_src_timebase(src_timebase: &cm::Timebase) -> Result<arc::R<Self>, os::Status> {
        Self::with_src_timebase_in(src_timebase, None)
    }

    /// Returns the immediate source timebase of a timebase.
    ///
    /// Returns None if the timebase actually has a source clock instead of a source timebase.
    #[doc(alias = "CMTimebaseCopySourceTimebase")]
    #[inline]
    pub fn src_timebase(&self) -> Option<arc::R<cm::Timebase>> {
        unsafe { CMTimebaseCopySourceTimebase(self) }
    }

    /// Returns the immediate source clock of a timebase.
    ///
    /// Returns None if the timebase actually has a source timebase instead of a source clock.
    #[doc(alias = "CMTimebaseCopySourceClock")]
    #[inline]
    pub fn src_clock(&self) -> Option<arc::R<cm::Clock>> {
        unsafe { CMTimebaseCopySourceClock(self) }
    }

    /// Returns the immediate source (either timebase or clock) of a timebase.
    #[doc(alias = "CMTimebaseCopySource")]
    #[inline]
    pub fn src(&self) -> arc::R<cm::ClockOrTimebase> {
        unsafe { CMTimebaseCopySource(self) }
    }

    /// Returns the source clock that is the source of all of a timebase's source timebases.
    #[doc(alias = "CMTimebaseCopyUltimateSourceClock")]
    #[inline]
    pub fn ultimate_src_clock(&self) -> arc::R<cm::Clock> {
        unsafe { CMTimebaseCopyUltimateSourceClock(self) }
    }

    /// Sets the source clock of a timebase.
    ///
    /// The timebase will stop receiving timing information from its current source clock or source
    /// timebase, and will begin receiving timing information from the new source clock.  Prior to
    /// the change a cm::Timebase::notification_src_will_change will be posted.  When
    /// the change has completed, a cm::Timebase::notification_src_did_change notification
    /// will be posted.
    #[doc(alias = "CMTimebaseSetSourceClock")]
    #[inline]
    pub fn set_src_clock(&mut self, val: &cm::Clock) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetSourceClock(self, val).result() }
    }

    /// Sets the source timebase of a timebase.
    ///
    /// The timebase will stop receiving timing information from its current source clock or source
    /// timebase, and will begin receiving timing information from the new source timebase.  Prior to
    /// the change a cm::Timebase::notification_src_will_change will be posted.  When
    /// the change has completed, a cm::Timebase::notification_src_did_change notification
    /// will be posted.
    #[doc(alias = "CMTimebaseSetSourceTimebase")]
    #[inline]
    pub fn set_src_timebase(&mut self, val: &cm::Timebase) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetSourceTimebase(self, val).result() }
    }

    #[doc(alias = "kCMTimebaseNotification_SourceWillChange")]
    #[inline]
    pub fn notification_src_will_change() -> &'static cf::NotificationName {
        unsafe { kCMTimebaseNotification_SourceWillChange }
    }

    #[doc(alias = "kCMTimebaseNotification_SourceDidChange")]
    #[inline]
    pub fn notification_src_did_change() -> &'static cf::NotificationName {
        unsafe { kCMTimebaseNotification_SourceDidChange }
    }

    #[doc(alias = "kCMTimebaseNotification_EffectiveRateChanged")]
    #[inline]
    pub fn notification_effective_rate_changed() -> &'static cf::NotificationName {
        unsafe { kCMTimebaseNotification_EffectiveRateChanged }
    }

    #[doc(alias = "kCMTimebaseNotification_TimeJumped")]
    #[inline]
    pub fn notification_time_jumped() -> &'static cf::NotificationName {
        unsafe { kCMTimebaseNotification_TimeJumped }
    }

    #[doc(alias = "kCMTimebaseNotificationKey_EventTime")]
    #[inline]
    pub fn notification_key_event_time() -> &'static cf::String {
        unsafe { kCMTimebaseNotificationKey_EventTime }
    }

    /// Retrieves the current time from a timebase.
    #[doc(alias = "CMTimebaseGetTime")]
    #[inline]
    pub fn time(&self) -> cm::Time {
        unsafe { CMTimebaseGetTime(self) }
    }

    /// Retrieves the current time from a timebase in the specified timescale.
    #[doc(alias = "CMTimebaseGetTimeWithTimeScale")]
    #[inline]
    pub fn time_with_scale(
        &self,
        timescale: cm::TimeScale,
        method: cm::TimeRoundingMethod,
    ) -> cm::Time {
        unsafe { CMTimebaseGetTimeWithTimeScale(self, timescale, method) }
    }

    /// Sets the current time of a timebase.  
    #[doc(alias = "CMTimebaseSetTime")]
    #[inline]
    pub fn set_time(&mut self, val: cm::Time) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetTime(self, val).result() }
    }

    /// Sets the time of a timebase at a particular source time.
    ///
    /// cm::Timebase::time's results will be interpolated from that anchor time.
    #[doc(alias = "CMTimebaseSetAnchorTime")]
    #[inline]
    pub fn set_achor_time(
        &mut self,
        timebase_time: cm::Time,
        immediate_src_time: cm::Time,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetAnchorTime(self, timebase_time, immediate_src_time).result() }
    }

    /// Retrieves the current rate of a timebase.  
    ///
    /// This is the rate relative to its immediate source clock or timebase.  
    /// For example, if a timebase is running at twice the rate of its source, its rate is 2.0.
    #[doc(alias = "CMTimebaseGetRate")]
    #[inline]
    pub fn rate(&self) -> f64 {
        unsafe { CMTimebaseGetRate(self) }
    }

    /// Retrieves the current time and rate of a timebase.
    ///
    /// You can use this function to take a consistent snapshot of the two values,
    /// avoiding possible inconsistencies due to external changes between retrieval of time and rate.
    #[doc(alias = "CMTimebaseGetTimeAndRate")]
    #[inline]
    pub fn time_and_rate(
        &self,
        time_out: &mut cm::Time,
        rate_out: &mut f64,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseGetTimeAndRate(self, time_out, rate_out).result() }
    }

    /// Sets the rate of a timebase.
    #[doc(alias = "CMTimebaseSetRate")]
    #[inline]
    pub fn set_rate(&mut self, val: f64) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetRate(self, val).result() }
    }

    /// Sets the time of a timebase at a particular source time, and changes the rate at exactly that time.
    #[doc(alias = "CMTimebaseSetRateAndAnchorTime")]
    #[inline]
    pub fn set_rate_and_anchor_time(
        &mut self,
        rate: f64,
        timebase_time: cm::Time,
        immediate_src_time: cm::Time,
    ) -> Result<(), os::Status> {
        unsafe {
            CMTimebaseSetRateAndAnchorTime(self, rate, timebase_time, immediate_src_time).result()
        }
    }

    /// Gets the effective rate of a timebase (which combines its rate with the rates of all its source timebases).
    #[doc(alias = "CMTimebaseGetEffectiveRate")]
    #[inline]
    pub fn effective_rate(&self) -> f64 {
        unsafe { CMTimebaseGetEffectiveRate(self) }
    }

    /// Adds the timer to the list of timers managed by the timebase.
    ///
    /// The timer must be a repeating run loop timer (with a very long interval at
    /// least as long as cm::Timebase::VERY_LONG_CF_TIME_INTERVAL), attached to a runloop.  
    /// The timebase will retain the timer, and will maintain its "NextFireDate"
    /// according to the cm::Time set using CMTimebaseSetTimerNextFireTime.
    /// Until the first call to CMTimebaseSetTimerNextFireTime, the "NextFireDate"
    /// will be set far, far in the future. The runloop that timer is attached to must be
    /// passed in and the timebase will retain that runloop. The retained runloop will be
    /// used to call cf::RunLoop::wake_up() any time the timebase modifies the timer's fire date.
    #[doc(alias = "CMTimebaseAddTimer")]
    #[inline]
    pub fn add_timer(
        &mut self,
        timer: &cf::RunLoopTimer,
        run_loop: &cf::RunLoop,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseAddTimer(self, timer, run_loop).result() }
    }

    /// quite a while
    pub const VERY_LONG_CF_TIME_INTERVAL: cf::TimeInterval = 256.0 * 365.0 * 24.0 * 60.0 * 60.0;

    /// quite a while from 2001
    pub const FAR_FUTURE_CF_ABS_TIME: cf::AbsTime = Self::VERY_LONG_CF_TIME_INTERVAL;

    /// Removes the timer from the list of timers managed by the timebase.
    ///
    /// The timebase will no longer maintain the timer's "NextFireDate".
    /// If the timer is invalidated, the timebase will eventually remove it
    /// from its list and release it even if this function is not called.
    #[doc(alias = "CMTimebaseRemoveTimer")]
    #[inline]
    pub fn remove_timer(&mut self, timer: &cf::RunLoopTimer) -> Result<(), os::Status> {
        unsafe { CMTimebaseRemoveTimer(self, timer).result() }
    }

    /// Sets the cm::Time on the timebase's timeline at which the timer should next be fired.
    ///
    /// The timer must be on the list of timers managed by the timebase.
    /// The timebase will continue to update the timer's "NextFireDate" according to time jumps
    /// and effective rate changes.
    /// If fireTime is not numeric, or if the timebase is not moving, the "NextFireDate"
    /// will be set to a date far, far in the future.
    ///
    /// IMPORTANT NOTE: Due to the way that cf::RunLoopTimers are implemented, if a timer passes
    /// through a state in which it is due to fire, it may fire even if its rescheduled before
    /// the runloop runs again.  Clients should take care to avoid temporarily scheduling timers
    /// in the past.  For example, set the timebase's rate or time before you set the timer's
    /// next fire time, if you are doing both at once.  (If setting the timebase's rate or time
    /// might put the timer's fire time in the past, you may need to set the fire time to
    /// cm::Time::invalid() across the timebase change.)
    #[doc(alias = "CMTimebaseSetTimerNextFireTime")]
    #[inline]
    pub fn set_timer_next_fire_time(
        &self,
        timer: &mut cf::RunLoopTimer,
        fire_time: cm::Time,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetTimerNextFireTime(self, timer, fire_time, 0).result() }
    }

    /// Sets the timer to fire immediately once, overriding any previous cm::Timebase::set_timer_next_fire_time call.
    ///
    ///
    #[doc(alias = "CMTimebaseSetTimerToFireImmediately")]
    #[inline]
    pub fn set_timer_to_fire_immediately(
        &self,
        timer: &mut cf::RunLoopTimer,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetTimerToFireImmediately(self, timer).result() }
    }

    /// Adds the timer dispatch source to the list of timers managed by the timebase.
    ///
    ///
    #[doc(alias = "CMTimebaseAddTimerDispatchSource")]
    #[inline]
    pub fn add_timer_dispatch_src(
        &mut self,
        timer_src: &dispatch::TimerSrc,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseAddTimerDispatchSource(self, timer_src).result() }
    }

    /// Removes the timer dispatch source from the list of timers managed by the timebase.
    ///
    /// The timebase will no longer maintain the timer source's start time.
    /// If the timer source is cancelled, the timebase will eventually remove it
    /// from its list and release it even if this function is not called.
    #[doc(alias = "CMTimebaseRemoveTimerDispatchSource")]
    #[inline]
    pub fn remove_timer_dispatch_src(
        &mut self,
        timer_src: &dispatch::TimerSrc,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseRemoveTimerDispatchSource(self, timer_src).result() }
    }

    /// Sets the cm::Time on the timebase's timeline at which the timer dispatch source should next be fired.
    ///
    /// The timer source must be on the list of timers managed by the timebase.
    /// The timebase will continue to update the timer dispatch source's start time
    /// according to time jumps and effective rate changes.
    /// If fireTime is not numeric, or if the timebase is not moving, the start time
    /// will be set to DISPATCH_TIME_FOREVER.
    ///
    /// IMPORTANT NOTE: Due to the way that timer dispatch sources are implemented, if a timer passes
    /// through a state in which it is due to fire, it may fire even if its rescheduled before
    /// the event handler is run.  Clients should take care to avoid temporarily scheduling timers
    /// in the past.  For example, set the timebase's rate or time before you set the timer's
    /// next fire time, if you are doing both at once.  (If setting the timebase's rate or time
    /// might put the timer's fire time in the past, you may need to set the fire time to
    /// kCMTimeInvalid across the timebase change.)
    #[doc(alias = "CMTimebaseSetTimerDispatchSourceNextFireTime")]
    #[inline]
    pub fn set_timer_dispatch_src_next_fire_time(
        &mut self,
        timer_src: &dispatch::TimerSrc,
        fire_time: cm::Time,
    ) -> Result<(), os::Status> {
        unsafe {
            CMTimebaseSetTimerDispatchSourceNextFireTime(self, timer_src, fire_time, 0).result()
        }
    }

    /// Sets the timer dispatch source to fire immediately once, overriding any previous
    /// cm::Timebase::set_timer_dispatch_src_next_fire_time call.
    #[doc(alias = "CMTimebaseSetTimerDispatchSourceToFireImmediately")]
    #[inline]
    pub fn set_timer_dispatch_src_to_fire_immediately(
        &mut self,
        timer_src: &dispatch::TimerSrc,
    ) -> Result<(), os::Status> {
        unsafe { CMTimebaseSetTimerDispatchSourceToFireImmediately(self, timer_src).result() }
    }

    /// Requests that the timebase wait until it is not posting any notifications.
    #[doc(alias = "CMTimebaseNotificationBarrier")]
    #[inline]
    pub fn notification_barrier(&self) -> Result<(), os::Status> {
        unsafe { CMTimebaseNotificationBarrier(self).result() }
    }
}

#[link(name = "CoreMedia", kind = "framework")]
extern "C-unwind" {
    fn CMTimebaseGetTypeID() -> cf::TypeId;

    fn CMTimebaseCreateWithSourceClock(
        allocator: Option<&cf::Allocator>,
        source_clock: &cm::Clock,
        timebase_out: *mut Option<arc::R<cm::Timebase>>,
    ) -> os::Status;

    fn CMTimebaseCreateWithSourceTimebase(
        allocator: Option<&cf::Allocator>,
        source_timebase: &cm::Timebase,
        timebase_out: *mut Option<arc::R<cm::Timebase>>,
    ) -> os::Status;

    fn CMTimebaseCopySourceTimebase(timebase: &cm::Timebase) -> Option<arc::R<cm::Timebase>>;

    fn CMTimebaseCopySourceClock(timebase: &cm::Timebase) -> Option<arc::R<cm::Clock>>;

    fn CMTimebaseCopySource(timebase: &cm::Timebase) -> arc::R<cm::ClockOrTimebase>;

    fn CMTimebaseCopyUltimateSourceClock(timebase: &cm::Timebase) -> arc::R<cm::Clock>;

    fn CMTimebaseSetSourceClock(
        timebase: &mut cm::Timebase,
        new_source_clock: &cm::Clock,
    ) -> os::Status;

    fn CMTimebaseSetSourceTimebase(
        timebase: &mut cm::Timebase,
        new_source_timebase: &cm::Timebase,
    ) -> os::Status;

    static kCMTimebaseNotification_SourceWillChange: &'static cf::NotificationName;
    static kCMTimebaseNotification_SourceDidChange: &'static cf::NotificationName;
    static kCMTimebaseNotification_EffectiveRateChanged: &'static cf::NotificationName;
    static kCMTimebaseNotification_TimeJumped: &'static cf::NotificationName;
    static kCMTimebaseNotificationKey_EventTime: &'static cf::String;

    fn CMTimebaseGetTime(timebase: &cm::Timebase) -> cm::Time;

    fn CMTimebaseGetTimeWithTimeScale(
        timebase: &cm::Timebase,
        timescale: cm::TimeScale,
        method: cm::TimeRoundingMethod,
    ) -> cm::Time;

    fn CMTimebaseSetTime(timebase: &mut cm::Timebase, time: cm::Time) -> os::Status;

    fn CMTimebaseSetAnchorTime(
        timebase: &mut cm::Timebase,
        timebase_time: cm::Time,
        immediate_src_time: cm::Time,
    ) -> os::Status;

    fn CMTimebaseGetRate(timebase: &cm::Timebase) -> f64;

    fn CMTimebaseGetTimeAndRate(
        timebase: &cm::Timebase,
        time_out: *mut cm::Time,
        rate_out: *mut f64,
    ) -> os::Status;

    fn CMTimebaseSetRate(timebase: &mut cm::Timebase, rate: f64) -> os::Status;

    fn CMTimebaseSetRateAndAnchorTime(
        timebase: &mut cm::Timebase,
        rate: f64,
        timebase_time: cm::Time,
        immediate_src_time: cm::Time,
    ) -> os::Status;

    fn CMTimebaseGetEffectiveRate(timebase: &cm::Timebase) -> f64;

    fn CMTimebaseAddTimer(
        timebase: &mut cm::Timebase,
        timer: &cf::RunLoopTimer,
        run_loop: &cf::RunLoop,
    ) -> os::Status;

    fn CMTimebaseRemoveTimer(timebase: &mut cm::Timebase, timer: &cf::RunLoopTimer) -> os::Status;

    fn CMTimebaseSetTimerNextFireTime(
        timebase: &cm::Timebase,
        timer: &mut cf::RunLoopTimer,
        fire_time: cm::Time,
        flags: u32,
    ) -> os::Status;

    fn CMTimebaseSetTimerToFireImmediately(
        timebase: &cm::Timebase,
        timer: &mut cf::RunLoopTimer,
    ) -> os::Status;

    fn CMTimebaseAddTimerDispatchSource(
        timebase: &mut cm::Timebase,
        timer_src: &dispatch::TimerSrc,
    ) -> os::Status;

    fn CMTimebaseRemoveTimerDispatchSource(
        timebase: &mut cm::Timebase,
        timer_src: &dispatch::TimerSrc,
    ) -> os::Status;

    fn CMTimebaseSetTimerDispatchSourceNextFireTime(
        timebase: &mut cm::Timebase,
        timer_src: &dispatch::TimerSrc,
        fire_time: cm::Time,
        flags: u32,
    ) -> os::Status;

    fn CMTimebaseSetTimerDispatchSourceToFireImmediately(
        timebase: &mut cm::Timebase,
        timer_src: &dispatch::TimerSrc,
    ) -> os::Status;

    fn CMTimebaseNotificationBarrier(timebase: &cm::Timebase) -> os::Status;

}

impl ClockOrTimebase {
    /// Queries the relative rate of one timebase or clock relative to another timebase or clock.
    #[doc(alias = "CMSyncGetRelativeRate")]
    #[inline]
    pub fn relative_rate(&self, relative_to: &Self) -> f64 {
        unsafe { CMSyncGetRelativeRate(self, relative_to) }
    }

    /// Queries the relative rate of one timebase or clock relative to another timebase or clock and the times of each timebase or
    /// clock at which the relative rate went into effect.
    #[doc(alias = "CMSyncGetRelativeRateAndAnchorTime")]
    #[inline]
    pub fn relative_rate_and_anchor_time(
        &self,
        relative_to: &Self,
        relative_rate: &mut f64,
        anchor_time: &mut cm::Time,
        relative_to_anchor_time: &mut cm::Time,
    ) -> Result<(), os::Status> {
        unsafe {
            CMSyncGetRelativeRateAndAnchorTime(
                self,
                relative_to,
                relative_rate,
                anchor_time,
                relative_to_anchor_time,
            )
            .result()
        }
    }

    /// Converts a time from one timebase or clock to another timebase or clock.
    ///
    /// If both have a common source, this calculation is performed purely based on the mathematical rates and offsets
    /// in the common tree rooted in that source.  
    /// If they have different source clocks (or are both clocks), this calculation also compensates
    /// for measured drift between the clocks.
    /// To convert to or from host time, pass cm::Clock::host_time_clock() as the appropriate argument.
    #[doc(alias = "CMSyncConvertTime")]
    #[inline]
    pub fn convert_time_to(&self, time: cm::Time, to: &cm::ClockOrTimebase) -> cm::Time {
        unsafe { CMSyncConvertTime(time, self, to) }
    }

    /// Reports whether it is possible for one timebase/clock to drift relative to the other.
    ///
    /// A timebase can drift relative to another if their ultimate source clocks that can drift relative
    /// to each other.
    #[doc(alias = "CMSyncMightDrift")]
    #[inline]
    pub fn might_drift(&self, other: &Self) -> bool {
        unsafe { CMSyncMightDrift(self, other) }
    }

    /// A helper function to get time from a clock or timebase.  
    #[doc(alias = "CMSyncGetTime")]
    #[inline]
    pub fn time(&self) -> cm::Time {
        unsafe { CMSyncGetTime(self) }
    }
}

#[link(name = "CoreMedia", kind = "framework")]
extern "C-unwind" {

    fn CMSyncGetRelativeRate(
        of_clock_or_timebase: &cm::ClockOrTimebase,
        relative_to_clock_or_timebase: &cm::ClockOrTimebase,
    ) -> f64;

    fn CMSyncGetRelativeRateAndAnchorTime(
        of_clock_or_timebase: &cm::ClockOrTimebase,
        relative_to_clock_or_timebase: &cm::ClockOrTimebase,
        out_relative_rate: *mut f64,
        out_of_clock_or_timebase_anchor_time: *mut cm::Time,
        out_relative_to_clock_or_timebase_anchor_time: *mut cm::Time,
    ) -> os::Status;

    fn CMSyncConvertTime(
        time: cm::Time,
        from: &cm::ClockOrTimebase,
        to: &cm::ClockOrTimebase,
    ) -> cm::Time;

    fn CMSyncMightDrift(a: &cm::ClockOrTimebase, b: &cm::ClockOrTimebase) -> bool;

    fn CMSyncGetTime(a: &cm::ClockOrTimebase) -> cm::Time;

}
