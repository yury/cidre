use crate::at::au;

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
}
