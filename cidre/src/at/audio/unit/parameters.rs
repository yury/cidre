use crate::at::au;

/// The following specifies the equivalent parameterID's for the Group scope for standard
/// MIDI Controllers. This list is not exhaustive. It represents the parameters, and their corresponding
/// MIDI messages, that should be supported in Group scope by MIDI capable AUs.
///
/// Group scope parameter IDs from 0 < 512 are reserved for mapping MIDI controllers.
impl au::ParamId {
    /// value 0 < 128
    pub const VOLUME: Self = Self(7);

    /// value 0-63 (off), 64-127 (on)
    pub const SUSTAIN: Self = Self(64);

    /// value 0-63 (off), 64-127 (on)
    pub const SOSTENUTO: Self = Self(66);

    /// value ignored
    pub const ALL_NOTES_OFF: Self = Self(123);

    /// value 0 < 128
    pub const MOD_WHEEL: Self = Self(1);

    /// value -8192 - 8191
    pub const PITCH_BEND: Self = Self(0xE0);

    /// value ignored
    pub const ALL_SOUND_OFF: Self = Self(120);

    /// value ignored
    pub const RESET_ALL_CONTROLLERS: Self = Self(121);

    /// value 0 < 128
    pub const PAN: Self = Self(10);

    /// value 0 < 128
    pub const FOOT: Self = Self(4);

    /// value 0 < 128
    pub const CHANNEL_PRESSURE: Self = Self(0xD0);

    /// value 0 < 128
    pub const KEY_PRESSURE: Self = Self(0xA0);

    /// value 0 < 128
    pub const EXPRESSION: Self = Self(11);

    /// value 0 < 128
    pub const DATA_ENTRY: Self = Self(6);

    /// value 0 < 128
    pub const VOLUME_LSB: Self = Self(Self::VOLUME.0 + 32);

    /// value 0 < 128
    pub const MOD_WHEEL_LSB: Self = Self(Self::MOD_WHEEL.0 + 32);

    /// value 0 < 128
    pub const PAN_LSB: Self = Self(Self::PAN.0 + 32);

    /// value 0 < 128
    pub const FOOT_LSB: Self = Self(Self::FOOT.0 + 32);

    /// value 0 < 128
    pub const EXPRESSION_LSB: Self = Self(Self::EXPRESSION.0 + 32);

    /// value 0 < 128
    pub const DATA_ENTRY_LSB: Self = Self(Self::DATA_ENTRY.0 + 32);

    /// value 0 < 128
    pub const KEY_PRESSURE_FIRST_KEY: Self = Self(256);

    /// value 0 < 128
    pub const KEY_PRESSURE_LAST_KEY: Self = Self(383);
}

/// Parameters for all Panner AudioUnits
#[cfg(target_os = "macos")]
impl au::ParamId {
    /// Global, Linear, 0->1, 1
    pub const GAIN: Self = Self(0);

    // Global, Degrees, -180->180, 0
    pub const AZIMUTH: Self = Self(1);

    // Global, Degrees, -90->90, 0
    pub const ELEVATION: Self = Self(2);

    // Global, Linear, 0->1, 1
    pub const DISTANCE: Self = Self(3);

    // Global, Meters, 0.01->1000, 1
    pub const COORD_SCALE: Self = Self(4);

    // Global, Meters, 0.01->1000, 1
    pub const REF_DISTANCE: Self = Self(5);
}

/// Parameters for the AUSpatialMixer unit
impl au::ParamId {
    /// Input, Degrees, -180->180, 0
    pub const SPATIAL_MIXER_AZIMUTH: Self = Self(0);

    /// Input, Degrees, -90->90, 0
    pub const SPATIAL_MIXER_ELEVATION: Self = Self(1);

    /// Input, Metres, 0->10000, 0
    pub const SPATIAL_MIXER_DISTANCE: Self = Self(2);

    /// Input/Output, dB, -120->20, 0
    pub const SPATIAL_MIXER_GAIN: Self = Self(3);

    /// Input, rate scaler	0.5 -> 2.0, 1.0
    pub const SPATIAL_MIXER_PLAYBACK_RATE: Self = Self(4);

    /// bus enable : 0 or 1, 1
    pub const SPATIAL_MIXER_ENABLE: Self = Self(5);

    /// Minimum input gain constraint : 0.0 -> 10.0, 0.0
    pub const SPATIAL_MIXER_MIN_GAIN: Self = Self(6);

    /// Maximum input gain constraint : 0.0 -> 10.0, 10.0
    pub const SPATIAL_MIXER_MAX_GAIN: Self = Self(7);

    /// Input, Dry/Wet equal-power blend, %	  0.0 -> 100.0, 30.0
    pub const SPATIAL_MIXER_REVERB_BLEND: Self = Self(8);

    /// Global, dB,		-40.0 -> +40.0, 0.0
    pub const SPATIAL_MIXER_GLOBAL_REVERB_GAIN: Self = Self(9);

    /// Input, Lowpass filter attenuation at 5KHz :		decibels -100.0dB -> 0.0dB, 0.0dB
    /// smaller values make both direct and reverb sound more muffled; a value of 0.0 indicates no filtering
    /// Occlusion is a filter applied to the sound prior to the reverb send
    pub const SPATIAL_MIXER_OCCLUSION_ATTENUATION: Self = Self(10);

    /// Input, Lowpass filter attenuation at 5KHz :		decibels -100.0dB -> 0.0dB, 0.0dB
    /// smaller values make direct sound more muffled; a value of 0.0 indicates no filtering
    /// Obstruction is a filter applied to the "direct" part of the sound (so is post reverb send)
    pub const SPATIAL_MIXER_OBSTRUCTION_ATTENUATION: Self = Self(11);

    /// Global, Degrees, -180->180, 0
    pub const SPATIAL_MIXER_HEAD_YAW: Self = Self(19);

    /// Global, Degrees, -90->90, 0
    pub const SPATIAL_MIXER_HEAD_PITCH: Self = Self(20);

    /// Global, Degrees, -180->180, 0
    pub const SPATIAL_MIXER_HEAD_ROLL: Self = Self(2);
}
