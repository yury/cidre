use crate::at::au;

/// The following specifies the equivalent parameterID's for the Group scope for standard
/// MIDI Controllers. This list is not exhaustive. It represents the parameters, and their corresponding
/// MIDI messages, that should be supported in Group scope by MIDI capable AUs.
///
/// Group scope parameter IDs from 0 < 512 are reserved for mapping MIDI controllers.
impl au::ParamId {
    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Volume")]
    pub const VOLUME: Self = Self(7);

    /// value 0-63 (off), 64-127 (on)
    #[doc(alias = "kAUGroupParameterID_Sustain")]
    pub const SUSTAIN: Self = Self(64);

    /// value 0-63 (off), 64-127 (on)
    #[doc(alias = "kAUGroupParameterID_Sostenuto")]
    pub const SOSTENUTO: Self = Self(66);

    /// value ignored
    #[doc(alias = "kAUGroupParameterID_AllNotesOff")]
    pub const ALL_NOTES_OFF: Self = Self(123);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_ModWheel")]
    pub const MOD_WHEEL: Self = Self(1);

    /// value -8192 - 8191
    #[doc(alias = "kAUGroupParameterID_PitchBend")]
    pub const PITCH_BEND: Self = Self(0xE0);

    /// value ignored
    #[doc(alias = "kAUGroupParameterID_AllSoundOff")]
    pub const ALL_SOUND_OFF: Self = Self(120);

    /// value ignored
    #[doc(alias = "kAUGroupParameterID_ResetAllControllers")]
    pub const RESET_ALL_CONTROLLERS: Self = Self(121);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Pan")]
    pub const PAN: Self = Self(10);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Foot")]
    pub const FOOT: Self = Self(4);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_ChannelPressure")]
    pub const CHANNEL_PRESSURE: Self = Self(0xD0);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_KeyPressure")]
    pub const KEY_PRESSURE: Self = Self(0xA0);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Expression")]
    pub const EXPRESSION: Self = Self(11);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_DataEntry")]
    pub const DATA_ENTRY: Self = Self(6);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Volume_LSB")]
    pub const VOLUME_LSB: Self = Self(Self::VOLUME.0 + 32);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_ModWheel_LSB")]
    pub const MOD_WHEEL_LSB: Self = Self(Self::MOD_WHEEL.0 + 32);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Pan_LSB")]
    pub const PAN_LSB: Self = Self(Self::PAN.0 + 32);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Foot_LSB")]
    pub const FOOT_LSB: Self = Self(Self::FOOT.0 + 32);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_Expression_LSB")]
    pub const EXPRESSION_LSB: Self = Self(Self::EXPRESSION.0 + 32);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_DataEntry_LSB")]
    pub const DATA_ENTRY_LSB: Self = Self(Self::DATA_ENTRY.0 + 32);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_KeyPressure_FirstKey")]
    pub const KEY_PRESSURE_FIRST_KEY: Self = Self(256);

    /// value 0 < 128
    #[doc(alias = "kAUGroupParameterID_KeyPressure_LastKey")]
    pub const KEY_PRESSURE_LAST_KEY: Self = Self(383);
}

/// Parameters for all Panner AudioUnits
#[cfg(target_os = "macos")]
impl au::ParamId {
    /// Global, Linear, 0->1, 1
    #[doc(alias = "kPannerParam_Gain")]
    pub const GAIN: Self = Self(0);

    /// Global, Degrees, -180->180, 0
    #[doc(alias = "kPannerParam_Azimuth")]
    pub const AZIMUTH: Self = Self(1);

    /// Global, Degrees, -90->90, 0
    #[doc(alias = "kPannerParam_Elevation")]
    pub const ELEVATION: Self = Self(2);

    /// Global, Linear, 0->1, 1
    #[doc(alias = "kPannerParam_Distance")]
    pub const DISTANCE: Self = Self(3);

    /// Global, Meters, 0.01->1000, 1
    #[doc(alias = "kPannerParam_CoordScale")]
    pub const COORD_SCALE: Self = Self(4);

