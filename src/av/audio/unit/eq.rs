use crate::{at, av::audio, cf, define_obj_type, objc::Id};

define_obj_type!(Eq(audio::UnitEffect));

/// Filter types available to use with EQ.
#[repr(isize)]
pub enum FilterType {
    /// Parametric filter based on Butterworth analog prototype.
    /// Required parameters: frequency (center), bandwidth, gain
    Parametric = 0,

    /// Simple Butterworth 2nd order low pass filter
    /// Required parameters: frequency (-3 dB cutoff at specified frequency)
    LowPass = 1,

    /// Simple Butterworth 2nd order high pass filter
    /// Required parameters: frequency (-3 dB cutoff at specified frequency)
    HighPass = 2,

    /// Low pass filter with resonance support (via bandwidth parameter)
    ///  Required parameters: frequency (-3 dB cutoff at specified frequency), bandwidth
    ResonantLowPass = 3,

    /// High pass filter with resonance support (via bandwidth parameter)
    /// Required parameters: frequency (-3 dB cutoff at specified frequency), bandwidth
    ResonantHighPass = 4,

    /// Band pass filter
    /// Required parameters: frequency (center), bandwidth
    BandPass = 5,

    /// Band stop filter (aka "notch filter")
    /// Required parameters: frequency (center), bandwidth
    BandStop = 6,

    /// Low shelf filter
    /// Required parameters: frequency (center), gain
    LowShelf = 7,

    /// High shelf filter
    /// Required parameters: frequency (center), gain
    HighShelf = 8,

    /// Low shelf filter with resonance support (via bandwidth parameter)
    /// Required parameters: frequency (center), bandwidth, gain
    ResonantLowShelf = 9,

    /// High shelf filter with resonance support (via bandwidth parameter)
    /// Required parameters: frequency (center), bandwidth, gain
    ResonantHighShelf = 10,
}


define_obj_type!(FilterParameters(Id));

impl FilterParameters {

}

define_obj_type!(UnitEq(audio::UnitEffect));

