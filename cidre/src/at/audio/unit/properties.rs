use std::ffi::c_void;

use crate::{arc, at::au, cf};

impl au::Scope {
    /// The context for audio unit characteristics that apply to the audio unit as a
    /// whole.
    #[doc(alias = "kAudioUnitScope_Global")]
    pub const GLOBAL: Self = Self(0);

    /// The context for audio data coming into an audio unit.
    #[doc(alias = "kAudioUnitScope_Input")]
    pub const INPUT: Self = Self(1);

    /// The context for audio data leaving an audio unit.
    #[doc(alias = "kAudioUnitScope_Output")]
    pub const OUTPUT: Self = Self(2);

    /// A context specific to the control scope of parameters (for instance, MIDI
    /// Channels is an example of this scope)
    #[doc(alias = "kAudioUnitScope_Group")]
    pub const GROUP: Self = Self(3);

    /// A distinct rendering context. For instance a single timbre in a
    /// multi-timbral instrument, a single loop in a multi looping capable looper
    /// unit, etc.
    #[doc(alias = "kAudioUnitScope_Part")]
    pub const PART: Self = Self(4);

    /// A scope that can be used to apply changes to an individual note. The
    /// elementID used with this scope is the unique note ID returned from a started
    /// note (see MusicDeviceStartNote)
    #[doc(alias = "kAudioUnitScope_Note")]
    pub const NOTE: Self = Self(5);

    /// A context which functions as a layer within a part and allows grouped
    /// control of LayerItem-scope parameters. An example is the percussive attack
    /// layer for an electric organ instrument.
    #[doc(alias = "kAudioUnitScope_Layer")]
    pub const LAYER: Self = Self(6);

    /// A scope which represents an individual element within a particular Layer
    /// scope. The individual sample zones, envelope generators, and filters within
    /// a synth are examples of this.
    #[doc(alias = "kAudioUnitScope_LayerItem")]
    pub const LAYER_ITEM: Self = Self(7);
}

impl au::PropId {
    /// Scope:      Global (or Part for a part scope preset)
    /// Value Type: CFDictionaryRef
    /// Access:     Read / Write
    ///
    /// The complete state of an audio unit if on global scope. An audio unit that supports
    /// part scope, may also support presets on the part scope
    /// that apply to individual parts.
    ///
    /// After a host sets this property it needs to notify the parameter listeners that the values of the parameters of an AU may have changed (so
    /// views, etc, can update their state).
    #[doc(alias = "kAudioUnitProperty_ClassInfo")]
    pub const CLASS_INFO: Self = Self(0);

    /// Scope:      Input
    /// Value Type: AudioUnitConnection
    /// Access:     Write
    #[doc(alias = "kAudioUnitProperty_MakeConnection")]
    pub const MAKE_CONNECTION: Self = Self(1);

    /// Scope:      Input / Output
    /// Value Type: Float64
    /// Access:     Read / Write
    #[doc(alias = "kAudioUnitProperty_SampleRate")]
    pub const SAMPLE_RATE: Self = Self(2);

    /// Scope:      Any
    /// Value Type: AudioUnitParamId
    /// Access:     Read
    #[doc(alias = "kAudioUnitProperty_ParameterList")]
    pub const PARAM_LIST: Self = Self(3);

    /// Scope:      Any
    /// Element:    AudioUnitParamId of the parameter being queried
    /// Value Type: AudioUnitParamInfo
    /// Access:     Read
    ///
    /// The info struct describes the general characteristics of an individual parameter_id
    #[doc(alias = "kAudioUnitProperty_ParameterInfo")]
    pub const PARAM_INFO: Self = Self(4);

    /// Scope:      Global
    /// Value Type: f64
    /// Access:     Read
    ///
    /// Can be used to retrieve the duty cycle (as a value from 0 to 1) of the render
    /// time that an audio unit is spending in its render call.
    #[doc(alias = "kAudioUnitProperty_CPULoad")]
    pub const CPU_LOAD: Self = Self(6);

    /// Scope:      Input / Output
    /// Value Type: audio::StreamBasicDesc
    /// Access:     Read / Write
    #[doc(alias = "kAudioUnitProperty_StreamFormat")]
    pub const STREAM_FORMAT: Self = Self(8);

    /// Scope:      Any (though Global scope will always have an element count of 1)
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Most audio units will only implement the read version of this call, thus they would
    /// have a fixed bus topology (number of input and output elements/buses).
    ///
    /// Some audio units possess the capability to add or remove elements, so in that case
    /// this property will be writable.
    #[doc(alias = "kAudioUnitProperty_ElementCount")]
    pub const ELEMENT_COUNT: Self = Self(11);

    /// Scope:      Global
    /// Value Type: f64
    /// Access:     Read
    ///
    /// The processing latency (the time it takes an audio unit to represent an input in its audio output)
    /// specified in seconds
    #[doc(alias = "kAudioUnitProperty_Latency")]
    pub const LATENCY: Self = Self(12);

    /// Scope:      Global
    /// Value Type: AUChannelInfo array
    /// Access:     Read
    ///
    /// The size of this property will represent the number of AUChannelInfo structs that an audio unit provides.
    /// Each entry describes a particular number of channels on any input, matched to a particular number
    /// of channels on any output. Thus an entry {2, 2} says the audio unit will support a channel configuration
    /// of 2 channels on an input and 2 channels on an output.
    ///
    /// Negative numbers (-1, -2) are used to indicate *any* number of channels. So {-1, -1} means any number
    /// of channels on input and output as long as they are the same. {-1, -2} means any number
    /// of channels on input or output buses
    ///
    /// A negative number less than -2 is used to indicate a total number of channels across every bus on that scope,
    /// regardless of how many channels are set on any particular bus.
    ///
    /// Zero on any side (typically only input) means that the audio unit doesn't have any input elements,
    /// and is expressing the capability of configuring its output channels.
    #[doc(alias = "kAudioUnitProperty_SupportedNumChannels")]
    pub const SUPPORTED_NUM_CHANNELS: Self = Self(13);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// This property is used to describe to an audio unit the maximum number of samples it will be asked
    /// to produce on any single given call to audio unit render.
    ///
    /// If an audio unit can require more or less input data than its output request, then it should limit
    /// any given request for input to this number of frames (that is, it should "break up" its input pulls).
    #[doc(alias = "kAudioUnitProperty_MaximumFramesPerSlice")]
    pub const MAX_FRAMES_PER_SLICE: Self = Self(14);

    /// Scope:      Any
    /// Element:    AudioUnitParameterID of the parameter being queried
    /// Value Type:	cf::ArrayOf<cf::String>
    /// Access:	    Read
    ///
    /// Some audio unit parameters that are of an index type, can also provide names for each value of the parameter.
    /// This property returns an array containing cf::Strings, where each element in the array is the name that should be used
    /// for that parameter value. The size of the array should be the same as the range between the parameters
    /// min and max values
    ///
    /// The array's strings can then be used to build a menu for that parameter.
    #[doc(alias = "kAudioUnitProperty_ParameterValueStrings")]
    pub const PARAM_VALUE_STRINGS: Self = Self(16);

    /// Scope:      Input/Output
    /// Value Type:	struct AudioChannelLayout
    /// Access:     read/write
    ///
    /// Describes for a given scope/element the order of channels within a given stream.
    /// The number of channels it describes must match the number of channels set for that
    /// scope/element. Each input and output bus in an audio unit can have one instance of
    /// this property.
    ///
    /// Some audio units require this property. For example, the 3DMixer unit must
    /// implement this property on its output bus. If a host application attempts to
    /// clear the value of this property on a bus that requires a valid value, the
    /// audio unit will return a kAudioUnitErr_InvalidPropertyValue error.
    ///
    /// Input and output buses can be in one of three states in regard to Audio
    /// channel layout:
    ///
    /// 1. implemented and set
    /// 2. implemented but not set
    /// 3. unimplemented
    ///
    /// Requesting the value of this property when it is implemented but not set
    /// results in a kAudioUnitErr_PropertyNotInUse error.
    ///
    /// Use the kAudioUnitProperty_AudioChannelLayout property whenever channel
    /// layout is relevant. By comparison, the kAudioUnitProperty_StreamFormat
    /// property cannot specify channel layout or purpose.
    ///
    /// See also: kAudioUnitProperty_SupportedChannelLayoutTags,
    /// kAudioUnitProperty_StreamFormat.
    #[doc(alias = "kAudioUnitProperty_AudioChannelLayout")]
    pub const AUDIO_CHANNEL_LAYOUT: Self = Self(19);

