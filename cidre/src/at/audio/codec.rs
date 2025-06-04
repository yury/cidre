use std::{marker::PhantomData, mem::MaybeUninit};

use crate::{
    at::audio,
    at::audio::component::{InitializedState, State, UninitializedState},
    os,
};

/// AudioCodec components translate audio data from one format to another. There
/// are three kinds of AudioCodec components. Decoder components ('adec')
/// translate data that isn't in linear PCM into linear PCM formatted data.
/// Encoder components ('aenc') translate linear PCM data into some other format.
/// Unity codecs ('acdc') translate between different flavors of the same type
/// (e.g. 16 bit signed integer linear PCM into 32 bit floating point linear PCM).
///
/// AudioCodec components are standard components and are managed by the Component
/// Manager.
///
/// Once an AudioCodec is found that implements the translation in question,
/// it has to be set up to do the translation. This can be done by setting the
/// appropriate properties or by calling AudioCodecInitialize. If the translation
/// is specified by properties, AudioCodecInitialize still needs to be called
/// prior to appending input data or producing output data.

/// AudioCodecInitialize puts the codec into the "initialized" state. In this state,
/// the format information for the translation cannot be changed. The codec
/// has to be in the initialized state for AudioCodecAppendInputData and
/// AudioCodecProduceOutputData to work. They will return kAudioCodecStateError
/// if the codec isn't initialized.
///
/// AudioCodecUninitialize will return the codec to the uninitialized state and
/// release any allocated resources. The codec may then be configured freely. It is not
/// necessary to call AudioCodecUninitialize prior to closing the codec.
///
/// Once in the initialized state, the codec is ready to receive input and produce
/// output using the `append_data` and `produce_data`
/// routines. Input data can be fed into an encoder and some decoders in any size (even
/// byte by byte). Input data fed to a decoder should be in terms of whole packets in the
/// encoded format if the format is variable bit rate and is not self framing (e.g. MPEG-4 AAC).
/// Output data can only be produced in whole packet sizes. Both routines will return
/// the amount of data they consume/produce.
///
/// AudioCodecProduceOutputData also returns a status code to the caller that
/// indicates the result of the operation (success or failure) as well as the
/// state of the input buffer.
///
/// The combination of `append_data` and `produce_packets` can be thought of a "push-pull"
/// model of data handling. First, the input data is pushed into the component and the
/// resulting output data gets pulled out of that same component.
///
/// Basic Workflow
/// 1. Find the appropriate codec component
/// 2. Open the codec component
/// 3. Configure it (AudioCodecGetPropertyInfo, AudioCodecGetProperty, AudioCodecSetProperty)
/// 4. AudioCodecInitialize
/// 5. Loop
///     a. append_data (EOF is signaled by passing a 0-sized buffer)
///     b. produce_packets
/// 6. Close the codec component
#[derive(Debug)]
#[repr(transparent)]
pub struct Codec(audio::component::Instance);

#[derive(Debug)]
pub struct CodecRef<S>(&'static mut Codec, PhantomData<S>)
where
    S: State<Codec>;

impl State<Codec> for InitializedState {
    fn release_resources(instance: &mut Codec) -> os::Result {
        unsafe { AudioCodecUninitialize(instance).result() }
    }
}
impl State<Codec> for UninitializedState {}

/// These properties reflect the capabilities of the underlying codec.
/// The values of these properties are independent of the codec's internal
/// state.
///
/// These properties can be read at any time the codec is open.
#[repr(transparent)]
pub struct GlobalPropId(pub u32);

/// Properties which can be set or read on an instance of the
/// underlying audio codec. These properties are dependent on the
/// codec's current state. A property may be read/write or read
/// only, depending on the data format of the codec.
///
/// These properties may have different values depending on whether the
/// codec is initialized or not. All properties can be read at any time
/// the codec is open. However, to ensure the codec is in a valid
/// operational state and therefore the property value is valid the codec
/// must be initialized at the time the property is read.
///
/// Properties that are writable are only writable when the codec
/// is not initialized.
#[repr(transparent)]
pub struct InstancePropId(pub u32);

impl GlobalPropId {
    /// An array of audio::StreamBasicDescription structs describing what formats
    /// the codec supports for input data
    #[doc(alias = "kAudioCodecPropertySupportedInputFormats")]
    pub const SUPPORTED_INPUT_FORMATS: Self = Self(u32::from_be_bytes(*b"ifm#"));

    /// An array of audio::StreamBasicDescription structs describing what formats
    /// the codec supports for output data
    #[doc(alias = "kAudioCodecPropertySupportedOutputFormats")]
    pub const SUPPORTED_OUTPUT_FORMATS: Self = Self(u32::from_be_bytes(*b"ofm#"));

    /// An array of audio::ValueRange indicating the valid ranges for the
    /// output sample rate of the codec for the current bit rate.
    /// This property is only relevant to encoders.
    /// See also kAudioCodecPropertyAvailableOutputSampleRates.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyAvailableInputSampleRates")]
    pub const AVAILABLE_INPUT_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"aisr"));

    /// An array of audio::ValueRange indicating the valid ranges for the
    /// output sample rate of the codec.
    /// Required for encoders.
    /// (see also kAudioCodecPropertyApplicableOutputSampleRates)
    #[doc(alias = "kAudioCodecPropertyAvailableOutputSampleRates")]
    pub const AVAILABLE_OUTPUT_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"aosr"));

    /// An array of audio::ValueRange that indicate the target bit rates
    /// supported by the encoder. This can be total bit rate or bit
    /// rate per channel as appropriate.
    /// This property is only relevant to encoders.
    /// (see also kAudioCodecPropertyApplicableBitRateRange)
    #[doc(alias = "kAudioCodecPropertyAvailableBitRateRange")]
    pub const AVAILABLE_BIT_RATE_RANGE: Self = Self(u32::from_be_bytes(*b"abrt"));

    /// A u32 indicating the minimum number of input packets
    /// that need to be supplied to the codec. The actual input the
    /// codec accepts could be less than this.
    /// For most codecs this value will be 1.
    #[doc(alias = "kAudioCodecPropertyMinimumNumberInputPackets")]
    pub const MINIMUM_NUMBER_INPUT_PACKETS: Self = Self(u32::from_be_bytes(*b"mnip"));

    /// A u32 indicating the minimum number of output packets
    /// that need to be handled from the codec. The actual output
    /// might be less than this.
    /// For most codecs this value will be 1.
    #[doc(alias = "kAudioCodecPropertyMinimumNumberOutputPackets")]
    pub const MINIMUM_NUMBER_OUTPUT_PACKETS: Self = Self(u32::from_be_bytes(*b"mnop"));

    /// An array of u32 that specifies the number of channels the codec is
    /// capable of encoding or decoding to. 0xFFFFFFFF means any number
    /// of channels.
    #[doc(alias = "kAudioCodecPropertyAvailableNumberChannels")]
    pub const AVAILABLE_NUMBER_CHANNELS: Self = Self(u32::from_be_bytes(*b"cmnc"));

    /// A u32 indicating if the codec wants to do a sample rate conversion (if
    /// necessary) because it can do it in a way that is meaningful for quality.
    /// Value is 1 if true, 0 otherwise.
    #[doc(alias = "kAudioCodecPropertyDoesSampleRateConversion")]
    pub const DOES_SAMPLE_RATE_CONVERSION: Self = Self(u32::from_be_bytes(*b"lmrc"));

    /// An array of audio::ChannelLayoutTag that specifies what channel layouts the codec is
    /// capable of using on input.
    #[doc(alias = "kAudioCodecPropertyAvailableInputChannelLayoutTags")]
    pub const AVAILABLE_INPUT_CHANNEL_LAYOUT_TAGS: Self = Self(u32::from_be_bytes(*b"aicl"));

    /// An array of audio::ChannelLayoutTag that specifies what channel layouts the codec is
    /// capable of using on output.
    #[doc(alias = "kAudioCodecPropertyAvailableOutputChannelLayoutTags")]
    pub const AVAILABLE_OUTPUT_CHANNEL_LAYOUT_TAGS: Self = Self(u32::from_be_bytes(*b"aocl"));

    /// An array of AudioStreamBasicDescription indicating what the codec supports
    /// for input data given an output format that's passed in as the first member of
    /// the array (and is overwritten on the reply). Always a subset of
    /// kAudioCodecPropertySupportedInputFormats
    #[doc(alias = "kAudioCodecPropertyInputFormatsForOutputFormat")]
    pub const INPUT_FORMATS_FOR_OUTPUT_FORMAT: Self = Self(u32::from_be_bytes(*b"if4o"));

    /// An array of AudioStreamBasicDescription indicating what the codec supports
    /// for output data given an input format that's passed in as the first member of
    /// the array (and is overwritten on the reply). Always a subset of
    /// kAudioCodecPropertySupportedOutputFormats
    #[doc(alias = "kAudioCodecPropertyOutputFormatsForInputFormat")]
    pub const OUTPUT_FORMATS_FOR_INPUT_FORMAT: Self = Self(u32::from_be_bytes(*b"of4i"));

    /// Takes an audio::FormatInfo on input. This audio::FormatInfo is validated either through
    /// the provided magic cookie or the AudioStreamBasicDescription and where applicable,
    /// wildcards are overwritten with default values.
    #[doc(alias = "kAudioCodecPropertyFormatInfo")]
    pub const FORMAT_INFO: Self = Self(u32::from_be_bytes(*b"acfi"));
}

