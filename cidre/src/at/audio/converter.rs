use std::{
    ffi::c_void,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{api, cat::audio, define_opts, os};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PropId(pub u32);

impl PropId {
    /// A u32 that indicates the size in bytes of the smallest buffer of input
    /// data that can be supplied via the AudioConverterInputProc or as the input to
    /// AudioConverterConvertBuffer
    #[doc(alias = "kAudioConverterPropertyMinimumInputBufferSize")]
    pub const MIN_INPUT_BUF_SIZE: Self = Self(u32::from_be_bytes(*b"mibs"));

    /// A UInt32 that indicates the size in bytes of the smallest buffer of output
    /// data that can be supplied to AudioConverterFillComplexBuffer or as the output to
    /// AudioConverterConvertBuffer
    #[doc(alias = "kAudioConverterPropertyMinimumOutputBufferSize")]
    pub const MIN_OUTPUT_BUF_SIZE: Self = Self(u32::from_be_bytes(*b"mobs"));

    /// a u32 that indicates the size in bytes of the largest single packet of
    /// data in the input format. This is mostly useful for variable bit rate
    /// compressed data (decoders).
    #[doc(alias = "kAudioConverterPropertyMaximumInputPacketSize")]
    pub const MAX_INPUT_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"xips"));

    /// a u32 that indicates the size in bytes of the largest single packet of
    /// data in the output format. This is mostly useful for variable bit rate
    /// compressed data (encoders).
    #[doc(alias = "kAudioConverterPropertyMaximumOutputPacketSize")]
    pub const MAX_OUTPUT_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"xops"));

    /// a u32 that on input holds a size in bytes that is desired for the output
    /// data. On output, it will hold the size in bytes of the input buffer required
    /// to generate that much output data. Note that some converters cannot do this
    /// calculation.
    #[doc(alias = "kAudioConverterPropertyCalculateInputBufferSize")]
    pub const CALCULATE_INPUT_BUF_SIZE: Self = Self(u32::from_be_bytes(*b"cibs"));

    /// a u32 that on input holds a size in bytes that is desired for the input
    /// data. On output, it will hold the size in bytes of the output buffer
    /// required to hold the output data that will be generated. Note that some
    /// converters cannot do this calculation.
    #[doc(alias = "kAudioConverterPropertyCalculateOutputBufferSize")]
    pub const CALCULATE_OUTPUT_BUF_SIZE: Self = Self(u32::from_be_bytes(*b"cobs"));

    /// The value of this property varies from format to format and is considered
    /// private to the format. It is treated as a buffer of untyped data.
    #[doc(alias = "kAudioConverterPropertyInputCodecParameters")]
    pub const INPUT_CODEC_PARAMS: Self = Self(u32::from_be_bytes(*b"icdp"));

    /// The value of this property varies from format to format and is considered
    /// private to the format. It is treated as a buffer of untyped data.
    #[doc(alias = "kAudioConverterPropertyOutputCodecParameters")]
    pub const OUTPUT_CODEC_PARAMS: Self = Self(u32::from_be_bytes(*b"ocdp"));

    /// An os::Type that specifies the sample rate converter algorithm to use (as defined in
    /// AudioToolbox/AudioUnitProperties.h)
    #[doc(alias = "kAudioConverterSampleRateConverterComplexity")]
    pub const SAMPLE_RATE_CONVERTER_COMPLEXITY: Self = Self(u32::from_be_bytes(*b"srca"));
    /// A u32 that specifies rendering quality of the sample rate converter (see
    /// enum constants below)
    #[doc(alias = "kAudioConverterSampleRateConverterQuality")]
    pub const SAMPLE_RATE_CONVERTER_QUALITY: Self = Self(u32::from_be_bytes(*b"srcq"));
    /// A f64 with value 0.0 <= x < 1.0 giving the initial subsample position of the
    /// sample rate converter.
    #[doc(alias = "kAudioConverterSampleRateConverterInitialPhase")]
    pub const SAMPLE_RATE_CONVERTER_INITIAL_PHASE: Self = Self(u32::from_be_bytes(*b"srcp"));

    /// A u32 that specifies rendering quality of a codec (see enum constants
    /// below)
    #[doc(alias = "kAudioConverterCodecQuality")]
    pub const CODEC_QUALITY: Self = Self(u32::from_be_bytes(*b"cdqu"));

    /// a u32 specifying priming method (usually for sample-rate converter) see
    /// explanation for struct AudioConverterPrimeInfo below along with enum
    /// constants
    #[doc(alias = "kAudioConverterPrimeMethod")]
    pub const PRIME_METHOD: Self = Self(u32::from_be_bytes(*b"prmm"));

    /// A pointer to AudioConverterPrimeInfo (see explanation for struct
    /// AudioConverterPrimeInfo below)
    #[doc(alias = "kAudioConverterPrimeInfo")]
    pub const PRIME_INFO: Self = Self(u32::from_be_bytes(*b"prim"));

    /// An array of i32's.  The size of the array is the number of output
    /// channels, and each element specifies which input channel's data is routed to
    /// that output channel (using a 0-based index of the input channels), or -1 if
    /// no input channel is to be routed to that output channel.  The default
    /// behavior is as follows. I = number of input channels, O = number of output
    ///  channels. When I > O, the first O inputs are routed to the first O outputs,
    /// and the remaining puts discarded.  When O > I, the first I inputs are routed
    /// to the first O outputs, and the remaining outputs are zeroed.
    ///
    /// A simple example for splitting mono input to stereo output (instead of routing
    /// the input to only the first output channel):
    ///
    /// ```C
    /// // this should be as large as the number of output channels:
    /// SInt32 channelMap[2] = { 0, 0 };
    /// AudioConverterSetProperty(theConverter, kAudioConverterChannelMap,
    /// sizeof(channelMap), channelMap);
    /// ```
    #[doc(alias = "kAudioConverterChannelMap")]
    pub const CHANNEL_MAP: Self = Self(u32::from_be_bytes(*b"chmp"));

    /// A *mut c_void pointing to memory set up by the caller. Required by some formats
    /// in order to decompress the input data.
    #[doc(alias = "kAudioConverterDecompressionMagicCookie")]
    pub const DECOMPRESSION_MAGIC_COOKIE: Self = Self(u32::from_be_bytes(*b"dmgc"));

    /// A *mut c_void pointing to memory set up by the caller. Returned by the converter
    /// so that it may be stored along with the output data. It can then be passed
    /// back to the converter for decompression at a later time.
    #[doc(alias = "kAudioConverterCompressionMagicCookie")]
    pub const COMPRESSION_MAGIC_COOKIE: Self = Self(u32::from_be_bytes(*b"cmgc"));

    /// A u32 containing the number of bits per second to aim for when encoding
    /// data. Some decoders will also allow you to get this property to discover the bit rate.
    #[doc(alias = "kAudioConverterEncodeBitRate")]
    pub const ENCODE_BIT_RATE: Self = Self(u32::from_be_bytes(*b"brat"));

    /// For encoders where the AudioConverter was created with an output sample rate
    /// of zero, and the codec can do rate conversion on its input, this provides a
    /// way to set the output sample rate. The property value is a f64.
    #[doc(alias = "kAudioConverterEncodeAdjustableSampleRate")]
    pub const ENCODE_ADJUSTABLE_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"ajsr"));

    /// The property value is an AudioChannelLayout.
    #[doc(alias = "kAudioConverterInputChannelLayout")]
    pub const INPUT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"icl "));

    /// The property value is an AudioChannelLayout.
    #[doc(alias = "kAudioConverterOutputChannelLayout")]
    pub const OUTPUT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"ocl "));

    /// The property value is an array of AudioValueRange describing applicable bit
    /// rates based on current settings.
    #[doc(alias = "kAudioConverterApplicableEncodeBitRates")]
    pub const APPLICABLE_ENCODE_BIT_RATES: Self = Self(u32::from_be_bytes(*b"aebr"));

    /// The property value is an array of AudioValueRange describing available bit
    /// rates based on the input format. You can get all available bit rates from
    /// the AudioFormat API.
    #[doc(alias = "kAudioConverterAvailableEncodeBitRates")]
    pub const AVAILABLE_ENCODE_BIT_RATES: Self = Self(u32::from_be_bytes(*b"vebr"));

    /// The property value is an array of AudioValueRange describing applicable
    /// sample rates based on current settings.
    #[doc(alias = "kAudioConverterApplicableEncodeSampleRates")]
    pub const APPLICABLE_ENCODE_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"aesr"));

    /// The property value is an array of AudioValueRange describing available
    /// sample rates based on the input format. You can get all available sample
    /// rates from the AudioFormat API.
    #[doc(alias = "kAudioConverterAvailableEncodeSampleRates")]
    pub const AVAILABLE_ENCODE_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"vesr"));

    /// The property value is an array of AudioChannelLayoutTags for the format and
    /// number of channels specified in the input format going to the encoder.
    #[doc(alias = "kAudioConverterAvailableEncodeChannelLayoutTags")]
    pub const AVAILABLE_ENCODE_CHANNEL_LAYOUT_TAGS: Self = Self(u32::from_be_bytes(*b"aecl"));

    /// Returns the current completely specified output AudioStreamBasicDescription.
    /// For example when encoding to AAC, your original output stream description
    /// will not have been completely filled out.
    #[doc(alias = "kAudioConverterCurrentOutputStreamDescription")]
    pub const CURRENT_OUTPUT_STREAM_DESC: Self = Self(u32::from_be_bytes(*b"acod"));

    /// Returns the current completely specified input AudioStreamBasicDescription.
    #[doc(alias = "kAudioConverterCurrentInputStreamDescription")]
    pub const CURRENT_INPUT_STREAM_DESC: Self = Self(u32::from_be_bytes(*b"acid"));

    /// Returns the a CFArray of property settings for converters.
    #[doc(alias = "kAudioConverterPropertySettings")]
    pub const SETTINGS: Self = Self(u32::from_be_bytes(*b"acps"));

    /// An i32 of the source bit depth to preserve. This is a hint to some
    /// encoders like lossless about how many bits to preserve in the input. The
    /// converter usually tries to preserve as many as possible, but a lossless
    /// encoder will do poorly if more bits are supplied than are desired in the
    /// output. The bit depth is expressed as a negative number if the source was floating point,
    /// e.g. -32 for float, -64 for double.
    #[doc(alias = "kAudioConverterPropertyBitDepthHint")]
    pub const BIT_DEPTH_HINT: Self = Self(u32::from_be_bytes(*b"acbd"));

    /// An array of AudioFormatListItem structs describing all the data formats produced by the
    /// encoder end of the AudioConverter. If the ioPropertyDataSize parameter indicates that
    /// outPropertyData is sizeof(AudioFormatListItem), then only the best format is returned.
    /// This property may be used for example to discover all the data formats produced by the AAC_HE2
    /// (AAC High Efficiency vers. 2) encoder.
    #[doc(alias = "kAudioConverterPropertyFormatList")]
    pub const FORMAT_LIST: Self = Self(u32::from_be_bytes(*b"flst"));

    /// A u32. Set to a value from the enum of dithering algorithms below.
    /// Zero means no dithering and is the default. (macOS only.)
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioConverterPropertyDithering")]
    pub const DITHERING: Self = Self(u32::from_be_bytes(*b"dith"));

    /// A u32. Dither is applied at this bit depth.  (macOS only.)
    #[cfg(target_os = "macos")]
    #[doc(alias = "kAudioConverterPropertyDitherBitDepth")]
    pub const DITHER_BIT_DEPTH: Self = Self(u32::from_be_bytes(*b"dbit"));
}

