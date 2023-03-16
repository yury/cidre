use std::{
    ffi::c_long,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

use crate::{at, define_options, ns, os};

/// These are the error codes returned from the APIs found through Core Audio related frameworks.
pub mod errors {
    use crate::os::Status;

    pub const UNIMPLEMENTED_ERROR: Status = Status(-4);
    pub const FILE_NOT_FOUND_ERROR: Status = Status(-43);
    pub const FILE_PERMISSION_ERROR: Status = Status(-54);
    pub const TOO_MANY_FILES_OPEN_ERROR: Status = Status(-42);
    pub const BAD_FILE_PATH_ERROR: Status = Status(i32::from_be_bytes(*b"!pth"));
    pub const PARAM_ERROR: Status = Status(-50);
    pub const MEM_FULL_ERROR: Status = Status(-108);
}

/// This structure holds a pair of numbers that represent a continuous range of values.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct ValueRange {
    pub minimum: f64,
    pub maximum: f64,
}

impl Default for ValueRange {
    fn default() -> Self {
        Self {
            minimum: 0.0f64,
            maximum: 0.0f64,
        }
    }
}

/// A structure to hold a buffer of audio data.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    /// The number of interleaved channels in the buffer.
    pub number_channels: u32,
    /// The number of bytes in the buffer pointed at by mData.
    pub data_bytes_size: u32,
    /// A pointer to the buffer of audio data.
    pub data: *mut u8,
}

/// A variable length array of AudioBuffer structures.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct BufferList<const N: usize = 1> {
    pub number_buffers: u32,
    /// this is a variable length array of `number_buffers` elements
    pub buffers: [Buffer; N],
}

impl<const N: usize> Default for BufferList<N> {
    fn default() -> Self {
        Self {
            number_buffers: N as _,
            buffers: [Buffer {
                number_channels: 1,
                data_bytes_size: 0,
                data: std::ptr::null_mut(),
            }; N],
        }
    }
}

pub struct BufferListCursor<'a, const N: usize> {
    original_buffers: [Option<Buffer>; N],
    list: &'a mut BufferList,
}

impl<'a, const N: usize> BufferListCursor<'a, N> {
    pub fn new(list: &'a mut BufferList) -> Self {
        let mut original_buffers = [None; N];
        let slice = list.as_slice();
        for i in 0..N {
            original_buffers[i] = Some(slice[i]);
        }

        Self {
            original_buffers,
            list,
        }
    }

    pub fn offset(
        &mut self,
        frame_offset: usize,
        frame_count: usize,
        asbd: &at::audio::StreamBasicDescription,
    ) -> &mut BufferList {
        for (i, buf) in self.list.as_mut_slice().iter_mut().enumerate().take(N) {
            buf.data_bytes_size = (asbd.bytes_per_packet as usize * frame_count) as _;
            buf.data = unsafe {
                self.original_buffers[i]
                    .unwrap_unchecked()
                    .data
                    .offset(asbd.bytes_per_packet as isize * frame_offset as isize)
            };
        }
        self.list
    }
}

impl<const N: usize> Drop for BufferListCursor<'_, N> {
    fn drop(&mut self) {
        for (i, buf) in self.list.as_mut_slice().iter_mut().enumerate().take(N) {
            *buf = unsafe { self.original_buffers[i].unwrap_unchecked() };
        }
    }
}

