use std::ffi::c_void;

use crate::{at, at::audio, cf, define_opts, os};

pub type _Unit = audio::ComponentInstance;

pub struct Unit(audio::ComponentInstanceRef);
pub struct UnitRef(audio::ComponentInstanceRef);

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

/// Apple effect audio unit sub types
impl SubType {
    /// A peak limiter
    #[doc(alias = "kAudioUnitSubType_PeakLimiter")]
    pub const PEAK_LIMITER: Self = Self(u32::from_be_bytes(*b"lmtr"));

    /// A dynamics compressor/expander
    #[doc(alias = "kAudioUnitSubType_DynamicsProcessor")]
    pub const DYNAMICS_PROCESSOR: Self = Self(u32::from_be_bytes(*b"dcmp"));

    /// A filter that passes frequencies below a specified cut-off frequency
    #[doc(alias = "kAudioUnitSubType_LowPassFilter")]
    pub const LOW_PASS_FILTER: Self = Self(u32::from_be_bytes(*b"lpas"));

    /// A filter that passes frequencies above a specified cut-off frequency
    #[doc(alias = "kAudioUnitSubType_HighPassFilter")]
    pub const HIGH_PASS_FILTER: Self = Self(u32::from_be_bytes(*b"hpas"));

    /// A filter that passes frequencies between a low and high cut-off frequency.
    #[doc(alias = "kAudioUnitSubType_BandPassFilter")]
    pub const BAND_PASS_FILTER: Self = Self(u32::from_be_bytes(*b"bpas"));

    /// A filter that can be used to implement a "treble" control
    #[doc(alias = "kAudioUnitSubType_HighShelfFilter")]
    pub const HIGH_SHELF_FILTER: Self = Self(u32::from_be_bytes(*b"hshf"));

    /// A filter that can be used to implement a "bass" control
    #[doc(alias = "kAudioUnitSubType_LowShelfFilter")]
    pub const LOW_SHELF_FILTER: Self = Self(u32::from_be_bytes(*b"lshf"));

    /// A parametric EQ filter
    #[doc(alias = "kAudioUnitSubType_ParametricEQ")]
    pub const PARAMETRIC_EQ: Self = Self(u32::from_be_bytes(*b"pmeq"));

    /// A distortion audio unit
    #[doc(alias = "kAudioUnitSubType_Distortion")]
    pub const DISTORTION: Self = Self(u32::from_be_bytes(*b"dist"));

    /// A delay audio unit
    #[doc(alias = "kAudioUnitSubType_Delay")]
    pub const DELAY: Self = Self(u32::from_be_bytes(*b"dely"));

    /// A delay that is used to delay the input a specified number of samples until
    /// the output
    #[doc(alias = "kAudioUnitSubType_SampleDelay")]
    pub const SAMPLE_DELAY: Self = Self(u32::from_be_bytes(*b"sdly"));

    /// A generalized N-band graphic EQ with specifiable filter types per-band
    #[doc(alias = "kAudioUnitSubType_NBandEQ")]
    pub const N_BAND_EQ: Self = Self(u32::from_be_bytes(*b"nbeq"));

    /// A lite reverb that can be used to simulate various and different spaces
    #[doc(alias = "kAudioUnitSubType_Reverb2")]
    pub const REVERB2: Self = Self(u32::from_be_bytes(*b"rvb2"));

    /// An audio unit that can be used to isolate a specified sound type
    #[doc(alias = "kAudioUnitSubType_AUSoundIsolation")]
    pub const SOUND_ISOLATION: Self = Self(u32::from_be_bytes(*b"vois"));
}

/// Apple effect audio unit sub types (macOS only)
#[cfg(target_os = "macos")]
impl SubType {
    /// A 10 or 31 band Graphic EQ
    #[doc(alias = "kAudioUnitSubType_GraphicEQ")]
    pub const GRAPHIC_EQ: Self = Self(u32::from_be_bytes(*b"greq"));

    /// A 4 band compressor/expander
    #[doc(alias = "kAudioUnitSubType_MultiBandCompressor")]
    pub const MULTI_BAND_COMPRESSOR: Self = Self(u32::from_be_bytes(*b"mcmp"));

