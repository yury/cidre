use crate::{cf, define_cf_type};

define_cf_type!(MediaType(cf::String));

impl MediaType {
    #[inline]
    pub fn video() -> &'static Self {
        unsafe { AVMediaTypeVideo }
    }

    #[inline]
    pub fn audio() -> &'static Self {
        unsafe { AVMediaTypeAudio }
    }

    #[inline]
    pub fn text() -> &'static Self {
        unsafe { AVMediaTypeText }
    }

    #[inline]
    pub fn closed_caption() -> &'static Self {
        unsafe { AVMediaTypeClosedCaption }
    }

    #[inline]
    pub fn subtitle() -> &'static Self {
        unsafe { AVMediaTypeSubtitle }
    }

    #[inline]
    pub fn timecode() -> &'static Self {
        unsafe { AVMediaTypeTimecode }
    }

    #[inline]
    pub fn metadata() -> &'static Self {
        unsafe { AVMediaTypeMetadata }
    }

    #[inline]
    pub fn muxed() -> &'static Self {
        unsafe { AVMediaTypeMuxed }
    }

    #[inline]
    #[cfg(not(target_os = "macos"))]
    pub fn metadata_object() -> &'static Self {
        unsafe { AVMediaTypeMetadataObject }
    }

    #[inline]
    pub fn depth_data() -> &'static Self {
        unsafe { AVMediaTypeDepthData }
    }
}

define_cf_type!(VideoRange(cf::String));

impl VideoRange {
    #[inline]
    pub fn sdr() -> &'static Self {
        unsafe { AVVideoRangeSDR }
    }

    #[inline]
    pub fn hlg() -> &'static Self {
        unsafe { AVVideoRangeHLG }
    }

    #[inline]
    pub fn pq() -> &'static Self {
        unsafe { AVVideoRangePQ }
    }
}

define_cf_type!(FileType(cf::String));

