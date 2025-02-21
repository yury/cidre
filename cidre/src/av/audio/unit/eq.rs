use crate::{arc, av::audio, define_cls, define_obj_type, ns, objc};

define_obj_type!(pub Eq(audio::UnitEffect));

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

define_obj_type!(pub FilterParameters(ns::Id));

impl FilterParameters {
    #[objc::msg_send(filterType)]
    pub fn filter_type(&self) -> FilterType;

    #[objc::msg_send(setFilterType:)]
    pub fn set_filter_type(&mut self, value: FilterType);

    #[objc::msg_send(frequency)]
    pub fn frequency(&self) -> f32;

    #[objc::msg_send(setFrequency:)]
    pub fn set_frequency(&mut self, value: f32);

    #[objc::msg_send(bandwidth)]
    pub fn bandwidth(&self) -> f32;

    #[objc::msg_send(setBandwidth:)]
    pub fn set_bandwidth(&mut self, value: f32);

    #[objc::msg_send(gain)]
    pub fn gain(&self) -> f32;

    #[objc::msg_send(setGain:)]
    pub fn set_gain(&mut self, value: f32);
}

define_obj_type!(pub UnitEq(audio::UnitEffect));

impl arc::A<UnitEq> {
    #[objc::msg_send(initWithNumberOfBands:)]
    pub fn init_with_number_of_bands(self, number_of_bands: usize) -> arc::R<UnitEq>;
}

/// UnitEffect that implements a Multi-Band Equalizer.
impl UnitEq {
    define_cls!(AV_AUDIO_UNIT_EQ);

    pub fn with_bands(number_of_bands: usize) -> arc::R<Self> {
        Self::alloc().init_with_number_of_bands(number_of_bands)
    }

    #[objc::msg_send(bands)]
    pub fn bands(&self) -> arc::R<ns::Array<FilterParameters>>;

    #[objc::msg_send(globalGain)]
    pub fn global_gain(&self) -> f32;

    #[objc::msg_send(setGlobalGain:)]
    pub fn set_global_gain(&mut self, value: f32);
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_AUDIO_UNIT_EQ: &'static objc::Class<UnitEq>;
}

#[cfg(test)]
mod tests {
    use crate::av::audio;

    #[test]
    fn basics() {
        let equ = audio::UnitEq::with_bands(10);

        let bands = equ.bands();
        bands.get(0).unwrap().set_gain(10.0);
        assert_eq!(bands.len(), 10);
        assert_eq!(equ.global_gain(), 0.0);
    }
}