impl BufferList {
    pub fn as_slice(&self) -> &[Buffer] {
        unsafe { &*slice_from_raw_parts(self.buffers.as_ptr(), self.number_buffers as _) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [Buffer] {
        unsafe {
            &mut *slice_from_raw_parts_mut(self.buffers.as_mut_ptr(), self.number_buffers as _)
        }
    }

    pub fn cursor<const N: usize>(&mut self) -> BufferListCursor<N> {
        BufferListCursor::new(self)
    }
}

/// A four char code indicating the general kind of data in the stream.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
#[doc(alias = "AudioFromatID")]
pub struct FormatID(pub u32);

/// The AudioFormatIDs used to identify individual formats of audio data.
impl FormatID {
    /// Linear PCM, uses the standard flags.
    #[doc(alias = "kAudioFormatLinearPCM")]
    pub const LINEAR_PCM: Self = Self(u32::from_be_bytes(*b"lpcm"));

    /// AC-3, has no flags.
    #[doc(alias = "kAudioFormatAC3")]
    pub const AC3: Self = Self(u32::from_be_bytes(*b"ac-3"));

    /// AC-3 packaged for transport over an IEC 60958 compliant digital audio
    /// interface. Uses the standard flags.
    #[doc(alias = "kAudioFormat60958AC3")]
    pub const _60958AC3: Self = Self(u32::from_be_bytes(*b"cac3"));

    /// Apples implementation of IMA 4:1 ADPCM, has no flags.
    #[doc(alias = "kAudioFormatAppleIMA4")]
    pub const APPLE_IMA4: Self = Self(u32::from_be_bytes(*b"ima4"));

    /// MPEG-4 Low Complexity AAC audio object, has no flags.
    #[doc(alias = "kAudioFormatMPEG4AAC")]
    pub const MPEG4_AAC: Self = Self(u32::from_be_bytes(*b"aac "));

    /// MPEG-4 CELP audio object, has no flags.
    #[doc(alias = "kAudioFormatMPEG4CELP")]
    pub const MPEG4_CELP: Self = Self(u32::from_be_bytes(*b"celp"));

    /// MPEG-4 HVXC audio object, has no flags.
    #[doc(alias = "kAudioFormatMPEG4HVXC")]
    pub const MPEG4_HVXC: Self = Self(u32::from_be_bytes(*b"hvxc"));

    /// MPEG-4 TwinVQ audio object type, has no flags.
    #[doc(alias = "kAudioFormatMPEG4TwinVQ")]
    pub const MPEG4_TWIN_VQ: Self = Self(u32::from_be_bytes(*b"twvq"));

    /// MACE 3:1, has no flags.
    #[doc(alias = "kAudioFormatMACE3")]
    pub const MACE3: Self = Self(u32::from_be_bytes(*b"MAC3"));

    /// MACE 6:1, has no flags.
    #[doc(alias = "kAudioFormatMACE6")]
    pub const MACE6: Self = Self(u32::from_be_bytes(*b"MAC6"));

    /// µLaw 2:1, has no flags.
    #[doc(alias = "kAudioFormatULaw")]
    pub const U_LAW: Self = Self(u32::from_be_bytes(*b"ulaw"));

    /// aLaw 2:1, has no flags.
    #[doc(alias = "kAudioFormatALaw")]
    pub const A_LAW: Self = Self(u32::from_be_bytes(*b"alaw"));

    /// QDesign music, has no flags
    #[doc(alias = "kAudioFormatQDesign")]
    pub const Q_DESIGN: Self = Self(u32::from_be_bytes(*b"QDMC"));

    /// QDesign2 music, has no flags
    #[doc(alias = "kAudioFormatQDesign2")]
    pub const Q_DESIGN2: Self = Self(u32::from_be_bytes(*b"QDM2"));

    /// QUALCOMM PureVoice, has no flags
    #[doc(alias = "kAudioFormatQUALCOMM")]
    pub const QUALCOMM: Self = Self(u32::from_be_bytes(*b"Qclp"));

    /// MPEG-1/2, Layer 1 audio, has no flags
    #[doc(alias = "kAudioFormatMPEGLayer1")]
    pub const MPEGLAYER1: Self = Self(u32::from_be_bytes(*b".mp1"));

    /// MPEG-1/2, Layer 2 audio, has no flags
    #[doc(alias = "kAudioFormatMPEGLayer2")]
    pub const MPEGLAYER2: Self = Self(u32::from_be_bytes(*b".mp2"));

    /// MPEG-1/2, Layer 3 audio, has no flags
    #[doc(alias = "kAudioFormatMPEGLayer3")]
    pub const MPEGLAYER3: Self = Self(u32::from_be_bytes(*b".mp3"));

    /// A stream of IOAudioTimeStamps, uses the IOAudioTimeStamp flags (see
    /// IOKit/audio/IOAudioTypes.h).
    #[doc(alias = "kAudioFormatTimeCode")]
    pub const TIME_CODE: Self = Self(u32::from_be_bytes(*b"time"));

    /// A stream of MIDIPacketLists where the time stamps in the MIDIPacketList are
    /// sample offsets in the stream. The mSampleRate field is used to describe how
    /// time is passed in this kind of stream and an AudioUnit that receives or
    /// generates this stream can use this sample rate, the number of frames it is
    /// rendering and the sample offsets within the MIDIPacketList to define the
    /// time for any MIDI event within this list. It has no flags.
    #[doc(alias = "kAudioFormatMIDIStream")]
    pub const MIDI_STREAM: Self = Self(u32::from_be_bytes(*b"midi"));

    /// A "side-chain" of Float32 data that can be fed or generated by an AudioUnit
    /// and is used to send a high density of parameter value control information.
    /// An AU will typically run a ParameterValueStream at either the sample rate of
    /// the AudioUnit's audio data, or some integer divisor of this (say a half or a
    /// third of the sample rate of the audio). The Sample Rate of the ASBD
    /// describes this relationship. It has no flags.
    #[doc(alias = "kAudioFormatParameterValueStream")]
    pub const PARAMETER_VALUE_STREAM: Self = Self(u32::from_be_bytes(*b"apvs"));

    /// Apple Lossless, the flags indicate the bit depth of the source material.
    #[doc(alias = "kAudioFormatAppleLossless")]
    pub const APPLE_LOSSLESS: Self = Self(u32::from_be_bytes(*b"alac"));

    /// MPEG-4 High Efficiency AAC audio object, has no flags.
    #[doc(alias = "kAudioFormatMPEG4AAC_HE")]
    pub const MPEG4_AAC_HE: Self = Self(u32::from_be_bytes(*b"aach"));

    /// MPEG-4 AAC Low Delay audio object, has no flags.
    #[doc(alias = "kAudioFormatMPEG4AAC_LD")]
    pub const MPEG4_AAC_LD: Self = Self(u32::from_be_bytes(*b"aacl"));

    /// MPEG-4 AAC Enhanced Low Delay audio object, has no flags. This is the formatID of
    /// the base layer without the SBR extension. See also kAudioFormatMPEG4AAC_ELD_SBR
    #[doc(alias = "kAudioFormatMPEG4AAC_ELD")]
    pub const MPEG4_AAC_ELD: Self = Self(u32::from_be_bytes(*b"aace"));

    /// MPEG-4 AAC Enhanced Low Delay audio object with SBR extension layer, has no flags.
    #[doc(alias = "kAudioFormatMPEG4AAC_ELD_SBR")]
    pub const MPEG4_AAC_ELD_SBR: Self = Self(u32::from_be_bytes(*b"aacf"));

    #[doc(alias = "kAudioFormatMPEG4AAC_ELD_V2")]
    pub const MPEG4_AAC_ELD_V2: Self = Self(u32::from_be_bytes(*b"aacg"));

    /// MPEG-4 High Efficiency AAC Version 2 audio object, has no flags.    
    /// note: https://lists.apple.com/archives/coreaudio-api/2011/Mar/msg00176.html
    /// frames_per_packet should be 2048
    #[doc(alias = "kAudioFormatMPEG4AAC_HE_V2")]
    pub const MPEG4_AAC_HE_V2: Self = Self(u32::from_be_bytes(*b"aacp"));

    /// MPEG-4 Spatial Audio audio object, has no flags.
    #[doc(alias = "kAudioFormatMPEG4AAC_Spatial")]
    pub const MPEG4_AAC_SPATIAL: Self = Self(u32::from_be_bytes(*b"aacs"));

    /// MPEG-D Unified Speech and Audio Coding, has no flags.
    #[doc(alias = "kAudioFormatMPEGD_USAC")]
    pub const MPEG_D_USAC: Self = Self(u32::from_be_bytes(*b"usac"));

    /// The AMR Narrow Band speech codec.
    #[doc(alias = "kAudioFormatAMR")]
    pub const AMR: Self = Self(u32::from_be_bytes(*b"samr"));

    /// The AMR Wide Band speech codec.
    #[doc(alias = "kAudioFormatAMR_WB")]
    pub const AMR_WB: Self = Self(u32::from_be_bytes(*b"sawb"));

    /// The format used for Audible audio books. It has no flags.
    #[doc(alias = "kAudioFormatAudible")]
    pub const AUDIBLE: Self = Self(u32::from_be_bytes(*b"AUDB"));

    /// The iLBC narrow band speech codec. It has no flags.
    #[doc(alias = "kAudioFormatiLBC")]
    pub const LBC: Self = Self(u32::from_be_bytes(*b"ilbc"));

    /// DVI/Intel IMA ADPCM - ACM code 17.
    #[doc(alias = "kAudioFormatDVIIntelIMA")]
    pub const DVI_INTEL_IMA: Self = Self(0x6D730011);

    /// Microsoft GSM 6.10 - ACM code 49.
    #[doc(alias = "kAudioFormatMicrosoftGSM")]
    pub const MICROSOFT_GSM: Self = Self(0x6D730031);

    /// This format is defined by AES3-2003, and adopted into MXF and MPEG-2
    /// containers and SDTI transport streams with SMPTE specs 302M-2002 and
    /// 331M-2000. It has no flags.
    #[doc(alias = "kAudioFormatAES3")]
    pub const AES3: Self = Self(u32::from_be_bytes(*b"aes3"));

    /// Enhanced AC-3, has no flags.
    #[doc(alias = "kAudioFormatEnhancedAC3")]
    pub const ENHANCED_AC3: Self = Self(u32::from_be_bytes(*b"ec-3"));

    /// Free Lossless Audio Codec, the flags indicate the bit depth of the source material.
    #[doc(alias = "kAudioFormatFLAC")]
    pub const FLAC: Self = Self(u32::from_be_bytes(*b"flac"));

    /// Opus codec, has no flags.
    #[doc(alias = "kAudioFormatOpus")]
    pub const OPUS: Self = Self(u32::from_be_bytes(*b"opus"));

    pub fn to_ns_number(self) -> &'static ns::Number {
        ns::Number::tagged_i32(self.0 as _)
    }
}

// Flags that are specific to each format.
// #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
// #[repr(transparent)]
// pub struct FormatFlags(pub u32);

define_options!(FormatFlags(u32));

/// ios app audio - IS_BIG_ENDIAN | IS_SIGNED_INTEGER | IS_PACKED
/// mic - IS_SIGNED_INTEGER | IS_PACKED

/// These are the standard AudioFormatFlags for use in the mFormatFlags field of the
/// AudioStreamBasicDescription structure.
/// Typically, when an ASBD is being used, the fields describe the complete layout
/// of the sample data in the buffers that are represented by this description -
/// where typically those buffers are represented by an AudioBuffer that is
/// contained in an AudioBufferList.
///
/// However, when an ASBD has the kAudioFormatFlagIsNonInterleaved flag, the
/// AudioBufferList has a different structure and semantic. In this case, the ASBD
/// fields will describe the format of ONE of the AudioBuffers that are contained in
/// the list, AND each AudioBuffer in the list is determined to have a single (mono)
/// channel of audio data. Then, the ASBD's mChannelsPerFrame will indicate the
/// total number of AudioBuffers that are contained within the AudioBufferList -
/// where each buffer contains one channel. This is used primarily with the
/// AudioUnit (and AudioConverter) representation of this list - and won't be found
/// in the AudioHardware usage of this structure.
impl FormatFlags {
    /// Set for floating point, clear for integer.
    pub const IS_FLOAT: Self = Self(1u32 << 0);

    /// Set for big endian, clear for little endian.
    pub const IS_BIG_ENDIAN: Self = Self(1u32 << 1);

    /// Set for signed integer, clear for unsigned integer. This is only valid if
    /// AudioFormatFlags::IS_FLOAT is clear.
    pub const IS_SIGNED_INTEGER: Self = Self(1u32 << 2);

