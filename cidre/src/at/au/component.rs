/// Different types of audio units
///
/// Audio units are classified into different types, where those types perform different roles and
/// functions.
///
/// There are some general categories of functionality that apply across different types of audio
/// units:
///
/// 1. Real-time usage.
///
///    The audio unit will complete its operations in less time than is represented by the render
///    buffer. All audio units with the exception of the OfflineEffect should meet this criteria.
///
/// 2. Real-time I/O.
///
///    Will request the same amount of audio input as it is being asked to produce for output.
///    Effects, Panners, Mixers and MusicDevices are required to adhere to this restriction.
///    FormatConverter's can with some constraints be used in this situation (for instance, sample
///    rate conversion, float-int), but the host of the audio unit is responsible for insuring
///    this.
///
/// 3. UI versus Programmatic usage.
///
///    UI usage covers the case of using an audio unit in a Digital Audio Workstation (DAW) with
///    appropriate UI (for example a filter in Garage Band or Logic). Effects, Panners,
///    MusicDevices are all expected to be usable within this context.
///
///    Programmatic usage is where an audio unit is used by a host app as part of a general signal
///    processing chain.
///
///    For instance, a mixer audio unit can be used to take several different audio sources in a
///    game and mix them together. Mixers, Output units are for programmatic usage. FormatConverter
///    and Generator types are generally programmatic audio units, but if they can be used in a UI
///    situation, they specify a custom view. The app can then use that to decide that, with
///    appropriate constraints, the audio unit could be presented in a DAW type application. For
///    instance, the AUConverter audio unit from apple can do sample rate conversion, etc, but has
///    not general utility for a user in a DAW app. Apple's Varispeed or AUTimePitch audio units
///    can be used to change the playback rate and pitch and so could be used to good affect by a
///    user in a DAW type environment, as well as just providing this general functionality to any
///    program.
#[doc(alias = "AudioUnitType")]
#[repr(transparent)]
pub struct Type(pub u32);

impl Type {
    /// An output unit can be used standalone or as part of an [`au::Graph`] or [`av::AudioEngine`]. Apple
    /// provides a number of output units that interface directly with an audio device.
    #[doc(alias = "kAudioUnitType_Output")]
    pub const OUTPUT: Self = Self(u32::from_be_bytes(*b"auou"));

    /// A software musical instrument such as a sampler or synthesizer. They respond to MIDI and
    /// create notes, which are then controlled through parameters or MIDI control messages.
    #[doc(alias = "kAudioUnitType_MusicDevice")]
    pub const MUSIC_DEVICE: Self = Self(u32::from_be_bytes(*b"aumu"));

    /// An audio unit that will process some x number of audio input samples to produce x number of
    /// audio output samples. The common case for an effect is to have a single input to a single
    /// output, though some effects take side-chain inputs as well. Effects can be run in "offline"
    /// contexts (such as processing a file), but they are expected to run in real-time. A delay
    /// unit or reverb is a good example of this.
    #[doc(alias = "kAudioUnitType_MusicEffect")]
    pub const MUSIC_EFFECT: Self = Self(u32::from_be_bytes(*b"aumf"));

    /// An audio unit that takes some number of inputs, mixing them to provide 1 or more audio
    /// outputs. A stere mixer (mono and stereo inputs to produce one stereo output) is an example
    /// of this.
    #[doc(alias = "kAudioUnitType_Mixer")]
    pub const MIXER: Self = Self(u32::from_be_bytes(*b"aumx"));

    /// A panner is a specialised effect that will pan a single audio input to a single output.
    /// Panner units are required to support a collection of standardised parameters that specify
    /// the panning coordinates (aside from whatever custom parameters the panner may provide). A
    /// surround panner is an example of this
    #[doc(alias = "kAudioUnitType_Panner")]
    pub const PANNER: Self = Self(u32::from_be_bytes(*b"aupn"));

    /// A generator will have no audio input, but will just produce audio output. In some ways it is
    /// similar to a MusicDevice, except that a generator provides no MIDI input, or notion of
    /// "notes". A tone generator is a good example of this.
    #[doc(alias = "kAudioUnitType_Generator")]
    pub const GENERATOR: Self = Self(u32::from_be_bytes(*b"augn"));

    /// An offline effect is used to process data from a file and is also used to publish a
    /// capability that cannot be run in real-time. For instance, the process of normalization
    /// requires seeing the entire audio input before the scalar to apply in the normalization
    /// process can be estimated. As such, offline effects also have a notion of a priming stage
    /// that can be performed before the actual rendering/processing phase is executed.
    #[doc(alias = "kAudioUnitType_OfflineEffect")]
    pub const OFFLINE_EFFECT: Self = Self(u32::from_be_bytes(*b"auol"));

    /// Plugins of this type process MIDI input and produce MIDI output. They do not produce audio.
    #[doc(alias = "kAudioUnitType_MIDIProcessor")]
    pub const MIDI_PROCESSOR: Self = Self(u32::from_be_bytes(*b"aumi"));

    /// An offline audio unit that produces synthesized speech audio output.
    #[doc(alias = "kAudioUnitType_SpeechSynthesizer")]
    pub const SPEECH_SYNTHESIZER: Self = Self(u32::from_be_bytes(*b"ausp"));
}