    /// Scope:      Global
    /// Value Type:	f64
    /// Access:     Read
    ///
    /// The time in seconds that will remain after the last valid input of any audio unit has been processed
    /// before the output is silent. For example, this could be the total decay time of a reverb or a delay.
    /// In general this will be a conservative estimate.
    #[doc(alias = "kAudioUnitProperty_TailTime")]
    pub const TAIL_TIME: Self = Self(20);

    /// Scope:      Global
    /// Value Type:	u32
    /// Access:     Read / Write
    ///
    /// A boolean value that can be used to bypass the processing in an effect unit, so that the input
    /// is passed unchanged through to the output
    #[doc(alias = "kAudioUnitProperty_BypassEffect")]
    pub const BYPASS_EFFECT: Self = Self(21);

    /// Scope:      Global
    /// Value Type:	os::Status
    /// Access:     Read
    ///
    /// This property is set if there is an error in AudioUnitRender. The AU will then fire a property
    /// changed notification to any listeners on this property and those listeners can then use this
    /// property id to retrieve that error.
    #[doc(alias = "kAudioUnitProperty_LastRenderError")]
    pub const LAST_RENDER_ERROR: Self = Self(22);

    /// Scope:      Input
    /// Value Type:	AURenderCallbackStruct
    /// Access:	    Write
    ///
    /// This is used to provide the audio unit with input on the specified element (input bus) with audio
    /// data from the provided callback. The callback is delivered a buffer list which it must fill in
    /// with audio data. If no data is available, it should set the audio data to 0 (silence).
    /// In the normal case, if an error is returned, the audio is not processed
    /// and the audio unit will return an error from AudioUnitRender.
    #[doc(alias = "kAudioUnitProperty_SetRenderCallback")]
    pub const SET_RENDER_CB: Self = Self(23);

    /// Scope:      Global
    /// Value Type:	cf::Array of AUPreset structures
    /// Access:	    Read
    ///
    /// An array of preset structures that provide a name and number for each preset.
    /// A factory preset is then chosen using the PresentPreset property.
    #[doc(alias = "kAudioUnitProperty_FactoryPresets")]
    pub const FACTORY_PRESETS: Self = Self(24);

    /// Scope:      Global
    /// Value Type:	u32
    /// Access:	    Read / Write
    ///
    /// A value (0 - 127) that can be used to control the quality (complexity) of the rendering operation.
    /// A typical usage is to set render quality to maximum for best quality, but
    /// if CPU usage is a concern a lesser quality can be set to trade off render quality.
    #[doc(alias = "kAudioUnitProperty_RenderQuality")]
    pub const RENDER_QUALITY: Self = Self(26);

    /// Scope:      Global
    /// Value Type:	HostCbInfo
    /// Access:     Write
    ///
    /// The audio unit should only call the host callbacks while it is in its render function.
    /// The audio unit must provide the client info when calling the callbacks as provided
    /// by the host. They are provided as a means for an audio unit to gain information from the host
    /// about parameters that may affect its rendering operation.
    /// For example, what is the current beat of the host, is the transport running, and so forth.
    ///
    /// Any of the parameters of the callback function, when called by the audio unit, can be NULL.
    /// This indicates that the unit doesn't want to know that particular information.
    /// The exception is that the unit must always specify the HostUserData which was be supplied to
    /// the unit when the property was set.
    ///
    /// If the host is unable to provide the requested information then it can return the
    /// kAudioUnitErr_CannotDoInCurrentContext error code
    #[doc(alias = "kAudioUnitProperty_HostCallbacks")]
    pub const HOST_CBS: Self = Self(27);

    /// Scope:      Global
    /// Value Type:	u32
    /// Access:     Read / Write
    ///
    /// A property that can be used to determine if the audio unit can process input data on the same
    /// data as is provided to it, and if so this can be turned off if the host has a particular buffer
    /// management strategy and such an operation would defeat that.
    #[doc(alias = "kAudioUnitProperty_InPlaceProcessing")]
    pub const IN_PLACE_PROCESSING: Self = Self(29);

    /// Scope:      any
    /// Value Type:	&cf::String
    /// Access:	    read/write
    ///
    /// The name of the specified element. The Host owns a reference to this property value
    /// (as with all other CF properties), and should release the string retrieved or used when setting.
    #[doc(alias = "kAudioUnitProperty_ElementName")]
    pub const ELEMENT_NAME: Self = Self(30);

    /// Scope:      Input/Output
    /// Value Type: AudioChannelLayoutTags[ variable number of elements ]
    /// Access:     read only
    ///
    /// Used with GetProperty to ascertain what an audio unit understands about
    /// laying out of channel orders. This will normally return one or more of the specified layout tags.
    ///
    /// When a specific set of layouts are returned, the client then uses the
    /// kAudioUnitProperty_AudioChannelLayout property (with one of those layout tags specified) to set
    /// the unit to use that layout. In this case the client (and the audio unit when reporting its
    /// AudioChannelLayout) is only expected to have set an AudioChannelLayout which only sets the
    /// layout tag as the valid field.
    ///
    /// Custom Channel Maps:
    /// Some audio units may return the tag: kAudioChannelLayoutTag_UseChannelDescriptions
    ///
    /// In this case, the host then can look at supported number of channels on that scope
    /// (using the kAudioUnitProperty_SupportedNumChannels), and supply an AudioChannelLayout with the
    /// kAudioUnitProperty_AudioChannelLayout property to specify the layout, number of channels
    /// and location of each of those channels. This custom channel map MUST have a channel valence
    /// that is supported by the Audio Unit.
    ///
    /// The UseChannelBitmap field is NOT used within the context of the AudioUnit
    #[doc(alias = "kAudioUnitProperty_SupportedChannelLayoutTags")]
    pub const SUPPORTED_CHANNEL_LAYOUT_TAGS: Self = Self(32);

    /// Scope:      Global/Part
    /// Value Type: Preset
    /// Access:     read/write
    ///
    /// This property replaces the deprecated CurrentPreset property, due to the ambiguity of
    /// ownership of the cf::String of the preset name in the older CurrentPreset property.
    /// With PresentPreset the client of the audio unit owns the CFString when it retrieves the
    /// preset with PresentPreset and is expected to release this (as with ALL properties
    /// that retrieve a CF object from an audio unit).
    #[doc(alias = "kAudioUnitProperty_PresentPreset")]
    pub const PRESENT_PRESET: Self = Self(36);

    /// Scope:      any
    /// Value Type: array of AUDependentParameter
    /// Access:     read
    ///
    /// This property is used for parameters with the kAudioUnitParameterFlag_IsGlobalMeta
    /// or kAudioUnitParameterFlag_IsElementMeta flags set. Hosts applications (and the
    /// AudioUnitParameterListener mechanism) can interrogate this property to determine which parameters
    /// are dependent on a meta-parameter.
    ///
    /// For parameters marked with kAudioUnitParameterFlag_IsGlobalMeta, any non-global
    /// dependent parameters are assumed to be dependent in every element of their scope.
    ///
    /// For parameters marked with kAudioUnitParameterFlag_IsElementMeta, then its dependent
    /// parameters must all be the same scope, and are assumed to apply only within a single element,
    /// not to other instances of the same parameter in other elements.
    #[doc(alias = "kAudioUnitProperty_DependentParameters")]
    pub const DEPENDENT_PARAMS: Self = Self(45);

