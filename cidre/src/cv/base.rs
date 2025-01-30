use crate::define_opts;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Time {
    pub value: i64,
    pub scale: i32,
    pub flags: Flags,
}

impl Time {
    /// Zero time or duration. For example,
    /// CVDisplayLinkGetOutputVideoLatency returns kCVZeroTime for zero video latency.
    #[doc(alias = "kCVZeroTime")]
    #[inline]
    pub fn zero() -> Self {
        unsafe { kCVZeroTime }
    }

    /// An unknown or indefinite time. For example,
    /// CVDisplayLinkGetNominalOutputVideoRefreshPeriod returns kCVIndefiniteTime
    /// if the display link specified is not valid.
    #[doc(alias = "kCVIndefiniteTime")]
    #[inline]
    pub fn indefinite() -> Self {
        unsafe { kCVIndefiniteTime }
    }
}

define_opts!(
    #[doc(alias = "CVTimeFlags")]
    pub Flags(i32)
);

impl Flags {
    pub const NONE: Self = Self(0);
    pub const IS_INDEFINITE: Self = Self(1);
}

#[doc(alias = "CVSmpteTime")]
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct SmpteTime {
    /// The number of subframes in the full message.
    pub subframes: i16,

    /// The number of subframes per frame (typically 80).
    pub subframes_divisor: i16,

    /// The total number of messages received.
    pub counter: u32,

    /// The kind of SMPTE time using the SMPTE time type constants.
    pub r#type: u32,

    /// A set of flags that indicate the SMPTE state.
    pub flags: u32,

    /// The number of hours in the full message.
    pub hours: i16,

    /// The number of minutes in the full message.
    pub minutes: i16,

    /// The number of seconds in the full message.
    pub seconds: i16,

    /// The number of frames in the full message.
    pub frames: i16,
}

#[doc(alias = "CVTimeStamp")]
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct TimeStamp {
    /// The current cv::TimeStamp is version 0
    pub version: u32,

    /// The scale (in units per second) of the videoTime and videoPeriod values
    pub video_time_scale: i32,

    /// This represents the start of a frame (or field for interlaced)
    pub video_time: i64,

    /// Host root timebase time
    pub host_time: u64,

    /// This is the current rate of the device as measured by the timestamps, divided by the nominal rate
    pub rate_scalar: f64,

    /// This is the nominal update period of the current output device
    pub video_refresh_period: i64,

    /// SMPTE time representation of the time stamp.
    pub smpte_time: SmpteTime,

    pub flags: TimeStampFlags,

    /// Reserved. Do not use.
    pub reserved: u64,
}

pub type OptionFlags = u64;

define_opts!(
    #[doc(alias = "CVTimeStampFlags")]
    pub TimeStampFlags(u64)
);

impl TimeStampFlags {
    pub const VIDEO_TIME_VALID: Self = Self(1 << 0);
    pub const HOST_TIME_VALID: Self = Self(1 << 1);
    pub const SMPTE_TIME_VALID: Self = Self(1 << 2);
    pub const VIDEO_REFRESH_PERIOD_VALID: Self = Self(1 << 3);
    pub const RATE_SCALAR_VALID: Self = Self(1 << 4);

    // There are flags for each field to make it easier to detect interlaced vs progressive output
    pub const TOP_FIELD: Self = Self(1 << 16);
    pub const BOTTOM_FIELD: Self = Self(1 << 17);

    // Some commonly used combinations of timestamp flags
    pub const VIDEO_HOST_TIME_VALID: Self =
        Self(Self::VIDEO_TIME_VALID.0 | Self::HOST_TIME_VALID.0);
    pub const IS_INTERLACED: Self = Self(Self::TOP_FIELD.0 | Self::BOTTOM_FIELD.0);
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    static kCVZeroTime: Time;
    static kCVIndefiniteTime: Time;
}
