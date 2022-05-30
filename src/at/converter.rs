use std::{ffi::c_void, intrinsics::transmute, mem::size_of};

use crate::{
    cat::{
        AudioBufferList, AudioStreamBasicDescription, AudioStreamPacketDescription, AudioValueRange,
    },
    cf, define_cf_type, os,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PropertyID(pub u32);

impl PropertyID {
    /// A u32 that indicates the size in bytes of the smallest buffer of input
    /// data that can be supplied via the AudioConverterInputProc or as the input to
    /// AudioConverterConvertBuffer
    pub const MINIMUM_INPUT_BUFFER_SIZE: Self = Self(u32::from_be_bytes(*b"mibs"));

    /// A UInt32 that indicates the size in bytes of the smallest buffer of output
    /// data that can be supplied to AudioConverterFillComplexBuffer or as the output to
    /// AudioConverterConvertBuffer
    pub const MINIMUM_OUTPUT_BUFFER_SIZE: Self = Self(u32::from_be_bytes(*b"mobs"));

    /// a u32 that indicates the size in bytes of the largest single packet of
    /// data in the input format. This is mostly useful for variable bit rate
    /// compressed data (decoders).
    pub const MAXIMUM_INPUT_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"xips"));

    /// a u32 that indicates the size in bytes of the largest single packet of
    /// data in the output format. This is mostly useful for variable bit rate
    /// compressed data (encoders).
    pub const MAXIMUM_OUTPUT_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"xops"));

    /// a u32 that on input holds a size in bytes that is desired for the output
    /// data. On output, it will hold the size in bytes of the input buffer required
    /// to generate that much output data. Note that some converters cannot do this
    /// calculation.
    pub const CALCULATE_INPUT_BUFFER_SIZE: Self = Self(u32::from_be_bytes(*b"cibs"));

    /// a u32 that on input holds a size in bytes that is desired for the input
    /// data. On output, it will hold the size in bytes of the output buffer
    /// required to hold the output data that will be generated. Note that some
    /// converters cannot do this calculation.
    pub const CALCULATE_OUTPUT_BUFFER_SIZE: Self = Self(u32::from_be_bytes(*b"cobs"));

    /// The value of this property varies from format to format and is considered
    /// private to the format. It is treated as a buffer of untyped data.
    pub const INPUT_CODEC_PARAMETERS: Self = Self(u32::from_be_bytes(*b"icdp"));

    /// The value of this property varies from format to format and is considered
    /// private to the format. It is treated as a buffer of untyped data.
    pub const OUTPUT_CODEC_PARAMETERS: Self = Self(u32::from_be_bytes(*b"ocdp"));

    /// An os::Type that specifies the sample rate converter algorithm to use (as defined in
    /// AudioToolbox/AudioUnitProperties.h)
    pub const SAMPLE_RATE_CONVERTER_COMPLEXITY: Self = Self(u32::from_be_bytes(*b"srca"));
    /// A u32 that specifies rendering quality of the sample rate converter (see
    /// enum constants below)
    pub const SAMPLE_RATE_CONVERTER_QUALITY: Self = Self(u32::from_be_bytes(*b"srcq"));
    /// A f64 with value 0.0 <= x < 1.0 giving the initial subsample position of the
    /// sample rate converter.
    pub const SAMPLE_RATE_CONVERTER_INITIAL_PHASE: Self = Self(u32::from_be_bytes(*b"srcp"));

    /// A u32 that specifies rendering quality of a codec (see enum constants
    /// below)
    pub const CODEC_QUALITY: Self = Self(u32::from_be_bytes(*b"cdqu"));

    /// a u32 specifying priming method (usually for sample-rate converter) see
    /// explanation for struct AudioConverterPrimeInfo below along with enum
    /// constants
    pub const PRIME_METHOD: Self = Self(u32::from_be_bytes(*b"prmm"));

    /// A pointer to AudioConverterPrimeInfo (see explanation for struct
    /// AudioConverterPrimeInfo below)
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
    pub const CHANNEL_MAP: Self = Self(u32::from_be_bytes(*b"chmp"));

    /// A *mut c_void pointing to memory set up by the caller. Required by some formats
    /// in order to decompress the input data.
    pub const DECOMPRESSION_MAGIC_COOKIE: Self = Self(u32::from_be_bytes(*b"dmgc"));

    /// A *mut c_void pointing to memory set up by the caller. Returned by the converter
    /// so that it may be stored along with the output data. It can then be passed
    /// back to the converter for decompression at a later time.
    pub const COMPRESSION_MAGIC_COOKIE: Self = Self(u32::from_be_bytes(*b"cmgc"));

    /// A u32 containing the number of bits per second to aim for when encoding
    /// data. Some decoders will also allow you to get this property to discover the bit rate.
    pub const ENCODE_BIT_RATE: Self = Self(u32::from_be_bytes(*b"brat"));

    /// For encoders where the AudioConverter was created with an output sample rate
    /// of zero, and the codec can do rate conversion on its input, this provides a
    /// way to set the output sample rate. The property value is a f64.
    pub const ENCODE_ADJUSTABLE_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"ajsr"));

    /// The property value is an AudioChannelLayout.
    pub const INPUT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"icl "));

    /// The property value is an AudioChannelLayout.
    pub const OUTPUT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"ocl "));

    /// The property value is an array of AudioValueRange describing applicable bit
    /// rates based on current settings.
    pub const APPLICABLE_ENCODE_BIT_RATES: Self = Self(u32::from_be_bytes(*b"aebr"));

    /// The property value is an array of AudioValueRange describing available bit
    /// rates based on the input format. You can get all available bit rates from
    /// the AudioFormat API.
    pub const AVAILABLE_ENCODE_BIT_RATES: Self = Self(u32::from_be_bytes(*b"vebr"));

    /// The property value is an array of AudioValueRange describing applicable
    /// sample rates based on current settings.
    pub const APPLICABLE_ENCODE_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"aesr"));

    /// The property value is an array of AudioValueRange describing available
    /// sample rates based on the input format. You can get all available sample
    /// rates from the AudioFormat API.
    pub const AVAILABLE_ENCODE_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"vesr"));

    /// The property value is an array of AudioChannelLayoutTags for the format and
    /// number of channels specified in the input format going to the encoder.
    pub const AVAILABLE_ENCODE_CHANNEL_LAYOUT_TAGS: Self = Self(u32::from_be_bytes(*b"aecl"));

    /// Returns the current completely specified output AudioStreamBasicDescription.
    /// For example when encoding to AAC, your original output stream description
    /// will not have been completely filled out.
    pub const CURRENT_OUTPUT_STREAM_DESCRIPTION: Self = Self(u32::from_be_bytes(*b"acod"));

    /// Returns the current completely specified input AudioStreamBasicDescription.
    pub const CURRENT_INPUT_STREAM_DESCRIPTION: Self = Self(u32::from_be_bytes(*b"acid"));

    /// Returns the a CFArray of property settings for converters.
    pub const SETTINGS: Self = Self(u32::from_be_bytes(*b"acps"));

    /// An i32 of the source bit depth to preserve. This is a hint to some
    /// encoders like lossless about how many bits to preserve in the input. The
    /// converter usually tries to preserve as many as possible, but a lossless
    /// encoder will do poorly if more bits are supplied than are desired in the
    /// output. The bit depth is expressed as a negative number if the source was floating point,
    /// e.g. -32 for float, -64 for double.
    pub const BIT_DEPTH_HINT: Self = Self(u32::from_be_bytes(*b"acbd"));

    /// An array of AudioFormatListItem structs describing all the data formats produced by the
    /// encoder end of the AudioConverter. If the ioPropertyDataSize parameter indicates that
    /// outPropertyData is sizeof(AudioFormatListItem), then only the best format is returned.
    /// This property may be used for example to discover all the data formats produced by the AAC_HE2
    /// (AAC High Efficiency vers. 2) encoder.
    pub const FORMAT_LIST: Self = Self(u32::from_be_bytes(*b"flst"));

    /// A u32. Set to a value from the enum of dithering algorithms below.
    /// Zero means no dithering and is the default. (macOS only.)
    pub const DITHERING: Self = Self(u32::from_be_bytes(*b"dith"));

    /// A u32. Dither is applied at this bit depth.  (macOS only.)
    pub const DITHER_BIT_DEPTH: Self = Self(u32::from_be_bytes(*b"dbit"));
}