    /// Set if the sample bits occupy the entire available bits for the channel,
    /// clear if they are high or low aligned within the channel. Note that even if
    /// this flag is clear, it is implied that this flag is set if the
    /// AudioStreamBasicDescription is filled out such that the fields have the
    /// following relationship:
    ///     ((bits_per_sample / 8) * channels_per_frame) == bytes_per_frame
    pub const IS_PACKED: Self = Self(1u32 << 3);

    /// Set if the sample bits are placed into the high bits of the channel, clear
    /// for low bit placement. This is only valid if kAudioFormatFlagIsPacked is
    /// clear.
    pub const IS_ALIGNED_HIGH: Self = Self(1u32 << 4);

    /// Set if the samples for each channel are located contiguously and the
    /// channels are layed out end to end, clear if the samples for each frame are
    /// layed out contiguously and the frames layed out end to end.
    pub const IS_NON_INTERLEAVED: Self = Self(1u32 << 5);

    /// Set to indicate when a format is non-mixable. Note that this flag is only
    /// used when interacting with the HAL's stream format information. It is not a
    /// valid flag for any other uses.
    pub const IS_NON_MIXABLE: Self = Self(1u32 << 6);

    /// Set if all the flags would be clear in order to preserve 0 as the wild card
    /// value.
    pub const ALL_CLEAR: Self = Self(0x80000000);

    pub const LINEAR_PCM_IS_FLOAT: Self = Self::IS_FLOAT;
    pub const LINEAR_PCM_IS_BIG_ENDIAN: Self = Self::IS_BIG_ENDIAN;
    pub const LINEAR_PCM_IS_SIGNED_INTEGER: Self = Self::IS_SIGNED_INTEGER;
    pub const LINEAR_PCM_IS_PACKED: Self = Self::IS_PACKED;
    pub const LINEAR_PCM_IS_ALIGNED_HIGH: Self = Self::IS_ALIGNED_HIGH;
    pub const LINEAR_PCM_IS_NON_INTERLEAVED: Self = Self::IS_NON_INTERLEAVED;
    pub const LINEAR_PCM_IS_NON_MIXABLE: Self = Self::IS_NON_MIXABLE;

    pub const NATIVE_ENDIAN: Self = Self(0);

    pub const NATIVE_FLOAT_PACKED: Self =
        Self(Self::IS_FLOAT.0 | Self::NATIVE_ENDIAN.0 | Self::IS_PACKED.0);

    /// The linear PCM flags contain a 6-bit bitfield indicating that an integer
    /// format is to be interpreted as fixed point. The value indicates the number
    /// of bits are used to represent the fractional portion of each sample value.
    /// This constant indicates the bit position (counting from the right) of the
    /// bitfield in mFormatFlags.
    pub const LINEAR_PCM_SAMPLE_FRACTION_SHIFT: Self = Self(7);

    /// number_fractional_bits = (mFormatFlags &
    /// kLinearPCMFormatFlagsSampleFractionMask) >>
    /// kLinearPCMFormatFlagsSampleFractionShift
    pub const LINEAR_PCM_SAMPLE_FRACTION_MASK: Self =
        Self(0x3f << Self::LINEAR_PCM_SAMPLE_FRACTION_SHIFT.0);

    pub const LINEAR_PCM_ARE_ALL_CLEAR: Self = Self::ALL_CLEAR;

    /// This flag is set for Apple Lossless data that was sourced from 16 bit native
    /// endian signed integer data.
    pub const APPLE_LOSSLESS_16_BIT_SOURCE_DATA: Self = Self(1);

    /// This flag is set for Apple Lossless data that was sourced from 20 bit native
    /// endian signed integer data aligned high in 24 bits.
    pub const APPLE_LOSSLESS_20_BIT_SOURCE_DATA: Self = Self(2);

    /// his flag is set for Apple Lossless data that was sourced from 24 bit native
    /// endian signed integer data.
    pub const APPLE_LOSSLESS_24_BIT_SOURCE_DATA: Self = Self(3);

    /// This flag is set for Apple Lossless data that was sourced from 32 bit native
    /// endian signed integer data.
    pub const APPLE_LOSSLESS_32_BIT_SOURCE_DATA: Self = Self(4);
}

/// This structure encapsulates all the information for describing the basic
/// format properties of a stream of audio data.
///
/// This structure is sufficient to describe any constant bit rate format that  has
/// channels that are the same size. Extensions are required for variable bit rate
/// data and for constant bit rate data where the channels have unequal sizes.
/// However, where applicable, the appropriate fields will be filled out correctly
/// for these kinds of formats (the extra data is provided via separate properties).
/// In all fields, a value of 0 indicates that the field is either unknown, not
/// applicable or otherwise is inapproprate for the format and should be ignored.
/// Note that 0 is still a valid value for most formats in the format_flags field.
///
/// In audio data a frame is one sample across all channels. In non-interleaved
/// audio, the per frame fields identify one channel. In interleaved audio, the per
/// frame fields identify the set of n channels. In uncompressed audio, a Packet is
/// one frame, (frames_per_packet == 1). In compressed audio, a Packet is an
/// indivisible chunk of compressed data, for example an AAC packet will contain
/// 1024 sample frames.
///
#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct StreamBasicDescription {
    /// The number of sample frames per second of the data in the stream.
    pub sample_rate: f64,
    /// The AudioFormatID indicating the general kind of data in the stream.
    pub format_id: FormatID,
    /// The AudioFormatFlags for the format indicated by mFormatID.
    pub format_flags: FormatFlags,
    /// The number of bytes in a packet of data.
    pub bytes_per_packet: u32,
    /// The number of sample frames in each packet of data.
    pub frames_per_packet: u32,
    /// The number of bytes in a single sample frame of data.
    pub bytes_per_frame: u32,
    /// The number of channels in each frame of data.
    pub channels_per_frame: u32,
    /// The number of bits of sample data for each channel in a frame of data.
    pub bits_per_channel: u32,
    /// Pads the structure out to force an even 8 byte alignment.
    pub reserved: u32,
}

impl StreamBasicDescription {
    #[inline]
    pub fn is_native_endian(&self) -> bool {
        self.format_id == FormatID::LINEAR_PCM
            && (self.format_flags.0 & FormatFlags::IS_BIG_ENDIAN.0 == FormatFlags::NATIVE_ENDIAN.0)
    }
}

/// This structure describes the packet layout of a buffer of data where the size of
/// each packet may not be the same or where there is extraneous data between
/// packets.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct StreamPacketDescription {
    /// The number of bytes from the start of the buffer to the beginning of the
    /// packet.
    pub start_offset: i64,
    /// The number of sample frames of data in the packet. For formats with a
    /// constant number of frames per packet, this field is set to 0.
    pub variable_frames_in_packet: u32,
    /// The number of bytes in the packet.
    pub data_byte_size: u32,
}

/// The format can use any sample rate. Note that this constant can only appear
/// in listings of supported formats. It will never appear in a current format.
pub const STREAM_ANY_RATE: f64 = 0.0;

#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct SMPTETimeType(pub u32);

impl SMPTETimeType {
    /// 24 Frame
    pub const _24: Self = Self(0);

    /// 25 Frame
    pub const _25: Self = Self(1);

    /// 30 Drop Frame
    pub const _30_DROP: Self = Self(2);

    /// 30 Frame
    pub const _30: Self = Self(3);

    /// 29.97 Frame
    pub const _29_97: Self = Self(4);

    /// 29.97 Drop Frame
    pub const _29_97_DROP: Self = Self(5);

    /// 60 Frame
    pub const _60: Self = Self(6);

    /// 59.94 Frame
    pub const _59_94: Self = Self(7);

    /// 60 Drop Frame
    pub const _60_DROP: Self = Self(8);

    /// 59.94 Drop Frame
    pub const _59_94_DROP: Self = Self(9);

    /// 50 Frame
    pub const _50: Self = Self(10);

