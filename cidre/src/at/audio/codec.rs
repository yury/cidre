use crate::{at::audio, os};

pub type Codec = audio::ComponentInstance;
pub struct CodecRef(audio::ComponentInstanceRef);
pub struct PropertyID(pub u32);

impl PropertyID {
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
        let res = unsafe { self.init_codec(input_format, output_format, magic_cookie) };
        if res.is_ok() {
            Ok(CodecRef(self))
        } else {
            Err(res.err().unwrap())
        }
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
        let res = unsafe {
            AudioCodecAppendInputData(
                &mut self.0,
                data.as_ptr(),
                &mut data_len,
                &mut packets_len,
                packets.as_ptr(),
            )
        };

        if res.is_ok() {
            Ok((data_len, packets_len))
        } else {
            Err(res)
        }
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
            let res = AudioCodecProduceOutputPackets(
                &mut self.0,
                data.as_mut_ptr(),
                &mut data_len,
                &mut packets_len,
                std::ptr::null_mut(),
                &mut status,
            );
            if res.is_ok() {
                Ok((data_len, status))
            } else {
                Err(res)
            }
        }
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
            let res = AudioCodecProduceOutputPackets(
                &mut self.0,
                data.as_mut_ptr(),
                &mut data_len,
                &mut packets_len,
                out_packet_descriptions.as_mut_ptr(),
                &mut status,
            );
            if res.is_ok() {
                Ok((data_len, packets_len, status))
            } else {
                Err(res)
            }
        }
    }

    #[inline]
    pub fn append_input_buffer_list(
        &mut self,
        in_buffer_list: &audio::BufferList,
    ) -> Result<u32, os::Status> {
        let mut bytes_consumed: u32 = 0;
        let mut packets_len: u32 = 0;
        unsafe {
            let res = AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buffer_list,
                &mut packets_len,
                std::ptr::null(),
                &mut bytes_consumed,
            );
            if res.is_ok() {
                Ok(bytes_consumed)
            } else {
                Err(res)
            }
        }
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
            let res = AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buffer_list,
                &mut packets_len,
                packet_descriptions.as_ptr(),
                &mut bytes_consumed,
            );
            if res.is_ok() {
                Ok((bytes_consumed, packets_len))
            } else {
                Err(res)
            }
        }
    }

    #[inline]
    pub fn produce_output_buffer_list(
        &mut self,
        buffer_list: &mut audio::BufferList,
    ) -> Result<os::Status, os::Status> {
        let mut number_packets: u32 = 0;
        let mut status = os::Status::NO_ERR;
        unsafe {
            let res = AudioCodecProduceOutputBufferList(
                &mut self.0,
                buffer_list,
                &mut number_packets,
                std::ptr::null_mut(),
                &mut status,
            );

            if res.is_ok() {
                Ok(status)
            } else {
                Err(res)
            }
        }
    }

    pub fn produce_output_buffer_list_with_descriptions(
        &mut self,
        buffer_list: &mut audio::BufferList,
        packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, os::Status), os::Status> {
        let mut number_packets: u32 = packet_descriptions.len() as _;
        let mut status = os::Status::NO_ERR;
        unsafe {
            let res = AudioCodecProduceOutputBufferList(
                &mut self.0,
                buffer_list,
                &mut number_packets,
                packet_descriptions.as_mut_ptr(),
                &mut status,
            );

            if res.is_ok() {
                Ok((number_packets, status))
            } else {
                Err(res)
            }
        }
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
            let res = match magic_cookie {
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
            };
            res.result()
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
    }
}