impl InstancePropId {
    /// A u32 indicating the maximum input buffer size for the codec
    /// in bytes.
    /// Not writable, but can vary on some codecs depending on the bit stream
    /// format being handled.
    #[doc(alias = "kAudioCodecPropertyInputBufferSize")]
    pub const INPUT_BUF_SIZE: Self = Self(u32::from_be_bytes(*b"tbuf"));

    /// A u32 indicating the number of frames of audio data encapsulated in each
    /// packet of data in the codec's format. For encoders, this is the
    /// output format. For decoders this is the input format.
    /// Formats with variable frames per packet should return a maximum value
    /// for this property.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyPacketFrameSize")]
    pub const PACKET_FRAME_SIZE: Self = Self(u32::from_be_bytes(*b"pakf"));

    /// A u32 where 0 indicates that all packets in the codec's format
    /// have the same byte size (sometimes referred to as CBR codecs),
    /// and 1 indicates that they vary in size (sometimes referred to as
    /// VBR codecs). The maximum size of a variable packet is up to
    /// the one indicated in kAudioCodecPropertyMaximumPacketByteSize.
    /// Any codec that reports 1 for this property must be able to handle packet
    /// descriptions, though it does not have to require them.
    /// May be writable.
    #[doc(alias = "kAudioCodecPropertyHasVariablePacketByteSizes")]
    pub const HAS_VARIABLE_PACKET_BYTE_SIZES: Self = Self(u32::from_be_bytes(*b"vpk?"));

    /// A u32 where 0 indicates that all packets in the codec's format
    /// are independently decodable, and 1 indicates that some may not be
    /// independently decodable.
    #[doc(alias = "kAudioCodecPropertyEmploysDependentPackets")]
    pub const EMPLOYS_DEPENDENT_PACKETS: Self = Self(u32::from_be_bytes(*b"dpk?"));

    /// A u32 indicating the maximum number of bytes a packet of data
    /// in the codec's format will be. If the format is constant bit rate,
    /// all packets will be this size. If it is variable bit rate, the packets
    /// will never exceed this size.
    /// This always refers to the encoded data, so for encoders it refers to the
    /// output data and for decoders the input data.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyMaximumPacketByteSize")]
    pub const MAXIMUM_PACKET_BYTE_SIZE: Self = Self(u32::from_be_bytes(*b"pakb"));

    /// A u32 indicating the maximum number of bits in an output packet of an encoder.
    /// The output packet size will not exceed this number. The size should be smaller
    /// than kAudioCodecPropertyMaximumPacketByteSize. This property will configure the
    /// encoder to VBR mode with the highest VBR quality that can maintain the packet
    /// size limit. kAudioCodecPropertySoundQualityForVBR can be used to retrieve the
    /// quality setting that will be used given that packet size limit.
    /// Writeable if supported.
    #[doc(alias = "kAudioCodecPropertyPacketSizeLimitForVBR")]
    pub const PACKET_SIZE_LIMIT_FOR_VBR: Self = Self(u32::from_be_bytes(*b"pakl"));

    /// An AudioStreamBasicDescription describing the format the codec
    /// expects its input data in
    /// Almost always writable, but if the codec only supports one unique input format
    /// it does not have to be
    #[doc(alias = "kAudioCodecPropertyCurrentInputFormat")]
    pub const CURRENT_INPUT_FORMAT: Self = Self(u32::from_be_bytes(*b"ifmt"));

    /// An audio::StreamBasicDescription describing the format the codec
    /// provides its output data in
    /// Almost always writable, but if the codec only supports one unique output format
    /// it does not have to be
    #[doc(alias = "kAudioCodecPropertyCurrentOutputFormat")]
    pub const CURRENT_OUTPUT_FORMAT: Self = Self(u32::from_be_bytes(*b"ofmt"));

    /// An untyped buffer of out of band configuration data the codec
    /// requires to process the stream of data correctly. The contents
    /// of this data is private to the codec.
    /// Not all codecs have magic cookies. If a call to AudioCodecGetPropertyInfo
    /// returns a size greater than 0 then the codec may take one.
    /// Writable if present.
    #[doc(alias = "kAudioCodecPropertyMagicCookie")]
    pub const MAGIC_COOKIE: Self = Self(u32::from_be_bytes(*b"kuki"));

    /// A u32 indicating the number of bytes in the codec's input
    /// buffer. The amount of available buffer space is simply the
    /// answer from kAudioCodecPropertyInputBufferSize minus the answer
    /// from this property.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyUsedInputBufferSize")]
    pub const USED_INPUT_BUF_SIZE: Self = Self(u32::from_be_bytes(*b"ubuf"));

    /// A u32 where 0 means the codec is uninitialized and anything
    /// else means the codec is initialized. This should never be settable directly.
    /// Must be set by AudioCodecInitialize and AudioCodecUninitialize.
    #[doc(alias = "kAudioCodecPropertyIsInitialized")]
    pub const IS_INITIALIZED: Self = Self(u32::from_be_bytes(*b"init"));

