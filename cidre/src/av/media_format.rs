use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "AVMediaType")]
    pub MediaType(ns::String)
);

/// Media types
impl MediaType {
    /// The media contains video.
    #[doc(alias = "AVMediaTypeVideo")]
    #[inline]
    pub fn video() -> &'static Self {
        unsafe { AVMediaTypeVideo }
    }

    /// The media contains audio media.
    #[doc(alias = "AVMediaTypeAudio")]
    #[inline]
    pub fn audio() -> &'static Self {
        unsafe { AVMediaTypeAudio }
    }

    /// The media contains text.
    #[doc(alias = "AVMediaTypeText")]
    #[inline]
    pub fn text() -> &'static Self {
        unsafe { AVMediaTypeText }
    }

    /// The media contains closed-caption content.
    #[doc(alias = "AVMediaTypeClosedCaption")]
    #[inline]
    pub fn closed_caption() -> &'static Self {
        unsafe { AVMediaTypeClosedCaption }
    }

    /// The media contains subtitles.
    #[doc(alias = "AVMediaTypeSubtitle")]
    #[inline]
    pub fn subtitle() -> &'static Self {
        unsafe { AVMediaTypeSubtitle }
    }

    /// The media contains a time code.
    #[doc(alias = "AVMediaTypeTimecode")]
    #[inline]
    pub fn timecode() -> &'static Self {
        unsafe { AVMediaTypeTimecode }
    }

    /// The media contains metadata.
    #[doc(alias = "AVMediaTypeMetadata")]
    #[inline]
    pub fn metadata() -> &'static Self {
        unsafe { AVMediaTypeMetadata }
    }

    /// The media contains muxed media.
    #[doc(alias = "AVMediaTypeMuxed")]
    #[inline]
    pub fn muxed() -> &'static Self {
        unsafe { AVMediaTypeMuxed }
    }

    /// The media contains haptic content.
    #[doc(alias = "AVMediaTypeHaptic")]
    #[inline]
    pub fn haptic() -> &'static Self {
        unsafe { AVMediaTypeHaptic }
    }

    #[inline]
    #[cfg(not(target_os = "macos"))]
    pub fn metadata_object() -> &'static Self {
        unsafe { AVMediaTypeMetadataObject }
    }

    /// The media contains depth data.
    #[doc(alias = "AVMediaTypeDepthData")]
    #[inline]
    pub fn depth_data() -> &'static Self {
        unsafe { AVMediaTypeDepthData }
    }
}

define_obj_type!(
    #[doc(alias = "AVVideoRange")]
    pub VideoRange(ns::String)
);

/// Constants that describe a video variant’s dynamic range.
impl VideoRange {
    /// Indicates standard-dynamic-range (SDR) video.
    #[doc(alias = "AVVideoRangeSDR")]
    #[inline]
    pub fn sdr() -> &'static Self {
        unsafe { AVVideoRangeSDR }
    }

    /// Indicates Hybrid-Log Gamma (HLG) high-dynamic-range video.
    #[doc(alias = "AVVideoRangeHLG")]
    #[inline]
    pub fn hlg() -> &'static Self {
        unsafe { AVVideoRangeHLG }
    }

    /// Indicates Perceptual Quantizer (PQ) high-dynamic-range video.
    #[doc(alias = "AVVideoRangePQ")]
    #[inline]
    pub fn pq() -> &'static Self {
        unsafe { AVVideoRangePQ }
    }
}
define_obj_type!(
    #[doc(alias = "AVMediaCharacteristic")]
    pub MediaCharacteristic(ns::String)
);

