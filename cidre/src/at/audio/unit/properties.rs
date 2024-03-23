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
}
