use crate::{at::audio, os};

pub type Codec = audio::ComponentInstance;
pub struct CodecRef(audio::ComponentInstanceRef);

#[repr(transparent)]
pub struct GlobalPropertyID(pub u32);

#[repr(transparent)]
pub struct InstancePropertyID(pub u32);

impl GlobalPropertyID {
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

    /// An array of AudioValueRange indicating the valid ranges for the
    /// output sample rate of the codec.
    /// Required for encoders.
    /// (see also kAudioCodecPropertyApplicableOutputSampleRates)
    #[doc(alias = "kAudioCodecPropertyAvailableOutputSampleRates")]
    pub const AVAILABLE_OUTPUT_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"aosr"));

    /// An array of AudioValueRange that indicate the target bit rates
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
impl InstancePropertyID {
    /// A u32 indicating the maximum input buffer size for the codec
    /// in bytes.
    /// Not writable, but can vary on some codecs depending on the bit stream
    /// format being handled.
    #[doc(alias = "kAudioCodecPropertyInputBufferSize")]
    pub const INPUT_BUFFER_SIZE: Self = Self(u32::from_be_bytes(*b"tbuf"));

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
    pub const USED_INPUT_BUFFER_SIZE: Self = Self(u32::from_be_bytes(*b"ubuf"));

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

    /// An array of AudioValueRange indicating the valid ranges for the
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

impl audio::ComponentInstanceRef {
    pub fn into_codec(
        mut self,
        input_format: *const audio::StreamBasicDescription,
        output_format: *const audio::StreamBasicDescription,
        magic_cookie: Option<&[u8]>,
    ) -> Result<CodecRef, os::Status> {
        unsafe {
            self.init_codec(input_format, output_format, magic_cookie)?;
        }
        Ok(CodecRef(self))
    }
}

impl Drop for CodecRef {
    #[inline]
    fn drop(&mut self) {
        let res = unsafe { self.0.uninitialize() };
        debug_assert!(res.is_ok());
    }
}

impl CodecRef {
    /// Append as much of the given data to the codec's input buffer as possible
    /// and return in (data_len, packets_len) the amount of data and packets used.
    #[inline]
    pub fn append_input_data(
        &mut self,
        data: &[u8],
        packets: &[audio::StreamPacketDescription],
    ) -> Result<(u32, u32), os::Status> {
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

        Ok((data_len, packets_len))
    }

    #[inline]
    pub fn produce_output_packets(
        &mut self,
        data: &mut [u8],
    ) -> Result<(u32, os::Status), os::Status> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = 0;
        let mut status = os::Status::NO_ERR;

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

    #[inline]
    pub fn produce_output_packets_with_descriptions(
        &mut self,
        data: &mut [u8],
        out_packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, u32, os::Status), os::Status> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = out_packet_descriptions.len() as _;
        let mut status = os::Status::NO_ERR;

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
        Ok((data_len, packets_len, status))
    }

    #[inline]
    pub fn append_input_buffer_list(
        &mut self,
        in_buffer_list: &audio::BufferList,
    ) -> Result<u32, os::Status> {
        let mut bytes_consumed: u32 = 0;
        let mut packets_len: u32 = 0;
        unsafe {
            AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buffer_list,
                &mut packets_len,
                std::ptr::null(),
                &mut bytes_consumed,
            )
            .result()?;
        }

        Ok(bytes_consumed)
    }

    #[inline]
    pub fn append_input_buffer_list_with_descriptions(
        &mut self,
        in_buffer_list: &audio::BufferList,
        packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, u32), os::Status> {
        let mut bytes_consumed: u32 = 0;
        let mut packets_len: u32 = packet_descriptions.len() as _;
        unsafe {
            AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buffer_list,
                &mut packets_len,
                packet_descriptions.as_ptr(),
                &mut bytes_consumed,
            )
            .result()?;
        }

        Ok((bytes_consumed, packets_len))
    }

    #[inline]
    pub fn produce_output_buffer_list(
        &mut self,
        buffer_list: &mut audio::BufferList,
    ) -> Result<os::Status, os::Status> {
        let mut number_packets: u32 = 0;
        let mut status = os::Status::NO_ERR;
        unsafe {
            AudioCodecProduceOutputBufferList(
                &mut self.0,
                buffer_list,
                &mut number_packets,
                std::ptr::null_mut(),
                &mut status,
            )
            .result()?;
        }

        Ok(status)
    }

    #[inline]
    pub fn produce_output_buffer_list_with_descriptions(
        &mut self,
        buffer_list: &mut audio::BufferList,
        packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, os::Status), os::Status> {
        let mut number_packets: u32 = packet_descriptions.len() as _;
        let mut status = os::Status::NO_ERR;
        unsafe {
            AudioCodecProduceOutputBufferList(
                &mut self.0,
                buffer_list,
                &mut number_packets,
                packet_descriptions.as_mut_ptr(),
                &mut status,
            )
            .result()?;
        }

        Ok((number_packets, status))
    }

    #[doc(alias = "AudioCodecSetProperty")]
    #[inline]
    pub unsafe fn set_prop(
        &mut self,
        property_id: u32,
        property_size: u32,
        property_data: *const u8,
    ) -> Result<(), os::Status> {
        unsafe {
            AudioCodecSetProperty(&mut self.0, property_id, property_size, property_data).result()
        }
    }

    #[doc(alias = "AudioCodecGetPropertyInfo")]
    #[inline]
    pub unsafe fn prop_info(&self, property_id: u32) -> Result<(u32, bool), os::Status> {
        let (mut size, mut writable) = (0u32, false);
        unsafe {
            AudioCodecGetPropertyInfo(&self.0, property_id, &mut size, &mut writable).result()?
        };
        Ok((size, writable))
    }

    #[inline]
    pub fn magic_cookie(&self) -> Result<Vec<u8>, os::Status> {
        unsafe {
            let (mut size, _) = self.prop_info(InstancePropertyID::MAGIC_COOKIE.0)?;
            let mut vec = vec![0u8; size as _];
            AudioCodecGetProperty(
                &self.0,
                InstancePropertyID::MAGIC_COOKIE.0,
                &mut size,
                vec.as_mut_ptr(),
            )
            .result()?;
            Ok(vec)
        }
    }

    #[inline]
    pub fn maximum_packet_byte_size(&self) -> Result<usize, os::Status> {
        let (mut value, mut size) = (0u32, 4u32);
        unsafe {
            AudioCodecGetProperty(
                &self.0,
                InstancePropertyID::MAXIMUM_PACKET_BYTE_SIZE.0,
                &mut size,
                &mut value as *mut u32 as *mut u8,
            )
            .result()?;
        }
        Ok(value as _)
    }
}

