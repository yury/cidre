use crate::{arc, at::audio::StreamBasicDesc, define_cls, define_obj_type, ns, objc};

use super::{channel_layout::ChannelLayout, ChannelCount};

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum CommonFormat {
    /// A format other than one of the common ones below.
    Other = 0,

    /// Native-endian floats (this is the standard format).
    PcmF32 = 1,

    /// Native-endian doubles.
    PcmF64 = 2,

    /// Signed 16-bit native-endian integers.
    PcmI16 = 3,

    /// Signed 32-bit native-endian integers.
    PcmI32 = 4,
}

define_obj_type!(Format(ns::Id));

impl arc::A<Format> {
    #[objc::msg_send(initWithStreamDescription:)]
    pub fn init_with_stream_desc(self, asbd: &StreamBasicDesc) -> Option<arc::R<Format>>;

    #[objc::msg_send(initWithStreamDescription:channelLayout:)]
    pub fn init_with_stream_desc_channel_layout(
        self,
        asbd: &StreamBasicDesc,
        layout: Option<&ChannelLayout>,
    ) -> Option<arc::R<Format>>;

    #[objc::msg_send(initStandardFormatWithSampleRate:channels:)]
    pub fn init_standard_with_sample_rate_channels(
        self,
        sample_rate: f64,
        channels: ChannelCount,
    ) -> Option<arc::R<Format>>;

    #[objc::msg_send(initStandardFormatWithSampleRate:channelLayout:)]
    pub fn init_standard_with_sample_rate_channel_layout(
        self,
        sample_rate: f64,
        layout: &ChannelLayout,
    ) -> arc::R<Format>;

    #[objc::msg_send(initWithCommonFormat:sampleRate:interleaved:channelLayout:)]
    pub fn init_with_common_format_sample_rate_interleaved_channel_layout(
        self,
        format: CommonFormat,
        sample_rate: f64,
        interleaved: bool,
        channel_layout: &ChannelLayout,
    ) -> arc::R<Format>;

    #[objc::msg_send(initWithSettings:)]
    pub fn init_with_settings(
        self,
        settings: &ns::Dictionary<ns::String, ns::Id>,
    ) -> Option<arc::R<Format>>;
}

/// av::AudioFormat wraps a Core Audio AudioStreamBasicDescription struct, with convenience
/// initializers and accessors for common formats, including Core Audio's standard deinterleaved
/// 32-bit floating point.
///
/// Instances of this class are immutable.
impl Format {
    define_cls!(AV_AUDIO_FORMAT);

    /// If the format specifies more than 2 channels, this method fails (returns None).
    pub fn with_asbd(asbd: &StreamBasicDesc) -> Option<arc::R<Self>> {
        Self::alloc().init_with_stream_desc(asbd)
    }

    /// the channel layout. Can be None only if asbd specifies 1 or 2 channels.
    pub fn with_asbd_and_channel_layout(
        asbd: &StreamBasicDesc,
        layout: Option<&ChannelLayout>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_stream_desc_channel_layout(asbd, layout)
    }

    pub fn standard_with_sample_rate_and_channels(
        sample_rate: f64,
        channels: ChannelCount,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_standard_with_sample_rate_channels(sample_rate, channels)
    }

    /// Initialize to deinterleaved float with the specified sample rate and channel layout.
    pub fn standard_with_sample_rate_and_channel_layout(
        sample_rate: f64,
        layout: &ChannelLayout,
    ) -> arc::R<Self> {
        Self::alloc().init_standard_with_sample_rate_channel_layout(sample_rate, layout)
    }

    /// Initialize to float with the specified sample rate, channel layout and interleavedness.
    pub fn with_common_format_sample_rate_interleaved_channel_layout(
        format: CommonFormat,
        sample_rate: f64,
        interleaved: bool,
        channel_layout: &ChannelLayout,
    ) -> arc::R<Format> {
        Self::alloc().init_with_common_format_sample_rate_interleaved_channel_layout(
            format,
            sample_rate,
            interleaved,
            channel_layout,
        )
    }

    pub fn with_settings(settings: &ns::Dictionary<ns::String, ns::Id>) -> Option<arc::R<Format>> {
        Self::alloc().init_with_settings(settings)
    }

    /// ```
    /// use cidre::av;
    ///
    /// let format = av::AudioFormat::standard_with_sample_rate_and_channels(44_100f64, 2).unwrap();
    /// let settings = format.settings();
    /// ```
    #[objc::msg_send(settings)]
    pub fn settings(&self) -> &ns::Dictionary<ns::String, ns::Id>;

    #[objc::msg_send(isInterleaved)]
    pub fn is_interleaved(&self) -> bool;

    #[objc::msg_send(commonFormat)]
    pub fn common_format(&self) -> CommonFormat;

    #[objc::msg_send(channelCount)]
    pub fn channel_count(&self) -> ChannelCount;

    #[objc::msg_send(streamDescription)]
    pub fn absd(&self) -> &StreamBasicDesc;

    #[objc::msg_send(channelLayout)]
    pub fn channel_layout(&self) -> Option<&ChannelLayout>;

    #[objc::msg_send(magicCookie)]
    pub fn magic_cookie(&self) -> Option<&ns::Data>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_FORMAT: &'static objc::Class<Format>;
}