    /// A reverb that can be used to simulate various and different spaces
    #[doc(alias = "kAudioUnitSubType_MatrixReverb")]
    pub const MATRIX_REVERB: Self = Self(u32::from_be_bytes(*b"mrev"));

    /// An audio unit used to change the pitch
    #[doc(alias = "kAudioUnitSubType_Pitch")]
    pub const PITCH: Self = Self(u32::from_be_bytes(*b"tmpt"));

    /// A filter unit that combines 5 different filters (low, 3 mids, high)
    #[doc(alias = "kAudioUnitSubType_AUFilter")]
    pub const FILTER: Self = Self(u32::from_be_bytes(*b"filt"));

    /// An audio unit that is used in conjunction with _NetReceive to send audio
    /// across the network (or between different applications)
    #[doc(alias = "kAudioUnitSubType_NetSend")]
    pub const NET_SEND: Self = Self(u32::from_be_bytes(*b"nsnd"));

    /// An audio unit that can be used to emit a short tone in gaps between speech
    /// similar to the tones used in a walkie-talkie
    #[doc(alias = "kAudioUnitSubType_RogerBeep")]
    pub const ROGER_BEEP: Self = Self(u32::from_be_bytes(*b"rogr"));
}

/// Apple mixer audio unit sub types
///
/// These are the subtypes for the various mixer units that apple ships
impl SubType {
    /// Can have any number of inputs, with any number of channels on any input to one
    /// output bus with any number of channels.
    #[doc(alias = "kAudioUnitSubType_MultiChannelMixer")]
    pub const MULTI_CHANNEL_MIXER: Self = Self(u32::from_be_bytes(*b"mcmx"));

    /// Any number of input and output buses with any number of channels on any bus.
    /// The mix is presented as a matrix of channels that can be controlled through
    /// input volume per channel, "cross-point" volume (a given input channel to a
    /// given output channel), output volume per channel and a global volume across
    /// the whole matrix
    #[doc(alias = "kAudioUnitSubType_MatrixMixer")]
    pub const MATRIX_MIXER: Self = Self(u32::from_be_bytes(*b"mxmx"));

    /// Inputs that are mono will be panned around using 3D coordinates and parameters.
    /// Stereo inputs are passed directly through to the output.
    /// A single output is presented with 2, 4, 5, 6, 7 or 8 channels.
    /// There is also a built in reverb.
    #[doc(alias = "kAudioUnitSubType_SpatialMixer")]
    pub const SPATIAL_MIXER: Self = Self(u32::from_be_bytes(*b"3dem"));
}

/// Apple mixer audio unit sub types (macOS only)
#[cfg(target_os = "macos")]
impl SubType {
    /// Inputs can be mono or stereo. Single stereo output
    #[doc(alias = "kAudioUnitSubType_StereoMixer")]
    pub const STEREO_MIXER: Self = Self(u32::from_be_bytes(*b"smxr"));
}

/// Apple generator audio unit sub types
impl SubType {
    /// A generator unit that is paired with _NetSend to receive the audio that unit
    /// sends. It presents a custom UI so can be used in a UI context as well
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitSubType_NetReceive")]
    pub const NET_RECEIVE: Self = Self(u32::from_be_bytes(*b"nrcv"));

    /// A generator unit that can be used to schedule slices of audio to be played at
    /// a specified time. The audio is scheduled using the time stamps for the render
    /// operation, and can be scheduled from any thread.
    #[doc(alias = "kAudioUnitSubType_ScheduledSoundPlayer")]
    pub const SCHEDULED_SOUND_PLAYER: Self = Self(u32::from_be_bytes(*b"sspl"));

    /// A generator unit that is used to play a file. It presents a custom UI so can
    /// be used in a UI context as well
    #[doc(alias = "kAudioUnitSubType_AudioFilePlayer")]
    pub const AUDIO_FILE_PLAYER: Self = Self(u32::from_be_bytes(*b"afpl"));
}