    /// Scope:      Global
    /// Value Type: struct AUInputSamplesInOutputCallbackStruct
    /// Access:     read/write
    ///
    /// An audio unit calls this callback at the end of its render call. The audio unit supplies the
    /// following information:
    ///
    ///    outputTime - The timestamp passed in to the audio unit's render call. This timestamp
    ///                 represents the time of the first output sample.
    ///    inputSample - The sample number of the first input sample that is present in the output
    ///                  audio.
    ///    numInputSamples - The number of input samples that were used and are present in the output
    ///                      audio.
    ///
    /// This property allows a host application to determine which input samples correspond to a sample
    /// in the output buffer. It is useful only for audio units that do time-stretching, such as the
    /// AUVarispeed and AUTimePitch units, where the relationship between input and output samples is
    /// non-trivial. For these units, the range of input samples that correspond to an output buffer
    /// typically differs from the range of input samples that were pulled for that render call.
    /// This difference arises because of internal buffering, processing latency, and other factors.
    #[doc(alias = "kAudioUnitProperty_InputSamplesInOutput")]
    pub const INPUT_SAMPLES_IN_OUTPUT: Self = Self(49);

    /// Scope:      input/output elements (settable per element)
    /// Value Type: u32
    /// Access:     read/write
    ///
    /// By default this value is true. This affects the allocations of the buffers for I/O (the mData field
    /// of the AudioBufferList used with AudioUnitRender, callbacks and connections)
    ///
    /// If true, the element will create a buffer for rendering into.
    ///
    /// If false, the element will not create a buffer for rendering.
    ///
    /// For example, if the audio unit is only ever going to have a connection as its input and never a callback,
    /// then it should not need to create a buffer (the API contract expects an audio unit to provide a buffer for
    /// callbacks, but no buffer for connections).
    ///
    /// If the audio unit is always going to be pulled for audio with the client providing audio data buffers to
    /// the AudioUnitRender call, then it will never need to create an audio buffer on the output side.
    ///
    /// So, this property can be used to control the default allocation strategy of an audio unit. If the audio unit
    /// needs a buffer, but one hasn't been allocated, then an error will be thrown from that call to AudioUnitRender.
    ///
    /// This property cannot be set on Initialised audio units as it may end up reallocating memory.
    #[doc(alias = "kAudioUnitProperty_ShouldAllocateBuffer")]
    pub const SHOULD_ALLOCATE_BUF: Self = Self(51);

    /// Scope:      input/output elements (settable per element)
    /// Value Type: AudioUnitFrequencyResponseBin
    /// Access:     read
    ///
    /// The property provides a way for a user interface view to get points for drawing a graph of the frequency
    /// response of the AU.
    ///
    /// An array of AudioUnitFrequencyResponseBin are passed in to kAudioUnitProperty_FrequencyResponse
    /// with the mFrequency field filled in. The array is returned with the mMagnitude fields filled in.
    /// If fewer than kNumberOfResponseFrequencies are needed, then the first unused bin should be marked with
    /// a negative frequency.
    #[doc(alias = "kAudioUnitProperty_FrequencyResponse")]
    pub const FREQUENCY_RESPONSE: Self = Self(51);

    /// Scope:      Global
    /// Value Type: AudioUnitParameterHistoryInfo
    /// Access:     read
    ///
    /// For parameters which have kAudioUnitParameterFlag_PlotHistory set, getting this property fills out the
    /// AudioUnitParameterHistoryInfo struct containing the recommended update rate and history duration.
    #[doc(alias = "kAudioUnitProperty_ParameterHistoryInfo")]
    pub const PARAM_HISTORY_INFO: Self = Self(53);

    /// Scope:      Global
    /// Value Type: Option<arc::R<cf::String>>
    /// Access:     read/write
    ///
    /// Provides a way for a host to set a custom name on an AU.
    ///
    /// An example of when this is useful is when a host is managing a processing chain that contains multiple AU
    /// instances of the same subtype (and type and manufacturer). The host uses this property to assign a
    /// unique name to each AU instance indicating what that particular instance's function is in the processing
    /// chain and can later query the property to distinguish between AU instances with the same type/subtype/manu
    /// tuple. It is the host's responsibility to keep the names unique if uniqueness is required.
    ///
    /// When getting this property, ownership follows Core Foundation's 'Copy Rule'. This property may return NULL
    /// which indicates that no name has been set on the AU.
    #[doc(alias = "kAudioUnitProperty_NickName")]
    pub const NICK_NAME: Self = Self(54);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// This is used by the host to indicate when an audio unit (that normally operates within a general real-time
    /// calling model) is rendering in an offline context. A typical usage of this is to set this to true when
    /// the rendering operation an audio unit is being used within is going to write out the results to a file.
    /// The value defaults to false, as the common usage of audio units is for real-time processing
    #[doc(alias = "kAudioUnitProperty_OfflineRender")]
    pub const OFFLINE_RENDER: Self = Self(37);

    /// Scope:      any
    /// Value Type: AudioUnitParameterIDName
    /// Access:     read
    ///
    /// An audio unit returns the full parameter name in the GetParameterInfo struct/property.
    /// In some display situations however, there may only be room for a few characters, and
    /// truncating this full name may give a less than optimal name for the user. Thus,
    /// this property can be used to ask the audio unit whether it can supply a truncated name, with
    /// the host suggesting a length (number of characters). If the unit returns a longer
    /// name than the host requests, that name may be truncated to the requested characters in display.
    /// The unit could return a shorter name than requested as well. The unit returns a CFString
    /// that should be released by the host. When using this property, the host asks for
    /// the name in the same scope and element as the unit publishes the parameter.
    #[doc(alias = "kAudioUnitProperty_ParameterIDName")]
    pub const PARAM_ID_NAME: Self = Self(34);

    /// Scope:      any
    /// Value Type: AudioUnitParameterStringFromValue
    /// Access:     read
    ///
    /// This property is used with parameters that are marked with the
    /// kAudioUnitParameterFlag_HasName parameter info flag. This indicates that some
    /// (or all) of the values represented by the parameter can and should be
    /// represented by a special display string.
    ///
    /// This is NOT to be confused with kAudioUnitProperty_ParameterValueStrings. That property
    /// is used with parameters that are indexed and is typically used for instance to build
    /// a menu item of choices for one of several parameter values.
    ///
    /// kAudioUnitProperty_ParameterStringFromValue can have a continuous range, and merely states
    /// to the host that if it is displaying those parameter's values, they should request
    /// a name any time any value of the parameter is set when displaying that parameter.
    ///
    /// For instance (a trivial example), a unit may present a gain parameter in a dB scale,
    /// and wish to display its minimum value as "negative infinity". In this case, the audio unit
    /// will not return names for any parameter value greater than its minimum value - so the host
    /// will then just display the parameter value as is. For values less than or equal to the
    /// minimum value, the audio unit will return a string for "negative infinity" which the host can
    /// use to display appropriately.
    ///
    /// A less trivial example might be a parameter that presents its values as seconds. However,
    /// in some situations this value should be better displayed in a SMPTE style of display:
    /// HH:MM:SS:FF
    /// In this case, the audio unit would return a name for any value of the parameter.
    ///
    /// The GetProperty call is used in the same scope and element as the inParamID
    /// that is declared in the struct passed in to this property.
    ///
    /// If the *inValue member is NULL, then the audio unit should take the current value
    /// of the specified parameter. If the *inValue member is NOT NULL, then the audio unit should
    /// return the name used for the specified value.
    ///
    /// On exit, the outName may point to a CFStringRef (which if so must be released by the caller).
    /// If the parameter has no special name that should be applied to that parameter value,
    /// then outName will be NULL, and the host should display the parameter value as
    /// appropriate.
    #[doc(alias = "kAudioUnitProperty_ParameterStringFromValue")]
    pub const PARAM_STRING_FROM_VALUE: Self = Self(33);

    /// Scope:      any
    /// Value Type: AudioUnitParameterIDName
    /// Access:     read
    ///
    /// This works in a similar manner to the ParameterIDName property, except that the inID
    /// value is one of the clumpID's that are returned with the audio unit's ParameterInfo
    /// structure.
    #[doc(alias = "kAudioUnitProperty_ParameterClumpName")]
    pub const PARAM_CLUMP_NAME: Self = Self(35);

    /// Scope:      any
    /// Value Type: AudioUnitParameterValueFromString
    /// Access:     read
    ///
    /// This property returns the value of a parameter from its string representation.
    /// See kAudioUnitProperty_ParameterStringFromValue.
    #[doc(alias = "kAudioUnitProperty_ParameterValueFromString")]
    pub const PARAM_VALUE_FROM_STRING: Self = Self(38);