    /// A u32 containing the number of bits per second to aim for when encoding
    /// data. This property is usually only relevant to encoders, but if a decoder
    /// can know what bit rate it's set to it may report it.
    /// This property is irrelevant if the encoder is configured as kAudioCodecBitRateControlMode_Variable.
    /// Writable on encoders if supported.
    #[doc(alias = "kAudioCodecPropertyCurrentTargetBitRate")]
    pub const CURRENT_TARGET_BIT_RATE: Self = Self(u32::from_be_bytes(*b"brat"));

    /// A f64 containing the current input sample rate in Hz. No Default.
    /// May be writable. If only one sample rate is supported it does not have to be.
    #[doc(alias = "kAudioCodecPropertyCurrentInputSampleRate")]
    pub const CURRENT_INPUT_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"cisr"));

    /// A f64 containing the current output sample rate in Hz. No Default.
    /// May be writable. If only one sample rate is supported it does not have to be.
    #[doc(alias = "kAudioCodecPropertyCurrentOutputSampleRate")]
    pub const CURRENT_OUTPUT_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"cosr"));

    /// A u32 that sets the tradeoff between sound quality and CPU time consumption.
    /// The property value is between [0 - 0x7F].
    /// Some enum constants are defined below for convenience.
    /// Writable if supported.
    #[doc(alias = "kAudioCodecPropertyQualitySetting")]
    pub const QUALITY_SETTING: Self = Self(u32::from_be_bytes(*b"srcq"));

    /// An array of audio::ValueRange indicating the target bit rates
    /// supported by the encoder in its current configuration.
    /// This property is only relevant to encoders.
    /// See also kAudioCodecPropertyAvailableBitRateRange.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyApplicableBitRateRange")]
    pub const APPLICABLE_BIT_RATE_RANGE: Self = Self(u32::from_be_bytes(*b"brta"));

    /// An array of AudioValueRange indicating the recommended bit rates
    /// at given sample rate.
    /// This property is only relevant to encoders.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyRecommendedBitRateRange")]
    pub const RECOMMENDED_BIT_RATE_RANGE: Self = Self(u32::from_be_bytes(*b"brtr"));

    /// An array of audio::ValueRange indicating the valid ranges for the
    /// input sample rate of the codec for the current bit rate.
    /// This property is only relevant to encoders.
    /// See also kAudioCodecPropertyAvailableInputSampleRates.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyApplicableInputSampleRates")]
    pub const APPLICABLE_INPUT_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"isra"));

    /// An array of audio::ValueRange indicating the valid ranges for the
    /// output sample rate of the codec for the current bit rate.
    /// This property is only relevant to encoders.
    /// See also kAudioCodecPropertyAvailableOutputSampleRates.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyApplicableOutputSampleRates")]
    pub const APPLICABLE_OUTPUT_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"osra"));

    /// A u32 indicating the number of zeros (samples) that were appended
    /// to the last packet of input data to make a complete packet encoding.
    /// Encoders only. No default.
    /// Not writable.
    #[doc(alias = "kAudioCodecPropertyPaddedZeros")]
    pub const PADDED_ZEROS: Self = Self(u32::from_be_bytes(*b"pad0"));

    /// A u32 specifying priming method.
    /// See enum below.
    /// May be writable. Some encoders offer the option of padding out the last packet, and this
    /// may be set here.
    #[doc(alias = "kAudioCodecPropertyPrimeMethod")]
    pub const PRIME_METHOD: Self = Self(u32::from_be_bytes(*b"prmm"));

    /// A pointer to an audio::CodecPrimeInfo struct.
    /// Not writable on encoders. On decoders this may be writable, telling the decoder to trim the
    /// first and/or last packet.
    #[doc(alias = "kAudioCodecPropertyPrimeInfo")]
    pub const PRIME_INFO: Self = Self(u32::from_be_bytes(*b"prim"));

    /// An audio::ChannelLayout that specifies the channel layout that the codec is using for input.
    /// May be writable. If only one channel layout is supported it does not have to be.
    #[doc(alias = "kAudioCodecPropertyCurrentInputChannelLayout")]
    pub const CURRENT_INPUT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"icl "));

    /// An audio::ChannelLayout that specifies the channel layout that the codec is using for output.
    /// If settable on a encoder, it means the encoder can re-map channels
    /// May be writable. If only one channel layout is supported or the codec does no channel remapping
    /// (ie, output channel layout always equals the input channel layout) it does not have to be.
    #[doc(alias = "kAudioCodecPropertyCurrentOutputChannelLayout")]
    pub const CURRENT_OUTPUT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"ocl "));

    /// A cf::Dictionary that lists both the settable codec settings and their values.
    /// Encoders only.
    /// Obviously this will be linked to many of the other properties listed herein and as such
    /// it potentially will cause synchronization problems. Therefore, when setting this property
    /// on an encoder a GetProperty should be done first to retrieve the current dictionary,
    /// and only one setting within the dictionary should change with each SetProperty call,
    /// as it is not guaranteed that changing one property will not have side effects.
    /// Writable if supported.
    #[doc(alias = "kAudioCodecPropertySettings")]
    pub const SETTINGS: Self = Self(u32::from_be_bytes(*b"acs "));

    /// An array of audio::FormatListItem structs list all formats that can be handled by the decoder
    /// For decoders, takes a Magic Cookie that gets passed in on the GetProperty call. No default.
    /// On input, the outPropertyData parameter passed to GetProperty should begin with a
    /// audio::CodecMagicCookieInfo struct which will be overwritten by the AudioFormatListItems
    /// returned from the property. For encoders, returns a list of formats which will be in the
    /// bitstream. No input data required.
    /// Important note: this encoder property is only applicable to audio formats which are made of
    /// two or more layers where the base layers(s) can be decoded by systems which aren't capable of
    /// handling the enhancement layers. For example, a High Efficiency AAC bitstream which contains
    /// an AAC Low Complexity base layer can be decoded by any AAC decoder.
    #[doc(alias = "kAudioCodecPropertyFormatList")]
    pub const FORMAT_LIST: Self = Self(u32::from_be_bytes(*b"acfl"));

    /// A u32 indicating which bit rate control mode will be applied to encoders that
    /// can produce variable packet sizes (sometimes referred to as VBR encoders).
    /// Although the packet size may be variable, a constant bit rate can be maintained
    /// over a transmission channel when decoding in real-time with a fixed end-to-end audio delay.
    /// E.g., MP3 and MPEG-AAC use a bit reservoir mechanism to meet that constraint.
    /// See enum below.
    /// Only needs to be settable if the codec supports multiple bit rate control strategies.
    #[doc(alias = "kAudioCodecPropertyBitRateControlMode")]
    pub const BIT_RATE_CONTROL_MODE: Self = Self(u32::from_be_bytes(*b"acbf"));

    /// A u32 that sets a target sound quality level.
    /// Unlike kAudioCodecPropertyQualitySetting which is relevant to all BitRate Control Modes,
    /// this property only needs to be set by an encoder configured at kAudioCodecBitRateControlMode_Variable.
    /// The property value is between [0 - 0x7F].
    /// See also kAudioCodecPropertyQualitySetting
    /// Writable if supported.
    #[doc(alias = "kAudioCodecPropertySoundQualityForVBR")]
    pub const SOUND_QUALITY_FOR_VBR: Self = Self(u32::from_be_bytes(*b"vbrq"));

    /// A u32 that can be used to set the target bit rate when the encoder is configured
    /// for VBR mode (kAudioCodecBitRateControlMode_Variable). Writable if supported.
    #[doc(alias = "kAudioCodecPropertyBitRateForVBR")]
    pub const BIT_RATE_FOR_VBR: Self = Self(u32::from_be_bytes(*b"vbrb"));

    /// A u32 specifying the delay mode. See enum below.
    /// Writable if supported.
    #[doc(alias = "kAudioCodecPropertyDelayMode")]
    pub const DELAY_MODE: Self = Self(u32::from_be_bytes(*b"dmod"));

    /// An i32 number in the range [-128, 127] to allow encoding quality adjustements on a packet by packet basis.
    /// This property can be set on an initialized encoder object without having to uninitialize and re-intialize it
    /// and allows to adjust the encoder quality level for every packet. This is useful for packets streamed over
    /// unreliable IP networks where the encoder needs to adapt immediately to network condition changes.
    /// Escape property ID's start with a '^' symbol as the first char code. This bypasses the initilization check.
    #[doc(alias = "kAudioCodecPropertyAdjustLocalQuality")]
    pub const ADJUST_LOCAL_QUALITY: Self = Self(u32::from_be_bytes(*b"^qal"));

    /// A f32 specifying the program target level in dB FS for decoders.
    /// Supported target levels are in the range of -31.0 to -20.0dB.
    /// This property controls the decoding of broadcast loudness
    /// normalization metadata with goal of achieving consistent loudness across various
    /// programs. The property complies with the target level defined in the MPEG Audio
    /// standard ISO/IEC 14496-3. It will override kAudioCodecPropertyProgramTargetLevelConstant.
    #[doc(alias = "kAudioCodecPropertyProgramTargetLevel")]
    pub const PROGRAM_TARGET_LEVEL: Self = Self(u32::from_be_bytes(*b"pptl"));

    /// A u32 specifying the DRC mode. Supported modes are defined as enum with the
    /// prefix kDynamicRangeControlMode (see below). This property controls which
    /// dynamic range compression scheme is applied if the information is present in
    /// the bitstream. The default is kDynamicRangeControlMode_None.
    #[doc(alias = "kAudioCodecPropertyDynamicRangeControlMode")]
    pub const DYNAMIC_RANGE_CONTROL_MODE: Self = Self(u32::from_be_bytes(*b"mdrc"));

    /// A u32 specifying the program target level constant in dB FS (Full Scale) for decoders.
    /// Supported target levels are defined as enum with the prefix kProgramTargetLevel
    /// (see below). This property controls the decoding of broadcast loudness
    /// normalization metadata with the goal of achieving consistent loudness across various
    /// programs. The property complies with the target level defined in the MPEG Audio
    /// standard ISO/IEC 14496-3. The default is kProgramTargetLevel_None.
    #[doc(alias = "kAudioCodecPropertyProgramTargetLevelConstant")]
    pub const PROGRAM_TARGET_LEVEL_CONSTANT: Self = Self(u32::from_be_bytes(*b"ptlc"));

    /// A u32 specifying the dynamic range compression profile to be applied in the decoder. Profiles are
    /// based on the DRC Effect Types in ISO/IEC 23003-4. Supported profiles are defined as DynamicRangeCompressionProfile
    /// enum (see below). The default profile is DynamicRangeCompressionProfile::None.
    /// This property can also be set on an initialized decoder object. It will be applied immediately, if supported.
    #[doc(alias = "kAudioCodecPropertyAdjustCompressionProfile")]
    pub const ADJUST_COMPRESSION_PROFILE: Self = Self(u32::from_be_bytes(*b"^pro"));

    /// A f32 specifying the program target loudness in LKFS for decoders. It has the same effect
    /// as kAudioCodecPropertyProgramTargetLevel, but this property can also be set on an initialized decoder
    /// object. It will be applied immediately, if supported.
    #[doc(alias = "kAudioCodecPropertyAdjustTargetLevel")]
    pub const ADJUST_TARGET_LEVEL: Self = Self(u32::from_be_bytes(*b"^ptl"));

    /// A u32 specifying the program target loudness in LKFS for decoders. It has the same effect
    /// as kAudioCodecPropertyProgramTargetLevelConstant, but this property can also be set on an
    /// initialized decoder object. It will be applied immediately, if supported.
    /// A value of kProgramTargetLevel_None removes a prior target level setting.
    #[doc(alias = "kAudioCodecPropertyAdjustTargetLevelConstant")]
    pub const ADJUST_TARGET_LEVEL_CONSTANT: Self = Self(u32::from_be_bytes(*b"^tlc"));
}