define_opts!(
    #[doc(alias = "AudioUnitRenderActionFlags")]
    pub RenderActionFlags(u32)
);

/// These flags can be set in a callback from an audio unit during an audio unit
/// render operation from either the RenderNotify Proc or the render input
/// callback.
impl RenderActionFlags {
    /// Called on a render notification Proc - which is called either before or after
    /// the render operation of the audio unit. If this flag is set, the proc is being
    /// called before the render operation is performed.
    #[doc(alias = "kAudioUnitRenderAction_PreRender")]
    pub const PRE_RENDER: Self = Self(1u32 << 2);

    /// Called on a render notification Proc - which is called either before or after
    /// the render operation of the audio unit. If this flag is set, the proc is being
    /// called after the render operation is completed.
    #[doc(alias = "kAudioUnitRenderAction_PostRender")]
    pub const POST_RENDER: Self = Self(1u32 << 3);

    /// The originator of a buffer, in a render input callback, or in an audio unit's
    /// render operation, may use this flag to indicate that the buffer contains
    /// only silence.
    ///
    /// The receiver of the buffer can then use the flag as a hint as to whether the
    /// buffer needs to be processed or not.
    ///
    /// Note that because the flag is only a hint, when setting the silence flag,
    /// the originator of a buffer must also ensure that it contains silence (zeroes).
    #[doc(alias = "kAudioUnitRenderAction_OutputIsSilence")]
    pub const OUTPUT_IS_SILENCE: Self = Self(1u32 << 4);

    /// This is used with offline audio units (of type 'auol'). It is used when an
    /// offline unit is being preflighted, which is performed prior to the actual
    /// offline rendering actions are performed. It is used for those cases where the
    /// offline process needs it (for example, with an offline unit that normalises an
    /// audio file, it needs to see all of the audio data first before it can perform
    /// its normalization)
    #[doc(alias = "kAudioOfflineUnitRenderAction_Preflight")]
    pub const PREFLIGHT: Self = Self(1u32 << 5);

    /// Once an offline unit has been successfully preflighted, it is then put into
    /// its render mode. So this flag is set to indicate to the audio unit that it is
    /// now in that state and that it should perform its processing on the input data.
    #[doc(alias = "kAudioOfflineUnitRenderAction_Render")]
    pub const RENDER: Self = Self(1u32 << 6);

    /// This flag is set when an offline unit has completed either its preflight or
    /// performed render operations
    #[doc(alias = "kAudioOfflineUnitRenderAction_Complete")]
    pub const COMPLETE: Self = Self(1u32 << 7);

    /// If this flag is set on the post-render call an error was returned by the
    /// AUs render operation. In this case, the error can be retrieved through the
    /// lastRenderError property and the audio data in ioData handed to the post-render
    /// notification will be invalid.
    #[doc(alias = "kAudioUnitRenderAction_PostRenderError")]
    pub const POST_RENDER_ERROR: Self = Self(1u32 << 8);

    /// If this flag is set, then checks that are done on the arguments provided to render
    /// are not performed. This can be useful to use to save computation time in
    /// situations where you are sure you are providing the correct arguments
    /// and structures to the various render calls
    #[doc(alias = "kAudioUnitRenderAction_DoNotCheckRenderArgs")]
    pub const DO_NOT_CHECK_RENDER_ARGS: Self = Self(1u32 << 9);
}

/// Audio unit errors
///
/// These are the various errors that can be returned by AudioUnit API calls
pub mod err {
    use crate::os::Status;

    /// The property is not supported
    #[doc(alias = "kAudioUnitErr_InvalidProperty")]
    pub const INVALID_PROPERTY: Status = Status(-10879);

    /// The parameter is not supported
    #[doc(alias = "kAudioUnitErr_InvalidParameter")]
    pub const INVALID_PARAMETER: Status = Status(-10878);

    /// The specified element is not valid
    #[doc(alias = "kAudioUnitErr_InvalidElement")]
    pub const INVALID_ELEMENT: Status = Status(-10877);