    /// Global, Meters, 0.01->1000, 1
    #[doc(alias = "kPannerParam_RefDistance")]
    pub const REF_DISTANCE: Self = Self(5);
}

/// Parameters for the AUSpatialMixer unit
impl au::ParamId {
    /// Input, Degrees, -180->180, 0
    #[doc(alias = "kSpatialMixerParam_Azimuth")]
    pub const SPATIAL_MIXER_AZIMUTH: Self = Self(0);

    /// Input, Degrees, -90->90, 0
    #[doc(alias = "kSpatialMixerParam_Elevation")]
    pub const SPATIAL_MIXER_ELEVATION: Self = Self(1);

    /// Input, Metres, 0->10000, 0
    #[doc(alias = "kSpatialMixerParam_Distance")]
    pub const SPATIAL_MIXER_DISTANCE: Self = Self(2);

    /// Input/Output, dB, -120->20, 0
    #[doc(alias = "kSpatialMixerParam_Gain")]
    pub const SPATIAL_MIXER_GAIN: Self = Self(3);

    /// Input, rate scaler	0.5 -> 2.0, 1.0
    #[doc(alias = "kSpatialMixerParam_PlaybackRate")]
    pub const SPATIAL_MIXER_PLAYBACK_RATE: Self = Self(4);

    /// bus enable : 0 or 1, 1
    #[doc(alias = "kSpatialMixerParam_Enable")]
    pub const SPATIAL_MIXER_ENABLE: Self = Self(5);

    /// Minimum input gain constraint : 0.0 -> 10.0, 0.0
    #[doc(alias = "kSpatialMixerParam_MinGain")]
    pub const SPATIAL_MIXER_MIN_GAIN: Self = Self(6);

    /// Maximum input gain constraint : 0.0 -> 10.0, 10.0
    #[doc(alias = "kSpatialMixerParam_MaxGain")]
    pub const SPATIAL_MIXER_MAX_GAIN: Self = Self(7);

    /// Input, Dry/Wet equal-power blend, %	  0.0 -> 100.0, 30.0
    #[doc(alias = "kSpatialMixerParam_ReverbBlend")]
    pub const SPATIAL_MIXER_REVERB_BLEND: Self = Self(8);

    /// Global, dB, -40.0 -> +40.0, 0.0
    #[doc(alias = "kSpatialMixerParam_GlobalReverbGain")]
    pub const SPATIAL_MIXER_GLOBAL_REVERB_GAIN: Self = Self(9);

    /// Input, Lowpass filter attenuation at 5KHz :		decibels -100.0dB -> 0.0dB, 0.0dB
    /// smaller values make both direct and reverb sound more muffled; a value of 0.0 indicates no filtering
    /// Occlusion is a filter applied to the sound prior to the reverb send
    #[doc(alias = "kSpatialMixerParam_OcclusionAttenuation")]
    pub const SPATIAL_MIXER_OCCLUSION_ATTENUATION: Self = Self(10);

    /// Input, Lowpass filter attenuation at 5KHz :		decibels -100.0dB -> 0.0dB, 0.0dB
    /// smaller values make direct sound more muffled; a value of 0.0 indicates no filtering
    /// Obstruction is a filter applied to the "direct" part of the sound (so is post reverb send)
    #[doc(alias = "kSpatialMixerParam_ObstructionAttenuation")]
    pub const SPATIAL_MIXER_OBSTRUCTION_ATTENUATION: Self = Self(11);

    /// Global, Degrees, -180->180, 0
    #[doc(alias = "kSpatialMixerParam_HeadYaw")]
    pub const SPATIAL_MIXER_HEAD_YAW: Self = Self(19);

    /// Global, Degrees, -90->90, 0
    #[doc(alias = "kSpatialMixerParam_HeadPitch")]
    pub const SPATIAL_MIXER_HEAD_PITCH: Self = Self(20);

