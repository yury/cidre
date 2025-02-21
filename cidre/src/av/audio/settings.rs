use crate::{arc, cf, ns};

/// keys for all formats
pub mod all_formats_keys {
    use crate::ns;

    /// value is an integer (format ID) from CoreAudioTypes.h
    pub fn id() -> &'static ns::String {
        unsafe { AVFormatIDKey }
    }

    /// value is floating point in Hertz in cf::Number
    pub fn sample_rate() -> &'static ns::String {
        unsafe { AVSampleRateKey }
    }

    /// value is an integer in cf::Number
    pub fn number_of_channels() -> &'static ns::String {
        unsafe { AVNumberOfChannelsKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVFormatIDKey: &'static ns::String;
        static AVSampleRateKey: &'static ns::String;
        static AVNumberOfChannelsKey: &'static ns::String;
    }
}

/// linear PCM keys
pub mod linear_pcm_keys {
    use crate::ns;
    /// value is an integer, one of: 8, 16, 24, 32 in cf::Number
    pub fn bit_depth() -> &'static ns::String {
        unsafe { AVLinearPCMBitDepthKey }
    }

    /// value is a BOOL in cf::Boolean ?
    pub fn is_big_endian() -> &'static ns::String {
        unsafe { AVLinearPCMIsBigEndianKey }
    }

    /// value is a BOOL in cf::Boolean ?
    pub fn is_float() -> &'static ns::String {
        unsafe { AVLinearPCMIsFloatKey }
    }

    /// value is a BOOL in cf::Boolean ?
    pub fn is_non_interleaved() -> &'static ns::String {
        unsafe { AVLinearPCMIsNonInterleaved }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVLinearPCMBitDepthKey: &'static ns::String;
        static AVLinearPCMIsBigEndianKey: &'static ns::String;
        static AVLinearPCMIsFloatKey: &'static ns::String;
        static AVLinearPCMIsNonInterleaved: &'static ns::String;
    }
}

pub mod file_keys {
    use crate::ns;

    /// value is an integer (audio file type) from AudioFile.h
    pub fn file_type() -> &'static ns::String {
        unsafe { AVAudioFileTypeKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVAudioFileTypeKey: &'static ns::String;
    }
}

pub mod encoder_propery_keys {
    use crate::ns;
    /// value is an integer from av::AudioQuality in cf::Number
    pub fn audio_quality() -> &'static ns::String {
        unsafe { AVEncoderAudioQualityKey }
    }

    /// value is an integer from AV::AudioQuality.
    /// only relevant for AVAudioBitRateStrategy_Variable
    pub fn audio_quality_for_vbr() -> &'static ns::String {
        unsafe { AVEncoderAudioQualityForVBRKey }
    }

    /// only one of bit_rate and bit_rate_per_channel should be provided
    /// value is an integer in cf::Number
    pub fn bit_rate() -> &'static ns::String {
        unsafe { AVEncoderBitRateKey }
    }

    /// only one of bit_rate and bit_rate_per_channel should be provided
    /// value is an integer in cf::Number
    pub fn bit_rate_per_channel() -> &'static ns::String {
        unsafe { AVEncoderBitRatePerChannelKey }
    }

    /// value is an bit_rate_strategy
    pub fn bit_rate_strategy() -> &'static ns::String {
        unsafe { AVEncoderBitRateStrategyKey }
    }

    /// value is an integer from 8 to 32 in cf::Number
    pub fn bit_depth_hint() -> &'static ns::String {
        unsafe { AVEncoderBitDepthHintKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVEncoderAudioQualityKey: &'static ns::String;
        static AVEncoderAudioQualityForVBRKey: &'static ns::String;

        static AVEncoderBitRateKey: &'static ns::String;
        static AVEncoderBitRatePerChannelKey: &'static ns::String;

        static AVEncoderBitRateStrategyKey: &'static ns::String;
        static AVEncoderBitDepthHintKey: &'static ns::String;
    }
}

/// sample rate converter property keys
pub mod sample_rate_converter_keys {
    use crate::ns;
    /// value is an AVSampleRateConverterAlgorithm constant. see below
    pub fn algorithm() -> &'static ns::String {
        unsafe { AVSampleRateConverterAlgorithmKey }
    }
    /// value is an integer from enum av::AudioQuality in cf::Number
    pub fn audio_quality() -> &'static ns::String {
        unsafe { AVSampleRateConverterAudioQualityKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVSampleRateConverterAlgorithmKey: &'static ns::String;
        static AVSampleRateConverterAudioQualityKey: &'static ns::String;
    }
}

/// value is an NSData containing an AudioChannelLayout
pub fn channel_layout_key() -> &'static ns::String {
    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVChannelLayoutKey: &'static ns::String;
    }

    unsafe { AVChannelLayoutKey }
}

// values for encoder_propery_keys::bit_rate_strategy key
pub mod bit_rate_strategy {
    use crate::ns;

    pub fn constant() -> &'static ns::String {
        unsafe { AVAudioBitRateStrategy_Constant }
    }

    pub fn long_term_average() -> &'static ns::String {
        unsafe { AVAudioBitRateStrategy_LongTermAverage }
    }

    pub fn variable_constrained() -> &'static ns::String {
        unsafe { AVAudioBitRateStrategy_VariableConstrained }
    }

    pub fn variable() -> &'static ns::String {
        unsafe { AVAudioBitRateStrategy_Variable }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVAudioBitRateStrategy_Constant: &'static ns::String;
        static AVAudioBitRateStrategy_LongTermAverage: &'static ns::String;
        static AVAudioBitRateStrategy_VariableConstrained: &'static ns::String;
        static AVAudioBitRateStrategy_Variable: &'static ns::String;
    }
}

// values for sample_rate_converter::algorithm
pub mod sample_rate_converer_algorithm {
    use crate::ns;

    pub fn normal() -> &'static ns::String {
        unsafe { AVSampleRateConverterAlgorithm_Normal }
    }

    pub fn mastering() -> &'static ns::String {
        unsafe { AVSampleRateConverterAlgorithm_Mastering }
    }

    pub fn minimum_phase() -> &'static ns::String {
        unsafe { AVSampleRateConverterAlgorithm_MinimumPhase }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    unsafe extern "C" {
        static AVSampleRateConverterAlgorithm_Normal: &'static ns::String;
        static AVSampleRateConverterAlgorithm_Mastering: &'static ns::String;
        static AVSampleRateConverterAlgorithm_MinimumPhase: &'static ns::String;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Quality(pub isize);

impl Quality {
    pub const MIN: Self = Self(0);
    pub const LOW: Self = Self(0x20);
    pub const MEDIUM: Self = Self(0x40);
    pub const HIGH: Self = Self(0x60);
    pub const MAX: Self = Self(0x7F);

    pub fn to_cf_number(&self) -> arc::R<cf::Number> {
        cf::Number::from_i64(self.0 as _)
    }
}