    /// There is no connection (generally an audio unit is asked to render but it has
    /// not input from which to gather data)
    #[doc(alias = "kAudioUnitErr_NoConnection")]
    pub const NO_CONNECTION: Status = Status(-10876);

    /// The audio unit is unable to be initialized
    #[doc(alias = "kAudioUnitErr_FailedInitialization")]
    pub const FAILED_INITIALIZATION: Status = Status(-10875);

    /// When an audio unit is initialized it has a value which specifies the max
    /// number of frames it will be asked to render at any given time. If an audio
    /// unit is asked to render more than this, this error is returned.
    #[doc(alias = "kAudioUnitErr_TooManyFramesToProcess")]
    pub const TOO_MANY_FRAMES_TO_PROCESS: Status = Status(-10874);

    /// If an audio unit uses external files as a data source, this error is returned
    /// if a file is invalid (Apple's DLS synth returns this error)
    #[doc(alias = "kAudioUnitErr_InvalidFile")]
    pub const INVALID_FILE: Status = Status(-10871);

    /// If an audio unit uses external files as a data source, this error is returned
    /// if a file is invalid (Apple's DLS synth returns this error)
    #[doc(alias = "kAudioUnitErr_UnknownFileType")]
    pub const UNKNOWN_FILE_TYPE: Status = Status(-10870);

    /// If an audio unit uses external files as a data source, this error is returned
    /// if a file hasn't been set on it
    /// (Apple's DLS synth returns this error)
    #[doc(alias = "kAudioUnitErr_FileNotSpecified")]
    pub const FILE_NOT_SPECIFIED: Status = Status(-10869);

    /// Returned if an input or output format is not supported
    #[doc(alias = "kAudioUnitErr_FormatNotSupported")]
    pub const FORMAT_NOT_SUPPORTED: Status = Status(-10868);

    /// Returned if an operation requires an audio unit to be initialized and it is
    /// not.
    #[doc(alias = "kAudioUnitErr_Uninitialized")]
    pub const UNINITIALIZED: Status = Status(-10867);

    /// The specified scope is invalid
    #[doc(alias = "kAudioUnitErr_InvalidScope")]
    pub const INVALID_SCOPE: Status = Status(-10866);

    /// The property cannot be written
    #[doc(alias = "kAudioUnitErr_PropertyNotWritable")]
    pub const PROPERTY_NOT_WRITABLE: Status = Status(-10865);

    /// Returned when an audio unit is in a state where it can't perform the requested
    /// action now - but it could later. Its usually used to guard a render operation
    /// when a reconfiguration of its internal state is being performed.
    #[doc(alias = "kAudioUnitErr_CannotDoInCurrentContext")]
    pub const CANNOT_DO_IN_CURRENT_CONTEXT: Status = Status(-10863);

    /// The property is valid, but the value of the property being provided is not
    #[doc(alias = "kAudioUnitErr_InvalidPropertyValue")]
    pub const INVALID_PROPERTY_VALUE: Status = Status(-10851);

    /// Returned when a property is valid, but it hasn't been set to a valid value at
    /// this time.
    #[doc(alias = "kAudioUnitErr_PropertyNotInUse")]
    pub const PROPERTY_NOT_IN_USE: Status = Status(-10850);

    /// Indicates the operation cannot be performed because the audio unit is
    /// initialized.
    #[doc(alias = "kAudioUnitErr_Initialized")]
    pub const INITIALIZED: Status = Status(-10849);

    /// Used to indicate that the offline render operation is invalid. For instance,
    /// when the audio unit needs to be pre-flighted,
    /// but it hasn't been.
    #[doc(alias = "kAudioUnitErr_InvalidOfflineRender")]
    pub const INVALID_OFFLINE_RENDER: Status = Status(-10848);

    /// Returned by either Open or Initialize, this error is used to indicate that the
    /// audio unit is not authorised, that it cannot be used. A host can then present
    /// a UI to notify the user the audio unit is not able to be used in its current
    /// state.
    #[doc(alias = "kAudioUnitErr_Unauthorized")]
    pub const UNAUTHORIZED: Status = Status(-10847);