    /// Global, Degrees, -180->180, 0
    #[doc(alias = "kSpatialMixerParam_HeadRoll")]
    pub const SPATIAL_MIXER_HEAD_ROLL: Self = Self(2);
}

/// Parameters for the AUMultiChannelMixer unit
/// these are available for both desktop and iphone
impl au::ParamId {
    /// Global, Linear Gain, 0->1, 1. (the volume value can actually be any finite number, including negative.)
    #[doc(alias = "kMultiChannelMixerParam_Volume")]
    pub const MULTI_CHANNEL_MIXER_VOLUME: Self = Self(0);

    /// Global, Boolean, 0->1, 1
    #[doc(alias = "kMultiChannelMixerParam_Enable")]
    pub const MULTI_CHANNEL_MIXER_ENABLE: Self = Self(1);

    /// Global, Pan, -1->1, 0
    ///
    /// -1 - 0 - 1, only valid when output is not mono
    /// setting kAudioUnitProperty_MatrixLevels overrides any
    /// previously set kMultiChannelMixerParam_Pan and vice versa
    #[doc(alias = "kMultiChannelMixerParam_Pan")]
    pub const MULTI_CHANNEL_MIXER_PAN: Self = Self(2);

    // read-only, Input or Output scope.
    // these report level in dB, as do the other mixers
    #[doc(alias = "kMultiChannelMixerParam_PreAveragePower")]
    pub const MULTI_CHANNEL_MIXER_PRE_AVERAGE_POWER: Self = Self(100);

    #[doc(alias = "kMultiChannelMixerParam_PrePeakHoldLevel")]
    pub const MULTI_CHANNEL_MIXER_PRE_PEAK_HOLD_LEVEL: Self = Self(200);

    #[doc(alias = "kMultiChannelMixerParam_PostAveragePower")]
    pub const MULTI_CHANNEL_MIXER_POST_AVERAGE_POWER: Self = Self(300);

    #[doc(alias = "kMultiChannelMixerParam_PostPeakHoldLevel")]
    pub const MULTI_CHANNEL_MIXER_POST_PEAK_HOLD_LEVEL: Self = Self(400);
}

/// Parameters for the AUMatrixMixer unit
// CF_ENUM(AudioUnitParameterID) {
impl au::ParamId {
    #[doc(alias = "kMatrixMixerParam_Volume")]
    pub const MATRIX_MIXER_VOLUME: Self = Self(0);

    #[doc(alias = "kMatrixMixerParam_Enable")]
    pub const MATRIX_MIXER_ENABLE: Self = Self(1);

    // read-only
    // these report level in dB, as do the other mixers
    #[doc(alias = "kMatrixMixerParam_PreAveragePower")]
    pub const MATRIX_MIXER_PRE_AVERAGE_POWER: Self = Self(1000);

    #[doc(alias = "kMatrixMixerParam_PrePeakHoldLevel")]
    pub const MATRIX_MIXER_PRE_PEAK_HOLD_LEVEL: Self = Self(2000);

    #[doc(alias = "kMatrixMixerParam_PostAveragePower")]
    pub const MATRIX_MIXER_POST_AVERAGE_POWER: Self = Self(3000);

    #[doc(alias = "kMatrixMixerParam_PostPeakHoldLevel")]
    pub const MATRIX_MIXER_POST_PEAK_HOLD_LEVEL: Self = Self(4000);

    // these report linear levels - for "expert" use only.
    #[doc(alias = "kMatrixMixerParam_PreAveragePowerLinear")]
    pub const MATRIX_MIXER_PRE_AVERAGE_POWER_LINEAR: Self = Self(5000);

    #[doc(alias = "kMatrixMixerParam_PrePeakHoldLevelLinear")]
    pub const MATRIX_MIXER_PRE_PEAK_HOLD_LEVEL_LINEAR: Self = Self(6000);

    #[doc(alias = "kMatrixMixerParam_PostAveragePowerLinear")]
    pub const MATRIX_MIXER_POST_AVERAGE_POWER_LINEAR: Self = Self(7000);

