#[cfg(not(target_os = "macos"))]
use crate::{arc, cf, cm, os};

#[cfg(not(target_os = "macos"))]
impl cm::Clock {
    /// Use `with_audio_default_output`
    #[doc(alias = "CMAudioClockCreate")]
    #[inline]
    pub unsafe fn with_default_audio_output_in(
        clock_out: *mut Option<arc::R<Self>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result {
        unsafe { CMAudioClockCreate(allocator, clock_out).result() }
    }

    /// Creates a clock that advances at the same rate as audio output.
    ///
    /// This clock will not drift from audio output, but may drift from cm::Clock::host_time_clock().
    /// When audio output is completely stopped, the clock continues to advance, tracking cm::Clock::host_time_clock()
    /// until audio output starts up again.
    /// This clock is suitable for use as av::Player.master_clock when synchronizing video-only playback
    /// with audio played through other APIs or objects.
    #[inline]
    pub fn with_default_audio_output() -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| Self::with_default_audio_output_in(res, None)) }
    }
}

#[cfg(not(target_os = "macos"))]
#[link(name = "CoreMedia", kind = "framework")]
unsafe extern "C-unwind" {
    fn CMAudioClockCreate(
        allocator: Option<&cf::Allocator>,
        clock_out: *mut Option<arc::R<cm::Clock>>,
    ) -> os::Status;
}