    /// Scope:      Global
    /// Value Type: cf::String
    /// Access:     Read / Write
    ///
    /// The host can set this as information to the audio unit to describe something about the context
    /// within which the audio unit is instantiated. For instance, "track 3" could
    /// be set as the context, so that the audio unit's view could then display "My audio unit on track 3"
    /// as information to the user of the particular context for any audio unit.
    #[doc(alias = "kAudioUnitProperty_ContextName")]
    pub const CONTEXT_NAME: Self = Self(25);

    /// Scope:      Input/Output
    /// Value Type: f64
    /// Access:     write
    ///
    /// This property is set by a host to describe to the audio unit the presentation latency of both
    /// any of its input and/or output audio data.
    /// It describes this latency in seconds. A value of zero means either no latency
    /// or an unknown latency.
    ///
    /// This is a write only property because the host is telling the audio unit the latency of both the
    /// data it provides it for input and the latency from getting the data from the unit until it is
    /// presented.
    ///
    /// The property is should be set on each active input and output bus (Scope/Element pair).
    /// For example, an audio unit with multiple outputs will have the output data it produces processed
    /// by different audio units, etc before it is mixed and presented. Thus, in this case, each output
    /// element could have a different presentation latency.
    ///
    /// This should not be confused with the Latency property, where the audio unit describes to the host
    /// any processing latency it introduces between its input and its output.
    ///
    /// For input:
    /// Describes how long ago the audio given to an audio unit was acquired. For instance, when
    /// reading from a file to the first audio unit, then its input presentation latency will be zero.
    /// When processing audio input from a  device, then this initial input latency will be the
    /// presentation latency of the device itself, the device's safety offset and latency.
    ///
    /// The next audio unit's (connected to that first unit) input presentation latency will be the
    /// input presentation latency of the first unit, plus the processing latency (as expressed by
    /// kAudioUnitProperty_Latency) of the first unit.
    ///
    /// For output:
    /// Describes how long before the output audio of an audio unit is to be presented. For instance,
    /// when writing to a file, then the last audio unit's output presentation latency will be zero.
    /// When the audio from that audio unit is to be played to an AudioDevice, then that initial
    /// presentation latency will be the latency of the device itself - which is the I/O buffer size,
    /// and the device's safety offset and latency
    ///
    /// The previous audio unit's (connected to this last unit) output presentation latency will be that
    /// initial presentation latency plus the processing latency (as expressed by
    /// kAudioUnitProperty_Latency) of the last unit.
    ///
    /// So, for a given audio unit anywhere within a mixing graph, the input and output presentation
    /// latencies describe to that unit how long from the moment of generation it will take for its
    /// input to arrive, and how long it will take for its output to be presented.
    ///
    /// You can use this property, for example, to provide metering for an audio unit that
    /// is generating output to be presented to the user at a future time.
    #[doc(alias = "kAudioUnitProperty_PresentationLatency")]
    pub const PRESENTATION_LATENCY: Self = Self(40);

    /// Scope:      Global
    /// Value Type: cf::Dictionary
    /// Access:     read/write
    ///
    /// If the audio unit implements this property then it is going to do different actions establishing
    /// its state from a document rather than from a user preset. Thus, a host app should use this property
    /// first (instead of kAudioUnitProperty_ClassInfo) when restoring the state of an audio unit when
    /// opening a document. If the audio unit returns an error (or doesn't implement this property) then
    /// the host should use the same preset with the kAudioUnitProperty_ClassInfo.
    #[doc(alias = "kAudioUnitProperty_ClassInfoFromDocument")]
    pub const CLASS_INFO_FROM_DOCUMENT: Self = Self(50);

    /// Scope:			Global
    /// Value Type:		block: void (^)(AUViewControllerBase *)
    /// Access:			write
    ///
    /// If the audio unit is implemented using the version 3 API, it may provide a
    /// view controller, returned via this property. As with any other CoreFoundation
    /// or Foundation object returned by AudioUnitGetProperty, the caller must
    /// release the returned reference (i.e. the Copy rule is used).
    #[doc(alias = "kAudioUnitProperty_RequestViewController")]
    pub const REQUEST_VIEW_CONTROLLER: Self = Self(56);

    /// Scope:      Global
    /// Value Type: variably-sized array of struct AudioUnitParameter
    /// Access:     read
    ///
    /// A host may query an audio unit for a list of its N most important
    /// parameters, via this property. The size of the array passed to
    /// AudioUnitGetProperty controls the number of AudioUnitParameter values
    /// returned.
    #[doc(alias = "kAudioUnitProperty_ParametersForOverview")]
    pub const PARAMS_FOR_OVERVIEW: Self = Self(57);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     read
    ///
    /// Indicates whether an audio unit supports Multi-dimensional Polyphonic Expression.
    #[doc(alias = "kAudioUnitProperty_SupportsMPE")]
    pub const SUPPORTS_MPE: Self = Self(58);

    /// Scope:      Global
    /// Value Type: AURenderContextObserver
    /// Access:     read-only
    ///
    /// Audio Units which create auxiliary realtime rendering threads should
    /// implement this property to return a block which will be called by the OS
    /// when the render context changes. Audio Unit hosts must not attempt to
    /// interact with the AudioUnit through this block; it is for the exclusive use
    /// of the OS.
    #[doc(alias = "kAudioUnitProperty_RenderContextObserver")]
    pub const RENDER_CONTEXT_OBSERVER: Self = Self(60);

    /// Scope:      Global
    /// Value Type: f64
    /// Access:     read-only
    ///
    /// The absolute sample frame time of the most recent render timestamp.
    #[doc(alias = "kAudioUnitProperty_LastRenderSampleTime")]
    pub const LAST_RENDER_SAMPLE_TIME: Self = Self(61);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     read-only
    ///
    /// Indicates whether an Audio Unit is loaded out-of-process, which might happen
    /// at the request of the host or when loading in-process is not possible.
    #[doc(alias = "kAudioUnitProperty_LoadedOutOfProcess")]
    pub const LOADED_OUT_OF_PROCESS: Self = Self(62);

    /// Scope:			Global
    /// Value Type:		void* (function pointer)
    /// Access:			Read
    ///
    /// The caller provides the selector for a given audio unit API, and retrieves a function pointer for that selector. For instance,
    /// this enables the caller to retrieve the function pointer for the AudioUnitRender call, so that call can be made directly
    /// through to the audio unit to avoid the overhead of the ComponentMgr's dispatch.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_FastDispatch")]
    pub const FAST_DISPATCH: Self = Self(5);

    /// Scope:      Global
    /// Value Type: AudioUnitExternalBuffer
    /// Access:     Write
    ///
    /// This is used to provide to an audio unit a buffer that it can use with its input render callback's audio buffer list
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_SetExternalBuffer")]
    pub const SET_EXTERNAL_BUF: Self = Self(15);

    /// Scope:      Any
    /// Value Type: AudioComponentDescription array
    /// Access:     Read
    ///
    /// Presents an array of AudioComponentDescription that are of type 'auvw' (AudioUnitCarbonView).
    /// These are the carbon based custom views for that audio unit.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_GetUIComponentList")]
    pub const GET_UI_COMPONENT_LIST: Self = Self(18);

    /// Scope:				Global
    /// Value Type:			struct AudioUnitCocoaViewInfo
    /// Access:				read
    ///
    /// Publishes the audio unit's custom Cocoa NSViews. The Host can determine how big this structure is by
    /// querying the size of the property (i.e., How many alternate UI classes there are for the unit)
    /// Typically, most audio units will provide 1 UI class per unit
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_CocoaUI")]
    pub const COCOA_UI: Self = Self(31);

    /// Scope:      Global
    /// Value Type: &cf::Url
    /// Access:     Read
    ///
    /// A URL that will specify the location of an icon file that can be used when presenting
    /// UI for this audio unit.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_IconLocation")]
    pub const ICON_LOCATION: Self = Self(39);