    #[doc(alias = "kMatrixMixerParam_PostPeakHoldLevelLinear")]
    pub const MATRIX_MIXER_POST_PEAK_HOLD_LEVEL_LINEAR: Self = Self(800);
}

/// Output Units
/// Parameters for the AudioDeviceOutput, DefaultOutputUnit, and SystemOutputUnit units
impl au::ParamId {
    // Global, LinearGain, 0->1, 1
    pub const HAL_OUTPUT_VOLUME: Self = Self(14);
}

/// Parameters for the AUTimePitch, AUTimePitch (offline), AUPitch units
impl au::ParamId {
    #[doc(alias = "kTimePitchParam_Rate")]
    pub const TIME_PITCH_RATE: Self = Self(0);

    #[doc(alias = "kTimePitchParam_Pitch")]
    pub const TIME_PITCH_PITCH: Self = Self(1);

    /// only for the AUPitch unit
    #[doc(alias = "kTimePitchParam_EffectBlend")]
    pub const TIME_PITCH_EFFECT_BLEND: Self = Self(2);
}

/// Parameters for AUNewTimePitch
impl au::ParamId {
    /// rate control.
    /// Global, rate, 1/32 -> 32.0, 1.0
    #[doc(alias = "kNewTimePitchParam_Rate")]
    pub const NEW_TIME_PITCH_RATE: Self = Self(0);

    /// pitch shift in cents.
    /// Global, Cents, -2400 -> 2400, 1.0
    #[doc(alias = "kNewTimePitchParam_Pitch")]
    pub const NEW_TIME_PITCH_PITCH: Self = Self(1);

    /// The generated output can be made to sound smoother by increasing
    /// the density of the processing time frames. The value is directly proportional
    /// to the CPU cost. When slowing down percussive audio, lower values may be better.
    /// Global, generic, 3.0 -> 32.0, 8.0
    #[doc(alias = "kNewTimePitchParam_Smoothness")]
    #[doc(alias = "kNewTimePitchParam_Overlap")]
    pub const NEW_TIME_PITCH_SMOOTHNESS: Self = Self(4);

    /// Spectral phase coherence is enabled through peak locking.
    /// This adds some computation cost but results in a less "phasey"
    /// or reverberant sound.
    /// Global, Boolean, 0->1, 1
    #[doc(alias = "kNewTimePitchParam_EnableSpectralCoherence")]
    #[doc(alias = "kNewTimePitchParam_EnablePeakLocking")]
    pub const NEW_TIME_PITCH_ENABLE_SPECTRAL_COHERENCE: Self = Self(6);

    /// Transient preservation uses group delay to identify transients.
    /// It resets the phase at points of transients to preserve the original
    /// spectral phase relationships. It also sets the stretch factor to 1 at
    /// those points.
    /// Global, Boolean, 0->1, 1
    #[doc(alias = "kNewTimePitchParam_EnableTransientPreservation")]
    pub const NEW_TIME_PITCH_ENABLE_TRANSIENT_PRESERVATION: Self = Self(7);
}

/// Parameters for the AUSampler unit
impl au::ParamId {
    /// Global, dB, -90->12, 0
    #[doc(alias = "kAUSamplerParam_Gain")]
    pub const AU_SAMPLER_GAIN: Self = Self(900);

    /// Global, Semitones, -24->24, 0
    #[doc(alias = "kAUSamplerParam_CoarseTuning")]
    pub const AU_SAMPLER_COARSE_TUNING: Self = Self(901);

    /// Global, Cents, -99->99, 0
    #[doc(alias = "kAUSamplerParam_FineTuning")]
    pub const AU_SAMPLER_FINE_TUNING: Self = Self(902);

    /// Global, -1.0->1.0, 0
    #[doc(alias = "kAUSamplerParam_Pan")]
    pub const AU_SAMPLER_PAN: Self = Self(903);
}
