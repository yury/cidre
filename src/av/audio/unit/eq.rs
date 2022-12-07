use crate::{av::audio, define_obj_type, objc::Id, cf::{Retained, self}};

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

    pub fn filter_type(&self) -> FilterType {
        unsafe {
            rsel_filterType(self)
        }
    }
    pub fn set_filter_type(&mut self, value: FilterType) {
        unsafe {
            wsel_setFilterType(self, value)
        }
    }

    pub fn frequency(&self) -> f32 {
        unsafe {
            rsel_frequency(self)
        }
    }

    pub fn set_frequency(&mut self, value: f32) {
        unsafe {
            wsel_setFrequency(self, value)
        }
    } 

    pub fn bandwidth(&self) -> f32 {
        unsafe {
            rsel_bandwidth(self)
        }
    }

    pub fn set_bandwidth(&mut self, value: f32) {
        unsafe {
            wsel_setBandwidth(self, value)
        }
    }

    pub fn gain(&self) -> f32 {
        unsafe {
            rsel_gain(self)
        }
    }

    pub fn set_gain(&mut self, value: f32) {
        unsafe {
            wsel_setGain(self, value)
        }
    } 
}

define_obj_type!(UnitEq(audio::UnitEffect));

/// UnitEffect that implements a Multi-Band Equalizer.
impl UnitEq {

    /// ```
    /// use cidre::av::audio;
    /// 
    /// let mut equ = audio::UnitEq::with_bands(10);
    /// 
    /// let bands = equ.bands_mut();
    /// bands[0].set_gain(10.0);
    /// assert_eq!(bands.len(), 10);
    /// assert_eq!(equ.global_gain(), 0.0);
    /// 
    /// ```
    pub fn with_bands(number_of_bands: usize) -> Retained<Self> {
        unsafe {
            AVAudioUnitEQ_initWithNumberOfBands(number_of_bands)
        }
    }

    pub fn bands(&self) -> &cf::ArrayOf<FilterParameters> {
        unsafe {
            rsel_bands(self)
        }
    }

    pub fn bands_mut(&mut self) -> &mut cf::ArrayOf<FilterParameters> {
        unsafe {
            rsel_bands(self)
        }
    }

    pub fn global_gain(&self) -> f32 {
        unsafe {
            rsel_globalGain(self)
        }
    }

    pub fn set_global_gain(&mut self, value: f32) {
        unsafe {
            wsel_setGlobalGain(self, value)
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_filterType(id: &Id) -> FilterType;
    fn wsel_setFilterType(id: &Id, value: FilterType);
    fn rsel_frequency(id: &Id) -> f32;
    fn wsel_setFrequency(id: &Id, value: f32);
    fn rsel_bandwidth(id: &Id) -> f32;
    fn wsel_setBandwidth(id: &Id, value: f32);
    fn rsel_gain(id: &Id) -> f32;
    fn wsel_setGain(id: &Id, value: f32); 

    fn AVAudioUnitEQ_initWithNumberOfBands(number_of_bands: usize) -> Retained<UnitEq>;
    fn rsel_bands(id: &Id) -> &mut cf::ArrayOf<FilterParameters>;

    fn rsel_globalGain(id: &Id) -> f32;
    fn wsel_setGlobalGain(id: &Id, value: f32);
}