    /// Scope:      Global
    /// Value Type: AUHostVersionIdentifier
    /// Access:     write
    ///
    /// Determine which application (and which version) an audio unit is being hosted by.
    /// This is made more complex through the intervention of audio units such as Kore, that are hosting
    /// other audio units (in this case of course, the real host of the audio unit is the hosting unit,
    /// not the host application, so the previous mechanism of getting the main bundle ID is no longer
    /// correct).
    ///
    /// There are also inconsistencies in the way that bundle identifiers are applied (with apps changing
    /// these from version to version), and we'd prefer to see a more consistent identifier used with
    /// this property. This is in spirit similar to the string returned by CFBundle API, except that we
    /// require this host string be consistent and reliable through different revisions of the host.
    ///
    /// The audio unit is responsible for retaining the hostName string if it needs to use it past the
    /// duration of the actual call. The host should set this property as early as possible within the
    /// lifetime of the unit in a session.
    ///
    /// This API used to take a NumVersion struct. It is redefined to take an AUHostVersionIdentifier struct
    /// which is binary compatible with the existing usage, but not source compatible.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_AUHostIdentifier")]
    pub const AU_HOST_IDENTIFIER: Self = Self(46);

    /// Scope:      Global
    /// Value Type: &cf::Array
    /// Access:     read
    ///
    /// Used to determine how many MIDI output streams the audio unit can generate  (and the name for
    /// each of these outputs). Each MIDI output is a complete MIDI or MIDIEventList data stream, such as embodied
    /// by a  MIDIEndpointRef in CoreMIDI.
    ///
    /// The host can retrieve an array of CFStringRefs published by the audio unit, where :
    ///    - the size of the array is the number of MIDI Outputs the Audio Unit supports
    ///    - each item in the array is the name for that output at that index
    ///
    /// The host owns this array and its elements and should release them when it is finished.
    ///
    /// Once the host has determined that the Audio Unit supports this feature, it can then provide a
    /// callback, through which the audio unit can send the MIDI data.
    /// See the documentation for the kAudioUnitProperty_MIDIOutputCallback property.
    #[doc(alias = "kAudioUnitProperty_MIDIOutputCallbackInfo")]
    pub const MIDI_OUTPUT_CB_INFO: Self = Self(47);

    /// Scope:      Global
    /// Value Type: AUMIDIOutputCallbackStruct
    /// Access:     write
    ///
    /// The host sets this property on the audio unit with the callback (and its user data) set
    /// appropriately.
    ///
    /// Operational Parameters:
    /// In the render call, just as is the expected usage of the AUHostCallbacks, the audio unit can
    /// call the provided callback to provide MIDI data to the host that it will associate with the
    /// current AudioUnitRender call in process.
    ///
    /// The audio unit in the callback provides:
    ///     - the user data provided by the host when the callback was established
    ///     - the AudioTimeStamp that was provided to the audio unit for this particular call of
    ///       AudioUnitRender
    ///     - the output number to associate this MIDI data with
    ///     - a MIDI Packet List containing MIDI data. The time stamp values contained within the
    ///       MIDIPackets in this list are **sample offsets*** from the AudioTimeStamp provided.
    ///       This allows MIDI data to be time-stamped with a sample offset that is directly associated
    ///       with the audio data it is generating in the current call to the AudioUnitRender function
    ///
    /// There is no implied or expected association between the number (or position) of an audio unit's
    /// audio or MIDI outputs.
    #[doc(alias = "kAudioUnitProperty_MIDIOutputCallback")]
    pub const MIDI_OUTPUT_CB: Self = Self(48);

    /// Scope:      Global
    /// Value Type: AUMIDIEventListBlock
    /// Access:     write
    ///
    /// The host sets this property on the Audio Unit with the callback set appropriately.
    ///
    /// Operational Parameters:
    /// In the render call, just as is the expected usage of the AUHostCallbacks, the audio unit can
    /// call the provided callback to provide MIDIEventList data to the host that it will associate with the
    /// current AudioUnitRender.
    ///
    /// The Audio Unit in the callback provides:
    ///  - the AUEventSampleTime that was provided to the audio unit for this particular call of
    ///    AudioUnitRender
    ///  - the output number to associate this MIDI data with
    ///  - a MIDIEventList containing MIDI data. The time stamp values contained within the
    ///    MIDIEventPacket in this list are **sample offsets*** from the AudioTimeStamp provided.
    ///    This allows MIDI data to be time-stamped with a sample offset that is directly associated
    ///    with the audio data it is generating in the current call to the AudioUnitRender function
    ///
    /// Host should setup in the following order:
    ///  - Set desired host MIDI protocol using kAudioUnitProperty_HostMIDIProtocol
    ///  - Set kAudioUnitProperty_MIDIOutputEventListCallback
    ///  - Initialize the Audio Unit
    ///
    /// Notes:
    ///  - kAudioUnitProperty_HostMIDIProtocol cannot be changed while the Audio Unit is initialized.
    ///  - The Audio Unit takes ownership of the provided block.
    ///  - kAudioUnitProperty_HostMIDIProtocol should be set before attempting to query
    ///    kAudioUnitProperty_AudioUnitMIDIProtocol or calling AudioUnitInitialize to allow Audio Units to
    ///    optionally match their input MIDI protocol to the desired host protocol and prevent protocol conversion.
    ///
    /// There is no implied or expected association between the number (or position) of an audio unit's
    /// audio or MIDI outputs.
    ///
    /// Compare to property kAudioUnitProperty_MIDIOutputCallback.
    #[doc(alias = "kAudioUnitProperty_MIDIOutputEventListCallback")]
    pub const MIDI_OUTPUT_EVENT_LIST_CB: Self = Self(63);

    /// Scope:      Global
    /// Value Type: i32
    /// Access:     read
    ///
    /// A signed 32-bit integer representing the audio unit's MIDI protocol. This should be one of the
    /// values in the MIDIProtocolID enum (see <CoreMIDI/MIDIServices.h>)..
    ///
    /// The framework will convert all incoming MIDI data to the protocol set in this property, the host can query
    /// this property to detect the audio unit's current MIDI protocol.
    ///
    /// Note: This property should not be changed after the audio has been initialized.
    #[doc(alias = "kAudioUnitProperty_AudioUnitMIDIProtocol")]
    pub const AUDIO_UNIT_MIDI_PROTOCOL: Self = Self(64);

    /// Scope:      Global
    /// Value Type: i32
    /// Access:     write
    ///
    /// A signed 32-bit integer representing the hosts MIDI protocol. This should be set to one of the values
    /// in the MIDIProtocolID enum (see <CoreMIDI/MIDIServices.h>).
    ///
    /// Hosts should set this property to the protocol that MIDI data is desired to be delivered in. The framework will
    /// convert all MIDI data sent to the host to the protocol value set in this property, an audio unit can query
    /// this property to detect the hosts MIDI protocol.
    ///
    /// Host should setup in the following order:
    /// - Set desired host MIDI protocol using kAudioUnitProperty_HostMIDIProtocol
    /// - Set kAudioUnitProperty_MIDIOutputEventListCallback
    /// - Initialize the Audio Unit
    ///
    /// Notes:
    /// - kAudioUnitProperty_HostMIDIProtocol cannot be changed after the audio unit has been initialized.
    /// - kAudioUnitProperty_HostMIDIProtocol should be set before attempting to query
    /// kAudioUnitProperty_AudioUnitMIDIProtocol or calling AudioUnitInitialize to allow Audio Units to
    /// optionally match their input MIDI protocol to the desired host protocol and prevent protocol conversion.
    #[doc(alias = "kAudioUnitProperty_HostMIDIProtocol")]
    pub const HOST_MIDI_PROTOCOL: Self = Self(65);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     read/write
    ///
    /// This property allows the plug-in to provide a hint to the framework regarding the size of
    /// its outgoing MIDI data buffer.
    ///
    /// If the plug-in produces more MIDI output data than the default size of the allocated buffer,
    /// then the plug-in can provide this property to increase the size of this buffer.
    ///
    /// The value represents the number of 3-byte Legacy MIDI messages that fit into the buffer or
    /// a single MIDIEventList containing 1 MIDIEventPacket of 2 words when using MIDI 2.0 (MIDIEventList based API's).
    ///
    /// This property is set to the default value by the framework.
    ///
    /// In case of kAudioUnitErr_MIDIOutputBufferFull errors caused by producing too much MIDI
    /// output in one render call, set this property to increase the buffer.
    ///
    /// This only provides a recommendation to the framework.
    #[doc(alias = "kAudioUnitProperty_MIDIOutputBufferSizeHint")]
    pub const MIDI_OUTPUT_BUF_SIZE_HINT: Self = Self(66);
}