    /// Returned during the render call, if the audio unit produces more MIDI output,
    /// than the default allocated buffer. The audio unit can provide a size hint, in
    /// case it needs a larger buffer. See the documentation for AUAudioUnit's
    /// MIDIOutputBufferSizeHint property.
    #[doc(alias = "kAudioUnitErr_MIDIOutputBufferFull")]
    pub const MIDI_OUTPUT_BUFFER_FULL: Status = Status(-66753);

    /// The audio unit did not satisfy the render request in time.
    #[doc(alias = "kAudioUnitErr_RenderTimeout")]
    pub const RENDER_TIMEOUT: Status = Status(-66745);

    /// The specified identifier did not match any Audio Unit Extensions.
    #[doc(alias = "kAudioUnitErr_ExtensionNotFound")]
    pub const EXTENSION_NOT_FOUND: Status = Status(-66744);

    /// The parameter value is not supported, e.g. the value specified is NaN or
    /// infinite.
    #[doc(alias = "kAudioUnitErr_InvalidParameterValue")]
    pub const INVALID_PARAMETER_VALUE: Status = Status(-66743);

    /// The file path that was passed is not supported. It is either too long or contains
    /// invalid characters.
    #[doc(alias = "kAudioUnitErr_InvalidFilePath")]
    pub const INVALID_FILE_PATH: Status = Status(-66742);

    /// A required key is missing from a dictionary object.
    #[doc(alias = "kAudioUnitErr_MissingKey")]
    pub const MISSING_KEY: Status = Status(-66741);

    /// The operation can not be performed for a component instance instantiated using the
    /// deprecated Component Manager. A host application should use the API functions
    /// AudioComponentInstantiate or AudioComponentInstanceNew when rebuilding
    /// against the macOS 11 or later SDK.
    #[doc(alias = "kAudioUnitErr_ComponentManagerNotSupported")]
    pub const COMPONENT_MANAGER_NOT_SUPPORTED: Status = Status(-66749);
}

pub mod component_err {
    use crate::os::Status;

    #[doc(alias = "kAudioComponentErr_InstanceTimedOut")]
    pub const INSTANCE_TIMED_OUT: Status = Status(-66754);

    #[doc(alias = "kAudioComponentErr_InstanceInvalidated")]
    pub const INSTANCE_INVALIDATED: Status = Status(-66749);

    /// a non-unique component description was provided to AudioOutputUnitPublish
    #[doc(alias = "kAudioComponentErr_DuplicateDescription")]
    pub const DUPLICATE_DESCRIPTION: Status = Status(-66752);

    /// an unsupported component type was provided to AudioOutputUnitPublish
    #[doc(alias = "kAudioComponentErr_UnsupportedType")]
    pub const UNSUPPORTED_TYPE: Status = Status(-66751);

    /// components published via AudioOutputUnitPublish may only have one instance
    #[doc(alias = "kAudioComponentErr_TooManyInstances")]
    pub const TOO_MANY_INSTANCES: Status = Status(-66750);

    /// app needs "inter-app-audio" entitlement or host app needs "audio" in its UIBackgroundModes.
    /// Or app is trying to register a component not declared in its Info.plist.
    #[doc(alias = "kAudioComponentErr_NotPermitted")]
    pub const NOT_PERMITTED: Status = Status(-66748);

    /// host did not render in a timely manner; must uninitialize and reinitialize.
    #[doc(alias = "kAudioComponentErr_InitializationTimedOut")]
    pub const INITIALIZATION_TIMED_OUT: Status = Status(-66747);

    /// inter-app AU element formats must have sample rates matching the hardware.
    #[doc(alias = "kAudioComponentErr_InvalidFormat")]
    pub const INVALID_FORMAT: Status = Status(-66746);
}

/// Type used for audio unit properties.
///
/// Properties are used to describe the state of an audio unit (for instance,
/// the input or output audio format)
#[doc(alias = "AudioUnitPropertyID")]
#[repr(transparent)]
pub struct PropId(pub u32);

