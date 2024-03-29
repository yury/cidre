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

/// Parameters for the AUBandpass unit
impl au::ParamId {
    /// Global, Hz, 20->(SampleRate/2), 5000
    #[doc(alias = "kBandpassParam_CenterFrequency")]
    pub const BANDPASS_CENTER_FREQUENCY: Self = Self(0);

    /// Global, Cents, 100->12000, 600
    #[doc(alias = "kBandpassParam_Bandwidth")]
    pub const BANDPASS_BANDWIDTH: Self = Self(1);
}

/// Parameters for the AUHipass unit
impl au::ParamId {
    /// Global, Hz, 10->(SampleRate/2), 6900
    #[doc(alias = "kHipassParam_CutoffFrequency")]
    pub const HI_PASS_CUTOFF_FREQUENCY: Self = Self(0);

    /// Global, dB, -20->40, 0
    #[doc(alias = "kHipassParam_Resonance")]
    pub const HI_PASS_RESONANCE: Self = Self(1);
}

/// Parameters for the AULowpass unit
impl au::ParamId {
    // Global, Hz, 10->(SampleRate/2), 6900
    #[doc(alias = "kLowPassParam_CutoffFrequency")]
    pub const LOW_PASS_CUTOFF_FREQUENCY: Self = Self(0);

    // Global, dB, -20->40, 0
    #[doc(alias = "kLowPassParam_Resonance")]
    pub const LOW_PASS_RESONANCE: Self = Self(1);
}

/// Parameters for the AUHighShelfFilter unit
impl au::ParamId {
    /// Global, Hz, 10000->(SampleRate/2), 10000
    pub const HIGH_SHELF_CUT_OFF_FREQUENCY: Self = Self(0);

    /// Global, dB, -40->40, 0
    pub const HIGH_SHELF_GAIN: Self = Self(1);
}

/// Parameters for the AULowShelfFilter unit
impl au::ParamId {
    // Global, Hz, 10->200, 80
    pub const LOW_SHELF_CUTOFF_FREQUENCY: Self = Self(0);

    // Global, dB, -40->40, 0
    pub const LOW_SHELF_GAIN: Self = Self(1);
}

/// Parameters for the AUParametricEQ unit
impl au::ParamId {
    /// Global, Hz, 20->(SampleRate/2), 2000
    pub const PARAMETRIC_EQ_CENTER_FREQ: Self = Self(0);

    /// Global, Hz, 0.1->20, 1.0
    pub const PARAMETRIC_EQ_Q: Self = Self(1);

    /// Global, dB, -20->20, 0
    pub const PARAMETRIC_EQ_GAIN: Self = Self(2);
}

/// Parameters for the AUPeakLimiter unit
impl au::ParamId {
    /// Global, Secs, 0.001->0.03, 0.012
    pub const LIMITER_ATTACK_TIME: Self = Self(0);

    /// Global, Secs, 0.001->0.06, 0.024
    pub const LIMITER_DECAY_TIME: Self = Self(1);

    /// Global, dB, -40->40, 0
    pub const LIMITER_PRE_GAIN: Self = Self(2);
}

/// Parameters for the AUDynamicsProcessor unit
/// Note that the dynamics processor does not have fixed compression ratios.
/// Instead, kDynamicsProcessorParam_HeadRoom adjusts the amount of compression.
/// Lower kDynamicsProcessorParam_HeadRoom values results in higher compression.
/// The compression ratio is automatically adjusted to not exceed kDynamicsProcessorParam_Threshold + kDynamicsProcessorParam_HeadRoom values.  
impl au::ParamId {
    /// Global, dB, -40->20, -20
    #[doc(alias = "kDynamicsProcessorParam_Threshold")]
    pub const DYNAMICS_PROCESSOR_THRESHOLD: Self = Self(0);

    /// Global, dB, 0.1->40.0, 5
    #[doc(alias = "kDynamicsProcessorParam_HeadRoom")]
    pub const DYNAMICS_PROCESSOR_HEAD_ROOM: Self = Self(1);