/// Inter-App Audio Property IDs
///
/// Properties used in inter-app audio.
impl au::PropId {
    /// Scope:      Global
    /// Value Type: AudioUnitRemoteControlEventListener
    /// Access:     read/write
    ///
    /// Provides a way for a host to receive AudioUnitRemoteControlEvents from a remote "node"
    /// audio unit. The supplied listener block is called when the audio unit sends a remote
    /// control event.
    #[doc(alias = "kAudioUnitProperty_RemoteControlEventListener")]
    pub const REMOTE_CONTROL_EVENT_LISTENER: Self = Self(100);

    /// Scope: Global
    /// Value Type: UInt32 (0-1)
    /// Access: read-only
    ///
    /// Both host and node apps can query and listen to this property to determine when
    /// the audio unit has been connected to another app.
    #[doc(alias = "kAudioUnitProperty_IsInterAppConnected")]
    pub const IS_INTER_APP_CONNECTED: Self = Self(101);

    /// Scope:      Global
    /// Value Type: &cf::Url
    /// Access:     read-only
    ///
    /// Both host and node apps can query this property to obtain a URL which, when
    /// opened, will activate the other app.
    ///
    /// N.B. This URL is only valid while the host has an open connection to the node
    #[doc(alias = "kAudioUnitProperty_PeerURL")]
    pub const PEER_URL: Self = Self(102);
}

/// The collection of properties for offline units
impl au::PropId {
    /// Scope:      Global
    /// Value Type: u64
    /// Access:     read/write
    ///
    /// Once this property is set, an audio unit will assume that its input samples
    /// have been reset to a new region. Setting this property will also cause the
    /// audio unit's internal DSP state to be reset. That is, the audio unit calls
    /// the AudioUnitReset function on itself.
    ///
    /// This property tells the offline unit how many samples to process. Once it
    /// knows this number it will then request from 0 to (nSamples-1) in its input
    /// callback. The host of the audio unit is then required to provide the samples
    /// specified in the sample count field of that Input's callback.
    #[doc(alias = "kAudioUnitOfflineProperty_InputSize")]
    pub const OFFLINE_INPUT_SIZE: Self = Self(3020);

    /// Scope:      Global
    /// Value Type: u64
    /// Access:     read
    ///
    /// The host can use this property to estimate how many output samples an audio
    /// unit will produce for the specified input samples. The property value
    /// is invalid if InputSize is not set.
    ///
    /// The host cannot assume that the value returned is exact.
    /// It is a guide only, so is suitable for use in a progress bar, for instance.
    ///
    /// Termination of processing is solely determined by the setting of the
    /// kAudioUnitStatus_OfflineRenderComplete property in the
    /// ioRenderActionFlags from the AudioUnitRender function.
    #[doc(alias = "kAudioUnitOfflineProperty_OutputSize")]
    pub const OFFLINE_OUTPUT_SIZE: Self = Self(3021);

    /// Scope:      Global
    /// Value Type: u64
    /// Access:     read/write
    ///
    /// The host sets this property to tell an audio unit that the start offset of
    /// the data it is processing has been changed. This should be set along with
    /// the InputSize property, so that the unit knows its input data has been set
    /// or changed.
    #[doc(alias = "kAudioUnitOfflineProperty_StartOffset")]
    pub const OFFLINE_START_OFFSET: Self = Self(3022);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     read
    ///
    /// Returns one of the kOfflinePreflight_ results (see the Offline Preflight
    /// enumeration).
    #[doc(alias = "kAudioUnitOfflineProperty_PreflightRequirements")]
    pub const OFFLINE_PREFLIGHT_REQUIREMENTS: Self = Self(3023);

    /// Scope:      Global
    /// Value Type: &cf::String
    /// Access:     read
    ///
    /// For an audio unit that allows or requires preflighting, this property lets
    /// the unit give its host application a name to describe the preflight
    /// operations.
    #[doc(alias = "kAudioUnitOfflineProperty_PreflightName")]
    pub const OFFLINE_PREFLIGHT_NAME: Self = Self(3024);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum OfflinePreflight {
    NotRequired = 0,
    Optional = 1,
    Required = 2,
}

/// Apple AUConverter Property IDs
///
/// The collection of property IDs for Apple AUConverter
impl au::PropId {
    /// Scope:      Global
    /// Value Type: u32
    /// Access:     read/write
    #[doc(alias = "kAudioUnitProperty_SampleRateConverterComplexity")]
    pub const SAMPLE_RATE_CONVERTER_COMPLEXITY: Self = Self(3014);
}

#[doc(alias = "kAudioUnitSampleRateConverterComplexity")]
pub mod sample_rate_converter_complexity {
    /// Linear interpolation
    #[doc(alias = "kAudioUnitSampleRateConverterComplexity_Linear")]
    pub const LINEAR: u32 = u32::from_be_bytes(*b"line");

    /// Normal quality sample rate conversion.
    /// The default
    #[doc(alias = "kAudioUnitSampleRateConverterComplexity_Normal")]
    pub const NORMAL: u32 = u32::from_be_bytes(*b"norm");

    #[doc(alias = "kAudioUnitSampleRateConverterComplexity_Mastering")]
    /// Mastering quality sample rate conversion. More expensive.
    pub const MASTERING: u32 = u32::from_be_bytes(*b"bats");
}

/// The collection of property IDs for Apple input/output units.
impl au::PropId {
    #[doc(alias = "kAudioOutputUnitProperty_CurrentDevice")]
    pub const OUTPUT_CURRENT_DEVICE: Self = Self(2000);

    #[doc(alias = "kAudioOutputUnitProperty_IsRunning")]
    pub const OUTPUT_IS_RUNNING: Self = Self(2001);

    #[doc(alias = "kAudioOutputUnitProperty_ChannelMap")]
    pub const CHANNEL_MAP: Self = Self(2002);

    #[doc(alias = "kAudioOutputUnitProperty_EnableIO")]
    pub const OUTPUT_ENABLE_IO: Self = Self(2003);

    #[doc(alias = "kAudioOutputUnitProperty_StartTime")]
    pub const OUTPUT_START_TIME: Self = Self(2004);

    #[doc(alias = "kAudioOutputUnitProperty_SetInputCallback")]
    pub const OUTPUT_SET_INPUT_CB: Self = Self(2005);

    #[doc(alias = "kAudioOutputUnitProperty_HasIO")]
    pub const OUTPUT_HAS_IO: Self = Self(2006);

    #[doc(alias = "kAudioOutputUnitProperty_StartTimestampsAtZero")]
    pub const OUTPUT_START_TS_AT_ZERO: Self = Self(2007);

    #[doc(alias = "kAudioOutputUnitProperty_OSWorkgroup")]
    pub const OUTPUT_OS_WORKGROUP: Self = Self(2015);

    #[doc(alias = "kAudioOutputUnitProperty_MIDICallbacks")]
    pub const OUTPUT_MIDI_CBS: Self = Self(2010);

    #[doc(alias = "kAudioOutputUnitProperty_HostReceivesRemoteControlEvents")]
    pub const OUTPUT_HOST_RECEIVES_REMOVE_CONTROL_EVENTS: Self = Self(2011);

    #[doc(alias = "kAudioOutputUnitProperty_RemoteControlToHost")]
    pub const OUTPUT_REMOTE_CONTROL_HOST: Self = Self(2012);

    #[doc(alias = "kAudioOutputUnitProperty_HostTransportState")]
    pub const OUTPUT_HOST_TRANSPORT_STATE: Self = Self(2013);

    #[doc(alias = "kAudioOutputUnitProperty_NodeComponentDescription")]
    pub const OUTPUT_NODE_COMPONENT_DESC: Self = Self(2013);
}
/// The collection of property IDs for Apple voice processing units.
impl au::PropId {
    #[doc(alias = "kAUVoiceIOProperty_BypassVoiceProcessing")]
    pub const VOICE_IO_BYPASS_VOICE_PROCESSING: Self = Self(2100);