/// Type used for audio unit scopes.
///
/// Apple reserves the 0 < 1024 range for
/// audio unit scope identifiers.  
/// Scopes are used to delineate a major attribute of an audio unit
/// (for instance, global, input, output)
#[doc(alias = "AudioUnitScope")]
#[repr(transparent)]
pub struct Scope(pub u32);

/// Type used for audio unit elements.
///
/// Scopes can have one or more member, and a member of a scope is
/// addressed / described by its element
/// For instance, input bus 1 is input scope, element 1
#[doc(alias = "AudioUnitElement")]
#[repr(transparent)]
pub struct Element(pub u32);

/// Type used for audio unit parameters.
///
/// Parameters are typically used to control and set render state
/// (for instance, filter cut-off frequency)
#[doc(alias = "AudioUnitParameterID")]
#[repr(transparent)]
pub struct ParamId(pub u32);

/// Type used for audio unit parameter values.
/// The value of a given parameter is specified using this type
#[doc(alias = "AudioUnitParameterValue")]
pub type ParamValue = f32;

/// The type of a parameter event (see AudioUnitScheduleParameter)
#[doc(alias = "AUParameterEventType")]
#[repr(u32)]
pub enum ParamEventType {
    /// The parameter event describes an immediate change to the parameter value to
    /// the new value
    Immediate = 1,

    /// The parameter event describes a change to the parameter value that should
    /// be applied over the specified period of time
    Ramped = 2,
}

#[repr(C, u32)]
pub enum ParamEventValue {
    Immediate {
        buf_offset: i32,
        value: ParamValue,
    } = ParamEventType::Immediate as u32,
    Ramped {
        start_buf_offset: i32,
        duration_in_frames: u32,
        start_value: ParamValue,
        end_value: ParamValue,
    } = ParamEventType::Ramped as u32,
}

impl ParamEventValue {
    pub fn type_(&self) -> ParamEventType {
        match self {
            Self::Ramped { .. } => ParamEventType::Ramped,
            Self::Immediate { .. } => ParamEventType::Immediate,
        }
    }
}

#[doc(alias = "AudioUnitParameterEvent")]
#[repr(C)]
pub struct ParamEvent {
    /// The scope for the parameter
    pub scope: Scope,

    /// The element for the parameter
    pub element: Element,

    /// The ParamID for the parameter
    pub param_id: ParamId,

    pub value: ParamEventValue,
}

#[doc(alias = "AudioUnitParameter")]
pub struct Param {
    pub unit: *const Unit,
    pub param_id: ParamId,
    pub scope: Scope,
    pub element: Element,
}

#[doc(alias = "AudioUnitProperty")]
pub struct Prop {
    pub unit: *const _Unit,
    pub prop_id: PropId,
    pub scope: Scope,
    pub element: Element,
}

#[doc(alias = "AURenderCallback")]
pub type RenderCb<T = c_void> = extern "C" fn(
    in_ref_con: *mut T,
    io_action_flags: &mut RenderActionFlags,
    in_timestamp: &at::AudioTimeStamp,
    in_bus_num: u32,
    in_number_frames: u32,
    io_data: *mut at::AudioBufList,
) -> os::Status;

#[doc(alias = "AudioUnitPropertyListenerProc")]
pub type PropListenerProc<T = c_void> = extern "C" fn(
    in_ref_con: *mut T,
    in_unit: *const Unit,
    in_id: PropId,
    in_scope: Scope,
    in_element: Element,
);

#[doc(alias = "AUInputSamplesInOutputCallback")]
pub type InputSamplesInOutputCb<T = c_void> = extern "C" fn(
    in_ref_con: *mut T,
    in_output_ts: &at::AudioTimeStamp,
    in_input_sample: f64,
    in_number_input_samples: f64,
);

impl cf::NotificationName {
    #[doc(alias = "kAudioComponentRegistrationsChangedNotification")]
    #[inline]
    pub fn audio_component_registrations_changed() -> &'static Self {
        unsafe { kAudioComponentRegistrationsChangedNotification }
    }

    #[doc(alias = "kAudioComponentInstanceInvalidationNotification")]
    #[inline]
    pub fn audio_component_instance_invalidation() -> &'static Self {
        unsafe { kAudioComponentInstanceInvalidationNotification }
    }
}