    /// 23.98 Frame
    pub const _23_98: Self = Self(11);
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct SMPTETimeFlags(pub u32);

impl SMPTETimeFlags {
    pub const UNKNOWN: Self = Self(0);
    pub const VALID: Self = Self(1u32 << 0);
    pub const RUNNING: Self = Self(1u32 << 1);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct SMPTETime {
    pub subframes: i16,
    pub subframes_divisor: i16,
    pub counter: u32,
    pub r#type: SMPTETimeType,
    pub flags: SMPTETimeFlags,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct TimeStamp {
    /// The absolute sample frame time.
    pub sample_time: f64,
    /// The host machine's time base, mach_absolute_time.
    pub host_time: u64,
    /// The ratio of actual host ticks per sample frame to the nominal host ticks
    /// per sample frame.
    pub rate_scalar: f64,
    /// The word clock time.
    pub work_clock_time: u64,
    /// The SMPTE time.
    pub smpte_time: SMPTETime,
    /// A set of flags indicating which representations of the time are valid.
    pub flags: TimeStampFlags,
    /// Pads the structure out to force an even 8 byte alignment.
    pub reserved: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimeStampFlags(pub u32);

impl TimeStampFlags {
    pub const NOTHING_VALID: Self = Self(0);
    /// The sample frame time is valid.
    pub const SAMPLE_TIME_VALID: Self = Self(1u32 << 0);
    /// The host time is valid.
    pub const HOST_TIME_VALID: Self = Self(1u32 << 1);
    /// The rate scalar is valid.
    pub const RATE_SCALAR_VALID: Self = Self(1u32 << 2);
    /// The word clock time is valid.
    pub const WORD_CLOCK_TIME_VALID: Self = Self(1u32 << 3);
    /// The SMPTE time is valid.
    pub const SMPTETIME_VALID: Self = Self(1u32 << 4);
    /// The sample frame time and the host time are valid.
    pub const SAMPLE_HOST_TIME_VALID: Self =
        Self(Self::SAMPLE_TIME_VALID.0 | Self::HOST_TIME_VALID.0);
}

/// This structure is used to describe codecs installed on the system.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ClassDescription {
    /// The four char code codec type.
    pub m_type: os::Type,
    /// The four char code codec subtype.
    pub m_sub_type: os::Type,
    /// The four char code codec manufacturer.
    pub m_manufacturer: os::Type,
}

/// A tag identifying how the channel is to be used.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ChannelLabel(pub u32);

impl ChannelLabel {
    /// unknown or unspecified other use
    pub const UNKNOWN: Self = Self(0xFFFFFFFF);
    /// channel is present, but has no intended use or destination
    pub const UNUSED: Self = Self(0);
    /// channel is described by the mCoordinates fields.
    pub const USE_COORDINATES: Self = Self(100);

    pub const LEFT: Self = Self(1);
    pub const RIGHT: Self = Self(2);
    pub const CENTER: Self = Self(3);
    pub const LFE_SCREEN: Self = Self(4);
    pub const LEFT_SURROUND: Self = Self(5);
    pub const RIGHT_SURROUND: Self = Self(6);
    pub const LEFT_CENTER: Self = Self(7);
    pub const RIGHT_CENTER: Self = Self(8);

    /// WAVE: "Back Center" or plain "Rear Surround"
    pub const CENTER_SURROUND: Self = Self(9);
    pub const LEFT_SURROUND_DIRECT: Self = Self(10);
    pub const RIGHT_SURROUND_DIRECT: Self = Self(11);
    pub const TOP_CENTER_SURROUND: Self = Self(12);

    /// WAVE: "Top Front Left
    pub const VERTICAL_HEIGHT_LEFT: Self = Self(13);

    /// WAVE: "Top Front Center"
    pub const VERTICAL_HEIGHT_CENTER: Self = Self(14);

    /// WAVE: "Top Front Right"
    pub const VERTICAL_HEIGHT_RIGHT: Self = Self(15);

    pub const TOP_BACK_LEFT: Self = Self(16);
    pub const TOP_BACK_CENTER: Self = Self(17);
    pub const TOP_BACK_RIGHT: Self = Self(18);

    pub const REAR_SURROUND_LEFT: Self = Self(33);
    pub const REAR_SURROUND_RIGHT: Self = Self(34);
    pub const LEFT_WIDE: Self = Self(35);
    pub const RIGHT_WIDE: Self = Self(36);
    pub const LFE2: Self = Self(37);
    /// matrix encoded 4 channels
    pub const LEFT_TOTAL: Self = Self(38);
    /// matrix encoded 4 channels
    pub const RIGHT_TOTAL: Self = Self(39);
    pub const HEARING_IMPAIRED: Self = Self(40);
    pub const NARRATION: Self = Self(41);
    pub const MONO: Self = Self(42);
    pub const DIALOG_CENTRIC_MIX: Self = Self(43);

    /// back center, non diffuse
    pub const CENTER_SURROUND_DIRECT: Self = Self(44);

    pub const HAPTIC: Self = Self(45);

    pub const LEFT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_LEFT;
    pub const CENTER_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_CENTER;
    pub const RIGHT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_RIGHT;
    pub const LEFT_TOP_MIDDLE: Self = Self(49);
    pub const CENTER_TOP_MIDDLE: Self = Self::TOP_CENTER_SURROUND;
    pub const RIGHT_TOP_MIDDLE: Self = Self(51);
    pub const LEFT_TOP_REAR: Self = Self(52);
    pub const CENTER_TOP_REAR: Self = Self(53);
    pub const RIGHT_TOP_REAR: Self = Self(54);

    // first order ambisonic channels

    pub const AMBISONIC_W: Self = Self(200);
    pub const AMBISONIC_X: Self = Self(201);
    pub const AMBISONIC_Y: Self = Self(202);
    pub const AMBISONIC_Z: Self = Self(203);

    // Mid/Side Recording

    pub const MS_MID: Self = Self(204);
    pub const MS_SIDE: Self = Self(205);

    // X-Y Recording

    pub const XY_X: Self = Self(206);
    pub const XY_Y: Self = Self(207);

    // Binaural Recording
    pub const BINAURAL_LEFT: Self = Self(208);
    pub const BINAURAL_RIGHT: Self = Self(209);

    // other
    pub const HEADPHONES_LEFT: Self = Self(301);
    pub const HEADPHONES_RIGHT: Self = Self(302);
    pub const CLICK_TRACK: Self = Self(304);
    pub const FOREIGN_LANGUAGE: Self = Self(305);

    // generic discrete channel
    pub const DISCRETE: Self = Self(400);

    // numbered discrete channel
    pub const DISCRETE_0: Self = Self(1u32 << 16);
    pub const DISCRETE_1: Self = Self((1u32 << 16) | 1);
    pub const DISCRETE_2: Self = Self((1u32 << 16) | 2);
    pub const DISCRETE_3: Self = Self((1u32 << 16) | 3);
    pub const DISCRETE_4: Self = Self((1u32 << 16) | 4);
    pub const DISCRETE_5: Self = Self((1u32 << 16) | 5);
    pub const DISCRETE_6: Self = Self((1u32 << 16) | 6);
    pub const DISCRETE_7: Self = Self((1u32 << 16) | 7);
    pub const DISCRETE_8: Self = Self((1u32 << 16) | 8);
    pub const DISCRETE_9: Self = Self((1u32 << 16) | 9);
    pub const DISCRETE_10: Self = Self((1u32 << 16) | 10);
    pub const DISCRETE_11: Self = Self((1u32 << 16) | 11);
    pub const DISCRETE_12: Self = Self((1u32 << 16) | 12);
    pub const DISCRETE_13: Self = Self((1u32 << 16) | 13);
    pub const DISCRETE_14: Self = Self((1u32 << 16) | 14);
    pub const DISCRETE_15: Self = Self((1u32 << 16) | 15);
    pub const DISCRETE_65535: Self = Self((1u32 << 16) | 65535);

    // generic HOA ACN channel
    pub const HOA_ACN: Self = Self(500);

    // numbered HOA ACN channels, SN3D normalization
    pub const HOA_ACN_0: Self = Self(2u32 << 16);
    pub const HOA_ACN_1: Self = Self((2u32 << 16) | 1);
    pub const HOA_ACN_2: Self = Self((2u32 << 16) | 2);
    pub const HOA_ACN_3: Self = Self((2u32 << 16) | 3);
    pub const HOA_ACN_4: Self = Self((2u32 << 16) | 4);
    pub const HOA_ACN_5: Self = Self((2u32 << 16) | 5);
    pub const HOA_ACN_6: Self = Self((2u32 << 16) | 6);
    pub const HOA_ACN_7: Self = Self((2u32 << 16) | 7);
    pub const HOA_ACN_8: Self = Self((2u32 << 16) | 8);
    pub const HOA_ACN_9: Self = Self((2u32 << 16) | 9);
    pub const HOA_ACN_10: Self = Self((2u32 << 16) | 10);
    pub const HOA_ACN_11: Self = Self((2u32 << 16) | 11);
    pub const HOA_ACN_12: Self = Self((2u32 << 16) | 12);
    pub const HOA_ACN_13: Self = Self((2u32 << 16) | 13);
    pub const HOA_ACN_14: Self = Self((2u32 << 16) | 14);
    pub const HOA_ACN_15: Self = Self((2u32 << 16) | 15);
    pub const HOA_ACN_65024: Self = Self((2u32 << 16) | 65024); // 254th order uses 65025 channels

    // Specific SN3D alias
    pub const HOA_SN3D: Self = Self::HOA_ACN_0; // Needs to be ORed with the channel index, not HOA order

    // HOA N3D
    pub const HOA_N3D: Self = Self(3u32 << 16); // Needs to be ORed with the channel index, not HOA order

    // Channel label values in this range are reserved for internal use
    pub const BEGIN_RESERVED: Self = Self(0xF0000000);
    pub const END_RESERVED: Self = Self(0xFFFFFFFE);
}

/// These constants are for use in the mChannelBitmap field of an
/// AudioChannelLayout structure
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ChannelBitmap(pub u32);

impl ChannelBitmap {
    pub const LEFT: Self = Self(1u32 << 0);
    pub const RIGHT: Self = Self(1u32 << 1);
    pub const CENTER: Self = Self(1u32 << 2);
    pub const LFE_SCREEN: Self = Self(1u32 << 3);
    pub const LEFT_SURROUND: Self = Self(1u32 << 4);
    pub const RIGHT_SURROUND: Self = Self(1u32 << 5);
    pub const LEFT_CENTER: Self = Self(1u32 << 6);
    pub const RIGHT_CENTER: Self = Self(1u32 << 7);
    pub const CENTER_SURROUND: Self = Self(1u32 << 8); // WAVE: "Back Center"
    pub const LEFT_SURROUND_DIRECT: Self = Self(1u32 << 9);
    pub const RIGHT_SURROUND_DIRECT: Self = Self(1u32 << 10);
    pub const TOP_CENTER_SURROUND: Self = Self(1u32 << 11);
    pub const VERTICAL_HEIGHT_LEFT: Self = Self(1u32 << 12); // WAVE: "Top Front Left"
    pub const VERTICAL_HEIGHT_CENTER: Self = Self(1u32 << 13); // WAVE: "Top Front Center"
    pub const VERTICAL_HEIGHT_RIGHT: Self = Self(1u32 << 14); // WAVE: "Top Front Right"
    pub const TOP_BACK_LEFT: Self = Self(1u32 << 15);
    pub const TOP_BACK_CENTER: Self = Self(1u32 << 16);
    pub const TOP_BACK_RIGHT: Self = Self(1u32 << 17);
    pub const LEFT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_LEFT;
    pub const CENTER_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_CENTER;
    pub const RIGHT_TOP_FRONT: Self = Self::VERTICAL_HEIGHT_RIGHT;
    pub const LEFT_TOP_MIDDLE: Self = Self(1u32 << 21);
    pub const CENTER_TOP_MIDDLE: Self = Self::TOP_CENTER_SURROUND;
    pub const RIGHT_TOP_MIDDLE: Self = Self(1u32 << 23);
    pub const LEFT_TOP_REAR: Self = Self(1u32 << 24);
    pub const CENTER_TOP_REAR: Self = Self(1u32 << 25);
    pub const RIGHT_TOP_REAR: Self = Self(1u32 << 26);
}

define_options!(ChannelFlags(u32));

/// These constants are used in the mChannelFlags field of an
/// AudioChannelDescription structure.
impl ChannelFlags {
    pub const ALL_OFF: Self = Self(0);

    /// The channel is specified by the cartesian coordinates of the speaker
    /// position. This flag is mutally exclusive with
    /// AudioChannelFlags::SPHERICAL_COORDINATES.
    pub const RECTANGULAR_COORDINATES: Self = Self(1u32 << 0);
    /// The channel is specified by the spherical coordinates of the speaker
    /// position. This flag is mutally exclusive with
    /// AudioChannelFlags::RECTANGULAR_COORDINATES.
    pub const SPHERICAL_COORDINATES: Self = Self(1u32 << 1);
    /// Set to indicate the units are in meters, clear to indicate the units are
    /// relative to the unit cube or unit sphere.
    pub const METERS: Self = Self(1u32 << 2);
}

/// Constants for indexing the mCoordinates array in an AudioChannelDescription
/// structure.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ChannelCoordinateIndex(pub u32);

/// Constants for indexing the coordinates array in an AudioChannelDescription
/// structure.
impl ChannelCoordinateIndex {
    /// For rectangular coordinates, negative is left and positive is right.
    pub const LEFT_RIGHT: Self = Self(0);

    ///  For rectangular coordinates, negative is back and positive is front.
    pub const BACK_FRONT: Self = Self(1);

    /// For rectangular coordinates, negative is below ground level, 0 is ground
    /// level, and positive is above ground level.
    pub const DOWN_UP: Self = Self(2);

    /// For spherical coordinates, 0 is front center, positive is right, negative is
    /// left. This is measured in degrees.
    pub const AZIMUTH: Self = Self(0);

    /// For spherical coordinates, +90 is zenith, 0 is horizontal, -90 is nadir.
    /// This is measured in degrees.
    pub const ELEVATION: Self = Self(1);

    /// For spherical coordinates, the units are described by flags.
    pub const DISTANCE: Self = Self(2);
}

/// Some channel abbreviations used below:
/// L - left
/// R - right
/// C - center
/// Ls - left surround
/// Rs - right surround
/// Cs - center surround
/// Rls - rear left surround
/// Rrs - rear right surround
/// Lw - left wide
/// Rw - right wide
/// Lsd - left surround direct
/// Rsd - right surround direct
/// Lc - left center
/// Rc - right center
/// Ts - top surround
/// Vhl - vertical height left
/// Vhc - vertical height center
/// Vhr - vertical height right
/// Ltm - left top middle
/// Rtm - right top middle
/// Ltr - left top rear
/// Ctr - center top rear
/// Rtr - right top rear
/// Lt - left matrix total. for matrix encoded stereo.
/// Rt - right matrix total. for matrix encoded stereo.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ChannelLayoutTag(pub u32);

/// These constants are used in the mChannelLayoutTag field of an AudioChannelLayout
/// structure.
impl ChannelLayoutTag {
    //  General layouts

    /// use the array of AudioChannelDescriptions to define the mapping.
    pub const USE_CHANNEL_DESCRIPTIONS: Self = Self(0u32 << 16);

    /// use the bitmap to define the mapping.
    pub const USE_CHANNEL_BITMAP: Self = Self(1u32 << 16);

    /// a standard mono stream
    pub const MONO: Self = Self((100u32 << 16) | 1);

    /// a standard stereo stream (L R) - implied playback
    pub const STEREO: Self = Self((101u32 << 16) | 2);

    /// a standard stereo stream (L R) - implied headphone playback
    pub const STEREO_HEADPHONES: Self = Self((102u32 << 16) | 2);

    /// a matrix encoded stereo stream (Lt, Rt)
    pub const MATRIX_STEREO: Self = Self((103u32 << 16) | 2);

    /// mid/side recording
    pub const MID_SIDE: Self = Self((104u32 << 16) | 2);

    /// coincident mic pair (often 2 figure 8's)
    pub const XY: Self = Self((105u32 << 16) | 2);

    /// binaural stereo (left, right)
    pub const BINAURAL: Self = Self((106u32 << 16) | 2);

    /// W, X, Y, Z
    pub const AMBISONIC_B_FORMAT: Self = Self((107u32 << 16) | 4);

    /// L R Ls Rs  -- 90 degree speaker separation
    pub const QUADRAPHONIC: Self = Self((108u32 << 16) | 4);

    /// L R Ls Rs C  -- 72 degree speaker separation
    pub const PENTAGONAL: Self = Self((109u32 << 16) | 5);
    /// L R Ls Rs C Cs  -- 60 degree speaker separation
    pub const HEXAGONAL: Self = Self((110u32 << 16) | 6);
    /// L R Ls Rs C Cs Lw Rw  -- 45 degree speaker separation
    pub const OCTAGONAL: Self = Self((111u32 << 16) | 8);

    /// left, right, rear left, rear right
    /// top left, top right, top rear left, top rear right
    pub const CUBE: Self = Self((112u32 << 16) | 8);

    ///  MPEG defined layouts

    /// C
    pub const MPEG_1_0: Self = Self::MONO;
    ///  L R
    pub const MPEG_2_0: Self = Self::STEREO;
    ///  L R C
    pub const MPEG_3_0_A: Self = Self((113u32 << 16) | 3);
    ///  C L R
    pub const MPEG_3_0_B: Self = Self((114u32 << 16) | 3);
    ///  L R C Cs
    pub const MPEG_4_0_A: Self = Self((115u32 << 16) | 4);
    ///  C L R Cs
    pub const MPEG_4_0_B: Self = Self((116u32 << 16) | 4);
    ///  L R C Ls Rs
    pub const MPEG_5_0_A: Self = Self((117u32 << 16) | 5);
    ///  L R Ls Rs C
    pub const MPEG_5_0_B: Self = Self((118u32 << 16) | 5);
    ///  L C R Ls Rs
    pub const MPEG_5_0_C: Self = Self((119u32 << 16) | 5);
    ///  C L R Ls Rs
    pub const MPEG_5_0_D: Self = Self((120u32 << 16) | 5);
    ///  L R C LFE Ls Rs
    pub const MPEG_5_1_A: Self = Self((121u32 << 16) | 6);
    ///  L R Ls Rs C LFE
    pub const MPEG_5_1_B: Self = Self((122u32 << 16) | 6);
    ///  L C R Ls Rs LFE
    pub const MPEG_5_1_C: Self = Self((123u32 << 16) | 6);
    ///  C L R Ls Rs LFE
    pub const MPEG_5_1_D: Self = Self((124u32 << 16) | 6);
    ///  L R C LFE Ls Rs Cs
    pub const MPEG_6_1_A: Self = Self((125u32 << 16) | 7);
    ///  L R C LFE Ls Rs Lc Rc
    pub const MPEG_7_1_A: Self = Self((126u32 << 16) | 8);
    ///  C Lc Rc L R Ls Rs LFE    (doc: IS-13818-7 MPEG2-AAC Table 3.1)
    pub const MPEG_7_1_B: Self = Self((127u32 << 16) | 8);
    ///  L R C LFE Ls Rs Rls Rrs
    pub const MPEG_7_1_C: Self = Self((128u32 << 16) | 8);
    ///  L R Ls Rs C LFE Lc Rc
    pub const EMAGIC_DEFAULT_7_1: Self = Self((129u32 << 16) | 8);
    ///  L R C LFE Ls Rs Lt Rt
    ///      (kAudioChannelLayoutTag_ITU_5_1 plus a matrix encoded stereo mix)
    pub const SMPTE_DTV: Self = Self((130u32 << 16) | 8);

    //  ITU defined layouts

    ///  C
    pub const ITU_1_0: Self = Self::MONO;
    ///  L R
    pub const ITU_2_0: Self = Self::STEREO;

    ///  L R Cs
    pub const ITU_2_1: Self = Self((131u32 << 16) | 3);

    ///  L R Ls Rs
    pub const ITU_2_2: Self = Self((132u32 << 16) | 4);

    ///  L R C
    pub const ITU_3_0: Self = Self::MPEG_3_0_A;

    ///  L R C Cs
    pub const ITU_3_1: Self = Self::MPEG_4_0_A;

    ///<  L R C Ls Rs
    pub const ITU_3_2: Self = Self::MPEG_5_0_A;

    ///  L R C LFE Ls Rs
    pub const ITU_3_2_1: Self = Self::MPEG_5_1_A;

    ///  L R C LFE Ls Rs Rls Rrs
    pub const ITU_3_4_1: Self = Self::MPEG_7_1_C;

    // DVD defined layouts

    /// C (mono)
    pub const DVD_0: Self = Self::MONO;
    /// L R
    pub const DVD_1: Self = Self::STEREO;
    /// L R Cs
    pub const DVD_2: Self = Self::ITU_2_1;
    /// L R Ls Rs
    pub const DVD_3: Self = Self::ITU_2_2;
    /// L R LFE
    pub const DVD_4: Self = Self((133u32 << 16) | 3);
    /// L R LFE Cs
    pub const DVD_5: Self = Self((134u32 << 16) | 4);
    /// L R LFE Ls Rs
    pub const DVD_6: Self = Self((135u32 << 16) | 5);
    /// L R C
    pub const DVD_7: Self = Self::MPEG_3_0_A;
    /// L R C Cs
    pub const DVD_8: Self = Self::MPEG_4_0_A;
    /// L R C Ls Rs
    pub const DVD_9: Self = Self::MPEG_5_0_A;
    /// L R C LFE
    pub const DVD_10: Self = Self((136u32 << 16) | 4);
    /// L R C LFE Cs
    pub const DVD_11: Self = Self((137u32 << 16) | 5);
    /// L R C LFE Ls Rs
    pub const DVD_12: Self = Self::MPEG_5_1_A;

    // 13 through 17 are duplicates of 8 through 12.

    /// L R C Cs
    pub const DVD_13: Self = Self::DVD_8;
    /// L R C Ls Rs
    pub const DVD_14: Self = Self::DVD_9;
    /// L R C LFE
    pub const DVD_15: Self = Self::DVD_10;
    /// L R C LFE Cs
    pub const DVD_16: Self = Self::DVD_11;
    /// L R C LFE Ls Rs
    pub const DVD_17: Self = Self::DVD_12;
    /// L R Ls Rs LFE
    pub const DVD_18: Self = Self((138u32 << 16) | 5);
    /// L R Ls Rs C
    pub const DVD_19: Self = Self::MPEG_5_0_B;
    /// L R Ls Rs C LFE
    pub const DVD_20: Self = Self::MPEG_5_1_B;

    // These layouts are recommended for AudioUnit usage
    // These are the symmetrical layouts
    pub const AUDIO_UNIT_4: Self = Self::QUADRAPHONIC;
    pub const AUDIO_UNIT_5: Self = Self::PENTAGONAL;
    pub const AUDIO_UNIT_6: Self = Self::HEXAGONAL;
    pub const AUDIO_UNIT_8: Self = Self::OCTAGONAL;

    // These are the surround-based layouts

    /// L R Ls Rs C
    pub const AUDIO_UNIT_5_0: Self = Self::MPEG_5_0_B;
    /// L R Ls Rs C Cs
    pub const AUDIO_UNIT_6_0: Self = Self((139u32 << 16) | 6);
    /// L R Ls Rs C Rls Rrs
    pub const AUDIO_UNIT_7_0: Self = Self((140u32 << 16) | 7);
    /// L R Ls Rs C Lc Rc
    pub const AUDIO_UNIT_7_0_FRONT: Self = Self((148u32 << 16) | 7);
    /// L R C LFE Ls Rs
    pub const AUDIO_UNIT_5_1: Self = Self::MPEG_5_1_A;
    /// L R C LFE Ls Rs Cs
    pub const AUDIO_UNIT_6_1: Self = Self::MPEG_6_1_A;
    /// L R C LFE Ls Rs Rls Rrs
    pub const AUDIO_UNIT_7_1: Self = Self::MPEG_7_1_C;
    /// L R C LFE Ls Rs Lc Rc
    pub const AUDIO_UNIT_7_1_FRONT: Self = Self::MPEG_7_1_A;

    pub const AAC_3_0: Self = Self::MPEG_3_0_B;
    /// C L R
    pub const AAC_QUADRAPHONIC: Self = Self::QUADRAPHONIC;
    /// L R Ls Rs
    pub const AAC_4_0: Self = Self::MPEG_4_0_B;
    /// C L R Cs
    pub const AAC_5_0: Self = Self::MPEG_5_0_D;
    /// C L R Ls Rs
    pub const AAC_5_1: Self = Self::MPEG_5_1_D;
    /// C L R Ls Rs Lfe
    pub const AAC_6_0: Self = Self((141u32 << 16) | 6);
    /// C L R Ls Rs Cs
    pub const AAC_6_1: Self = Self((142u32 << 16) | 7);
    /// C L R Ls Rs Cs Lfe
    pub const AAC_7_0: Self = Self((143u32 << 16) | 7);
    /// C L R Ls Rs Rls Rrs
    pub const AAC_7_1: Self = Self::MPEG_7_1_B;
    /// C Lc Rc L R Ls Rs Lfe
    pub const AAC_7_1_B: Self = Self((183u32 << 16) | 8);
    /// C L R Ls Rs Rls Rrs LFE
    pub const AAC_7_1_C: Self = Self((184u32 << 16) | 8);
    /// C L R Ls Rs LFE Vhl Vhr
    pub const AAC_OCTAGONAL: Self = Self((144u32 << 16) | 8);
    /// C L R Ls Rs Rls Rrs Cs

    /// L R C Vhc Lsd Rsd Ls Rs Vhl Vhr Lw Rw Csd Cs LFE1 LFE2
    pub const TMH_10_2_STD: Self = Self((145u32 << 16) | 16);

    /// TMH_10_2_std plus: Lc Rc HI VI Haptic
    pub const TMH_10_2_FULL: Self = Self((146u32 << 16) | 21);

    /// C LFE
    pub const AC3_1_0_1: Self = Self((149u32 << 16) | 2);
    /// L C R
    pub const AC3_3_0: Self = Self((150u32 << 16) | 3);
    /// L C R Cs
    pub const AC3_3_1: Self = Self((151u32 << 16) | 4);
    /// L C R LFE
    pub const AC3_3_0_1: Self = Self((152u32 << 16) | 4);
    /// L R Cs LFE
    pub const AC3_2_1_1: Self = Self((153u32 << 16) | 4);
    /// L C R Cs LFE
    pub const AC3_3_1_1: Self = Self((154u32 << 16) | 5);

    /// L C R Ls Rs Cs
    pub const EAC_6_0_A: Self = Self((155u32 << 16) | 6);
    /// L C R Ls Rs Rls Rrs
    pub const EAC_7_0_A: Self = Self((156u32 << 16) | 7);

    /// L C R Ls Rs LFE Cs
    pub const EAC3_6_1_A: Self = Self((157u32 << 16) | 7);
    /// L C R Ls Rs LFE Ts
    pub const EAC3_6_1_B: Self = Self((158u32 << 16) | 7);
    /// L C R Ls Rs LFE Vhc
    pub const EAC3_6_1_C: Self = Self((159u32 << 16) | 7);
    /// L C R Ls Rs LFE Rls Rrs
    pub const EAC3_7_1_A: Self = Self((160u32 << 16) | 8);
    /// L C R Ls Rs LFE Lc Rc
    pub const EAC3_7_1_B: Self = Self((161u32 << 16) | 8);
    /// L C R Ls Rs LFE Lsd Rsd
    pub const EAC3_7_1_C: Self = Self((162u32 << 16) | 8);
    /// L C R Ls Rs LFE Lw Rw
    pub const EAC3_7_1_D: Self = Self((163u32 << 16) | 8);
    /// L C R Ls Rs LFE Vhl Vhr
    pub const EAC3_7_1_E: Self = Self((164u32 << 16) | 8);

    /// L C R Ls Rs LFE Cs Ts
    pub const EAC3_7_1_F: Self = Self((165u32 << 16) | 8);
    /// L C R Ls Rs LFE Cs Vhc
    pub const EAC3_7_1_G: Self = Self((166u32 << 16) | 8);
    /// L C R Ls Rs LFE Ts Vhc
    pub const EAC3_7_1_H: Self = Self((167u32 << 16) | 8);

    /// C L R LFE
    pub const DTS_3_1: Self = Self((168u32 << 16) | 4);
    /// C L R Cs LFE
    pub const DTS_4_1: Self = Self((169u32 << 16) | 5);
    /// Lc Rc L R Ls Rs
    pub const DTS_6_0_A: Self = Self((170u32 << 16) | 6);
    /// C L R Rls Rrs Ts
    pub const DTS_6_0_B: Self = Self((171u32 << 16) | 6);
    /// C Cs L R Rls Rrs
    pub const DTS_6_0_C: Self = Self((172u32 << 16) | 6);
    /// Lc Rc L R Ls Rs LFE
    pub const DTS_6_1_A: Self = Self((173u32 << 16) | 7);
    /// C L R Rls Rrs Ts LFE
    pub const DTS_6_1_B: Self = Self((174u32 << 16) | 7);
    /// C Cs L R Rls Rrs LFE
    pub const DTS_6_1_C: Self = Self((175u32 << 16) | 7);
    /// Lc C Rc L R Ls Rs
    pub const DTS_7_0: Self = Self((176u32 << 16) | 7);
    /// Lc C Rc L R Ls Rs LFE
    pub const DTS_7_1: Self = Self((177u32 << 16) | 8);
    /// Lc Rc L R Ls Rs Rls Rrs
    pub const DTS_8_0_A: Self = Self((178u32 << 16) | 8);
    /// Lc C Rc L R Ls Cs Rs
    pub const DTS_8_0_B: Self = Self((179u32 << 16) | 8);
    /// Lc Rc L R Ls Rs Rls Rrs LFE
    pub const DTS_8_1_A: Self = Self((180u32 << 16) | 9);
    /// Lc C Rc L R Ls Cs Rs LFE
    pub const DTS_8_1_B: Self = Self((181u32 << 16) | 9);
    /// C L R Ls Rs LFE Cs
    pub const DTS_6_1_D: Self = Self((182u32 << 16) | 7);

    /// 3 channels, L R LFE
    pub const WAVE_2_1: Self = Self::DVD_4;
    /// 3 channels, L R C
    pub const WAVE_3_0: Self = Self::MPEG_3_0_A;
    /// 4 channels, L R Ls Rs
    pub const WAVE_4_0_A: Self = Self::ITU_2_2;
    /// 4 channels, L R Rls Rrs
    pub const WAVE_4_0_B: Self = Self((185u32 << 16) | 4);
    /// 5 channels, L R C Ls Rs
    pub const WAVE_5_0_A: Self = Self::MPEG_5_0_A;
    /// 5 channels, L R C Rls Rrs
    pub const WAVE_5_0_B: Self = Self((186u32 << 16) | 5);
    /// 6 channels, L R C LFE Ls Rs
    pub const WAVE_5_1_A: Self = Self::MPEG_5_1_A;
    /// 6 channels, L R C LFE Rls Rrs
    pub const WAVE_5_1_B: Self = Self((187u32 << 16) | 6);
    /// 7 channels, L R C LFE Cs Ls Rs
    pub const WAVE_6_1: Self = Self((188u32 << 16) | 7);
    /// 8 channels, L R C LFE Rls Rrs Ls Rs
    pub const WAVE_7_1: Self = Self((189u32 << 16) | 8);

    /// Higher Order Ambisonics, Ambisonics Channel Number, SN3D normalization
    /// needs to be ORed with the actual number of channels (not the HOA order)
    pub const HOA_ACN_SN3D: Self = Self(190u32 << 16);
    /// Higher Order Ambisonics, Ambisonics Channel Number, N3D normalization
    /// needs to be ORed with the actual number of channels (not the HOA order)
    pub const HOA_ACN_N3D: Self = Self(191u32 << 16);

    /// L R C LFE Ls Rs Ltm Rtm
    pub const ATMOS_5_1_2: Self = Self((194u32 << 16) | 8);
    /// L R C LFE Ls Rs Vhl Vhr Ltr Rtr
    pub const ATMOS_5_1_4: Self = Self((195u32 << 16) | 10);
    /// L R C LFE Ls Rs Rls Rrs Ltm Rtm
    pub const ATMOS_7_1_2: Self = Self((196u32 << 16) | 10);
    /// L R C LFE Ls Rs Rls Rrs Vhl Vhr Ltr Rtr
    pub const ATMOS_7_1_4: Self = Self((192u32 << 16) | 12);
    /// L R C LFE Ls Rs Rls Rrs Lw Rw Vhl Vhr Ltm Rtm Ltr Rtr
    pub const ATMOS_9_1_6: Self = Self((193u32 << 16) | 16);

    /// C
    pub const LOGIC_MONO: Self = Self::MONO;
    /// L R
    pub const LOGIC_STEREO: Self = Self::STEREO;
    /// L R Ls Rs
    pub const LOGIC_QUADRAPHONIC: Self = Self::QUADRAPHONIC;
    /// L R C Cs
    pub const LOGIC_4_0_A: Self = Self::MPEG_4_0_A;
    /// C L R Cs
    pub const LOGIC_4_0_B: Self = Self::MPEG_4_0_B;
    /// L R Cs C
    pub const LOGIC_4_0_C: Self = Self((197u32 << 16) | 4);
    /// L R C Ls Rs
    pub const LOGIC_5_0_A: Self = Self::MPEG_5_0_A;
    /// L R Ls Rs C
    pub const LOGIC_5_0_B: Self = Self::MPEG_5_0_B;
    /// L C R Ls Rs
    pub const LOGIC_5_0_C: Self = Self::MPEG_5_0_C;
    /// C L R Ls Rs
    pub const LOGIC_5_0_D: Self = Self::MPEG_5_0_D;
    /// L R C LFE Ls Rs
    pub const LOGIC_5_1_A: Self = Self::MPEG_5_1_A;
    /// L R Ls Rs C LFE
    pub const LOGIC_5_1_B: Self = Self::MPEG_5_1_B;
    /// L C R Ls Rs LFE
    pub const LOGIC_5_1_C: Self = Self::MPEG_5_1_C;
    /// C L R Ls Rs LFE
    pub const LOGIC_5_1_D: Self = Self::MPEG_5_1_D;
    /// C L R Ls Rs Cs
    pub const LOGIC_6_0_A: Self = Self::AAC_6_0;
    /// L R Ls Rs Cs C
    pub const LOGIC_6_0_B: Self = Self((198u32 << 16) | 6);
    /// L R Ls Rs C Cs
    pub const LOGIC_6_0_C: Self = Self::AUDIO_UNIT_6_0;
    /// C L R Ls Rs Cs LFE
    pub const LOGIC_6_1_A: Self = Self::AAC_6_1;
    /// L R Ls Rs Cs C LFE
    pub const LOGIC_6_1_B: Self = Self((199u32 << 16) | 7);
    /// L R C LFE Ls Rs Cs
    pub const LOGIC_6_1_C: Self = Self::MPEG_6_1_A;
    /// L C R Ls Cs Rs LFE
    pub const LOGIC_6_1_D: Self = Self((200u32 << 16) | 7);
    /// L R C LFE Ls Rs Rls Rrs
    pub const LOGIC_7_1_A: Self = Self::AUDIO_UNIT_7_1;
    /// L R Ls Rs Rls Rrs C LFE
    pub const LOGIC_7_1_B: Self = Self((201u32 << 16) | 8);
    /// L R C LFE Ls Rs Rls Rrs
    pub const LOGIC_7_1_C: Self = Self::MPEG_7_1_C;
    /// L R C LFE Ls Rs Lc Rc
    pub const LOGIC_7_1_SDDS_A: Self = Self::MPEG_7_1_A;
    /// C Lc Rc L R Ls Rs LFE
    pub const LOGIC_7_1_SDDS_B: Self = Self::MPEG_7_1_B;
    /// L R Ls Rs C LFE Lc Rc
    pub const LOGIC_7_1_SDDS_C: Self = Self::EMAGIC_DEFAULT_7_1;
    /// L R C LFE Ls Rs Ltm Rtm
    pub const LOGIC_ATMOS_5_1_2: Self = Self::ATMOS_5_1_2;
    /// L R C LFE Ls Rs Vhl Vhr Ltr Rtr
    pub const LOGIC_ATMOS_5_1_4: Self = Self::ATMOS_5_1_4;
    /// L R C LFE Ls Rs Rls Rrs Ltm Rtm
    pub const LOGIC_ATMOS_7_1_2: Self = Self::ATMOS_7_1_2;
    /// L R C LFE Ls Rs Rls Rrs Vhl Vhr Ltr Rtr
    pub const LOGIC_ATMOS_7_1_4_A: Self = Self::ATMOS_7_1_4;
    /// L R Rls Rrs Ls Rs C LFE Vhl Vhr Ltr Rtr
    pub const LOGIC_ATMOS_7_1_4_B: Self = Self((202u32 << 16) | 12);
    /// L R Rls Rrs Ls Rs C LFE Vhl Vhr Ltm Rtm Ltr Rtr
    pub const LOGIC_ATMOS_7_1_6: Self = Self((203u32 << 16) | 14);

    /// needs to be ORed with the actual number of channels
    pub const DISCRETE_IN_ORDER: Self = Self(147u32 << 16);

    /// Channel layout tag values in this range are reserved for internal use
    pub const BEGIN_RESERVED: Self = Self(0xF0000000);
    /// Channel layout tag values in this range are reserved for internal use
    pub const END_RESERVED: Self = Self(0xFFFEFFFF);

    /// needs to be ORed with the actual number of channels
    pub const UNKNOWN: Self = Self(0xFFFF0000);
}

/// This structure describes a single channel.
#[derive(Debug)]
#[repr(C)]
pub struct ChannelDescription {
    /// The AudioChannelLabel that describes the channel.
    pub channel_label: ChannelLabel,

    /// Flags that control the interpretation of mCoordinates.
    pub channel_flags: ChannelFlags,

    /// An ordered triple that specifies a precise speaker location.
    pub coordinates: [f32; 3],
}

#[derive(Debug)]
#[repr(C)]
pub struct ChannelLayout<const N: usize> {
    pub channel_layout_tag: ChannelLayoutTag,
    pub channel_bitmap: ChannelBitmap,
    pub number_channel_descriptions: u32,
    pub channel_descriptions: [ChannelDescription; N],
}

#[derive(Debug)]
#[repr(C)]
pub struct FormatListItem {
    pub asbd: StreamBasicDescription,
    pub channel_layout_tag: ChannelLayoutTag,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct MPEG4ObjectID(pub c_long);

impl MPEG4ObjectID {
    pub const AAC_MAIN: Self = Self(1);
    pub const AAC_LC: Self = Self(2);
    pub const AAC_SSR: Self = Self(3);
    pub const AAC_LTP: Self = Self(4);
    pub const AAC_SBR: Self = Self(5);
    pub const AAC_SCALABLE: Self = Self(6);
    pub const TWIN_VQ: Self = Self(7);
    pub const CELP: Self = Self(8);
    pub const HVXC: Self = Self(9);
}