/// Constants to be used with kAudioCodecPropertyProgramTargetLevelConstant to set the target loudness
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ProgramTargetLevel {
    #[doc(alias = "kProgramTargetLevel_None")]
    None = 0,

    #[doc(alias = "kProgramTargetLevel_Minus31dB")]
    Minus31dB = 1,

    #[doc(alias = "kProgramTargetLevel_Minus23dB")]
    Minus23dB = 2,

    #[doc(alias = "kProgramTargetLevel_Minus20dB")]
    Minus20dB = 3,
}

/// Constants to be used with kAudioCodecPropertyAdjustCompressionProfile to control
/// the DRC Effect Type based on ISO/IEC 23003-4
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum DynamicRangeCompressionProfile {
    /// No compression
    #[doc(alias = "kDynamicRangeCompressionProfile_None")]
    None = 0,

    /// Compression to avoid disturbing others and listening at lower volume
    #[doc(alias = "kDynamicRangeCompressionProfile_LateNight")]
    LateNight = 1,

    /// Compression suitable for listening in noisy environments
    #[doc(alias = "kDynamicRangeCompressionProfile_NoisyEnvironment")]
    NoisyEnvironment = 2,

    /// Compression for transducers with limited dynamic range
    #[doc(alias = "kDynamicRangeCompressionProfile_LimitedPlaybackRange")]
    LimitedPlaybackRange = 3,

    /// General purpose compression
    #[doc(alias = "kDynamicRangeCompressionProfile_GeneralCompression")]
    GeneralCompression = 6,
}

/// Constants defining various bit rate control modes
/// to be used with kAudioCodecPropertyBitRateControlMode.
/// These modes are only applicable to encoders that can produce
/// variable packet sizes, such as AAC.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum BitRateControlMode {
    /// The encoder maintains a constant bit rate suitable for use over a transmission
    /// channel when decoding in real-time with a fixed end-to-end audio delay.  
    /// Note that while a constant bit rate is maintained in this mode, the number of bits
    /// allocated to encode each fixed length of audio data may be variable
    /// (ie. packet sizes are variable).
    /// E.g., MP3 and MPEG-AAC use a bit reservoir mechanism to meet that constraint.
    Constant = 0,

    ///  The provided target bit rate is achieved over a long term average
    ///  (typically after the first 1000 packets). This mode is similar to
    ///  BitRateControlMode::Constant in the sense that the
    ///  target bit rate will be maintained in a long term average. However, it does not
    ///  provide constant delay when using constant bit rate transmission. This mode offers
    ///  a better sound quality than BitRateControlMode::Constant
    ///  can, that is, a more efficient encoding is performed.
    LongTermAverage = 1,

    /// Encoder dynamically allocates the bit resources according to the characteristics
    /// of the underlying signal. However, some constraints are applied in order to limit
    /// the variation of the bit rate.
    VariableConstrained = 2,

    /// Similar to the VBR constrained mode, however the packet size is virtually unconstrained.
    /// The coding process targets constant sound quality, and the sound quality level is
    /// set by kAudioCodecPropertySoundQualityForVBR.
    /// This mode usually provides the best tradeoff between quality and bit rate.
    Variable = 3,
}