#[repr(transparent)]
pub struct DitherAlgorithm(pub u32);

impl DitherAlgorithm {
    pub const TPDF: Self = Self(1);
    pub const NOISE_SHAPING: Self = Self(2);
}

#[repr(transparent)]
pub struct Quality(pub u32);

/// kAudioConverterSampleRateConverterQuality
impl Quality {
    /// maximum quality
    pub const MAX: Self = Self(0x7F);

    /// high quality
    pub const HIGH: Self = Self(0x60);

    /// medium quality
    pub const MEDIUM: Self = Self(0x40);

    /// low quality
    pub const LOW: Self = Self(0x20);

    /// minimum quality
    pub const MIN: Self = Self(0);
}

impl Default for Quality {
    fn default() -> Self {
        Self::MEDIUM
    }
}

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
    pub const LINEAR: Self = Self(u32::from_be_bytes(*b"line"));

    // Normal quality sample rate conversion. (Default)
    pub const NORMAL: Self = Self(u32::from_be_bytes(*b"norm"));

    // Mastering quality sample rate conversion. More expensive.
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
    pub const MINIMUM_PHASE: Self = Self(u32::from_be_bytes(*b"minp"));
}

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

pub mod errors {
    use crate::os::Status;

    pub const FORMAT_NOT_SUPPORTED: Status = Status(i32::from_be_bytes(*b"fmt?"));

