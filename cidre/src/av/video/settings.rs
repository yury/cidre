use crate::{define_obj_type, ns};

pub mod keys {
    use crate::ns;

    #[doc(alias = "AVVideoCodecKey")]
    pub fn codec() -> &'static ns::String {
        unsafe { AVVideoCodecKey }
    }

    #[doc(alias = "AVVideoPixelAspectRatioKey")]
    pub fn pixel_aspect_ratio() -> &'static ns::String {
        unsafe { AVVideoPixelAspectRatioKey }
    }

    #[doc(alias = "AVVideoCleanApertureKey")]
    pub fn clean_aperture() -> &'static ns::String {
        unsafe { AVVideoCleanApertureKey }
    }

    #[doc(alias = "AVVideoScalingModeKey")]
    pub fn scaling_mode() -> &'static ns::String {
        unsafe { AVVideoScalingModeKey }
    }

    #[doc(alias = "AVVideoColorPropertiesKey")]
    pub fn color_props() -> &'static ns::String {
        unsafe { AVVideoColorPropertiesKey }
    }

    #[doc(alias = "AVVideoAllowWideColorKey")]
    pub fn allow_wide_color() -> &'static ns::String {
        unsafe { AVVideoAllowWideColorKey }
    }

    #[doc(alias = "AVVideoCompressionPropertiesKey")]
    pub fn compression_props() -> &'static ns::String {
        unsafe { AVVideoCompressionPropertiesKey }
    }

    #[doc(alias = "AVVideoDecompressionPropertiesKey")]
    pub fn decompression_props() -> &'static ns::String {
        unsafe { AVVideoDecompressionPropertiesKey }
    }

    #[doc(alias = "AVVideoEncoderSpecificationKey")]
    pub fn encoder_spec() -> &'static ns::String {
        unsafe { AVVideoEncoderSpecificationKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
    #[doc(alias = "AVVideoWidthKey")]
    pub fn width() -> &'static ns::String {
        unsafe { AVVideoWidthKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
    #[doc(alias = "AVVideoHeightKey")]
    pub fn height() -> &'static ns::String {
        unsafe { AVVideoHeightKey }
    }

    extern "C" {
        static AVVideoWidthKey: &'static ns::String;
        static AVVideoHeightKey: &'static ns::String;

        static AVVideoCodecKey: &'static ns::String;
        static AVVideoPixelAspectRatioKey: &'static ns::String;
        static AVVideoCleanApertureKey: &'static ns::String;
        static AVVideoScalingModeKey: &'static ns::String;
        static AVVideoColorPropertiesKey: &'static ns::String;
        static AVVideoAllowWideColorKey: &'static ns::String;
        static AVVideoCompressionPropertiesKey: &'static ns::String;
        static AVVideoDecompressionPropertiesKey: &'static ns::String;
        static AVVideoEncoderSpecificationKey: &'static ns::String;
    }
}

define_obj_type!(pub Codec(ns::String));

impl Codec {
    /// The HEVC video codec.
    #[doc(alias = "AVVideoCodecTypeHEVC")]
    pub fn hevc() -> &'static Codec {
        unsafe { AVVideoCodecTypeHEVC }
    }

    /// The H.264 video codec.
    #[doc(alias = "AVVideoCodecTypeH264")]
    pub fn h264() -> &'static Codec {
        unsafe { AVVideoCodecTypeH264 }
    }

    /// The JPEG video codec.
    #[doc(alias = "AVVideoCodecTypeJPEG")]
    pub fn jpeg() -> &'static Codec {
        unsafe { AVVideoCodecTypeJPEG }
    }

    /// The Apple ProRes 4444 video codec.
    #[doc(alias = "AVVideoCodecTypeAppleProRes4444")]
    pub fn apple_pro_res_4444() -> &'static Codec {
        unsafe { AVVideoCodecTypeAppleProRes4444 }
    }

    /// The Apple ProRes 422 video codec.
    #[doc(alias = "AVVideoCodecTypeAppleProRes422")]
    pub fn apple_pro_res_422() -> &'static Codec {
        unsafe { AVVideoCodecTypeAppleProRes422 }
    }

    /// The Apple ProRes 422 HQ video codec.
    #[doc(alias = "AVVideoCodecTypeAppleProRes422HQ")]
    pub fn apple_pro_res_422_hq() -> &'static Codec {
        unsafe { AVVideoCodecTypeAppleProRes422HQ }
    }

    /// The Apple ProRes 422 LT video codec.
    #[doc(alias = "AVVideoCodecTypeAppleProRes422LT")]
    pub fn apple_pro_res_422_lt() -> &'static Codec {
        unsafe { AVVideoCodecTypeAppleProRes422LT }
    }

    /// The Apple ProRes 422 Proxy video codec.
    #[doc(alias = "AVVideoCodecTypeAppleProRes422Proxy")]
    pub fn apple_pro_res_422_proxy() -> &'static Codec {
        unsafe { AVVideoCodecTypeAppleProRes422Proxy }
    }

    /// The HEVC video codec that supports an alpha channel.
    ///
    /// MPORTANT NOTE: this constant is used to select the appropriate encoder,
    /// but is NOT used on the encoded content, which is backwards compatible
    /// and hence uses 'hvc1' as its codec type.
    #[doc(alias = "AVVideoCodecTypeHEVCWithAlpha")]
    pub fn hevc_with_alpha() -> &'static Codec {
        unsafe { AVVideoCodecTypeHEVCWithAlpha }
    }
}

extern "C" {
    static AVVideoCodecTypeHEVC: &'static Codec;
    static AVVideoCodecTypeH264: &'static Codec;
    static AVVideoCodecTypeJPEG: &'static Codec;
    static AVVideoCodecTypeAppleProRes4444: &'static Codec;
    static AVVideoCodecTypeAppleProRes422: &'static Codec;
    static AVVideoCodecTypeAppleProRes422HQ: &'static Codec;
    static AVVideoCodecTypeAppleProRes422LT: &'static Codec;
    static AVVideoCodecTypeAppleProRes422Proxy: &'static Codec;
    static AVVideoCodecTypeHEVCWithAlpha: &'static Codec;
}
