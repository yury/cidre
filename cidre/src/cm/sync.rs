use crate::{arc, cf, cm, define_cf_type, os};

define_cf_type!(Clock(cf::Type));
define_cf_type!(Timebase(cf::Type));
define_cf_type!(ClockOrTimebase(cf::Type));

impl Clock {
    pub fn get_type_id() -> cf::TypeId {
        unsafe { CMClockGetTypeID() }
    }

    /// Returns a reference to the singleton clock logically identified with host time.
    #[inline]
    pub fn host_time_clock() -> &'static Clock {
        unsafe { CMClockGetHostTimeClock() }
    }

    /// Use `audio_clock`
    #[inline]
    pub unsafe fn audio_clock_create_in(
        allocator: Option<&cf::Allocator>,
        clock_out: *mut Option<arc::R<Clock>>,
    ) -> os::Status {
        CMAudioClockCreate(allocator, clock_out)
    }

    /// Creates a clock that advances at the same rate as audio output.
    ///
    /// This clock will not drift from audio output, but may drift from CMClockGetHostTimeClock().
    /// When audio output is completely stopped, the clock continues to advance, tracking CMClockGetHostTimeClock()
    /// until audio output starts up again.
    /// This clock is suitable for use as AVPlayer.masterClock when synchronizing video-only playback
    /// with audio played through other APIs or objects.
    #[inline]
    pub fn audio_clock() -> Result<arc::R<Clock>, os::Status> {
        let mut clock_out = None;
        unsafe {
            let res = Self::audio_clock_create_in(None, &mut clock_out);
            res.to_result_unchecked(clock_out)
        }
    }

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
    #[inline]
    pub fn convert_host_time_to_sys_units(host_time: cm::Time) -> u64 {
        unsafe { CMClockConvertHostTimeToSystemUnits(host_time) }
    }

    /// Converts a host time from native units to cm::Time.
    #[inline]
    pub fn make_host_time_from_sys_units(host_time: u64) -> cm::Time {
        unsafe { CMClockMakeHostTimeFromSystemUnits(host_time) }
    }
}

extern "C" {
    fn CMClockGetTypeID() -> cf::TypeId;
    fn CMClockGetHostTimeClock() -> &'static Clock;
    fn CMAudioClockCreate(
        allocator: Option<&cf::Allocator>,
        clock_out: *mut Option<arc::R<Clock>>,
    ) -> os::Status;

    fn CMClockGetTime(clock: &Clock) -> cm::Time;
    fn CMClockConvertHostTimeToSystemUnits(host_time: cm::Time) -> u64;
    fn CMClockMakeHostTimeFromSystemUnits(host_time: u64) -> cm::Time;
}