#[doc(alias = "kAudioDecoderComponentType")]
pub const DECODER_COMPONENT_TYPE: os::Type = u32::from_be_bytes(*b"adec");

#[doc(alias = "kAudioEncoderComponentType")]
pub const ENCODER_COMPONENT_TYPE: os::Type = u32::from_be_bytes(*b"aenc");

#[doc(alias = "kAudioUnityCodecComponentType")]
pub const UNITY_CODEC_COMPONENT_TYPE: os::Type = u32::from_be_bytes(*b"acdc");

/// Structure holding the magic cookie information.
#[repr(C)]
pub struct MagicCookieInfo {
    /// The size of the magic cookie
    pub size: u32,
    /// Generic const pointer to magic cookie
    /// usually it is esds
    pub value: *const u8,
}

#[derive(Debug)]
pub struct Consumed {
    pub bytes: u32,
    pub packets: u32,
}

#[derive(Debug)]
pub struct Produced {
    pub bytes: u32,
    pub packets: u32,
    pub status: ProduceOutputPacketStatus,
}

impl CodecRef<UninitializedState> {
    pub fn new_apple_aac_encoder() -> os::Result<Self> {
        let desc = audio::ComponentDesc {
            type_: audio::ENCODER_COMPONENT_TYPE,
            sub_type: audio::Format::MPEG4_AAC.0,
            manufacturer: audio::unit::Manufacturer::APPLE.0,
            ..Default::default()
        };

        let comp = desc
            .into_iter()
            .next()
            .ok_or(audio::unit::component_err::UNSUPPORTED_TYPE)?;
        comp.open_codec()
    }

    pub fn new_apple_opus_encoder() -> os::Result<Self> {
        let desc = audio::ComponentDesc {
            type_: audio::ENCODER_COMPONENT_TYPE,
            sub_type: audio::Format::OPUS.0,
            manufacturer: audio::unit::Manufacturer::APPLE.0,
            ..Default::default()
        };

        let comp = desc
            .into_iter()
            .next()
            .ok_or(audio::unit::component_err::UNSUPPORTED_TYPE)?;
        comp.open_codec()
    }

    pub fn initialize(
        self,
        input_format: *const audio::StreamBasicDesc,
        output_format: *const audio::StreamBasicDesc,
        magic_cookie: Option<&[u8]>,
    ) -> os::Result<CodecRef<InitializedState>> {
        unsafe {
            self.0
                .init_codec(input_format, output_format, magic_cookie)?;
            Ok(CodecRef(std::mem::transmute(self), PhantomData))
        }
    }

    #[inline]
    pub fn set_quality(&mut self, val: u32) -> os::Result {
        self.0.set_quality(val)
    }
}

impl<S: State<Codec>> Drop for CodecRef<S> {
    #[inline]
    fn drop(&mut self) {
        let res = S::release_resources(self.0);
        debug_assert!(res.is_ok());
        let res = unsafe { self.0.0.dispose() };
        debug_assert!(res.is_ok());
    }
}

impl CodecRef<InitializedState> {
    /// This call will move the codec from the initialized state back to the
    /// uninitialized state. The codec will release any resources it allocated
    /// or claimed in AudioCodecInitialize.
    pub fn uninitialize(mut self) -> os::Result<CodecRef<UninitializedState>> {
        Ok(unsafe {
            AudioCodecUninitialize(&mut self.0).result()?;
            std::mem::transmute(self)
        })
    }

    #[doc(alias = "AudioCodecAppendInputData")]
    #[inline]
    pub fn append(&mut self, data: &[u8]) -> os::Result<u32> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = 0;
        unsafe {
            AudioCodecAppendInputData(
                &mut self.0,
                data.as_ptr(),
                &mut data_len,
                &mut packets_len,
                std::ptr::null(),
            )
            .result()?;
        }
        Ok(data_len)
    }

    /// Append as much of the given data to the codec's input buffer as possible
    /// and return in (data_len, packets_len) the amount of data and packets used.
    #[doc(alias = "AudioCodecAppendInputData")]
    #[inline]
    pub fn append_data(
        &mut self,
        data: &[u8],
        packets: &[audio::StreamPacketDesc],
    ) -> os::Result<Consumed> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = packets.len() as _;
        unsafe {
            AudioCodecAppendInputData(
                &mut self.0,
                data.as_ptr(),
                &mut data_len,
                &mut packets_len,
                packets.as_ptr(),
            )
            .result()?;
        }

        Ok(Consumed {
            bytes: data_len,
            packets: packets_len,
        })
    }

    #[doc(alias = "AudioCodecProduceOutputPackets")]
    #[inline]
    pub fn produce_packet(
        &mut self,
        data: &mut [u8],
    ) -> os::Result<(u32, ProduceOutputPacketStatus)> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = 1;
        let mut status = ProduceOutputPacketStatus::Failure;

        unsafe {
            AudioCodecProduceOutputPackets(
                &mut self.0,
                data.as_mut_ptr(),
                &mut data_len,
                &mut packets_len,
                std::ptr::null_mut(),
                &mut status,
            )
            .result()?;
        }

        Ok((data_len, status))
    }

    #[doc(alias = "AudioCodecProduceOutputPackets")]
    #[inline]
    pub fn produce_packets(
        &mut self,
        data: &mut [u8],
        out_packet_descriptions: &mut [audio::StreamPacketDesc],
    ) -> os::Result<Produced> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = out_packet_descriptions.len() as _;
        let mut status = ProduceOutputPacketStatus::Failure;

        unsafe {
            AudioCodecProduceOutputPackets(
                &mut self.0,
                data.as_mut_ptr(),
                &mut data_len,
                &mut packets_len,
                out_packet_descriptions.as_mut_ptr(),
                &mut status,
            )
            .result()?;
        }
        Ok(Produced {
            bytes: data_len,
            packets: packets_len,
            status,
        })
    }

    #[doc(alias = "AudioCodecAppendInputBufferList")]
    #[inline]
    pub fn append_buf_list(
        &mut self,
        in_buf_list: &audio::BufList,
        packet_descriptions: &mut [audio::StreamPacketDesc],
    ) -> os::Result<Consumed> {
        let mut bytes_consumed: u32 = 0;
        let mut packets_len: u32 = packet_descriptions.len() as _;
        unsafe {
            AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buf_list,
                &mut packets_len,
                packet_descriptions.as_ptr(),
                &mut bytes_consumed,
            )
            .result()?;
        }

        Ok(Consumed {
            bytes: bytes_consumed,
            packets: packets_len,
        })
    }

    #[doc(alias = "AudioCodecProduceOutputBufferList")]
    #[inline]
    pub fn produce_buf_list(
        &mut self,
        buf_list: &mut audio::BufList,
        number_of_packets: &mut u32,
    ) -> os::Result<os::Status> {
        let mut status = os::Status::NO_ERR;
        unsafe {
            AudioCodecProduceOutputBufferList(
                &mut self.0,
                buf_list,
                number_of_packets,
                std::ptr::null_mut(),
                &mut status,
            )
            .result()?;
        }

        Ok(status)
    }

    #[doc(alias = "AudioCodecProduceOutputBufferList")]
    #[inline]
    pub fn produce_buf_list_with_descs(
        &mut self,
        buf_list: &mut audio::BufList,
        packet_descriptions: &mut [audio::StreamPacketDesc],
    ) -> os::Result<(u32, os::Status)> {
        let mut number_packets: u32 = packet_descriptions.len() as _;
        let mut status = os::Status::NO_ERR;
        unsafe {
            AudioCodecProduceOutputBufferList(
                &mut self.0,
                buf_list,
                &mut number_packets,
                packet_descriptions.as_mut_ptr(),
                &mut status,
            )
            .result()?;
        }

        Ok((number_packets, status))
    }
}