impl FileType {
    /// A UTI for the QuickTime movie file format.
    ///
    /// The value of this UTI is @"com.apple.quicktime-movie".
    /// Files are identified with the .mov and .qt extensions.
    pub fn qt() -> &'static FileType {
        unsafe { AVFileTypeQuickTimeMovie }
    }

    /// A UTI for the MPEG-4 file format.
    ///
    /// The value of this UTI is @"public.mpeg-4".
    /// Files are identified with the .mp4 extension.
    #[inline]
    pub fn mp4() -> &'static FileType {
        unsafe { AVFileTypeMPEG4 }
    }

    /// The value of this UTI is @"com.apple.m4v-video".
    /// Files are identified with the .m4v extension.
    pub fn m4v() -> &'static FileType {
        unsafe { AVFileTypeAppleM4V }
    }

    /// The value of this UTI is @"com.apple.m4a-audio".
    /// Files are identified with the .m4a extension.
    #[inline]
    pub fn m4a() -> &'static FileType {
        unsafe { AVFileTypeAppleM4A }
    }

    /// A UTI for the 3GPP file format.
    ///
    /// The value of this UTI is @"public.3gpp".
    /// Files are identified with the .3gp, .3gpp, and .sdv extensions.
    pub fn _3gpp() -> &'static FileType {
        unsafe { AVFileType3GPP }
    }

    /// A UTI for the 3GPP file format.
    ///
    /// The value of this UTI is @"public.3gpp2".
    /// Files are identified with the .3g2, .3gp2 extensions.
    pub fn _3gpp2() -> &'static FileType {
        unsafe { AVFileType3GPP2 }
    }

    /// A UTI for the CoreAudio file format.
    ///
    /// The value of this UTI is @"com.apple.coreaudio-format".
    /// Files are identified with the .caf extension.
    pub fn caf() -> &'static FileType {
        unsafe { AVFileTypeCoreAudioFormat }
    }

    /// A UTI for the WAVE audio file format.
    ///
    /// The value of this UTI is @"com.microsoft.waveform-audio".
    /// Files are identified with the .wav, .wave, and .bwf extensions.
    pub fn wav() -> &'static FileType {
        unsafe { AVFileTypeWAVE }
    }

    /// A UTI for the AIFF audio file format.
    ///
    /// The value of this UTI is @"public.aiff-audio".
    /// Files are identified with the .aif and .aiff extensions.
    pub fn aif() -> &'static FileType {
        unsafe { AVFileTypeAIFF }
    }

    /// A UTI for the AIFC audio file format.
    ///
    /// The value of this UTI is @"public.aifc-audio".
    /// Files are identified with the .aifc and .cdda extensions.
    pub fn aifc() -> &'static FileType {
        unsafe { AVFileTypeAIFC }
    }

    /// A UTI for the adaptive multi-rate audio file format.
    ///
    /// The value of this UTI is @"org.3gpp.adaptive-multi-rate-audio".
    /// Files are identified with the .amr extension.
    pub fn amr() -> &'static FileType {
        unsafe { AVFileTypeAMR }
    }

    /// A UTI for the MPEG layer 3 audio file format.
    ///
    /// The value of this UTI is @"public.mp3".
    /// Files are identified with the .mp3 extension.
    #[inline]
    pub fn mp3() -> &'static FileType {
        unsafe { AVFileTypeMPEGLayer3 }
    }

    /// A UTI for the Sun/NeXT audio file format.
    ///
    /// The value of this UTI is @"public.au-audio".
    /// Files are identified with the .au and .snd extensions.
    pub fn au() -> &'static FileType {
        unsafe { AVFileTypeSunAU }
    }

    /// A UTI for the AC-3 audio file format.
    ///
    /// The value of this UTI is @"public.ac3-audio".
    /// Files are identified with the .ac3 extension.
    pub fn ac3() -> &'static FileType {
        unsafe { AVFileTypeAC3 }
    }

    /// A UTI for the enhanced AC-3 audio file format.
    ///
    /// The value of this UTI is @"public.enhanced-ac3-audio".
    /// Files are identified with the .eac3 extension.
    pub fn eac3() -> &'static FileType {
        unsafe { AVFileTypeEnhancedAC3 }
    }

    /// A UTI for the JPEG (JFIF) format.
    ///
    /// The value of this UTI is @"public.jpeg".
    /// Files are identified with the .jpg or .jpeg extension.
    pub fn jpeg() -> &'static FileType {
        unsafe { AVFileTypeJPEG }
    }

    /// A UTI for the Adobe digital negative file format.
    ///
    /// The value of this UTI is @"com.adobe.raw-image".
    /// Files are identified with the .dng extension.
    pub fn dng() -> &'static FileType {
        unsafe { AVFileTypeDNG }
    }

    /// A UTI for the high efficiency image file format containing HEVC compressed images.
    ///
    /// The value of this UTI is @"public.heic".
    /// Files are identified with the .heic extension.
    #[inline]
    pub fn heic() -> &'static FileType {
        unsafe { AVFileTypeHEIC }
    }

    /// A UTI for the high efficiency image file format containing H.264 compressed images.
    ///
    /// The value of this UTI is @"public.avci".
    /// Files are identified with the .avci extension.
    pub fn avci() -> &'static FileType {
        unsafe { AVFileTypeAVCI }
    }
    /// A UTI for the high efficiency image file format containing images compressed with any codec.
    ///
    /// The value of this UTI is @"public.heif".
    /// Files are identified with the .heif extension.
    pub fn heif() -> &'static FileType {
        unsafe { AVFileTypeHEIF }
    }

    /// A UTI for the tagged image file format.
    ///
    /// The value of this UTI is @"public.tiff".
    /// Files are identified with the .tiff or .tif extension.
    pub fn tiff() -> &'static FileType {
        unsafe { AVFileTypeTIFF }
    }

    /// A UTI for the Apple iTT caption file format
    ///
    /// The value of this UTI is @"com.apple.itunes-timed-text".
    /// Files are identified with the .itt extension.
    pub fn apple_itt() -> &'static FileType {
        unsafe { AVFileTypeAppleiTT }
    }
    /// A UTI for the Scenarist closed caption file format
    ///
    /// The value of this UTI is @"com.scenarist.closed-caption".
    /// Files are identified with the .scc extension.
    pub fn scc() -> &'static FileType {
        unsafe { AVFileTypeSCC }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
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

    static AVMediaTypeVideo: &'static MediaType;
    static AVMediaTypeAudio: &'static MediaType;
    static AVMediaTypeText: &'static MediaType;
    static AVMediaTypeClosedCaption: &'static MediaType;
    static AVMediaTypeSubtitle: &'static MediaType;
    static AVMediaTypeTimecode: &'static MediaType;
    static AVMediaTypeMetadata: &'static MediaType;
    static AVMediaTypeMuxed: &'static MediaType;

    #[cfg(not(target_os = "macos"))]
    static AVMediaTypeMetadataObject: &'static MediaType;
    static AVMediaTypeDepthData: &'static MediaType;

    static AVVideoRangeSDR: &'static VideoRange;
    static AVVideoRangeHLG: &'static VideoRange;
    static AVVideoRangePQ: &'static VideoRange;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type() {
        let _s = MediaType::audio();
    }
}