    /// Global, rate, 1->50.0, 2
    #[doc(alias = "kDynamicsProcessorParam_ExpansionRatio")]
    pub const DYNAMICS_PROCESSOR_EXPANSION_RATIO: Self = Self(2);

    /// Global, dB
    #[doc(alias = "kDynamicsProcessorParam_ExpansionThreshold")]
    pub const DYNAMICS_PROCESSOR_EXPANSION_THRESHOLD: Self = Self(3);

    /// Global, secs, 0.0001->0.2, 0.001
    #[doc(alias = "kDynamicsProcessorParam_AttackTime")]
    pub const DYNAMICS_PROCESSOR_ATTACK_TIME: Self = Self(4);

    /// Global, secs, 0.01->3, 0.05
    #[doc(alias = "kDynamicsProcessorParam_ReleaseTime")]
    pub const DYNAMICS_PROCESSOR_RELEASE_TIME: Self = Self(5);

    /// Global, dB, -40->40, 0
    #[doc(alias = "kDynamicsProcessorParam_OverallGain")]
    #[doc(alias = "kDynamicsProcessorParam_MasterGain")]
    pub const DYNAMICS_PROCESSOR_OVERALL_GAIN: Self = Self(6);

    /// Global, dB, read-only parameter
    #[doc(alias = "kDynamicsProcessorParam_CompressionAmount")]
    pub const DYNAMICS_PROCESSOR_COMPRESSION_AMOUNT: Self = Self(1000);

    #[doc(alias = "kDynamicsProcessorParam_InputAmplitude")]
    pub const DYNAMICS_PROCESSOR_INPUT_AMPLITUDE: Self = Self(2000);

    #[doc(alias = "kDynamicsProcessorParam_OutputAmplitude")]
    pub const DYNAMICS_PROCESSOR_OUTPUT_AMPLITUDE: Self = Self(3000);
}

/// Parameters for the AUVarispeed unit
impl au::ParamId {
    /// Global, Rate, 0.25 -> 4.0, 1.0
    #[doc(alias = "kVarispeedParam_PlaybackRate")]
    pub const VARISPEED_PLAYBACK_RATE: Self = Self(0);

    /// Global, Cents, -2400 -> 2400, 0.0
    #[doc(alias = "kVarispeedParam_PlaybackCents")]
    pub const VARISPEED_PLAYBACK_CENTS: Self = Self(1);
}

/// Parameters for the Distortion unit
impl au::ParamId {
    /// Global, Milliseconds, 0.1 -> 500, 0.1
    #[doc(alias = "kDistortionParam_Delay")]
    pub const DISTORTION_DELAY: Self = Self(0);

    /// Global, Rate, 0.1 -> 50, 1.0
    #[doc(alias = "kDistortionParam_Decay")]
    pub const DISTORTION_DECAY: Self = Self(1);

    /// Global, Percent, 0 -> 100, 50
    #[doc(alias = "kDistortionParam_DelayMix")]
    pub const DISTORTION_DELAY_MIX: Self = Self(2);

    /// Global, Percent, 0 -> 100
    #[doc(alias = "kDistortionParam_Decimation")]
    pub const DISTORTION_DECIMATION: Self = Self(3);

    /// Global, Percent, 0 -> 100, 0
    #[doc(alias = "kDistortionParam_Rounding")]
    pub const DISTORTION_ROUNDING: Self = Self(4);

    /// Global, Percent, 0 -> 100, 50
    #[doc(alias = "kDistortionParam_DecimationMix")]
    pub const DISTORTION_DECIMATION_MIX: Self = Self(5);

    /// Global, Linear Gain, 0 -> 1, 1
    #[doc(alias = "kDistortionParam_LinearTerm")]
    pub const DISTORTION_LINEAR_TERM: Self = Self(6);

    /// Global, Linear Gain, 0 -> 20, 0
    #[doc(alias = "kDistortionParam_SquaredTerm")]
    pub const DISTORTION_SQUARED_TERM: Self = Self(7);

    /// Global, Linear Gain, 0 -> 20, 0
    #[doc(alias = "kDistortionParam_CubicTerm")]
    pub const DISTORTION_CUBIC_TERM: Self = Self(8);

