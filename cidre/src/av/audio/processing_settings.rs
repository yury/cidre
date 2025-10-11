use crate::{define_obj_type, define_opts, ns};

define_obj_type!(
    #[doc(alias = "AVAudioTimePitchAlgorithm")]
    pub TimePitchAlgorithm(ns::String)
);

impl TimePitchAlgorithm {
    /// Modest quality, less expensive. Suitable for voice.
    ///
    /// Variable rate from 1/32 to 32.
    #[doc(alias = "AVAudioTimePitchAlgorithmTimeDomain")]
    pub fn time_domain() -> &'static Self {
        unsafe { AVAudioTimePitchAlgorithmTimeDomain }
    }

    /// Highest quality, most computationally expensive. Suitable for music.
    ///
    /// Variable rate from 1/32 to 32.
    #[doc(alias = "AVAudioTimePitchAlgorithmSpectral")]
    pub fn spectral() -> &'static Self {
        unsafe { AVAudioTimePitchAlgorithmSpectral }
    }

    /// High quality, no pitch correction. Pitch varies with rate.
    ///
    /// Variable rate from 1/32 to 32.
    #[doc(alias = "AVAudioTimePitchAlgorithmVarispeed")]
    pub fn varispeed() -> &'static Self {
        unsafe { AVAudioTimePitchAlgorithmVarispeed }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVAudioTimePitchAlgorithmTimeDomain: &'static TimePitchAlgorithm;
    static AVAudioTimePitchAlgorithmSpectral: &'static TimePitchAlgorithm;
    static AVAudioTimePitchAlgorithmVarispeed: &'static TimePitchAlgorithm;
}

define_opts!(
    #[doc(alias = "AVAudioSpatializationFormats")]
    pub SpatializationFormats(usize)
);

impl SpatializationFormats {
    pub const NONE: Self = Self(0);
    pub const MONO_AND_STEREO: Self = Self(0x3);
    pub const MULTICHANNEL: Self = Self(0x4);
    pub const MONO_STEREO_AND_MULTICHANNEL: Self = Self(0x7);
}
