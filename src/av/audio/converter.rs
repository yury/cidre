use crate::{av::audio, cf, define_obj_type, ns};

#[repr(isize)]
pub enum PrimeMethod {
    /// Primes with leading + trailing input frames.
    Pre = 0,
    /// Only primes with trailing (zero latency). Leading frames are assumed to be silence.
    Normal = 1,
    /// Acts in "latency" mode. Both leading and trailing frames assumed to be silence.
    None = 2,
}

#[repr(C)]
pub struct PrimeInfo {
    pub leading_frames: audio::FrameCount,
    pub trailing_frames: audio::FrameCount,
}

#[repr(isize)]
pub enum InputStatus {
    /// This is the normal case where you supply data to the converter.
    HaveData = 0,
    /// If you are out of data for now, set *ioNumberOfPackets = 0 and return
    /// AVAudioConverterInputStatus_NoDataNow; the  conversion routine will return as much output as
    /// could be converted with the input already supplied.
    NoDataNow = 1,
    /// If you are at the end of stream, set *ioNumberOfPackets = 0 and return
    /// AVAudioConverterInputStatus_EndOfStream.
    EndOfStream = 2,
}

#[repr(isize)]
pub enum OutputStatus {
    /// All of the requested data was returned.
    HaveData = 0,
    /// Not enough input was available to satisfy the request at the current time. The output buffer
    /// contains as much as could be converted.
    InputRanDry = 1,
    /// The end of stream has been reached. No data was returned.
    EndOfStream = 2,
    /// An error occurred.
    Error = 3,
}

define_obj_type!(Converter(ns::Id));

impl Converter {
    /*! @property bitRateStrategy
        @abstract When encoding, an AVEncoderBitRateStrategyKey value constant as defined in AVAudioSettings.h. Returns nil if not encoding.
    */
    // @property (nonatomic, retain, nullable) NSString *bitRateStrategy;
    // pub fn bit_rate_strategy(&self) -> Option<&av::Enc>

    /// The maximum size of an output packet, in bytes.
    /// When encoding it is useful to know how large a packet can be in order to allocate a buffer to receive the output.
    pub fn maximum_output_packet_size(&self) -> isize {
        unsafe { rsel_maximumOutputPacketSize(self) }
    }

    /// When encoding, an NSArray of NSNumber of all bit rates provided by the codec.
    /// Returns None if not encoding.
    pub fn available_encode_bit_rates(&self) -> Option<&cf::ArrayOf<cf::Number>> {
        unsafe { rsel_availableEncodeBitRates(self) }
    }

    /// When encoding, an NSArray of NSNumber of bit rates that can be applied based on the current formats and settings.
    /// Returns None if not encoding.
    pub fn applicable_encode_bit_rates(&self) -> Option<&cf::ArrayOf<cf::Number>> {
        unsafe { rsel_applicableEncodeBitRates(self) }
    }

    /// When encoding, an NSArray of NSNumber of all output sample rates provided by the codec.
    /// Returns None if not encoding.
    pub fn available_encode_sample_rates(&self) -> Option<&cf::ArrayOf<cf::Number>> {
        unsafe { rsel_availableEncodeSampleRates(self) }
    }

    /// When encoding, an NSArray of NSNumber of output sample rates that can be applied based on the current formats and settings.
    /// Returns None if not encoding.
    pub fn applicable_encode_sample_rates(&self) -> Option<&cf::ArrayOf<cf::Number>> {
        unsafe { rsel_applicableEncodeSampleRates(self) }
    }
    /// When encoding, an NSArray of NSNumber of all output channel layout tags provided by the codec.
    /// Returns None if not encoding
    pub fn available_encode_channel_layout_tags(&self) -> Option<&cf::ArrayOf<cf::Number>> {
        unsafe { rsel_availableEncodeChannelLayoutTags(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_availableEncodeChannelLayoutTags(id: &ns::Id) -> Option<&cf::ArrayOf<cf::Number>>;
    fn rsel_applicableEncodeSampleRates(id: &ns::Id) -> Option<&cf::ArrayOf<cf::Number>>;
    fn rsel_availableEncodeSampleRates(id: &ns::Id) -> Option<&cf::ArrayOf<cf::Number>>;
    fn rsel_applicableEncodeBitRates(id: &ns::Id) -> Option<&cf::ArrayOf<cf::Number>>;
    fn rsel_availableEncodeBitRates(id: &ns::Id) -> Option<&cf::ArrayOf<cf::Number>>;

    fn rsel_maximumOutputPacketSize(id: &ns::Id) -> isize;
}
