use crate::cf;

/// keys for all formats
pub mod all_formats_keys {
    use crate::cf;

    /// value is an integer (format ID) from CoreAudioTypes.h
    pub fn id() -> &'static cf::String {
        unsafe { AVFormatIDKey }
    }

    /// value is floating point in Hertz in cf::Number
    pub fn sample_rate() -> &'static cf::String {
        unsafe { AVSampleRateKey }
    }

    /// value is an integer in cf::Number
    pub fn number_of_channels() -> &'static cf::String {
        unsafe { AVNumberOfChannelsKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVFormatIDKey: &'static cf::String;
        static AVSampleRateKey: &'static cf::String;
        static AVNumberOfChannelsKey: &'static cf::String;
    }
}

/// linear PCM keys
pub mod linear_pcm_keys {
    use crate::cf;
    /// value is an integer, one of: 8, 16, 24, 32 in cf::Number
    pub fn bit_depth() -> &'static cf::String {
        unsafe { AVLinearPCMBitDepthKey }
    }

    /// value is a BOOL in cf::Boolean ?
    pub fn is_big_endian() -> &'static cf::String {
        unsafe { AVLinearPCMIsBigEndianKey }
    }

    /// value is a BOOL in cf::Boolean ?
    pub fn is_float() -> &'static cf::String {
        unsafe { AVLinearPCMIsFloatKey }
    }

    /// value is a BOOL in cf::Boolean ?
    pub fn is_non_interleaved() -> &'static cf::String {
        unsafe { AVLinearPCMIsNonInterleaved }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVLinearPCMBitDepthKey: &'static cf::String;
        static AVLinearPCMIsBigEndianKey: &'static cf::String;
        static AVLinearPCMIsFloatKey: &'static cf::String;
        static AVLinearPCMIsNonInterleaved: &'static cf::String;
    }
}

pub mod file_keys {
    use crate::cf;

    /// value is an integer (audio file type) from AudioFile.h
    pub fn file_type() -> &'static cf::String {
        unsafe { AVAudioFileTypeKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVAudioFileTypeKey: &'static cf::String;
    }
}

pub mod encoder_propery_keys {
    use crate::cf;
    /// value is an integer from av::AudioQuality in cf::Number
    pub fn audio_quality() -> &'static cf::String {
        unsafe { AVEncoderAudioQualityKey }
    }

    /// value is an integer from AV::AudioQuality.
    /// only relevant for AVAudioBitRateStrategy_Variable
    pub fn audio_quality_for_vbr() -> &'static cf::String {
        unsafe { AVEncoderAudioQualityForVBRKey }
    }

    /// only one of bit_rate and bit_rate_per_channel should be provided
    /// value is an integer in cf::Number
    pub fn bit_rate() -> &'static cf::String {
        unsafe { AVEncoderBitRateKey }
    }

    /// only one of bit_rate and bit_rate_per_channel should be provided
    /// value is an integer in cf::Number
    pub fn bit_rate_per_channel() -> &'static cf::String {
        unsafe { AVEncoderBitRatePerChannelKey }
    }

    /// value is an bit_rate_strategy
    pub fn bit_rate_strategy() -> &'static cf::String {
        unsafe { AVEncoderBitRateStrategyKey }
    }

    /// value is an integer from 8 to 32 in cf::Number
    pub fn bit_depth_hint() -> &'static cf::String {
        unsafe { AVEncoderBitDepthHintKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVEncoderAudioQualityKey: &'static cf::String;
        static AVEncoderAudioQualityForVBRKey: &'static cf::String;

        static AVEncoderBitRateKey: &'static cf::String;
        static AVEncoderBitRatePerChannelKey: &'static cf::String;

        static AVEncoderBitRateStrategyKey: &'static cf::String;
        static AVEncoderBitDepthHintKey: &'static cf::String;
    }
}

/// sample rate converter property keys
pub mod sample_rate_converter_keys {
    use crate::cf;
    /// value is an AVSampleRateConverterAlgorithm constant. see below
    pub fn algorithm() -> &'static cf::String {
        unsafe { AVSampleRateConverterAlgorithmKey }
    }
    /// value is an integer from enum av::AudioQuality in cf::Number
    pub fn audio_quality() -> &'static cf::String {
        unsafe { AVSampleRateConverterAudioQualityKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVSampleRateConverterAlgorithmKey: &'static cf::String;
        static AVSampleRateConverterAudioQualityKey: &'static cf::String;
    }
}

/// value is an NSData containing an AudioChannelLayout
pub fn channel_layout_key() -> &'static cf::String {
    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVChannelLayoutKey: &'static cf::String;
    }

    unsafe { AVChannelLayoutKey }
}

// values for encoder_propery_keys::bit_rate_strategy key
pub mod bit_rate_strategy {
    use crate::cf;

    pub fn constant() -> &'static cf::String {
        unsafe { AVAudioBitRateStrategy_Constant }
    }

    pub fn long_term_average() -> &'static cf::String {
        unsafe { AVAudioBitRateStrategy_LongTermAverage }
    }

    pub fn variable_constrained() -> &'static cf::String {
        unsafe { AVAudioBitRateStrategy_VariableConstrained }
    }

    pub fn variable() -> &'static cf::String {
        unsafe { AVAudioBitRateStrategy_Variable }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVAudioBitRateStrategy_Constant: &'static cf::String;
        static AVAudioBitRateStrategy_LongTermAverage: &'static cf::String;
        static AVAudioBitRateStrategy_VariableConstrained: &'static cf::String;
        static AVAudioBitRateStrategy_Variable: &'static cf::String;
    }
}

// values for sample_rate_converter::algorithm
pub mod sample_rate_converer_algorithm {
    use crate::cf;

    pub fn normal() -> &'static cf::String {
        unsafe { AVSampleRateConverterAlgorithm_Normal }
    }

    pub fn mastering() -> &'static cf::String {
        unsafe { AVSampleRateConverterAlgorithm_Mastering }
    }

    pub fn minimum_phase() -> &'static cf::String {
        unsafe { AVSampleRateConverterAlgorithm_MinimumPhase }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {
        static AVSampleRateConverterAlgorithm_Normal: &'static cf::String;
        static AVSampleRateConverterAlgorithm_Mastering: &'static cf::String;
        static AVSampleRateConverterAlgorithm_MinimumPhase: &'static cf::String;
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

    pub fn to_cf_number<'a>(&self) -> cf::Retained<'a, cf::Number> {
        cf::Number::from_i64(self.0 as _)
    }
}
