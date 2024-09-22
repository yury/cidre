use crate::{arc, cf, cm, define_cf_type, os};

define_cf_type!(
    /// A timing source object.
    ///
    /// A clock represents a source of time information: generally, a piece of hardware that measures the passage of time.
    /// One example of a clock is the host time clock, accessible via cm::Clock::get_host_time_clock().
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
    /// This clock will not drift from audio output, but may drift from cm::Clock::get_host_time_clock().
    /// When audio output is completely stopped, the clock continues to advance, tracking cm::Clock::get_host_time_clock()
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