    /// 'op??', integer used because of trigraph
    pub const OPERATION_NOT_SUPPORTED: Status = Status(0x6F703F3F);
    pub const PROPERTY_NOT_SUPPORTED: Status = Status(i32::from_be_bytes(*b"prop"));
    pub const INVALID_INPUT_SIZE: Status = Status(i32::from_be_bytes(*b"insz"));
    pub const INVALID_OUTPUT_SIZE: Status = Status(i32::from_be_bytes(*b"otsz"));
    // e.g. byte size is not a multiple of the frame size
    pub const UNSPECIFIED_ERROR: Status = Status(i32::from_be_bytes(*b"what"));
    pub const BAD_PROPERTY_SIZE_ERROR: Status = Status(i32::from_be_bytes(*b"!siz"));
    pub const REQUIRES_PACKET_DESCRIPTIONS_ERROR: Status = Status(i32::from_be_bytes(*b"!pkd"));
    pub const INPUT_SAMPLE_RATE_OUT_OF_RANGE: Status = Status(i32::from_be_bytes(*b"!isr"));
    pub const OUTPUT_SAMPLE_RATE_OUT_OF_RANGE: Status = Status(i32::from_be_bytes(*b"!osr"));

    // ios only

    /// Returned from AudioConverterFillComplexBuffer if the underlying hardware codec has
    /// become unavailable, probably due to an interruption. In this case, your application
    /// must stop calling AudioConverterFillComplexBuffer. If the converter can resume from an
    /// interruption (see kAudioConverterPropertyCanResumeFromInterruption), you must
    /// wait for an EndInterruption notification from AudioSession, and call AudioSessionSetActive(true)
    /// before resuming.
    pub const HARDWARE_IN_USE: Status = Status(i32::from_be_bytes(*b"hwiu"));

    /// Returned from AudioConverterNew if the new converter would use a hardware codec
    /// which the application does not have permission to use.
    pub const NO_HARDWARE_PERMISSION: Status = Status(i32::from_be_bytes(*b"perm"));
}

define_cf_type!(Converter(cf::Type));

pub type ComplexInputDataProc<const L: usize, const N: usize, D> = extern "C" fn(
    converter: &Converter,
    io_number_data_packets: &mut u32,
    io_data: &mut AudioBufferList<L, N>,
    out_data_packet_description: *mut *mut AudioStreamBasicDescription,
    in_user_data: *mut D,
) -> os::Status;