/// kAudioConverterPropertyDithering
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct DitherAlgorithm(pub u32);

impl DitherAlgorithm {
    /// Dither signal is generated by a white noise source with
    /// a triangular probability density function
    #[doc(alias = "kDitherAlgorithm_TPDF")]
    pub const TPDF: Self = Self(1);

    /// Use a static, perceptually weighted noise shaped dither
    #[doc(alias = "kDitherAlgorithm_NoiseShaping")]
    pub const NOISE_SHAPING: Self = Self(2);
}

#[doc(alias = "AudioConverterQuality")]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Quality(pub u32);

/// kAudioConverterSampleRateConverterQuality
impl Quality {
    /// maximum quality
    #[doc(alias = "kAudioConverterQuality_Max")]
    pub const MAX: Self = Self(0x7F);

    /// high quality
    #[doc(alias = "kAudioConverterQuality_High")]
    pub const HIGH: Self = Self(0x60);

    /// medium quality
    #[doc(alias = "kAudioConverterQuality_Medium")]
    pub const MEDIUM: Self = Self(0x40);

    /// low quality
    #[doc(alias = "kAudioConverterQuality_Low")]
    pub const LOW: Self = Self(0x20);

    /// minimum quality
    #[doc(alias = "kAudioConverterQuality_Min")]
    pub const MIN: Self = Self(0);
}