    /// Global, Percent, 0 -> 100, 50
    #[doc(alias = "kDistortionParam_PolynomialMix")]
    pub const DISTORTION_POLYNOMIAL_MIX: Self = Self(9);

    /// Global, Hertz, 0.5 -> 8000, 100
    #[doc(alias = "kDistortionParam_RingModFreq1")]
    pub const DISTORTION_RING_MOD_FREQ1: Self = Self(10);

    /// Global, Hertz, 0.5 -> 8000, 100
    #[doc(alias = "kDistortionParam_RingModFreq2")]
    pub const DISTORTION_RING_MOD_FREQ2: Self = Self(11);

    /// Global, Percent, 0 -> 100, 50
    #[doc(alias = "kDistortionParam_RingModBalance")]
    pub const DISTORTION_RING_MOD_BALANCE: Self = Self(12);

    /// Global, Percent, 0 -> 100, 0
    #[doc(alias = "kDistortionParam_RingModMix")]
    pub const DISTORTION_RING_MOD_MIX: Self = Self(13);

    /// Global, dB, -80 -> 20, -6
    #[doc(alias = "kDistortionParam_SoftClipGain")]
    pub const DISTORTION_SOFT_CLIP_GAIN: Self = Self(14);

    /// Global, Percent, 0 -> 100, 50
    #[doc(alias = "kDistortionParam_FinalMix")]
    pub const DISTORTION_FINAL_MIX: Self = Self(15);
}

/// Parameters for the AUDelay unit
impl au::ParamId {
    /// Global, EqPow Crossfade, 0->100, 50
    pub const DELAY_WET_DRY_MIX: Self = Self(0);

    /// Global, Secs, 0->2, 1
    pub const DELAY_DELAY_TIME: Self = Self(1);

    /// Global, Percent, -100->100, 50
    pub const DELAY_FEEDBACK: Self = Self(2);

    /// Global, Hz, 10->(SampleRate/2), 15000
    pub const DELAY_LOPASS_CUTOFF: Self = Self(3);
}

/// Parameters for the AUSampleDelay unit
impl au::ParamId {
    /// Global, Samples, 0->(SampleRate), 0
    #[doc(alias = "kSampleDelayParam_DelayFrames")]
    pub const SAMPLE_DELAY_FRAMES: Self = Self(0);
}

/// Parameters for the AUNBandEQ unit
///
/// Note that the parameter IDs listed correspond to band 0 (zero) of the unit. The parameter IDs for
/// higher bands can be obtained by adding the zero-indexed band number to the corresponding band 0
/// parameter ID up to the number of bands minus one, where the number of bands is described by the
/// AUNBandEQ property kAUNBandEQProperty_NumberOfBands. For example, the parameter ID corresponding
/// to the filter type of band 4 would be kAUNBandEQParam_FilterType + 3.
/// kAUNBandEQParam_GlobalsGain is an overall gain and does not have a band.
impl au::ParamId {
    // Global, dB, -96->24, 0
    #[doc(alias = "kAUNBandEQParam_GlobalGain")]
    pub const NBAND_EQ_GLOBAL_GAIN: Self = Self(0);

    // Global, Boolean, 0 or 1, 1
    #[doc(alias = "kAUNBandEQParam_BypassBand")]
    pub const NBAND_EQ_BYPASS_BAND: Self = Self(1000);

    // Global, Indexed, 0->kNumAUNBandEQFilterTypes-1, 0
    #[doc(alias = "kAUNBandEQParam_FilterType")]
    pub const NBAND_EQ_FILTER_TYPE: Self = Self(2000);

    // Global, Hz, 20->(SampleRate/2), 1000
    #[doc(alias = "kAUNBandEQParam_Frequency")]
    pub const NBAND_EQ_FREQUENCY: Self = Self(3000);

    // Global, dB, -96->24, 0
    #[doc(alias = "kAUNBandEQParam_Gain")]
    pub const NBAND_EQ_GAIN: Self = Self(4000);