impl MediaCharacteristic {
    /// A media characteristic that indicates that a track or media selection option
    /// includes visual content.
    #[doc(alias = "AVMediaCharacteristicVisual")]
    #[inline]
    pub fn visial() -> &'static Self {
        unsafe { AVMediaCharacteristicVisual }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// includes audible content.
    #[doc(alias = "AVMediaCharacteristicAudible")]
    #[inline]
    pub fn audible() -> &'static Self {
        unsafe { AVMediaCharacteristicAudible }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// includes legible content.
    #[doc(alias = "AVMediaCharacteristicLegible")]
    #[inline]
    pub fn legible() -> &'static Self {
        unsafe { AVMediaCharacteristicLegible }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// includes content that's frame-based
    #[doc(alias = "AVMediaCharacteristicFrameBased")]
    #[inline]
    pub fn frame_based() -> &'static Self {
        unsafe { AVMediaCharacteristicFrameBased }
    }

    /// A media characteristic that indicates that a track uses a wide gamut color space
    /// and therefore may make use of colors that cannot be accurately represented otherwise.
    #[doc(alias = "AVMediaCharacteristicUsesWideGamutColorSpace")]
    #[inline]
    pub fn uses_wide_gamut_color_space() -> &'static Self {
        unsafe { AVMediaCharacteristicUsesWideGamutColorSpace }
    }

    /// A media characteristic that indicates that a track contains HDR video.
    #[doc(alias = "AVMediaCharacteristicContainsHDRVideo")]
    #[inline]
    pub fn contains_hdr_video() -> &'static Self {
        unsafe { AVMediaCharacteristicContainsHDRVideo }
    }

    /// A media characteristic that indicates that a track contains an alpha channel.
    #[doc(alias = "AVMediaCharacteristicContainsAlphaChannel")]
    #[inline]
    pub fn contains_alpha_channel() -> &'static Self {
        unsafe { AVMediaCharacteristicContainsAlphaChannel }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// includes content that's marked by the content author as intrinsic to the presentation of the asset
    #[doc(alias = "AVMediaCharacteristicIsMainProgramContent")]
    #[inline]
    pub fn is_main_program_content() -> &'static Self {
        unsafe { AVMediaCharacteristicIsMainProgramContent }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// includes content that's marked by the content author as auxiliary to the presentation of the asset.
    #[doc(alias = "AVMediaCharacteristicIsAuxiliaryContent")]
    #[inline]
    pub fn is_auxiliary() -> &'static Self {
        unsafe { AVMediaCharacteristicIsAuxiliaryContent }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// includes content that's marked by the content author as original to the principal
    /// production of the media, as opposed to supplementary or derivative content created by
    /// means of language translation or by other means.
    #[doc(alias = "AVMediaCharacteristicIsOriginalContent")]
    #[inline]
    pub fn is_original_content() -> &'static Self {
        unsafe { AVMediaCharacteristicIsOriginalContent }
    }

    /// A media characteristic that indicates that a track or media selection option presents only
    /// forced subtitles.
    #[doc(alias = "AVMediaCharacteristicContainsOnlyForcedSubtitles")]
    #[inline]
    pub fn contains_only_forces_subtitles() -> &'static Self {
        unsafe { AVMediaCharacteristicContainsOnlyForcedSubtitles }
    }

    #[doc(alias = "AVMediaCharacteristicTranscribesSpokenDialogForAccessibility")]
    #[inline]
    pub fn transcribes_spoken_dialog_for_accessibility() -> &'static Self {
        unsafe { AVMediaCharacteristicTranscribesSpokenDialogForAccessibility }
    }

    #[doc(alias = "AVMediaCharacteristicDescribesMusicAndSoundForAccessibility")]
    #[inline]
    pub fn transcribes_music_and_sound_for_accessibility() -> &'static Self {
        unsafe { AVMediaCharacteristicDescribesMusicAndSoundForAccessibility }
    }

    #[doc(alias = "AVMediaCharacteristicEnhancesSpeechIntelligibility")]
    #[inline]
    pub fn enhances_speech_intelligibility() -> &'static Self {
        unsafe { AVMediaCharacteristicEnhancesSpeechIntelligibility }
    }

    #[doc(alias = "AVMediaCharacteristicEasyToRead")]
    #[inline]
    pub fn easy_to_read() -> &'static Self {
        unsafe { AVMediaCharacteristicEasyToRead }
    }

    #[doc(alias = "AVMediaCharacteristicDescribesVideoForAccessibility")]
    #[inline]
    pub fn describes_video_for_accessibility() -> &'static Self {
        unsafe { AVMediaCharacteristicDescribesVideoForAccessibility }
    }

    /// A media characteristic that indicates that a track or media selection option
    /// contains a language or dialect translation of originally or previously produced
    /// content, intended to be used as a substitute for that content by users who prefer
    /// its designated language.
    #[doc(alias = "AVMediaCharacteristicLanguageTranslation")]
    #[inline]
    pub fn language_translation() -> &'static Self {
        unsafe { AVMediaCharacteristicLanguageTranslation }
    }

    #[doc(alias = "AVMediaCharacteristicDubbedTranslation")]
    #[inline]
    pub fn dubbed_translation() -> &'static Self {
        unsafe { AVMediaCharacteristicDubbedTranslation }
    }

    #[doc(alias = "AVMediaCharacteristicVoiceOverTranslation")]
    #[inline]
    pub fn voice_over_translation() -> &'static Self {
        unsafe { AVMediaCharacteristicVoiceOverTranslation }
    }

    #[doc(alias = "AVMediaCharacteristicTactileMinimal")]
    #[inline]
    pub fn tactile_minimal() -> &'static Self {
        unsafe { AVMediaCharacteristicTactileMinimal }
    }

    #[doc(alias = "AVMediaCharacteristicContainsStereoMultiviewVideo")]
    #[inline]
    pub fn contains_stereo_multiview_video() -> &'static Self {
        unsafe { AVMediaCharacteristicContainsStereoMultiviewVideo }
    }

    #[doc(alias = "AVMediaCharacteristicCarriesVideoStereoMetadata")]
    #[inline]
    pub fn carries_video_stereo_metadata() -> &'static Self {
        unsafe { AVMediaCharacteristicCarriesVideoStereoMetadata }
    }

    #[doc(alias = "AVMediaCharacteristicIndicatesHorizontalFieldOfView")]
    #[inline]
    pub fn indicates_horizontal_field_of_view() -> &'static Self {
        unsafe { AVMediaCharacteristicIndicatesHorizontalFieldOfView }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVMediaCharacteristicVisual: &'static MediaCharacteristic;
    static AVMediaCharacteristicAudible: &'static MediaCharacteristic;
    static AVMediaCharacteristicLegible: &'static MediaCharacteristic;
    static AVMediaCharacteristicFrameBased: &'static MediaCharacteristic;
    static AVMediaCharacteristicUsesWideGamutColorSpace: &'static MediaCharacteristic;
    static AVMediaCharacteristicContainsHDRVideo: &'static MediaCharacteristic;
    static AVMediaCharacteristicContainsAlphaChannel: &'static MediaCharacteristic;
    static AVMediaCharacteristicIsMainProgramContent: &'static MediaCharacteristic;
    static AVMediaCharacteristicIsAuxiliaryContent: &'static MediaCharacteristic;
    static AVMediaCharacteristicIsOriginalContent: &'static MediaCharacteristic;
    static AVMediaCharacteristicContainsOnlyForcedSubtitles: &'static MediaCharacteristic;
    static AVMediaCharacteristicTranscribesSpokenDialogForAccessibility:
        &'static MediaCharacteristic;
    static AVMediaCharacteristicDescribesMusicAndSoundForAccessibility:
        &'static MediaCharacteristic;
    static AVMediaCharacteristicEnhancesSpeechIntelligibility: &'static MediaCharacteristic;
    static AVMediaCharacteristicEasyToRead: &'static MediaCharacteristic;
    static AVMediaCharacteristicDescribesVideoForAccessibility: &'static MediaCharacteristic;
    static AVMediaCharacteristicLanguageTranslation: &'static MediaCharacteristic;
    static AVMediaCharacteristicDubbedTranslation: &'static MediaCharacteristic;
    static AVMediaCharacteristicVoiceOverTranslation: &'static MediaCharacteristic;
    static AVMediaCharacteristicTactileMinimal: &'static MediaCharacteristic;
    static AVMediaCharacteristicContainsStereoMultiviewVideo: &'static MediaCharacteristic;
    static AVMediaCharacteristicCarriesVideoStereoMetadata: &'static MediaCharacteristic;
    static AVMediaCharacteristicIndicatesHorizontalFieldOfView: &'static MediaCharacteristic;
}