impl Codec {
    pub unsafe fn init_codec(
        &mut self,
        input_format: *const audio::StreamBasicDesc,
        output_format: *const audio::StreamBasicDesc,
        magic_cookie: Option<&[u8]>,
    ) -> os::Result {
        unsafe {
            match magic_cookie {
                Some(cookie) => AudioCodecInitialize(
                    self,
                    input_format,
                    output_format,
                    cookie.as_ptr(),
                    cookie.len() as _,
                )
                .result(),
                None => {
                    AudioCodecInitialize(self, input_format, output_format, std::ptr::null(), 0)
                        .result()
                }
            }
        }
    }

    #[doc(alias = "AudioCodecGetPropertyInfo")]
    #[inline]
    pub fn prop_info(&self, prop_id: u32) -> os::Result<(u32, bool)> {
        let (mut size, mut writable) = (0u32, false);
        unsafe { AudioCodecGetPropertyInfo(&self, prop_id, &mut size, &mut writable).result()? };
        Ok((size, writable))
    }

    #[doc(alias = "AudioCodecGetProperty")]
    #[inline]
    pub unsafe fn prop_vec<T: Sized + Default + Clone>(&self, prop_id: u32) -> os::Result<Vec<T>> {
        let (mut size, _) = self.prop_info(prop_id)?;
        if size == 0 {
            return Ok(vec![]);
        }
        let mut vec = vec![T::default(); size as usize / std::mem::size_of::<T>()];
        unsafe {
            AudioCodecGetProperty(self, prop_id, &mut size, vec.as_mut_ptr() as _).result()?;
        }
        Ok(vec)
    }

    #[doc(alias = "AudioCodecGetProperty")]
    pub fn prop<T: Sized>(&self, prop_id: u32) -> os::Result<T> {
        let mut size = std::mem::size_of::<T>() as u32;
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            AudioCodecGetProperty(self, prop_id, &mut size, value.as_mut_ptr() as _).result()?;
            Ok(value.assume_init())
        }
    }

    pub unsafe fn set_prop<T: Sized>(&mut self, property_id: u32, val: &T) -> os::Result {
        let size = std::mem::size_of::<T>() as u32;
        unsafe { AudioCodecSetProperty(self, property_id, size, val as *const _ as _).result() }
    }

    #[inline]
    pub fn quality(&self) -> os::Result<u32> {
        self.prop(InstancePropId::QUALITY_SETTING.0)
    }

    #[inline]
    pub fn set_quality(&mut self, val: u32) -> os::Result {
        unsafe { self.set_prop(InstancePropId::QUALITY_SETTING.0, &val) }
    }

    #[inline]
    pub fn bit_rate_control_mode(&self) -> os::Result<BitRateControlMode> {
        self.prop(InstancePropId::BIT_RATE_CONTROL_MODE.0)
    }

    #[inline]
    pub fn set_bit_rate_control_mode(&mut self, val: BitRateControlMode) -> os::Result {
        unsafe { self.set_prop(InstancePropId::BIT_RATE_CONTROL_MODE.0, &val) }
    }

    #[inline]
    pub fn current_target_bit_rate(&self) -> os::Result<u32> {
        self.prop(InstancePropId::CURRENT_TARGET_BIT_RATE.0)
    }

    #[inline]
    pub fn set_current_target_bit_rate(&mut self, val: u32) -> os::Result {
        unsafe { self.set_prop(InstancePropId::CURRENT_TARGET_BIT_RATE.0, &val) }
    }

    #[inline]
    pub fn current_input_sample_rate(&self) -> os::Result<f64> {
        self.prop(InstancePropId::CURRENT_INPUT_SAMPLE_RATE.0)
    }

    /// A f64 containing the current input sample rate in Hz. No Default.
    /// May be writable. If only one sample rate is supported it does not have to be.
    #[inline]
    pub fn set_current_input_sample_rate(&mut self, val: f64) -> os::Result {
        unsafe { self.set_prop(InstancePropId::CURRENT_INPUT_SAMPLE_RATE.0, &val) }
    }

    /// A f64 containing the current output sample rate in Hz. No Default.
    /// May be writable. If only one sample rate is supported it does not have to be.
    #[inline]
    pub fn current_output_sample_rate(&self) -> os::Result<f64> {
        self.prop(InstancePropId::CURRENT_OUTPUT_SAMPLE_RATE.0)
    }

    #[inline]
    pub fn set_current_output_sample_rate(&mut self, val: f64) -> os::Result {
        unsafe { self.set_prop(InstancePropId::CURRENT_OUTPUT_SAMPLE_RATE.0, &val) }
    }

    #[inline]
    pub fn set_current_input_channel_layout<const N: usize>(
        &mut self,
        val: &audio::ChannelLayout<N>,
    ) -> os::Result {
        unsafe { self.set_prop(InstancePropId::CURRENT_INPUT_CHANNEL_LAYOUT.0, val) }
    }

    #[inline]
    pub fn current_input_channel_layout<const N: usize>(
        &self,
    ) -> os::Result<audio::ChannelLayout<N>> {
        self.prop(InstancePropId::CURRENT_INPUT_CHANNEL_LAYOUT.0)
    }

    #[inline]
    pub fn set_current_output_channel_layout<const N: usize>(
        &mut self,
        val: &audio::ChannelLayout<N>,
    ) -> os::Result {
        unsafe { self.set_prop(InstancePropId::CURRENT_OUTPUT_CHANNEL_LAYOUT.0, val) }
    }

    #[inline]
    pub fn current_output_channel_layout<const N: usize>(
        &self,
    ) -> os::Result<audio::ChannelLayout<N>> {
        self.prop(InstancePropId::CURRENT_OUTPUT_CHANNEL_LAYOUT.0)
    }

    #[inline]
    pub fn applicable_input_sample_rates(&self) -> os::Result<Vec<audio::ValueRange>> {
        unsafe { self.prop_vec(InstancePropId::APPLICABLE_INPUT_SAMPLE_RATES.0) }
    }

    #[inline]
    pub fn applicable_output_sample_rates(&self) -> os::Result<Vec<audio::ValueRange>> {
        unsafe { self.prop_vec(InstancePropId::APPLICABLE_OUTPUT_SAMPLE_RATES.0) }
    }

    #[inline]
    pub fn recommended_bit_rate_range(&self) -> os::Result<Vec<audio::ValueRange>> {
        unsafe { self.prop_vec(InstancePropId::RECOMMENDED_BIT_RATE_RANGE.0) }
    }

    #[inline]
    pub fn applicable_bit_rate_range(&self) -> os::Result<Vec<audio::ValueRange>> {
        unsafe { self.prop_vec(InstancePropId::APPLICABLE_BIT_RATE_RANGE.0) }
    }

    #[inline]
    pub fn supported_input_formats(&self) -> os::Result<Vec<audio::StreamBasicDesc>> {
        unsafe { self.prop_vec(GlobalPropId::SUPPORTED_INPUT_FORMATS.0) }
    }

    #[inline]
    pub fn supported_output_formats(&self) -> os::Result<Vec<audio::StreamBasicDesc>> {
        unsafe { self.prop_vec(GlobalPropId::SUPPORTED_OUTPUT_FORMATS.0) }
    }

    /// Flushes all the data in the codec and clears the input buffer. Note that
    /// the formats, and magic cookie will be retained so they won't need to be
    /// set up again to decode the same data.
    #[doc(alias = "AudioCodecReset")]
    pub fn reset(&mut self) -> os::Result {
        unsafe { AudioCodecReset(self).result() }
    }
}

