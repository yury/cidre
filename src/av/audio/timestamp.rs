// struct AudioTimeStamp
// {
//     Float64             mSampleTime;
//     UInt64              mHostTime;
//     Float64             mRateScalar;
//     UInt64              mWordClockTime;
//     SMPTETime           mSMPTETime;
//     AudioTimeStampFlags mFlags;
//     UInt32              mReserved;
// };

use crate::cv::SMPTETime;

#[repr(C)]
pub struct TimeStamp {
    /// The absolute sample frame time.
    pub sample_time: f64,
    /// The host machine's time base, mach_absolute_time.
    pub host_time: u64,
    /// The ratio of actual host ticks per sample frame to the nominal host ticks
    /// per sample frame.
    pub rate_scalar: f64,
    /// The word clock time.
    pub work_clock_time: u64,
    /// The SMPTE time.
    pub smpte_time: SMPTETime,
    /// A set of flags indicating which representations of the time are valid.
    pub flags: Flags,
    /// Pads the structure out to force an even 8 byte alignment.
    pub reserved: u32
}

#[repr(transparent)]
pub struct Flags(pub u32);

impl Flags {
    pub const NOTHING_VALID: Self = Self(0);
    /// The sample frame time is valid.
    pub const SAMPLE_TIME_VALID: Self = Self(1u32 << 0);
    /// The host time is valid.
    pub const HOST_TIME_VALID: Self = Self(1u32 << 1);
    /// The rate scalar is valid.
    pub const RATE_SCALAR_VALID: Self = Self(1u32 << 2);
    /// The word clock time is valid.
    pub const WORD_CLOCK_TIME_VALID: Self = Self(1u32 << 3);
    /// The SMPTE time is valid.
    pub const SMPTETIME_VALID: Self       = Self(1u32 << 4);
    /// The sample frame time and the host time are valid.
    pub const SAMPLE_HOST_TIME_VALID: Self  = Self(Self::SAMPLE_TIME_VALID.0 | Self::HOST_TIME_VALID.0);
}

// struct AudioClassDescription {
//   OSType  mType;
//   OSType  mSubType;
//   OSType  mManufacturer;
// };
// typedef struct AudioClassDescription    AudioClassDescription;