define_obj_type!(
    #[doc(alias = "AVFileType")]
    pub FileType(ns::String)
);

impl FileType {
    /// A UTI for the QuickTime movie file format.
    ///
    /// The value of this UTI is @"com.apple.quicktime-movie".
    /// Files are identified with the .mov and .qt extensions.
    #[doc(alias = "AVFileTypeQuickTimeMovie")]
    pub fn qt() -> &'static Self {
        unsafe { AVFileTypeQuickTimeMovie }
    }

    /// A UTI for the MPEG-4 file format.
    ///
    /// The value of this UTI is @"public.mpeg-4".
    /// Files are identified with the .mp4 extension.
    #[doc(alias = "AVFileTypeMPEG4")]
    #[inline]
    pub fn mp4() -> &'static Self {
        unsafe { AVFileTypeMPEG4 }
    }

    /// The value of this UTI is @"com.apple.m4v-video".
    /// Files are identified with the .m4v extension.
    #[doc(alias = "AVFileTypeAppleM4V")]
    #[inline]
    pub fn m4v() -> &'static Self {
        unsafe { AVFileTypeAppleM4V }
    }

    /// The value of this UTI is @"com.apple.m4a-audio".
    /// Files are identified with the .m4a extension.
    #[doc(alias = "AVFileTypeAppleM4A")]
    #[inline]
    pub fn m4a() -> &'static Self {
        unsafe { AVFileTypeAppleM4A }
    }

    /// A UTI for the 3GPP file format.
    ///
    /// The value of this UTI is @"public.3gpp".
    /// Files are identified with the .3gp, .3gpp, and .sdv extensions.
    #[doc(alias = "AVFileType3GPP")]
    pub fn _3gpp() -> &'static Self {
        unsafe { AVFileType3GPP }
    }

    /// A UTI for the 3GPP file format.
    ///
    /// The value of this UTI is @"public.3gpp2".
    /// Files are identified with the .3g2, .3gp2 extensions.
    #[doc(alias = "AVFileType3GPP2")]
    pub fn _3gpp2() -> &'static Self {
        unsafe { AVFileType3GPP2 }
    }

    /// A UTI for the CoreAudio file format.
    ///
    /// The value of this UTI is @"com.apple.coreaudio-format".
    /// Files are identified with the .caf extension.
    #[doc(alias = "AVFileTypeCoreAudioFormat")]
    pub fn caf() -> &'static Self {
        unsafe { AVFileTypeCoreAudioFormat }
    }

    /// A UTI for the WAVE audio file format.
    ///
    /// The value of this UTI is @"com.microsoft.waveform-audio".
    /// Files are identified with the .wav, .wave, and .bwf extensions.
    #[doc(alias = "AVFileTypeWAVE")]
    pub fn wav() -> &'static Self {
        unsafe { AVFileTypeWAVE }
    }

    /// A UTI for the AIFF audio file format.
    ///
    /// The value of this UTI is @"public.aiff-audio".
    /// Files are identified with the .aif and .aiff extensions.
    #[doc(alias = "AVFileTypeAIFF")]
    pub fn aif() -> &'static Self {
        unsafe { AVFileTypeAIFF }
    }

    /// A UTI for the AIFC audio file format.
    ///
    /// The value of this UTI is @"public.aifc-audio".
    /// Files are identified with the .aifc and .cdda extensions.
    #[doc(alias = "AVFileTypeAIFC")]
    pub fn aifc() -> &'static Self {
        unsafe { AVFileTypeAIFC }
    }

    /// A UTI for the adaptive multi-rate audio file format.
    ///
    /// The value of this UTI is @"org.3gpp.adaptive-multi-rate-audio".
    /// Files are identified with the .amr extension.
    #[doc(alias = "AVFileTypeAMR")]
    pub fn amr() -> &'static Self {
        unsafe { AVFileTypeAMR }
    }

    /// A UTI for the MPEG layer 3 audio file format.
    ///
    /// The value of this UTI is @"public.mp3".
    /// Files are identified with the .mp3 extension.
    #[doc(alias = "AVFileTypeMPEGLayer3")]
    #[inline]
    pub fn mp3() -> &'static Self {
        unsafe { AVFileTypeMPEGLayer3 }
    }

    /// A UTI for the Sun/NeXT audio file format.
    ///
    /// The value of this UTI is @"public.au-audio".
    /// Files are identified with the .au and .snd extensions.
    #[doc(alias = "AVFileTypeSunAU")]
    pub fn sun_au() -> &'static Self {
        unsafe { AVFileTypeSunAU }
    }

    /// A UTI for the AC-3 audio file format.
    ///
    /// The value of this UTI is @"public.ac3-audio".
    /// Files are identified with the .ac3 extension.
    #[doc(alias = "AVFileTypeAC3")]
    pub fn ac3() -> &'static Self {
        unsafe { AVFileTypeAC3 }
    }

    /// A UTI for the enhanced AC-3 audio file format.
    ///
    /// The value of this UTI is @"public.enhanced-ac3-audio".
    /// Files are identified with the .eac3 extension.
    #[doc(alias = "AVFileTypeEnhancedAC3")]
    pub fn eac3() -> &'static Self {
        unsafe { AVFileTypeEnhancedAC3 }
    }

    /// A UTI for the JPEG (JFIF) format.
    ///
    /// The value of this UTI is @"public.jpeg".
    /// Files are identified with the .jpg or .jpeg extension.
    #[doc(alias = "AVFileTypeJPEG")]
    pub fn jpeg() -> &'static Self {
        unsafe { AVFileTypeJPEG }
    }

    /// A UTI for the Adobe digital negative file format.
    ///
    /// The value of this UTI is @"com.adobe.raw-image".
    /// Files are identified with the .dng extension.
    #[doc(alias = "AVFileTypeDNG")]
    pub fn dng() -> &'static Self {
        unsafe { AVFileTypeDNG }
    }

    /// A UTI for the high efficiency image file format containing HEVC compressed images.
    ///
    /// The value of this UTI is @"public.heic".
    /// Files are identified with the .heic extension.
    #[doc(alias = "AVFileTypeHEIC")]
    #[inline]
    pub fn heic() -> &'static Self {
        unsafe { AVFileTypeHEIC }
    }

    /// A UTI for the high efficiency image file format containing H.264 compressed images.
    ///
    /// The value of this UTI is @"public.avci".
    /// Files are identified with the .avci extension.
    #[doc(alias = "AVFileTypeAVCI")]
    pub fn avci() -> &'static Self {
        unsafe { AVFileTypeAVCI }
    }
    /// A UTI for the high efficiency image file format containing images compressed
    /// with any codec.
    ///
    /// The value of this UTI is @"public.heif".
    /// Files are identified with the .heif extension.
    #[doc(alias = "AVFileTypeHEIF")]
    pub fn heif() -> &'static Self {
        unsafe { AVFileTypeHEIF }
    }

    /// A UTI for the tagged image file format.
    ///
    /// The value of this UTI is @"public.tiff".
    /// Files are identified with the .tiff or .tif extension.
    #[doc(alias = "AVFileTypeTIFF")]
    pub fn tiff() -> &'static Self {
        unsafe { AVFileTypeTIFF }
    }

    /// A UTI for the Apple iTT caption file format
    ///
    /// The value of this UTI is @"com.apple.itunes-timed-text".
    /// Files are identified with the .itt extension.
    #[doc(alias = "VFileTypeAppleiTT")]
    pub fn apple_itt() -> &'static Self {
        unsafe { AVFileTypeAppleiTT }
    }
    /// A UTI for the Scenarist closed caption file format
    ///
    /// The value of this UTI is @"com.scenarist.closed-caption".
    /// Files are identified with the .scc extension.
    #[doc(alias = "AVFileTypeSCC")]
    pub fn scc() -> &'static Self {
        unsafe { AVFileTypeSCC }
    }

    /// A UTI for the Apple Haptics Audio Pattern file format.
    ///
    /// The value of this UTI is @"public.haptics-content".
    /// Files are identified with the .ahap extension.
    #[doc(alias = "AVFileTypeAHAP")]
    pub fn ahap() -> &'static Self {
        unsafe { AVFileTypeAHAP }
    }
}