impl<S> CodecRef<S>
where
    S: State<Codec>,
{
    #[inline]
    pub fn supported_input_formats(&self) -> os::Result<Vec<audio::StreamBasicDesc>> {
        self.0.supported_input_formats()
    }

    #[inline]
    pub fn supported_output_formats(&self) -> os::Result<Vec<audio::StreamBasicDesc>> {
        self.0.supported_output_formats()
    }

    #[inline]
    pub fn bit_rate_control_mode(&self) -> os::Result<BitRateControlMode> {
        self.0.bit_rate_control_mode()
    }

    #[inline]
    pub fn recommended_bit_rate_range(&self) -> os::Result<Vec<audio::ValueRange>> {
        self.0.recommended_bit_rate_range()
    }

    #[inline]
    pub fn applicable_output_sample_rates(&self) -> os::Result<Vec<audio::ValueRange>> {
        self.0.applicable_output_sample_rates()
    }

    #[inline]
    pub fn applicable_input_sample_rates(&self) -> os::Result<Vec<audio::ValueRange>> {
        self.0.applicable_input_sample_rates()
    }

    #[inline]
    pub fn applicable_bit_rate_range(&self) -> os::Result<Vec<audio::ValueRange>> {
        self.0.applicable_bit_rate_range()
    }

    #[inline]
    pub fn magic_cookie(&self) -> os::Result<Vec<u8>> {
        unsafe { self.0.prop_vec(InstancePropId::MAGIC_COOKIE.0) }
    }

    #[inline]
    pub fn current_output_format(&self) -> os::Result<audio::StreamBasicDesc> {
        let mut value = audio::StreamBasicDesc::default();
        let mut size = std::mem::size_of::<audio::StreamBasicDesc>() as u32;
        unsafe {
            AudioCodecGetProperty(
                &self.0,
                InstancePropId::CURRENT_OUTPUT_FORMAT.0,
                &mut size,
                &mut value as *mut _ as _,
            )
            .result()?;
        }
        Ok(value)
    }

    #[inline]
    pub fn current_input_format(&self) -> os::Result<audio::StreamBasicDesc> {
        let mut value = audio::StreamBasicDesc::default();
        let mut size = std::mem::size_of::<audio::StreamBasicDesc>() as u32;
        unsafe {
            AudioCodecGetProperty(
                &self.0,
                InstancePropId::CURRENT_INPUT_FORMAT.0,
                &mut size,
                &mut value as *mut _ as _,
            )
            .result()?;
        }
        Ok(value)
    }

    #[inline]
    pub fn max_packet_byte_size(&self) -> os::Result<u32> {
        let (mut value, mut size) = (0u32, 4u32);
        unsafe {
            AudioCodecGetProperty(
                &self.0,
                InstancePropId::MAXIMUM_PACKET_BYTE_SIZE.0,
                &mut size,
                &mut value as *mut u32 as *mut u8,
            )
            .result()?;
        }
        Ok(value)
    }

    #[inline]
    pub fn input_buf_size(&self) -> os::Result<u32> {
        let (mut value, mut size) = (0u32, 4u32);
        unsafe {
            AudioCodecGetProperty(
                &self.0,
                InstancePropId::INPUT_BUF_SIZE.0,
                &mut size,
                &mut value as *mut u32 as *mut u8,
            )
            .result()?;
        }
        Ok(value)
    }

    #[inline]
    pub fn quality(&self) -> os::Result<u32> {
        self.0.quality()
    }
}

impl audio::Component {
    pub fn open_codec(&self) -> os::Result<CodecRef<UninitializedState>> {
        Ok(unsafe { std::mem::transmute(self.open()?) })
    }
}

#[doc(alias = "AudioCodecProduceOutputPacketStatus")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum ProduceOutputPacketStatus {
    /// Couldn't complete the request due to an error. It is possible
    /// that some output data was produced. This is reflected in the value
    /// returned in ioNumberPackets.
    #[doc(alias = "kAudioCodecProduceOutputPacketFailure")]
    Failure = 1,

    /// The number of requested output packets was produced without incident
    /// and there isn't any more input data to process
    #[doc(alias = "kAudioCodecProduceOutputPacketSuccess")]
    Success = 2,

    /// The number of requested output packets was produced and there is
    /// enough input data to produce at least one more packet of output data
    #[doc(alias = "kAudioCodecProduceOutputPacketSuccessHasMore")]
    SuccessHasMore = 3,

    /// There was insufficient input data to produce the requested
    /// number of output packets, The value returned in ioNumberPackets
    /// holds the number of output packets produced.
    #[doc(alias = "kAudioCodecProduceOutputPacketNeedsMoreInputData")]
    NeedsMoreInputData = 4,

    /// The end-of-file marker was hit during the processing. Fewer
    /// than the requested number of output packets may have been
    /// produced. Check the value returned in ioNumberPackets for the
    /// actual number produced. Note that not all formats have EOF
    /// markers in them.    
    #[doc(alias = "kAudioCodecProduceOutputPacketAtEOF")]
    AtEof = 5,

    /// No input packets were provided, but the decoder supports packet
    /// loss concealment, so output packets were still created.
    #[doc(alias = "kAudioCodecProduceOutputPacketSuccessConcealed")]
    SuccessConcealed = 6,
}

/// Constants to be used with kAudioCodecPropertyQualitySetting
pub mod quality {
    #[doc(alias = "kAudioCodecQuality_Max")]
    pub const MAX: u32 = 0x7F;

    #[doc(alias = "kAudioCodecQuality_High")]
    pub const HIGH: u32 = 0x60;

    #[doc(alias = "kAudioCodecQuality_Medium")]
    pub const MEDIUM: u32 = 0x40;

    #[doc(alias = "kAudioCodecQuality_Low")]
    pub const LOW: u32 = 0x20;