impl Default for Quality {
    fn default() -> Self {
        Self::MEDIUM
    }
}

#[doc(alias = "AudioConverterSampleRateConverterComplexity")]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct SampleRateConverterComplexity(pub u32);

impl Default for SampleRateConverterComplexity {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl SampleRateConverterComplexity {
    /// Linear interpolation. lowest quality, cheapest.
    /// InitialPhase and PrimeMethod properties are not operative with this mode.
    #[doc(alias = "kAudioConverterSampleRateConverterComplexity_Linear")]
    pub const LINEAR: Self = Self(u32::from_be_bytes(*b"line"));

    // Normal quality sample rate conversion. (Default)
    #[doc(alias = "kAudioConverterSampleRateConverterComplexity_Normal")]
    pub const NORMAL: Self = Self(u32::from_be_bytes(*b"norm"));

    // Mastering quality sample rate conversion. More expensive.
    #[doc(alias = "kAudioConverterSampleRateConverterComplexity_Mastering")]
    pub const MASTERING: Self = Self(u32::from_be_bytes(*b"bats"));

    /// Minimum phase impulse response. Stopband attenuation varies with quality setting.
    /// The InitialPhase and PrimeMethod properties are not operative with this mode.
    /// There are three levels of quality provided.
    ///    Quality::Low (or Min)  : noise floor to -96 dB
    ///    Quality::Medium        : noise floor to -144 dB
    ///    Quality::High (or Max) : noise floor to -160 dB (this uses double precision internally)
    /// Quality equivalences to the other complexity modes are very roughly as follows:
    /// MinimumPhase Low    is somewhat better than Normal Medium.
    /// MinimumPhase Medium is similar to Normal Max.
    /// MinimumPhase High   is similar to Mastering Low.
    /// In general, MinimumPhase performs better than Normal and Mastering for the equivalent qualities listed above.
    /// MinimumPhase High is several times faster than Mastering Low.
    #[doc(alias = "kAudioConverterSampleRateConverterComplexity_MinimumPhase")]
    pub const MINIMUM_PHASE: Self = Self(u32::from_be_bytes(*b"minp"));
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum PrimeMethod {
    /// Primes with leading + trailing input frames.
    Pre = 0,

    /// Only primes with trailing (zero latency). Leading frames are assumed to be
    /// silence.
    Normal,

    /// Acts in "latency" mode. Both leading and trailing frames assumed to be
    /// silence.
    None,
}

impl Default for PrimeMethod {
    fn default() -> Self {
        Self::Normal
    }
}

/// When using AudioConverterFillComplexBuffer() (either a single call or a series of calls), some
/// conversions, particularly involving sample-rate conversion, ideally require a certain
/// number of input frames previous to the normal start input frame and beyond the end of
/// the last expected input frame in order to yield high-quality results.
///
/// These are expressed in the leadingFrames and trailing_frames members of the structure.
///
/// The very first call to AudioConverterFillComplexBuffer(), or first call after
/// AudioConverterReset(), will request additional input frames beyond those normally
/// expected in the input proc callback to fulfill this first AudioConverterFillComplexBuffer()
/// request. The number of additional frames requested, depending on the prime method, will
/// be approximately:
///
/// Prime method                  | Additional frames
/// ------------------------------|----------------------
///          PrimeMethod::Pre     | leading_frames + trailing_frames
///          PrimeMethod::Normal  | trailing_frames
///          PrimeMethod::None    | 0
///
/// Thus, in effect, the first input proc callback(s) may provide not only the leading
/// frames, but also may "read ahead" by an additional number of trailing frames depending
/// on the prime method.
///
/// PrimeMethod::None is useful in a real-time application processing live input,
/// in which case trailing_frames (relative to input sample rate) of through latency will be
/// seen at the beginning of the output of the AudioConverter.  In other real-time
/// applications such as DAW systems, it may be possible to provide these initial extra
/// audio frames since they are stored on disk or in memory somewhere and
/// PrimeMethod::Pre may be preferable.  The default method is
/// PrimeMethod::Normal, which requires no pre-seeking of the input stream and
/// generates no latency at the output.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct PrimeInfo {
    /// Specifies the number of leading (previous) input frames, relative to the normal/desired
    /// start input frame, required by the converter to perform a high quality conversion. If
    /// using kConverterPrimeMethod_Pre, the client should "pre-seek" the input stream provided
    /// through the input proc by leadingFrames. If no frames are available previous to the
    /// desired input start frame (because, for example, the desired start frame is at the very
    /// beginning of available audio), then provide "leadingFrames" worth of initial zero frames
    /// in the input proc.  Do not "pre-seek" in the default case of
    /// PrimeMethod::Normal or when using PrimeMethod::None.
    pub leading_frames: u32,