    #[doc(alias = "kAUVoiceIOProperty_VoiceProcessingEnableAGC")]
    pub const VOICE_IO_ENABLE_AGC: Self = Self(2101);

    #[doc(alias = "kAUVoiceIOProperty_MuteOutput")]
    pub const VOICE_IO_MUTE_OUTPUT: Self = Self(2104);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum VoiceIoSpeechActivityEvent {
    #[doc(alias = "kAUVoiceIOSpeechActivityHasStarted")]
    Started = 0,
    #[doc(alias = "kAUVoiceIOSpeechActivityHasEnded")]
    Ended = 1,
}

pub type VoiceIoMutedSpeechActivityEventListener =
    crate::blocks::EscBlock<fn(VoiceIoSpeechActivityEvent)>;

impl au::PropId {
    /// Scope: Global
    /// Value Type: au::VoiceIOMutedSpeechActivityEventListener
    /// Access: write only
    ///
    /// Register a listener to be notified when speech activity event occurs while the client is muted.
    /// Continuous presence of or lack of speech activity during mute will not cause redundant notification.
    /// In order to use this API, it's expected to implement the mute via the kAUVoiceIOProperty_MuteOutput.
    #[doc(alias = "kAUVoiceIOProperty_MutedSpeechActivityEventListener")]
    pub const VOICE_IO_MUTED_SPEECH_ACTIVITY_EVENT_LISTENER: Self = Self(2106);
}

/// Ducking level applied to other (i.e. non-voice) audio by AUVoiceIO.
pub mod voice_io_other_audio_ducking_level {

    #[doc(alias = "kAUVoiceIOOtherAudioDuckingLevelDefault")]
    pub const DEFAULT: u32 = 0;

    #[doc(alias = "kAUVoiceIOOtherAudioDuckingLevelMin")]
    pub const MIN: u32 = 10;

    #[doc(alias = "kAUVoiceIOOtherAudioDuckingLevelMid")]
    pub const MID: u32 = 20;

    #[doc(alias = "kAUVoiceIOOtherAudioDuckingLevelMax")]
    pub const MAX: u32 = 30;
}

#[doc(alias = "AUVoiceIOOtherAudioDuckingConfiguration")]
pub struct VoiceIoOtherAudioDuckingCfg {
    pub enable_advanced_ducking: bool,
    pub ducking_level: u32,
}

impl au::PropId {
    /// Scope: Global
    /// Value Type: au::VoiceIoOtherAudioDuckingCfg
    /// Access: read/write
    ///
    /// Configures the ducking of other (i.e. non-voice) audio, including advanced ducking enablement and ducking level.
    /// In general, when other audio is played during voice chat, applying a higher level of ducking could increase the intelligibility of the voice chat.
    /// If not set, the default ducking configuration is to disable advanced ducking, with a ducking level set to kAUVoiceIOOtherAudioDuckingLevelDefault.
    #[doc(alias = "kAUVoiceIOProperty_OtherAudioDuckingConfiguration")]
    pub const VOICE_IO_OTHER_AUDIO_DUCKING_CFG: Self = Self(2108);
}

/// The collection of property IDs for the Apple N-Band EQ Audio Unit.
impl au::PropId {
    /// Scope: Global
    /// Value Type: u32
    /// Access: read/write
    ///
    /// Specifies the number of equalizer bands. If more than kAUNBandEQProperty_MaxNumberOfBands
    /// are specified, an error is returned.
    /// Can only be set if the unit is uninitialized.
    #[doc(alias = "kAUNBandEQProperty_NumberOfBands")]
    pub const BAND_EQ_NUMBER_OF_BANDS: Self = Self(2200);

    /// Scope: Global
    /// Value Type: u32
    /// Access: read-only
    ///
    /// Returns the maximum number of equalizer bands.
    #[doc(alias = "kAUNBandEQProperty_MaxNumberOfBands")]
    pub const BAND_EQ_MAX_NUMBER_OF_BANDS: Self = Self(2201);

    /// Scope: Global
    /// Value Type: array of f64
    /// Access: read-only
    ///
    /// Returns an array of f64 values, 5 per band.
    #[doc(alias = "kAUNBandEQProperty_BiquadCoefficients")]
    pub const BAND_EQ_BIQUAD_COEFFICIENTS: Self = Self(2203);
}

/// The collection of property IDs for Apple mixers
impl au::PropId {
    /// Scope: { scope / element }
    /// Value Type: u32
    /// Access: read/write
    ///
    /// Enable or disable metering on a particular scope/element
    #[doc(alias = "kAudioUnitProperty_MeteringMode")]
    pub const METERING_MODE: Self = Self(3007);

    /// This property can be used for both the AUMatrixMixer and AUMultiChannelMixer.
    ///
    /// AUMatrixMixer
    /// Scope:      Global
    /// Value Type: f32 array
    /// Access:     read/write
    ///
    /// This property is used to retrieve the entire state of a matrix mixer. The size required is
    /// the number of (input  channels + 1) * (output channels + 1) - see _MatrixDimensions
    ///
    /// So a matrix mixer that has 2 input channels and 2 output channels, will need a 3 x 3 array of Float32
    ///
    /// Global volume is stored at volumes[2][2]
    /// Input volumes are stored in the last column (volumes[0][2] for the first input channel,  volumes[1][2] for the second)
    /// Output volumes are stored in the last row (volumes [2][0] and [2][1])
    /// Cross point volumes are stored at their expected locations ([0][1], etc)
    ///
    /// AUMultiChannelMixer
    /// Scope:      Input
    /// Value Type: f32 array
    /// Access:     read/write
    ///
    /// Gets/sets the matrix levels for one input element. This allows arbitrary mixing configurations
    /// from all input channels to all output channels.
    /// The size required is the number of (input channels) * (output channels).
    /// The matrix stores only the crosspoint gains, there are no overall input or output channel gains.
    #[doc(alias = "kAudioUnitProperty_MatrixLevels")]
    pub const MATRIX_LEVELS: Self = Self(3006);

    /// Scope:      Global
    /// Value Type: 2 x u32
    /// Access:     Read only
    ///
    /// Returns the total number of channels for input and output of a given matrix mixer
    #[doc(alias = "kAudioUnitProperty_MatrixDimensions")]
    pub const MATRIX_DIMENSIONS: Self = Self(3009);

    /// Scope:      Global
    /// Value Type: AudioUnitMeterClipping
    /// Access:     Read
    ///
    /// A mixer returns an au::MeterClipping structure.
    #[doc(alias = "kAudioUnitProperty_MeterClipping")]
    pub const METER_CLIPPING: Self = Self(3011);

    /// Multichannel Mixer
    /// Scope:			Input
    /// Value Type:		AudioTimeStamp
    /// Access:			Read / Write
    ///
    /// There are situations, for example moving an input between mixers, where the
    /// input's sample time timeline needs to be made continuous. This facilitates
    /// the operation: after disconnecting the input, fetch its anchor time stamp,
    /// then before reconnecting it, set the same anchor time stamp. The input's
    /// sequence of sample times will be maintained.
    ///
    /// This property cannot be accessed while the input is rendering.
    #[doc(alias = "kAudioUnitProperty_InputAnchorTimeStamp")]
    pub const INPUT_ANCHOR_TS: Self = Self(3011);
}

#[doc(alias = "AudioUnitMeterClipping")]
#[repr(C)]
pub struct MeterClipping {
    pub peak_value_since_last_call: f32,
    pub saw_infinity: bool,
    pub saw_nan: bool,
}

/// SpatialMixer
impl au::PropId {
    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    #[doc(alias = "kAudioUnitProperty_ReverbRoomType")]
    pub const REVERB_ROOM_TYPE: Self = Self(10);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    #[doc(alias = "kAudioUnitProperty_UsesInternalReverb")]
    pub const USES_INTERNAL_REVERB: Self = Self(1005);

    /// Scope:      Input
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Used to set the spatialisation algorithm used by an input of AUSpatialMixer.
    /// See kSpatializationAlgorithm_
    #[doc(alias = "kAudioUnitProperty_SpatializationAlgorithm")]
    pub const SPATIALIZATION_ALGORITHM: Self = Self(3000);