    #[doc(alias = "kAudioCodecQuality_Min")]
    pub const MIN: u32 = 0x00;
}

unsafe extern "C-unwind" {
    fn AudioCodecReset(in_codec: &mut Codec) -> os::Status;

    fn AudioCodecInitialize(
        in_codec: &mut Codec,
        in_input_format: *const audio::StreamBasicDesc,
        in_output_format: *const audio::StreamBasicDesc,
        in_magic_cookie: *const u8,
        in_magic_cookie_size: u32,
    ) -> os::Status;

    fn AudioCodecUninitialize(in_codec: &mut Codec) -> os::Status;

    fn AudioCodecAppendInputData(
        in_codec: &mut Codec,
        in_input_data: *const u8,
        io_input_data_byte_size: &mut u32,
        io_number_packets: &mut u32,
        in_packet_description: *const audio::StreamPacketDesc,
    ) -> os::Status;

    fn AudioCodecProduceOutputPackets(
        in_codec: &mut Codec,
        out_output_data: *mut u8,
        io_output_data_byte_size: &mut u32,
        io_number_packets: &mut u32,
        out_packet_description: *mut audio::StreamPacketDesc,
        out_status: &mut ProduceOutputPacketStatus,
    ) -> os::Status;

    fn AudioCodecAppendInputBufferList(
        in_codec: &mut Codec,
        in_buffer_list: *const audio::BufList,
        io_number_packets: &mut u32,
        in_packet_descriptions: *const audio::StreamPacketDesc,
        out_bytes_consumed: &mut u32,
    ) -> os::Status;

    fn AudioCodecProduceOutputBufferList(
        in_codec: &mut Codec,
        io_buffer_list: &mut audio::BufList,
        io_number_packets: &mut u32,
        out_packet_description: *mut audio::StreamPacketDesc,
        out_status: &mut os::Status,
    ) -> os::Status;

    fn AudioCodecSetProperty(
        in_codec: &mut Codec,
        in_property_id: u32,
        in_property_size: u32,
        in_property_data: *const u8,
    ) -> os::Status;

    fn AudioCodecGetProperty(
        in_codec: &Codec,
        in_property_id: u32,
        io_property_data_size: &mut u32,
        out_property_data: *mut u8,
    ) -> os::Status;

    fn AudioCodecGetPropertyInfo(
        in_codec: &Codec,
        in_property_id: u32,
        out_size: *mut u32,
        out_writable: *mut bool,
    ) -> os::Status;

}

#[cfg(test)]
mod tests {
    use audio::CodecRef;

    use crate::{at::audio, four_cc_to_str};

    #[test]
    fn basics() {
        let channels_per_frame = 2;
        let sample_rate = 44_100.0;
        let src_asbd = audio::StreamBasicDesc {
            sample_rate,
            channels_per_frame,
            format: audio::Format::LINEAR_PCM,
            format_flags: audio::FormatFlags::IS_FLOAT | audio::FormatFlags::IS_PACKED,
            bytes_per_packet: 4 * channels_per_frame,
            frames_per_packet: 1,
            bytes_per_frame: 4 * channels_per_frame,
            bits_per_channel: 32,
            ..Default::default()
        };

        let dst_asbd = audio::StreamBasicDesc {
            sample_rate,
            channels_per_frame,
            format: audio::Format::MPEG4_AAC,
            format_flags: audio::FormatFlags::ALL_CLEAR,
            frames_per_packet: 1024,
            ..Default::default()
        };

        let desc = audio::ComponentDesc {
            type_: audio::ENCODER_COMPONENT_TYPE,
            sub_type: u32::from_be_bytes(*b"aac "),
            ..Default::default()
        };

        let inst = desc.into_iter().last().unwrap();

        let codec = inst.open_codec().unwrap();

        let recommended_bit_rate_range = codec.recommended_bit_rate_range().unwrap();
        println!("{recommended_bit_rate_range:?}");
        assert!(!recommended_bit_rate_range.is_empty());

        let applicable_output_sample_rates = codec.applicable_output_sample_rates().unwrap();
        println!("{applicable_output_sample_rates:?}");
        assert!(!applicable_output_sample_rates.is_empty());

        let supported_input_formats = codec.supported_input_formats().unwrap();
        println!("{supported_input_formats:?}");
        assert!(!supported_input_formats.is_empty());

        let applicable_output_sample_rates = codec.applicable_output_sample_rates().unwrap();
        println!("{applicable_output_sample_rates:?}");
        assert!(!applicable_output_sample_rates.is_empty());

        let mode = codec.bit_rate_control_mode().unwrap();
        assert_eq!(audio::CodecBitRateControlMode::LongTermAverage, mode);

        let codec = codec.initialize(&src_asbd, &dst_asbd, None).unwrap();
        let cookie_info = codec.magic_cookie().unwrap();
        assert!(!cookie_info.is_empty());
        let max_packet_size = codec.max_packet_byte_size().unwrap();
        assert_eq!(max_packet_size, 1536);

        let quality = codec.quality().unwrap();
        assert_eq!(quality, audio::codec_quality::MEDIUM);
    }

    #[test]
    fn encoder_list() {
        let desc = audio::ComponentDesc {
            type_: audio::ENCODER_COMPONENT_TYPE,
            manufacturer: audio::UnitManufacturer::APPLE.0,
            ..Default::default()
        };

        for comp in desc.into_iter() {
            println!(
                "desc {:?} {:?} {}",
                comp.name(),
                comp.desc(),
                four_cc_to_str(&mut comp.desc().unwrap().sub_type.to_be_bytes())
            );
        }
    }

    #[test]
    fn codec_init() {
        let sample_rate = 48000.0;
        let channels_per_frame = 2;
        let src_asbd = audio::StreamBasicDesc {
            sample_rate,
            format: audio::Format(1819304813),
            // format: audio::FormatID::LINEAR_PCM,
            format_flags: audio::FormatFlags(41),
            // format_flags: audio::FormatFlags(0),
            bytes_per_packet: 4,
            frames_per_packet: 1,
            bytes_per_frame: 4,
            channels_per_frame,
            bits_per_channel: 32,
            reserved: 0,
        };

        let src_asbd2 = audio::StreamBasicDesc {
            sample_rate,
            channels_per_frame,
            format: audio::Format::LINEAR_PCM,
            format_flags: audio::FormatFlags::NATIVE_FLOAT_PACKED, //audio::FormatFlags::IS_FLOAT | audio::FormatFlags::IS_PACKED,
            bytes_per_packet: 4 * channels_per_frame,
            frames_per_packet: 1,
            bytes_per_frame: 4 * channels_per_frame,
            bits_per_channel: 32,
            ..Default::default()
        };

        println!(
            "
                {src_asbd:#?}
                {src_asbd2:#?}
            "
        );

        let dst_asbd = audio::StreamBasicDesc {
            sample_rate,
            channels_per_frame,
            format: audio::Format::MPEG4_AAC,
            format_flags: audio::FormatFlags::ALL_CLEAR,
            frames_per_packet: 1024,
            ..Default::default()
        };

        let encoder = CodecRef::new_apple_aac_encoder().unwrap();
        let _encoder = encoder.initialize(&src_asbd2, &dst_asbd, None).unwrap();
    }
}