    /// Specifies the number of trailing input frames (past the normal/expected end input frame)
    /// required by the converter to perform a high quality conversion.  The client should be
    /// prepared to provide this number of additional input frames except when using
    /// PrimeMethod::None. If no more frames of input are available in the input stream
    /// (because, for example, the desired end frame is at the end of an audio file), then zero
    /// (silent) trailing frames will be synthesized for the client.
    pub trailing_frames: u32,
}

pub mod err {
    use crate::os::Error;

    /// 0x666D743F, 1718449215
    pub const FORMAT_NOT_SUPPORTED: Error = Error::from_be_bytes(*b"fmt?");

    /// 'op??', integer used because of trigraph
    pub const OPERATION_NOT_SUPPORTED: Error = Error::new_unchecked(0x6F703F3F);

    /// 0x70726F70, 1886547824
    pub const PROPERTY_NOT_SUPPORTED: Error = Error::from_be_bytes(*b"prop");
    pub const INVALID_INPUT_SIZE: Error = Error::from_be_bytes(*b"insz");
    pub const INVALID_OUTPUT_SIZE: Error = Error::from_be_bytes(*b"otsz");
    /// e.g. byte size is not a multiple of the frame size
    /// 0x77686174, 2003329396
    pub const UNSPECIFIED_ERROR: Error = Error::from_be_bytes(*b"what");

    /// 0x2173697A, 561211770
    pub const BAD_PROPERTY_SIZE_ERROR: Error = Error::from_be_bytes(*b"!siz");
    pub const REQUIRES_PACKET_DESCRIPTIONS_ERROR: Error = Error::from_be_bytes(*b"!pkd");
    pub const INPUT_SAMPLE_RATE_OUT_OF_RANGE: Error = Error::from_be_bytes(*b"!isr");
    pub const OUTPUT_SAMPLE_RATE_OUT_OF_RANGE: Error = Error::from_be_bytes(*b"!osr");

    // ios only

    /// Returned from AudioConverterFillComplexBuffer if the underlying hardware codec has
    /// become unavailable, probably due to an interruption. In this case, your application
    /// must stop calling AudioConverterFillComplexBuffer. If the converter can resume from an
    /// interruption (see kAudioConverterPropertyCanResumeFromInterruption), you must
    /// wait for an EndInterruption notification from AudioSession, and call AudioSessionSetActive(true)
    /// before resuming.
    pub const HARDWARE_IN_USE: Error = Error::from_be_bytes(*b"hwiu");