    // Global, octaves, 0.05->5.0, 0.5
    #[doc(alias = "kAUNBandEQParam_Bandwidth")]
    pub const NBAND_EQ_BANDWIDTH: Self = Self(5000);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum NBandEQFilterType {
    /// Parametric filter based on Butterworth analog prototype. Uses parameterization where
    /// the bandwidth is specifed as the relationship of the upper bandedge frequency to the
    /// lower bandedge frequency in octaves, where the upper and lower bandedge frequencies are
    /// the respective frequencies above and below the center frequency at which the gain is
    /// equal to half the peak gain.
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Gain (peak gain)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_Parametric")]
    Parametric = 0,

    /// Simple Butterworth 2nd order low pass filter
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (-3 dB cutoff frequency)
    #[doc(alias = "kAUNBandEQFilterType_2ndOrderButterworthLowPass")]
    _2ndOrderButterworthLowPass = 1,

    /// Simple Butterworth 2nd order high pass filter
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (-3 dB cutoff frequency)
    #[doc(alias = "kAUNBandEQFilterType_2ndOrderButterworthHighPass")]
    _2ndOrderButterworthHighPass = 2,

    /// Low pass filter with resonance support (via bandwidth parameter)
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (-3 dB cutoff frequency)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_ResonantLowPass")]
    ResonantLowPass = 3,

    /// High pass filter with resonance support (via bandwidth parameter)
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (-3 dB cutoff frequency)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_ResonantHighPass")]
    ResonantHighPass = 4,

    /// Band pass filter
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_BandPass")]
    BandPass = 5,

    /// Band stop filter (aka "notch filter")
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_BandStop")]
    BandStop = 6,

    /// Low shelf filter
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Gain (shelf gain)
    #[doc(alias = "kAUNBandEQFilterType_LowShelf")]
    LowShelf = 7,

    /// High shelf filter
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Gain (shelf gain)
    #[doc(alias = "kAUNBandEQFilterType_HighShelf")]
    HighShelf = 8,

    /// Low shelf filter with resonance support (via bandwidth parameter)
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Gain (shelf gain)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_ResonantLowShelf")]
    ResonantLowShelf = 9,

    /// High shelf filter with resonance support (via bandwidth parameter)
    /// Applicable parameters:
    /// - kAUNBandEQParam_Frequency (center frequency)
    /// - kAUNBandEQParam_Gain (shelf gain)
    /// - kAUNBandEQParam_Bandwidth
    #[doc(alias = "kAUNBandEQFilterType_ResonantLowShelf")]
    ResonantHighShelf = 10,

    #[doc(alias = "kNumAUNBandEQFilterTypes")]
    NumTypes = 11,
}

/// Parameters for the AURoundTripAACParam unit
impl au::ParamId {
    /// Global, indexed : AAC, HE-AAC, HE-AACv2
    #[doc(alias = "kRoundTripAACParam_Format")]
    pub const ROUND_TRIP_AAC_FORMAT: Self = Self(0);

    /// Global, indexed
    #[doc(alias = "kRoundTripAACParam_EncodingStrategy")]
    pub const ROUND_TRIP_AAC_ENCODING_STRATEGY: Self = Self(1);

    /// Global, indexed
    #[doc(alias = "kRoundTripAACParam_RateOrQuality")]
    pub const ROUND_TRIP_AAC_RATE_OR_QUALITY: Self = Self(2);
}

/// Parameters for the AUSoundIsolation unit
impl au::ParamId {
    // Global, Percent, 0->100, 100
    #[doc(alias = "kAUSoundIsolationParam_WetDryMixPercent")]
    pub const SOUND_ISOLATION_WET_DRY_MIX_PERCENT: Self = Self(0);

    // Global, Indexed, 1->1, 1
    #[doc(alias = "kAUSoundIsolationParam_SoundToIsolate")]
    pub const SOUND_ISOLATION_SOUND_TO_ISOLATE: Self = Self(1);
}

/// Constants available as values for parameter kAUSoundIsolation_SoundToIsolate.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum SoundIsolationSoundType {
    /// Isolate voice signal from the input signal.
    Voice = 1,
}