    /// Scope:			Input
    /// Value Type:		UInt32
    /// Access:			Read / Write
    ///
    /// Used to enable various rendering operations on a given input for the 3DMixer.
    /// See k3DMixerRenderingFlags_
    #[doc(alias = "kAudioUnitProperty_SpatialMixerRenderingFlags")]
    pub const SPATIAL_MIXER_RENDERING_FLAGS: Self = Self(3003);

    /// Scope:      Input
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Sets how individual channels of an input bus are rendered. See kSpatialMixerSourceMode_
    #[doc(alias = "kAudioUnitProperty_SpatialMixerSourceMode")]
    pub const SPATIAL_MIXER_SRC_MODE: Self = Self(3005);

    /// Scope:      Input
    /// Value Type: MixerDistanceParams
    /// Access:     Read / Write
    #[doc(alias = "kAudioUnitProperty_SpatialMixerDistanceParams")]
    pub const SPATIAL_MIXER_DISTANCE_PARAMS: Self = Self(3010);

    /// Scope:      Input
    /// Value Type: u32
    /// Access:     Read / Write
    #[doc(alias = "kAudioUnitProperty_SpatialMixerAttenuationCurve")]
    pub const SPATIAL_MIXER_ATTENUATION_CURVE: Self = Self(3013);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Sets the type of output hardware used with kSpatializationAlgorithm_UseOutputType.
    /// See kSpatialMixerOutputType_
    #[doc(alias = "kAudioUnitProperty_SpatialMixerOutputType")]
    pub const SPATIAL_MIXER_OUTPUT_TYPE: Self = Self(3100);

    /// Scope:      Input
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Sets in-head rendering mode when using kSpatializationAlgorithm_UseOutputType and
    /// kSpatialMixerSourceMode_PointSource. See kSpatialMixerPointSourceInHeadMode_
    #[doc(alias = "kAudioUnitProperty_SpatialMixerPointSourceInHeadMode")]
    pub const SPATIAL_MIXER_POINT_SRC_IN_HEAD_MODE: Self = Self(3103);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Enables or disables head-tracking using AirPods motion sensors. This tracking will
    /// apply a second rotation on top of head yaw, pitch, and roll parameters.
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioUnitProperty_SpatialMixerEnableHeadTracking")]
    pub const SPATIAL_MIXER_ENABLE_HEAD_TRACKING: Self = Self(3111);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read / Write
    ///
    /// Sets personalized head-related transfer function (HRTF) mode for spatial audio rendering
    /// with kSpatializationAlgorithm_UseOutputType and kSpatialMixerOutputType_Headphones.
    #[doc(alias = "kAudioUnitProperty_SpatialMixerPersonalizedHRTFMode")]
    pub const SPATIAL_MIXER_PERSONALIZED_HRTF_MODE: Self = Self(3113);

    /// Scope:      Global
    /// Value Type: u32
    /// Access:     Read
    ///
    /// Returns a Boolean value that indicates whether AUSpatialMixer is currently using personalized
    /// HRTF or not. The property should be queried after AU is initialized for a reliable outcome.
    ///
    /// Personalization of spatial audio rendering is subject to various factors such as data availability or
    /// whether AUSpatialMixer is rendering for headphones with kSpatializationAlgorithm_UseOutputType.
    /// Hence, the value of kAudioUnitProperty_SpatialMixerPersonalizedHRTFMode alone does not
    /// guarantee that personalized HRTF is being used for spatial audio rendering.
    #[doc(alias = "kAudioUnitProperty_SpatialMixerAnyInputIsUsingPersonalizedHRTF")]
    pub const SPATIAL_MIXER_ANY_INPUT_IS_USING_PERSONALIZED_HRTF: Self = Self(3116);
}

/// Keys contains in an audio unit preset (ClassInfo) dictionary
/// These strings are used as keys in the AUPreset-"classInfo" dictionary
pub mod preset_key {
    use crate::cf;

    #[doc(alias = "kAUPresetVersionKey")]
    pub fn version() -> &'static cf::String {
        cf::str!(c"version")
    }

    #[doc(alias = "kAUPresetTypeKey")]
    pub fn type_() -> &'static cf::String {
        cf::str!(c"type")
    }

    #[doc(alias = "kAUPresetSubtypeKey")]
    pub fn subtype() -> &'static cf::String {
        cf::str!(c"subtype")
    }

    #[doc(alias = "kAUPresetManufacturerKey")]
    pub fn manufacturer() -> &'static cf::String {
        cf::str!(c"manufacturer")
    }

    #[doc(alias = "kAUPresetDataKey")]
    pub fn data() -> &'static cf::String {
        cf::str!(c"data")
    }

    #[doc(alias = "kAUPresetNameKey")]
    pub fn name() -> &'static cf::String {
        cf::str!(c"name")
    }

    #[doc(alias = "kAUPresetNumberKey")]
    pub fn preset_number() -> &'static cf::String {
        cf::str!(c"preset-number")
    }

    #[doc(alias = "kAUPresetRenderQualityKey")]
    pub fn render_quality() -> &'static cf::String {
        cf::str!(c"render-quality")
    }

    #[doc(alias = "kAUPresetCPULoadKey")]
    pub fn cpu_load() -> &'static cf::String {
        cf::str!(c"cpu-load")
    }

    #[doc(alias = "kAUPresetElementNameKey")]
    pub fn element_name() -> &'static cf::String {
        cf::str!(c"element-name")
    }

    #[doc(alias = "kAUPresetExternalFileRefs")]
    pub fn file_refs() -> &'static cf::String {
        cf::str!(c"file-references")
    }

    #[cfg(target_os = "macos")]
    #[doc(alias = "kAUPresetVSTDataKey")]
    pub fn vst_data() -> &'static cf::String {
        cf::str!(c"vstdata")
    }

    #[cfg(target_os = "macos")]
    #[doc(alias = "kAUPresetVSTPresetKey")]
    pub fn vst_preset() -> &'static cf::String {
        cf::str!(c"vstpreset")
    }

    #[cfg(target_os = "macos")]
    #[doc(alias = "kAUPresetMASDataKey")]
    pub fn mas_data() -> &'static cf::String {
        cf::str!(c"masdata")
    }

    /// This key if present, distinguishes a global preset that is set
    /// on the global scope with a part-based preset that is set on the part scope.
    /// The value of this key is audio unit defined
    #[doc(alias = "kAUPresetPartKey")]
    pub fn part() -> &'static cf::String {
        cf::str!(c"part")
    }
}

/// This structure contains the information needed to make a connection between a source
/// and destination audio unit.
#[repr(C)]
#[doc(alias = "AudioUnitConnection")]
pub struct Connection {
    /// The audio unit that is the source for the connection
    pub src_au: Option<au::UnitRef>,

    /// The source audio unit's output element to be used in the connection
    pub src_output_num: u32,

    /// The destination audio unit's input element to be used in the connection
    pub dst_input_num: u32,
}

/// Define an audio unit's channel handling capabilities
#[repr(C)]
#[doc(alias = "AUChannelInfo")]
pub struct ChannelInfo {
    pub in_channels: i16,
    pub out_channels: i16,
}

/// Allow a host to tell an audio unit to use the provided memory for its input callback
#[repr(C)]
#[doc(alias = "AudioUnitExternalBuffer")]
pub struct ExternalBuf {
    pub buf: *mut u8,
    pub size: u32,
}

/// Used by a host when registering a callback with the audio unit to provide input
#[repr(C)]
#[doc(alias = "AURenderCallbackStruct")]
pub struct RenderCbStruct {
    pub proc: *const au::RenderCb,
    pub proc_ref_con: *const c_void,
}

#[repr(C)]
#[doc(alias = "AUPreset")]
pub struct Preset {
    pub number: i32,
    pub name: Option<arc::R<cf::String>>,
}

/// Structure used to get the magnitude of the frequency response
/// at a particular frequency via kAudioUnitProperty_FrequencyResponse.
#[repr(C)]
#[doc(alias = "AudioUnitFrequencyResponseBin")]
pub struct FrequenceyResponseBin {
    pub frequency: f64,
    pub magniture: f64,
}