    /// Returned from AudioConverterNew if the new converter would use a hardware codec
    /// which the application does not have permission to use.
    pub const NO_HARDWARE_PERMISSION: Error = Error::from_be_bytes(*b"perm");
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Converter(c_void);

/// Useful link on formats <https://tech.ebu.ch/docs/techreview/trev_305-moser.pdf>
#[derive(Debug)]
#[repr(transparent)]
pub struct ConverterRef(NonNull<Converter>);

unsafe impl Send for ConverterRef {}

impl Deref for ConverterRef {
    type Target = Converter;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for ConverterRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

/// Callback function for supplying input data to AudioConverterFillComplexBuffer.
///
/// This callback function supplies input to AudioConverterFillComplexBuffer.
/// The AudioConverter requests a minimum number of packets (*ioNumberDataPackets).
/// The callback may return one or more packets. If this is less than the minimum,
/// the callback will simply be called again in the near future. Note that ioNumberDataPackets counts
/// packets in terms of the converter's input format (not its output format).
///
/// The callback may be asked to provide multiple input packets in a single call, even for compressed
/// formats. The callback must update the number of packets pointed to by ioNumberDataPackets
/// to indicate the number of packets actually being provided, and if the packets require packet
/// descriptions, these must be filled into the array pointed to by outDataPacketDescription, one
/// packet description per packet.
///
/// The callback is given an audio buffer list pointed to by ioData.  This buffer list may refer to
/// existing buffers owned and allocated by the audio converter, in which case the callback may
/// use them and copy input audio data into them.  However, the buffer list may also be empty
/// (data_byte_size == 0 and/or data == NULL), in which case the callback must provide its own
/// buffers.  The callback manipulates the members of io_data to point to one or more buffers
/// of audio data (multiple buffers are used with non-interleaved PCM data). The
/// callback is responsible for not freeing or altering this buffer until it is called again.
///
/// For input data that varies from one packet to another in either size (bytes per packet)
/// or duration (frames per packet), such as when decoding compressed audio, the callback
/// should expect outDataPacketDescription to be non-null and point to array of packet descriptions,
/// which the callback must fill in, one for every packet provided by the callback.  Each packet must
/// have a valid packet description, regardless of whether or not these descriptions are different
/// from each other.  Packet descriptions are required even if there is only one packet.
///
/// If the callback returns an error, it must return zero packets of data.
/// AudioConverterFillComplexBuffer will stop producing output and return whatever
/// output has already been produced to its caller, along with the error code. This
/// mechanism can be used when an input proc has temporarily run out of data, but
/// has not yet reached end of stream.
#[doc(alias = "AudioConverterComplexInputDataProc")]
pub type ComplexInputDataProc<const N: usize = 1, D = c_void> = extern "C-unwind" fn(
    converter: &Converter,
    io_number_data_packets: &mut u32,
    io_data: &mut audio::BufList<N>,
    out_data_packet_descriptions: *mut *mut audio::StreamPacketDesc,
    in_user_data: *mut D,
) -> os::Status;

#[doc(alias = "AudioConverterComplexInputDataProcRealtimeSafe")]
pub type ComplexInputDataProcRealtimeSafe<const N: usize = 1, D = c_void> =
    extern "C-unwind" fn(
        converter: &Converter,
        io_number_data_packets: &mut u32,
        io_data: &mut audio::BufList<N>,
        out_data_packet_descriptions: *mut *mut audio::StreamPacketDesc,
        in_user_data: *mut D,
    ) -> os::Status;

impl ConverterRef {
    /// # Safety
    /// use `with_formats`
    pub unsafe fn new(
        src_format: &audio::StreamBasicDesc,
        dst_format: &audio::StreamBasicDesc,
        out_audio_converter: *mut Option<Self>,
    ) -> os::Status {
        unsafe { AudioConverterNew(src_format, dst_format, out_audio_converter) }
    }

    pub fn with_formats(
        src_fmt: &audio::StreamBasicDesc,
        dst_fmt: &audio::StreamBasicDesc,
    ) -> os::Result<Self> {
        unsafe { os::result_unchecked(|val| Self::new(src_fmt, dst_fmt, val)) }
    }
}

impl ConverterRef {
    #[doc(alias = "AudioConverterNewWithOptions")]
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0, visionos = 2.0)]
    pub fn new_with_options(
        src_format: &audio::StreamBasicDesc,
        dst_format: &audio::StreamBasicDesc,
        options: Opts,
        out_audio_converter: *mut Option<Self>,
    ) -> os::Status {
        unsafe {
            AudioConverterNewWithOptions(src_format, dst_format, options, out_audio_converter)
        }
    }

    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0, visionos = 2.0)]
    pub fn with_options(
        src_fmt: &audio::StreamBasicDesc,
        dst_fmt: &audio::StreamBasicDesc,
        options: Opts,
    ) -> os::Result<Self> {
        unsafe {
            os::result_unchecked(|val| Self::new_with_options(src_fmt, dst_fmt, options, val))
        }
    }
}

impl Converter {
    #[inline]
    pub fn reset(&self) -> os::Result {
        unsafe { AudioConverterReset(self).result() }
    }

    #[inline]
    pub fn property_info(&self, prop_id: PropId) -> os::Result<PropInfo> {
        let mut size = 0;
        let mut writable = false;
        unsafe {
            AudioConverterGetPropertyInfo(self, prop_id, &mut size, &mut writable).result()?;
        }
        Ok(PropInfo { size, writable })
    }

    #[inline]
    pub unsafe fn get_property(
        &self,
        prop_id: PropId,
        io_prop_data_size: *mut u32,
        out_prop_data: *mut c_void,
    ) -> os::Result {
        unsafe {
            AudioConverterGetProperty(self, prop_id, io_prop_data_size, out_prop_data).result()
        }
    }

    #[inline]
    pub unsafe fn set_property(
        &mut self,
        prop_id: PropId,
        in_prop_data_size: u32,
        in_prop_data: *const c_void,
    ) -> os::Result {
        unsafe {
            AudioConverterSetProperty(self, prop_id, in_prop_data_size, in_prop_data).result()
        }
    }