impl Converter {
    pub unsafe fn new<'a>(
        in_source_format: &AudioStreamBasicDescription,
        in_destination_format: &AudioStreamBasicDescription,
        out_audio_converer: &mut Option<cf::Retained<'a, Converter>>,
    ) -> os::Status {
        AudioConverterNew(in_source_format, in_destination_format, out_audio_converer)
    }

    pub fn with_formats<'a>(
        source_fmt: &AudioStreamBasicDescription,
        dest_fmt: &AudioStreamBasicDescription,
    ) -> Result<cf::Retained<'a, Converter>, os::Status> {
        unsafe {
            let mut out_converter = None;
            Self::new(source_fmt, dest_fmt, &mut out_converter).to_result(out_converter)
        }
    }

    #[inline]
    pub fn reset(&self) -> Result<(), os::Status> {
        unsafe { AudioConverterReset(self).result() }
    }

    #[inline]
    pub fn property_info(&self, property_id: PropertyID) -> Result<PropertyInfo, os::Status> {
        unsafe {
            let mut size = 0;
            let mut writable = false;
            let r = AudioConverterGetPropertyInfo(self, property_id, &mut size, &mut writable);

            if r.is_ok() {
                Ok(PropertyInfo { size, writable })
            } else {
                Err(r)
            }
        }
    }

    #[inline]
    pub unsafe fn get_property(
        &self,
        property_id: PropertyID,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> os::Status {
        AudioConverterGetProperty(self, property_id, io_property_data_size, out_property_data)
    }

    #[inline]
    pub unsafe fn set_property(
        &self,
        property_id: PropertyID,
        in_property_data_size: u32,
        in_property_data: *const c_void,
    ) -> os::Status {
        AudioConverterSetProperty(self, property_id, in_property_data_size, in_property_data)
    }

    pub unsafe fn set_prop<T: Sized>(
        &self,
        property_id: PropertyID,
        value: &T,
    ) -> Result<(), os::Status> {
        let size = size_of::<T>() as u32;
        self.set_property(property_id, size, value as *const _ as _)
            .result()
    }

    #[inline]
    pub unsafe fn prop_vec<T: Sized>(&self, property_id: PropertyID) -> Result<Vec<T>, os::Status> {
        let mut info = self.property_info(property_id)?;
        let len = info.size as usize / size_of::<T>();
        let mut vec = Vec::with_capacity(len);
        vec.set_len(len);
        self.get_property(property_id, &mut info.size, vec.as_mut_ptr() as _)
            .result()?;
        Ok(vec)
    }

    #[inline]
    pub unsafe fn prop<T: Sized + Default>(
        &self,
        property_id: PropertyID,
    ) -> Result<T, os::Status> {
        let mut size = size_of::<T>() as u32;
        let mut value = Default::default();
        let res = self.get_property(property_id, &mut size, &mut value as *mut _ as _);
        if res.is_ok() {
            Ok(value)
        } else {
            Err(res)
        }
    }

    #[inline]
    pub fn maximum_output_packet_size(&self) -> Result<u32, os::Status> {
        unsafe { self.prop(PropertyID::MAXIMUM_OUTPUT_PACKET_SIZE) }
    }

    #[inline]
    pub fn sample_rate_converter_quality(&self) -> Result<Quality, os::Status> {
        unsafe { self.prop(PropertyID::SAMPLE_RATE_CONVERTER_QUALITY) }
    }

    #[inline]
    pub fn sample_rate_converter_complexity(
        &self,
    ) -> Result<SampleRateConverterComplexity, os::Status> {
        unsafe { self.prop(PropertyID::SAMPLE_RATE_CONVERTER_COMPLEXITY) }
    }

    #[inline]
    pub fn codec_quality(&self) -> Result<Quality, os::Status> {
        unsafe { self.prop(PropertyID::CODEC_QUALITY) }
    }

    #[inline]
    pub fn applicable_encode_bit_rates(&self) -> Result<Vec<AudioValueRange>, os::Status> {
        unsafe { self.prop(PropertyID::APPLICABLE_ENCODE_BIT_RATES) }
    }

    #[inline]
    pub fn applicable_encode_sample_rates(&self) -> Result<Vec<AudioValueRange>, os::Status> {
        unsafe { self.prop(PropertyID::APPLICABLE_ENCODE_SAMPLE_RATES) }
    }

    #[inline]
    pub fn current_output_stream_description(
        &self,
    ) -> Result<AudioStreamBasicDescription, os::Status> {
        unsafe { self.prop(PropertyID::CURRENT_OUTPUT_STREAM_DESCRIPTION) }
    }

    #[inline]
    pub fn current_input_stream_description(
        &self,
    ) -> Result<AudioStreamBasicDescription, os::Status> {
        unsafe { self.prop(PropertyID::CURRENT_INPUT_STREAM_DESCRIPTION) }
    }

    #[inline]
    pub fn encode_bit_rate(&self) -> Result<u32, os::Status> {
        unsafe { self.prop(PropertyID::ENCODE_BIT_RATE) }
    }

    #[inline]
    pub fn set_encode_bit_rate(&self, value: u32) -> Result<(), os::Status> {
        unsafe { self.set_prop(PropertyID::ENCODE_BIT_RATE, &value) }
    }

    #[inline]
    pub unsafe fn fill_complex_buffer(
        &self,
        in_input_data_proc: ComplexInputDataProc<1, 1, c_void>,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut AudioBufferList<1, 1>,
        out_packet_description: *mut AudioStreamPacketDescription,
    ) -> os::Status {
        AudioConverterFillComplexBuffer(
            self,
            in_input_data_proc,
            in_input_data_proc_user_data,
            io_output_data_packet_size,
            out_output_data,
            out_packet_description,
        )
    }

    /// Converts PCM data from an input buffer list to an output buffer list.
    /// This function will fail for any conversion where there is a
    /// variable relationship between the input and output data buffer sizes. This
    /// includes sample rate conversions and most compressed formats. In these cases,
    /// use AudioConverterFillComplexBuffer. Generally this function is only appropriate for
    /// PCM-to-PCM conversions where there is no sample rate conversion.
    /// #[inline]
    pub unsafe fn convert_complex_buffer(
        &self,
        in_number_pcm_frames: u32,
        in_input_data: *const AudioBufferList<1, 1>,
        out_output_data: *mut AudioBufferList<1, 1>,
    ) -> os::Status {
        AudioConverterConvertComplexBuffer(
            self,
            in_number_pcm_frames,
            in_input_data,
            out_output_data,
        )
    }

    #[inline]
    pub fn convert_complex_buf<
        const IL: usize,
        const IN: usize,
        const OL: usize,
        const ON: usize,
    >(
        &self,
        frames: u32,
        input: &AudioBufferList<IL, IN>,
        output: &mut AudioBufferList<OL, ON>,
    ) -> Result<(), os::Status> {
        unsafe { self.convert_complex_buffer(frames, transmute(input), transmute(output)) }.result()
    }

    #[inline]
    pub unsafe fn convert_buffer(
        &self,
        in_input_data_size: u32,
        in_input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> os::Status {
        AudioConverterConvertBuffer(
            self,
            in_input_data_size,
            in_input_data,
            io_output_data_size,
            out_output_data,
        )
    }

    pub fn fill_complex_buf<D>(
        &self,
        proc: ComplexInputDataProc<1, 1, D>,
        user_data: &mut D,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut AudioBufferList<1, 1>,
    ) -> Result<AudioStreamPacketDescription, os::Status> {
        let mut aspd = AudioStreamPacketDescription::default();
        unsafe {
            let res = self.fill_complex_buffer(
                transmute(proc),
                user_data as *mut _ as _,
                io_output_data_packet_size,
                out_output_data,
                &mut aspd,
            );
            if res.is_ok() {
                Ok(aspd)
            } else {
                Err(res)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PropertyInfo {
    pub size: u32,
    pub writable: bool,
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioConverterNew<'a>(
        in_source_format: &AudioStreamBasicDescription,
        in_destination_format: &AudioStreamBasicDescription,
        out_audio_converer: &mut Option<cf::Retained<'a, Converter>>,
    ) -> os::Status;

    fn AudioConverterReset(converter: &Converter) -> os::Status;
    fn AudioConverterGetPropertyInfo(
        converter: &Converter,
        property_id: PropertyID,
        out_size: *mut u32,
        out_writable: *mut bool,
    ) -> os::Status;

    fn AudioConverterGetProperty(
        converter: &Converter,
        property_id: PropertyID,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> os::Status;

    fn AudioConverterSetProperty(
        converter: &Converter,
        property_id: PropertyID,
        in_property_data_size: u32,
        in_property_data: *const c_void,
    ) -> os::Status;

    fn AudioConverterFillComplexBuffer(
        converter: &Converter,
        in_input_data_proc: ComplexInputDataProc<1, 1, c_void>,
        in_input_data_proc_user_data: *mut c_void,
        io_output_data_packet_size: &mut u32,
        out_output_data: &mut AudioBufferList<1, 1>,
        out_packet_description: *mut AudioStreamPacketDescription,
    ) -> os::Status;

    fn AudioConverterConvertComplexBuffer(
        converter: &Converter,
        in_number_pcm_frames: u32,
        in_input_data: *const AudioBufferList<1, 1>,
        out_output_data: *mut AudioBufferList<1, 1>,
    ) -> os::Status;

    fn AudioConverterConvertBuffer(
        converter: &Converter,
        in_input_data_size: u32,
        in_input_data: *const c_void,
        io_output_data_size: *mut u32,
        out_output_data: *mut c_void,
    ) -> os::Status;

}
