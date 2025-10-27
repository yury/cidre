pub mod audio;
pub use audio::BalanceFade as AudioBalanceFade;
pub use audio::Buf as AudioBuf;
pub use audio::BufList as AudioBufList;
pub use audio::BufListN as AudioBufListN;
pub use audio::ChannelBitmap as AudioChannelBitmap;
pub use audio::ChannelCoordinateIndex as AudioChannelCoordinateIndex;
pub use audio::ChannelDesc as AudioChannelDesc;
pub use audio::ChannelFlags as AudioChannelFlags;
pub use audio::ChannelLabel as AudioChannelLabel;
pub use audio::ChannelLayout as AudioChannelLayout;
pub use audio::ChannelLayoutTag as AudioChannelLayoutTag;
pub use audio::ClassDesc as AudioClassDesc;
pub use audio::Converter as AudioConverter;
pub use audio::ConverterQuality as AudioConverterQuality;
pub use audio::ConverterRef as AudioConverterRef;
pub use audio::ConverterSampleRateConverterComplexity as AudioConverterSampleRateConverterComplexity;
pub use audio::FileId as AudioFileId;
pub use audio::FormatPropId as AudioFormatPropId;
pub use audio::TimeStamp as AudioTimeStamp;

pub use audio::unit as au;

pub use audio::UnitElement as AudioUnitElement;
pub use audio::UnitManufacturer as AudioUnitManufacturer;
pub use audio::UnitParam as AudioUnitParam;
pub use audio::UnitParamId as AudioUnitParamId;
pub use audio::UnitParamValue as AudioUnitParamValue;

pub use audio::Codec as AudioCodec;
pub use audio::CodecGlobalPropId as AudioCodecGlobalPropId;
pub use audio::CodecRef as AudioCodecRef;

mod extended_audio_file;
pub use extended_audio_file::ExtAudioFile;
pub use extended_audio_file::ExtAudioFilePropId;
pub use extended_audio_file::ExtAudioFileRef;
pub use extended_audio_file::err as ext_audio_file_err;