    pub unsafe fn set_prop<T: Sized>(&mut self, prop_id: PropId, val: &T) -> os::Result {
        let size = size_of::<T>() as u32;
        unsafe { self.set_property(prop_id, size, val as *const _ as _) }
    }

    #[inline]
    pub unsafe fn prop_vec<T: Sized>(&self, prop_id: PropId) -> os::Result<Vec<T>> {
        let mut info = self.property_info(prop_id)?;
        let mut vec = Vec::with_capacity(info.size as usize / size_of::<T>());
        unsafe { self.get_property(prop_id, &mut info.size, vec.as_mut_ptr() as _) }?;
        unsafe { vec.set_len(info.size as usize / size_of::<T>()) };
        Ok(vec)
    }

    #[inline]
    pub unsafe fn set_prop_vec<T: Sized>(&mut self, prop_id: PropId, val: Vec<T>) -> os::Result {
        unsafe {
            self.set_property(
                prop_id,
                (val.len() * std::mem::size_of::<T>()) as u32,
                val.as_ptr() as _,
            )
        }?;
        Ok(())
    }

    #[inline]
    pub unsafe fn prop<T: Sized + Default>(&self, prop_id: PropId) -> os::Result<T> {
        let mut size = size_of::<T>() as u32;
        let mut value = Default::default();
        unsafe { self.get_property(prop_id, &mut size, &mut value as *mut _ as _) }?;
        Ok(value)
    }

    #[inline]
    pub fn max_output_packet_size(&self) -> os::Result<u32> {
        unsafe { self.prop(PropId::MAX_OUTPUT_PACKET_SIZE) }
    }

    #[inline]
    pub fn sample_rate_converter_quality(&self) -> os::Result<Quality> {
        unsafe { self.prop(PropId::SAMPLE_RATE_CONVERTER_QUALITY) }
    }

    #[inline]
    pub fn set_sample_rate_converter_quality(&mut self, val: Quality) -> os::Result {
        unsafe { self.set_prop(PropId::SAMPLE_RATE_CONVERTER_QUALITY, &val) }
    }

    #[inline]
    pub fn sample_rate_converter_complexity(&self) -> os::Result<SampleRateConverterComplexity> {
        unsafe { self.prop(PropId::SAMPLE_RATE_CONVERTER_COMPLEXITY) }
    }

    #[inline]
    pub fn set_sample_rate_converter_complexity(
        &mut self,
        val: SampleRateConverterComplexity,
    ) -> os::Result {
        unsafe { self.set_prop(PropId::SAMPLE_RATE_CONVERTER_COMPLEXITY, &val) }
    }

    #[inline]
    pub fn codec_quality(&self) -> os::Result<Quality> {
        unsafe { self.prop(PropId::CODEC_QUALITY) }
    }

    #[inline]
    pub fn set_codec_quality(&mut self, val: Quality) -> os::Result {
        unsafe { self.set_prop(PropId::CODEC_QUALITY, &val) }
    }

    #[inline]
    pub fn applicable_encode_bit_rates(&self) -> os::Result<Vec<audio::ValueRange>> {
        unsafe { self.prop_vec(PropId::APPLICABLE_ENCODE_BIT_RATES) }
    }

    #[inline]
    pub fn applicable_encode_sample_rates(&self) -> os::Result<Vec<audio::ValueRange>> {
        unsafe { self.prop_vec(PropId::APPLICABLE_ENCODE_SAMPLE_RATES) }
    }

    #[inline]
    pub fn compression_magic_cookie(&self) -> os::Result<Vec<u8>> {
        unsafe { self.prop_vec(PropId::COMPRESSION_MAGIC_COOKIE) }
    }

    #[inline]
    pub fn decompression_magic_cookie(&self) -> os::Result<Vec<u8>> {
        unsafe { self.prop_vec(PropId::DECOMPRESSION_MAGIC_COOKIE) }
    }

    #[inline]
    pub fn set_decompression_magic_cookie(&mut self, val: Vec<u8>) -> os::Result {
        unsafe { self.set_prop_vec(PropId::DECOMPRESSION_MAGIC_COOKIE, val) }
    }

    #[inline]
    pub fn current_output_stream_desc(&self) -> os::Result<audio::StreamBasicDesc> {
        unsafe { self.prop(PropId::CURRENT_OUTPUT_STREAM_DESC) }
    }

    #[inline]
    pub fn current_input_stream_desc(&self) -> os::Result<audio::StreamBasicDesc> {
        unsafe { self.prop(PropId::CURRENT_INPUT_STREAM_DESC) }
    }

    #[inline]
    pub fn encode_bit_rate(&self) -> os::Result<u32> {
        unsafe { self.prop(PropId::ENCODE_BIT_RATE) }
    }

    #[inline]
    pub fn set_encode_bit_rate(&mut self, val: u32) -> os::Result {
        unsafe { self.set_prop(PropId::ENCODE_BIT_RATE, &val) }
    }

    #[inline]
    pub fn prime_method(&self) -> os::Result<PrimeMethod> {
        unsafe { self.prop(PropId::PRIME_METHOD) }
    }