#[doc(alias = "AudioUnitManufacturer")]
#[repr(transparent)]
pub struct Manufacturer(pub u32);

impl Manufacturer {
    #[doc(alias = "kAudioUnitManufacturer_Apple")]
    pub const APPLE: Self = Self(u32::from_be_bytes(*b"appl"));
}

/// Apple input/output audio unit sub types
///
/// Subtypes for the various input/output units that Apple ships.
/// Input/output units add an additional notion of Start and Stop.
#[doc(alias = "AudioUnitSubType")]
#[repr(transparent)]
pub struct SubType(pub u32);

impl SubType {
    /// A generic output unit provides the start/stop API, and provides the basic
    /// services to convert Linear PCM formats.
    #[doc(alias = "kAudioUnitSubType_GenericOutput")]
    pub const GENERIC_OUTPUT: Self = Self(u32::from_be_bytes(*b"genr"));

    /// This audio unit can do input as well as output. Bus 0 is used for the output
    /// side, bus 1 is used to get audio input (thus, on the iPhone, it works in a
    /// very similar way to the Remote I/O). This audio unit does signal processing on
    /// the incoming audio (taking out any of the audio that is played from the device
    /// at a given time from the incoming audio).
    #[doc(alias = "kAudioUnitSubType_VoiceProcessingIO")]
    pub const VOICE_PROCESSING_IO: Self = Self(u32::from_be_bytes(*b"vpio"));

    /// (a.k.a. "AUHAL")  The audio unit that interfaces to any audio device. The user specifies
    /// which audio device to track. Bus 0 is used to send audio output to the device; bus 1 is used
    /// to receive audio input from the device. Available on macOS only.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitSubType_HALOutput")]
    pub const HAL_OUTPUT: Self = Self(u32::from_be_bytes(*b"ahal"));

    /// A specialization of AUHAL that is used to track the user's selection of the default device
    /// as set in the Sound Prefs. Available on macOS only.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitSubType_DefaultOutput")]
    pub const DEFAULT_OUTPUT: Self = Self(u32::from_be_bytes(*b"def "));

    /// A specialization of AUHAL that is used to track the user's selection of the device to use
    /// for sound effects, alerts and other UI sounds. Available on macOS only.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitSubType_SystemOutput")]
    pub const SYSTEM_OUTPUT: Self = Self(u32::from_be_bytes(*b"sys "));

    /// The audio unit that interfaces to the iOS audio system. Bus 0 is used to send audio output;
    /// bus 1 is used to receive audio input.
    #[cfg(not(target_os = "macos"))]
    #[doc(alias = "kAudioUnitSubType_RemoteIO")]
    pub const REMOTE_IO: Self = Self(u32::from_be_bytes(*b"rioc"));
}

/// Apple music instrument audio unit sub types
impl SubType {
    /// A multi-timbral music device that can use sample banks in either DLS or
    /// SoundFont formats. It fully supports GM-MIDI and the basic extensions of
    /// GS-MIDI.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitSubType_DLSSynth")]
    pub const DLS_SYNTH: Self = Self(u32::from_be_bytes(*b"dls "));

    /// A mono-timbral music device which is a sampler synthesizer and supports full
    /// interactive editing of all state.
    #[doc(alias = "kAudioUnitSubType_Sampler")]
    pub const SAMPLER: Self = Self(u32::from_be_bytes(*b"samp"));

    /// A fully GM-compatible multi-timbral music device which is a sampler synthesizer.
    /// It can load instruments from sample banks in either DLS or SoundFont formats.
    #[doc(alias = "kAudioUnitSubType_MIDISynth")]
    pub const MIDI_SYNTH: Self = Self(u32::from_be_bytes(*b"msyn"));
}

/// Apple converter audio unit sub types
impl SubType {
    /// An audio unit that uses an AudioConverter to do Linear PCM conversions (sample
    /// rate, bit depth, interleaving).
    #[doc(alias = "kAudioUnitSubType_AUConverter")]
    pub const CONVERTER: Self = Self(u32::from_be_bytes(*b"conv"));

    #[doc(alias = "kAudioUnitSubType_Varispeed")]
    pub const VARISPEED: Self = Self(u32::from_be_bytes(*b"vari"));

    #[doc(alias = "kAudioUnitSubType_DeferredRenderer")]
    pub const DEFERRED_RENDERER: Self = Self(u32::from_be_bytes(*b"defr"));

    #[doc(alias = "kAudioUnitSubType_Splitter")]
    pub const SPLITTER: Self = Self(u32::from_be_bytes(*b"splt"));

    #[doc(alias = "kAudioUnitSubType_MultiSplitter")]
    pub const MUTLI_SPLITTER: Self = Self(u32::from_be_bytes(*b"mspl"));

    #[doc(alias = "kAudioUnitSubType_Merger")]
    pub const MERGER: Self = Self(u32::from_be_bytes(*b"merg"));

    #[doc(alias = "kAudioUnitSubType_NewTimePitch")]
    pub const NEW_TIME_PITCH: Self = Self(u32::from_be_bytes(*b"nutp"));

    #[doc(alias = "kAudioUnitSubType_RoundTripAAC")]
    pub const ROUND_TRIP_AAC: Self = Self(u32::from_be_bytes(*b"raac"));
}