define_obj_type!(
    #[doc(alias = "AVFileTypeProfile")]
    pub FileTypeProfile(ns::String)
);

/// File type profiles
impl FileTypeProfile {
    /// Apple HTTP Live Streaming profile
    ///
    /// The profile that is suitable for Apple HTTP Live Streaming.
    #[doc(alias = "AVFileTypeProfileMPEG4AppleHLS")]
    pub fn mpeg4_apple_hls() -> &'static Self {
        unsafe { AVFileTypeProfileMPEG4AppleHLS }
    }

    /// CMAF compliant profile
    ///
    /// The profile that is compliance with CMAF format.
    #[doc(alias = "AVFileTypeProfileMPEG4CMAFCompliant")]
    pub fn mpeg4_cmaf_compliant() -> &'static Self {
        unsafe { AVFileTypeProfileMPEG4CMAFCompliant }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVFileTypeQuickTimeMovie: &'static FileType;
    static AVFileTypeMPEG4: &'static FileType;
    static AVFileTypeAppleM4V: &'static FileType;
    static AVFileTypeAppleM4A: &'static FileType;
    static AVFileType3GPP: &'static FileType;
    static AVFileType3GPP2: &'static FileType;
    static AVFileTypeCoreAudioFormat: &'static FileType;
    static AVFileTypeWAVE: &'static FileType;
    static AVFileTypeAIFF: &'static FileType;
    static AVFileTypeAIFC: &'static FileType;
    static AVFileTypeAMR: &'static FileType;
    static AVFileTypeMPEGLayer3: &'static FileType;
    static AVFileTypeSunAU: &'static FileType;
    static AVFileTypeAC3: &'static FileType;
    static AVFileTypeEnhancedAC3: &'static FileType;
    static AVFileTypeJPEG: &'static FileType;
    static AVFileTypeDNG: &'static FileType;
    static AVFileTypeHEIC: &'static FileType;
    static AVFileTypeAVCI: &'static FileType;
    static AVFileTypeHEIF: &'static FileType;
    static AVFileTypeTIFF: &'static FileType;
    static AVFileTypeAppleiTT: &'static FileType;
    static AVFileTypeSCC: &'static FileType;
    static AVFileTypeAHAP: &'static FileType;

    static AVMediaTypeVideo: &'static MediaType;
    static AVMediaTypeAudio: &'static MediaType;
    static AVMediaTypeText: &'static MediaType;
    static AVMediaTypeClosedCaption: &'static MediaType;
    static AVMediaTypeSubtitle: &'static MediaType;
    static AVMediaTypeTimecode: &'static MediaType;
    static AVMediaTypeMetadata: &'static MediaType;
    static AVMediaTypeMuxed: &'static MediaType;
    static AVMediaTypeHaptic: &'static MediaType;

    #[cfg(not(target_os = "macos"))]
    static AVMediaTypeMetadataObject: &'static MediaType;
    static AVMediaTypeDepthData: &'static MediaType;

    static AVVideoRangeSDR: &'static VideoRange;
    static AVVideoRangeHLG: &'static VideoRange;
    static AVVideoRangePQ: &'static VideoRange;

    static AVFileTypeProfileMPEG4AppleHLS: &'static FileTypeProfile;
    static AVFileTypeProfileMPEG4CMAFCompliant: &'static FileTypeProfile;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_type() {
        let _s = MediaType::audio();
    }
}