    #[inline]
    pub fn set_prime_method(&mut self, val: PrimeMethod) -> os::Result {
        unsafe { self.set_prop(PropId::PRIME_METHOD, &val) }
    }

    ///
    /// # Safety
    /// use fill_complex_buf
    ///
    #[inline]
    pub unsafe fn fill_complex_buffer<const NI: usize, const NO: usize, D>(
        &self,
        in_input_data_proc: ComplexInputDataProc<NI, D>,
        in_input_data_proc_user_data: *mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList<NO>,
        out_packet_description: *mut audio::StreamPacketDesc,
    ) -> os::Result {
        unsafe {
            AudioConverterFillComplexBuffer(
                self,
                std::mem::transmute(in_input_data_proc),
                in_input_data_proc_user_data as _,
                io_output_data_packet_size,
                std::mem::transmute(out_output_data),
                out_packet_description,
            )
            .result()
        }
    }

    #[inline]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, watchos = 26.0, visionos = 26.0)]
    pub unsafe fn fill_complex_buffer_realtime_safe<const NI: usize, const NO: usize, D>(
        &self,
        in_input_data_proc: ComplexInputDataProcRealtimeSafe<NI, D>,
        in_input_data_proc_user_data: *mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList<NO>,
        out_packet_description: *mut audio::StreamPacketDesc,
    ) -> os::Status {
        unsafe {
            AudioConverterFillComplexBufferRealtimeSafe(
                self,
                std::mem::transmute(in_input_data_proc),
                in_input_data_proc_user_data as _,
                io_output_data_packet_size,
                std::mem::transmute(out_output_data),
                out_packet_description,
            )
        }
    }

    /// Converts PCM data from an input buffer list to an output buffer list.
    /// This function will fail for any conversion where there is a
    /// variable relationship between the input and output data buffer sizes. This
    /// includes sample rate conversions and most compressed formats. In these cases,
    /// use AudioConverterFillComplexBuffer. Generally this function is only appropriate for
    /// PCM-to-PCM conversions where there is no sample rate conversion.
    ///
    /// # Safety
    ///
    /// Use [`.convert_complex_buf()`].
    #[inline]
    pub unsafe fn convert_complex_buffer<const N1: usize, const N2: usize>(
        &self,
        in_number_pcm_frames: u32,
        in_input_data: *const audio::BufList<N1>,
        out_output_data: *mut audio::BufList<N2>,
    ) -> os::Result {
        unsafe {
            AudioConverterConvertComplexBuffer(
                self,
                in_number_pcm_frames,
                in_input_data as _,
                out_output_data as _,
            )
            .result()
        }
    }

    /// Converts PCM data from an input buffer list to an output buffer list.
    ///
    /// This function will fail for any conversion where there is a
    /// variable relationship between the input and output data buffer sizes. This
    /// includes sample rate conversions and most compressed formats. In these cases,
    /// use AudioConverterFillComplexBuffer. Generally this function is only appropriate for
    /// PCM-to-PCM conversions where there is no sample rate conversion.
    #[doc(alias = "AudioConverterConvertComplexBuffer")]
    #[inline]
    pub fn convert_complex_buf<const N1: usize, const N2: usize>(
        &self,
        frames: u32,
        input: &audio::BufList<N1>,
        output: &mut audio::BufList<N2>,
    ) -> os::Result {
        unsafe { self.convert_complex_buffer(frames, input as *const _, output as *mut _) }
    }

    #[doc(alias = "AudioConverterConvertBuffer")]
    #[inline]
    pub fn convert_buf(&self, input: &[u8], output: &mut [u8]) -> os::Result<usize> {
        let mut n = output.len() as u32;
        unsafe {
            self.convert_buffer(
                input.len() as _,
                input.as_ptr() as _,
                &mut n,
                output.as_mut_ptr() as _,
            )
        }?;
        Ok(n as _)
    }