impl _Unit {
    fn prop_info(
        &self,
        prop_id: PropId,
        scope: Scope,
        element: Element,
    ) -> Result<(u32, bool), os::Status> {
        let mut size = 0u32;
        let mut writable = false;
        unsafe {
            AudioUnitGetPropertyInfo(&self, prop_id, scope, element, &mut size, &mut writable)
                .result()?;
        }
        Ok((size, writable))
    }

    fn render_offline(&self) -> bool {
        todo!()
    }

    fn set_render_offline(&mut self, val: bool) {
        todo!()
    }
}

impl Unit {
    pub fn initialize(mut self) -> Result<UnitRef, os::Status> {
        unsafe {
            AudioUnitInitialize(&mut self.0).result()?;
            Ok(std::mem::transmute(self))
        }
    }

    pub fn prop_info(
        &self,
        prop_id: PropId,
        scope: Scope,
        element: Element,
    ) -> Result<(u32, bool), os::Status> {
        self.0.prop_info(prop_id, scope, element)
    }
}

impl UnitRef {
    fn _uninitialize(&mut self) -> os::Status {
        unsafe { AudioUnitUninitialize(&mut self.0) }
    }

    pub fn unintialize(mut self) -> Result<Unit, os::Status> {
        self._uninitialize().result()?;
        Ok(unsafe { std::mem::transmute(self) })
    }

    pub fn prop_info(
        &self,
        prop_id: PropId,
        scope: Scope,
        element: Element,
    ) -> Result<(u32, bool), os::Status> {
        self.0.prop_info(prop_id, scope, element)
    }
}

impl Drop for UnitRef {
    fn drop(&mut self) {
        let res = self._uninitialize();
        debug_assert!(res.is_ok());
    }
}

impl audio::Component {
    pub fn new_unit(&self) -> Result<Unit, os::Status> {
        Ok(Unit(self.new_instance()?))
    }
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    static kAudioComponentRegistrationsChangedNotification: &'static cf::NotificationName;
    static kAudioComponentInstanceInvalidationNotification: &'static cf::NotificationName;

    fn AudioUnitInitialize(in_unit: &mut _Unit) -> os::Status;
    fn AudioUnitUninitialize(in_unit: &mut _Unit) -> os::Status;

    fn AudioUnitGetPropertyInfo(
        in_unit: &_Unit,
        in_id: PropId,
        in_scope: Scope,
        in_element: Element,
        out_data_size: *mut u32,
        out_writable: *mut bool,
    ) -> os::Status;

    fn AudioUnitGetProperty(
        in_unit: &_Unit,
        in_id: PropId,
        in_scope: Scope,
        in_element: Element,
        out_data: *mut c_void,
        io_data_size: *mut u32,
    ) -> os::Status;

    fn AudioUnitSetProperty(
        in_unit: &mut _Unit,
        in_id: PropId,
        in_scope: Scope,
        in_element: Element,
        in_data: *const c_void,
        in_data_size: u32,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::{at::au, at::audio};
    #[test]
    fn basics() {
        let desc = audio::ComponentDesc {
            type_: au::Type::MIXER.0,
            sub_type: au::SubType::SPATIAL_MIXER.0,
            manufacturer: au::Manufacturer::APPLE.0,
            ..Default::default()
        };

        let comp = desc.into_iter().next().unwrap();
        let mixer = comp.new_unit().unwrap();
        let (size, writable) = mixer
            .prop_info(
                au::PropId::OFFLINE_RENDER,
                au::Scope::GLOBAL,
                au::Element::OUTPUT,
            )
            .unwrap();

        assert!(writable);
        assert_eq!(4, size);
        println!("size: {size}, writable: {writable}");

        let mixer = mixer.initialize().unwrap();
        let (size, writable) = mixer
            .prop_info(
                au::PropId::OFFLINE_RENDER,
                au::Scope::GLOBAL,
                au::Element::OUTPUT,
            )
            .unwrap();
        assert!(!writable);
        assert_eq!(4, size);
    }
}