impl Codec {
    pub unsafe fn init_codec(
        &mut self,
        input_format: *const audio::StreamBasicDescription,
        output_format: *const audio::StreamBasicDescription,
        magic_cookie: Option<&[u8]>,
    ) -> Result<(), os::Status> {
        unsafe {
            match magic_cookie {
                Some(cookie) => AudioCodecInitialize(
                    self,
                    input_format,
                    output_format,
                    cookie.as_ptr(),
                    cookie.len() as _,
                ),
                None => {
                    AudioCodecInitialize(self, input_format, output_format, std::ptr::null(), 0)
                }
            }
            .result()
        }
    }

    /// Flushes all the data in the codec and clears the input buffer. Note that
    /// the formats, and magic cookie will be retained so they won't need to be
    /// set up again to decode the same data.
    #[doc(alias = "AudioCodecReset")]
    pub fn reset(&mut self) -> Result<(), os::Status> {
        unsafe { AudioCodecReset(self).result() }
    }

    /// This call will move the codec from the initialized state back to the
    /// uninitialized state. The codec will release any resources it allocated
    /// or claimed in AudioCodecInitialize.
    pub unsafe fn uninitialize(&mut self) -> Result<(), os::Status> {
        AudioCodecUninitialize(self).result()
    }
}

impl Drop for Codec {
    fn drop(&mut self) {
        let res = unsafe { self.uninitialize() };
        debug_assert!(res.is_ok());
    }
}

extern "C" {
    fn AudioCodecReset(in_codec: &mut Codec) -> os::Status;
    fn AudioCodecInitialize(
        in_codec: &mut Codec,
        in_input_format: *const audio::StreamBasicDescription,
        in_output_format: *const audio::StreamBasicDescription,
        in_magic_cookie: *const u8,
        in_magic_cookie_size: u32,
    ) -> os::Status;

    fn AudioCodecUninitialize(in_codec: &mut Codec) -> os::Status;

    fn AudioCodecAppendInputData(
        in_codec: &mut Codec,
        in_input_data: *const u8,
        io_input_data_byte_size: &mut u32,
        io_number_packets: &mut u32,
        in_packet_description: *const audio::StreamPacketDescription,
    ) -> os::Status;

    fn AudioCodecProduceOutputPackets(
        in_codec: &mut Codec,
        out_output_data: *mut u8,
        io_output_data_byte_size: &mut u32,
        io_number_packets: &mut u32,
        out_packet_description: *mut audio::StreamPacketDescription,
        out_status: &mut os::Status,
    ) -> os::Status;

    fn AudioCodecAppendInputBufferList(
        in_codec: &mut Codec,
        in_buffer_list: *const audio::BufferList,
        io_number_packets: &mut u32,
        in_packet_descriptions: *const audio::StreamPacketDescription,
        out_bytes_consumed: &mut u32,
    ) -> os::Status;

    fn AudioCodecProduceOutputBufferList(
        in_codec: &mut Codec,
        io_buffer_list: &mut audio::BufferList,
        io_number_packets: &mut u32,
        out_packet_description: *mut audio::StreamPacketDescription,
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
    use crate::at::audio;

    #[test]
    fn basics() {
        let channels_per_frame = 2;
        let sample_rate = 44_100.0;
        let src_asbd = audio::StreamBasicDescription {
            sample_rate,
            channels_per_frame,
            format_id: audio::FormatID::LINEAR_PCM,
            format_flags: audio::FormatFlags::IS_FLOAT | audio::FormatFlags::IS_PACKED,
            bytes_per_packet: 4 * channels_per_frame,
            frames_per_packet: 1,
            bytes_per_frame: 4 * channels_per_frame,
            bits_per_channel: 32,
            ..Default::default()
        };

        let dst_asbd = audio::StreamBasicDescription {
            sample_rate,
            channels_per_frame,
            format_id: audio::FormatID::MPEG4_AAC,
            format_flags: audio::FormatFlags::ALL_CLEAR,
            frames_per_packet: 1024,
            ..Default::default()
        };

        let desc = audio::ComponentDescription {
            type_: audio::ENCODER_COMPONENT_TYPE,
            sub_type: u32::from_be_bytes(*b"aac "),
            ..Default::default()
        };

        let inst = desc.into_iter().last().unwrap().new_instance().unwrap();
        let codec = inst.into_codec(&src_asbd, &dst_asbd, None).unwrap();
        let cookie_info = codec.magic_cookie().unwrap();
        assert!(!cookie_info.is_empty());
        let max_packet_size = codec.maximum_packet_byte_size().unwrap();
        assert_eq!(max_packet_size, 1536);

        codec.show();
    }
}