    /// # Safety
    ///
    /// use `self.convert_buf()`
    #[doc(alias = "AudioConverterConvertBuffer")]
    #[inline]
    pub unsafe fn convert_buffer(
        &self,
        in_input_data_size: u32,
        in_input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> os::Result {
        unsafe {
            AudioConverterConvertBuffer(
                self,
                in_input_data_size,
                in_input_data,
                io_output_data_size,
                out_output_data,
            )
            .result()
        }
    }

    #[inline]
    pub fn fill_complex_buf_desc<const NI: usize, const NO: usize, D>(
        &self,
        proc: ComplexInputDataProc<NI, D>,
        user_data: &mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList<NO>,
        out_packet_description: &mut Vec<audio::StreamPacketDesc>,
    ) -> os::Result {
        unsafe {
            self.fill_complex_buffer(
                proc,
                user_data,
                io_output_data_packet_size,
                out_output_data,
                out_packet_description.as_mut_ptr(),
            )
        }
    }

    #[inline]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, watchos = 26.0, visionos = 26.0)]
    pub fn fill_complex_buf_desc_realtime_safe<const NI: usize, const NO: usize, D>(
        &self,
        proc: ComplexInputDataProcRealtimeSafe<NI, D>,
        user_data: &mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList<NO>,
        out_packet_description: &mut Vec<audio::StreamPacketDesc>,
    ) -> os::Result {
        unsafe {
            self.fill_complex_buffer_realtime_safe(
                proc,
                user_data,
                io_output_data_packet_size,
                out_output_data,
                out_packet_description.as_mut_ptr(),
            )
            .result()
        }
    }

    #[inline]
    pub fn fill_complex_buf<const NI: usize, const NO: usize, D>(
        &self,
        proc: ComplexInputDataProc<NI, D>,
        user_data: *mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList<NO>,
    ) -> os::Result {
        unsafe {
            self.fill_complex_buffer(
                proc,
                user_data,
                io_output_data_packet_size,
                out_output_data,
                std::ptr::null_mut(),
            )
        }
    }

    #[inline]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, watchos = 26.0, visionos = 26.0)]
    pub fn fill_complex_buf_realtime_safe<const NI: usize, const NO: usize, D>(
        &self,
        proc: ComplexInputDataProcRealtimeSafe<NI, D>,
        user_data: *mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList<NO>,
    ) -> os::Result {
        unsafe {
            self.fill_complex_buffer_realtime_safe(
                proc,
                user_data,
                io_output_data_packet_size,
                out_output_data,
                std::ptr::null_mut(),
            )
            .result()
        }
    }
}

impl Drop for ConverterRef {
    fn drop(&mut self) {
        let res = unsafe { AudioConverterDispose(self) };
        debug_assert!(res.is_ok());
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PropInfo {
    pub size: u32,
    pub writable: bool,
}

define_opts!(
    #[doc(alias = "AudioConverterOptions")]
    pub Opts(u32)
);

impl Opts {
    pub const UNBUFFERED: Self = Self(1 << 16);
}

#[link(name = "AudioToolbox", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    fn AudioConverterNew(
        in_source_format: &audio::StreamBasicDesc,
        in_destination_format: &audio::StreamBasicDesc,
        out_audio_converer: *mut Option<ConverterRef>,
    ) -> os::Status;

    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0, visionos = 2.0)]
    fn AudioConverterNewWithOptions(
        in_source_format: &audio::StreamBasicDesc,
        in_destination_format: &audio::StreamBasicDesc,
        in_options: Opts,
        out_audio_converer: *mut Option<ConverterRef>,
    ) -> os::Status;

    fn AudioConverterReset(converter: &Converter) -> os::Status;
    fn AudioConverterGetPropertyInfo(
        converter: &Converter,
        prop_id: PropId,
        out_size: *mut u32,
        out_writable: *mut bool,
    ) -> os::Status;

    fn AudioConverterGetProperty(
        converter: &Converter,
        prop_id: PropId,
        io_prop_data_size: *mut u32,
        out_prop_data: *mut c_void,
    ) -> os::Status;

    fn AudioConverterSetProperty(
        converter: &Converter,
        prop_id: PropId,
        in_prop_data_size: u32,
        in_prop_data: *const c_void,
    ) -> os::Status;

    fn AudioConverterFillComplexBuffer(
        converter: &Converter,
        in_input_data_proc: ComplexInputDataProc,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList,
        out_packet_description: *mut audio::StreamPacketDesc,
    ) -> os::Status;

    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, watchos = 26.0, visionos = 26.0)]
    fn AudioConverterFillComplexBufferRealtimeSafe(
        converter: &Converter,
        in_input_data_proc: ComplexInputDataProcRealtimeSafe,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut audio::BufList,
        out_packet_description: *mut audio::StreamPacketDesc,
    ) -> os::Status;

    fn AudioConverterConvertComplexBuffer(
        converter: &Converter,
        in_number_pcm_frames: u32,
        in_input_data: *const audio::BufList,
        out_output_data: *mut audio::BufList,
    ) -> os::Status;

    fn AudioConverterConvertBuffer(
        converter: &Converter,
        in_input_data_size: u32,
        in_input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> os::Status;

    fn AudioConverterDispose(converter: &Converter) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::at;

    #[cfg(not(feature = "macos_15_0"))]
    use crate::api;

    #[cfg(feature = "macos_15_0")]
    #[test]
    fn basics() {
        let src_fmt = at::audio::StreamBasicDesc::common_f32(44_100.0f64, 2, true);
        let dst_fmt = at::audio::StreamBasicDesc::common_f32(44_100.0f64, 2, false);
        let opts = at::audio::ConverterOpts::UNBUFFERED;
        let converter = at::audio::ConverterRef::with_options(&src_fmt, &dst_fmt, opts).unwrap();
        let size = converter.max_output_packet_size().unwrap();
        assert_eq!(size, 4);
    }

    #[cfg(not(feature = "macos_15_0"))]
    #[test]
    fn basics() {
        let src_fmt = at::audio::StreamBasicDesc::common_f32(44_100.0f64, 2, true);
        let dst_fmt = at::audio::StreamBasicDesc::common_f32(44_100.0f64, 2, false);
        let converter = if api::version!(macos = 15.0, ios = 18.0, tvos = 18.0) {
            let opts = at::audio::ConverterOpts::UNBUFFERED;
            unsafe { at::audio::ConverterRef::with_options(&src_fmt, &dst_fmt, opts).unwrap() }
        } else {
            at::audio::ConverterRef::with_formats(&src_fmt, &dst_fmt).unwrap()
        };

        let size = converter.max_output_packet_size().unwrap();
        assert_eq!(size, 4);
    }
}
