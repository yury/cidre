use std::intrinsics::transmute;

use crate::{arc, at::audio::StreamBasicDescription, cf, define_obj_type, ns};

use super::{channel_layout::ChannelLayout, ChannelCount};

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum CommonFormat {
    /// A format other than one of the common ones below.
    Other = 0,

    /// Native-endian floats (this is the standard format).
    PCMFloat32 = 1,
    /// Native-endian doubles.
    PCMFloat64 = 2,
    /// Signed 16-bit native-endian integers.
    PCMInt16 = 3,
    /// Signed 32-bit native-endian integers.
    PCMInt32 = 4,
}

define_obj_type!(Format(ns::Id));

/// AVAudioFormat wraps a Core Audio AudioStreamBasicDescription struct, with convenience
/// initializers and accessors for common formats, including Core Audio's standard deinterleaved
/// 32-bit floating point.
///
/// Instances of this class are immutable.
impl Format {
    /// If the format specifies more than 2 channels, this method fails (returns None).
    pub fn with_asbd(asbd: &StreamBasicDescription) -> Option<arc::R<Format>> {
        unsafe { AVAudioFormat_initWithStreamDescription(asbd) }
    }

    /// the channel layout. Can be None only if asbd specifies 1 or 2 channels.
    pub fn with_asbd_and_channel_layout(
        asbd: &StreamBasicDescription,
        layout: Option<&ChannelLayout>,
    ) -> Option<arc::R<Format>> {
        unsafe { AVAudioFormat_initWithStreamDescription_channelLayout(asbd, layout) }
    }

    pub fn standard_with_sample_rate_and_channels(
        sample_rate: f64,
        channels: ChannelCount,
    ) -> Option<arc::R<Format>> {
        unsafe { AVAudioFormat_initStandardFormatWithSampleRate_channels(sample_rate, channels) }
    }

    /// Initialize to deinterleaved float with the specified sample rate and channel layout.
    pub fn standard_with_sample_rate_and_channel_layout(
        sample_rate: f64,
        layout: &ChannelLayout,
    ) -> arc::R<Format> {
        unsafe { AVAudioFormat_initStandardFormatWithSampleRate_channelLayout(sample_rate, layout) }
    }

    /// Initialize to float with the specified sample rate, channel layout and interleavedness.
    pub fn with_common_format_sample_rate_interleaved_channel_layout(
        format: CommonFormat,
        sample_rate: f64,
        interleaved: bool,
        channel_layout: &ChannelLayout,
    ) -> arc::R<Format> {
        unsafe {
            AVAudioFormat_initWithCommonFormat_sampleRate_interleaved_channelLayout(
                format,
                sample_rate,
                interleaved,
                channel_layout,
            )
        }
    }

    pub fn with_settings(settings: &cf::Dictionary) -> Option<arc::R<Format>> {
        unsafe { AVAudioFormat_initWithSettings(settings) }
    }

    /// ```
    /// use cidre::av;
    ///
    /// let format = av::AudioFormat::standard_with_sample_rate_and_channels(44_100f64, 2).unwrap();
    /// let settings = format.settings();
    /// ```
    pub fn settings(&self) -> &cf::DictionaryOf<cf::String, ns::Id> {
        unsafe { transmute(rsel_settings(self)) }
    }

    #[inline]
    pub fn is_interleaved(&self) -> bool {
        unsafe { rsel_isInterleaved(self) }
    }

    #[inline]
    pub fn common_format(&self) -> CommonFormat {
        unsafe { rsel_commonFormat(self) }
    }

    #[inline]
    pub fn channel_count(&self) -> ChannelCount {
        unsafe { av_format_rsel_channelCount(self) }
    }

    #[inline]
    pub fn absd(&self) -> &StreamBasicDescription {
        unsafe { rsel_streamDescription(self) }
    }

    #[inline]
    pub fn channel_layout(&self) -> Option<&ChannelLayout> {
        unsafe { rsel_channelLayout(self) }
    }

    #[inline]
    pub fn magic_cookie(&self) -> Option<&cf::Data> {
        unsafe { rsel_magicCookie(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {

    fn AVAudioFormat_initWithStreamDescription(
        asbd: &StreamBasicDescription,
    ) -> Option<arc::R<Format>>;

    fn AVAudioFormat_initWithStreamDescription_channelLayout(
        asbd: &StreamBasicDescription,
        layout: Option<&ChannelLayout>,
    ) -> Option<arc::R<Format>>;

    fn AVAudioFormat_initStandardFormatWithSampleRate_channels(
        sample_rate: f64,
        channels: ChannelCount,
    ) -> Option<arc::R<Format>>;

    fn AVAudioFormat_initStandardFormatWithSampleRate_channelLayout(
        sample_rate: f64,
        layout: &ChannelLayout,
    ) -> arc::R<Format>;

    fn AVAudioFormat_initWithCommonFormat_sampleRate_interleaved_channelLayout(
        format: CommonFormat,
        sample_rate: f64,
        interleaved: bool,
        channel_layout: &ChannelLayout,
    ) -> arc::R<Format>;

    fn AVAudioFormat_initWithSettings(settings: &cf::Dictionary) -> Option<arc::R<Format>>;

    fn rsel_settings(id: &ns::Id) -> &cf::Dictionary;
    fn rsel_isInterleaved(id: &ns::Id) -> bool;
    fn rsel_commonFormat(id: &ns::Id) -> CommonFormat;
    fn av_format_rsel_channelCount(id: &ns::Id) -> ChannelCount;
    fn rsel_streamDescription(id: &ns::Id) -> &StreamBasicDescription;
    fn rsel_channelLayout(id: &ns::Id) -> Option<&ChannelLayout>;

    fn rsel_magicCookie(id: &ns::Id) -> Option<&cf::Data>;

}
