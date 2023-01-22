use crate::{define_obj_type, ns};

pub mod keys {
    use crate::ns;

    pub fn codec() -> &'static ns::String {
        unsafe { AVVideoCodecKey }
    }

    pub fn pixel_aspect_ratio() -> &'static ns::String {
        unsafe { AVVideoPixelAspectRatioKey }
    }

    pub fn clean_aperture() -> &'static ns::String {
        unsafe { AVVideoCleanApertureKey }
    }

    pub fn scaling_mode() -> &'static ns::String {
        unsafe { AVVideoScalingModeKey }
    }

    pub fn color_properites() -> &'static ns::String {
        unsafe { AVVideoColorPropertiesKey }
    }

    pub fn allow_wide_color() -> &'static ns::String {
        unsafe { AVVideoAllowWideColorKey }
    }

    pub fn compression_properties() -> &'static ns::String {
        unsafe { AVVideoCompressionPropertiesKey }
    }

    pub fn decompression_properties() -> &'static ns::String {
        unsafe { AVVideoDecompressionPropertiesKey }
    }

    pub fn encoder_specification() -> &'static ns::String {
        unsafe { AVVideoEncoderSpecificationKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
    pub fn width() -> &'static ns::String {
        unsafe { AVVideoWidthKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
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

define_obj_type!(CodecType(ns::String));

impl CodecType {
    /// The HEVC video codec.
    pub fn hevc() -> &'static CodecType {
        unsafe { AVVideoCodecTypeHEVC }
    }

    /// The H.264 video codec.
    pub fn h264() -> &'static CodecType {
        unsafe { AVVideoCodecTypeH264 }
    }

    /// The JPEG video codec.
    pub fn jpeg() -> &'static CodecType {
        unsafe { AVVideoCodecTypeJPEG }
    }

    /// The Apple ProRes 4444 video codec.
    pub fn apple_pro_res_4444() -> &'static CodecType {
        unsafe { AVVideoCodecTypeAppleProRes4444 }
    }

    /// The Apple ProRes 422 video codec.
    pub fn apple_pro_res_422() -> &'static CodecType {
        unsafe { AVVideoCodecTypeAppleProRes422 }
    }

    /// The Apple ProRes 422 HQ video codec.
    pub fn apple_pro_res_422_hq() -> &'static CodecType {
        unsafe { AVVideoCodecTypeAppleProRes422HQ }
    }

    /// The Apple ProRes 422 LT video codec.
    pub fn apple_pro_res_422_lt() -> &'static CodecType {
        unsafe { AVVideoCodecTypeAppleProRes422LT }
    }

    /// The Apple ProRes 422 Proxy video codec.
    pub fn apple_pro_res_422_proxy() -> &'static CodecType {
        unsafe { AVVideoCodecTypeAppleProRes422Proxy }
    }

    /// The HEVC video codec that supports an alpha channel.
    ///
    /// MPORTANT NOTE: this constant is used to select the appropriate encoder,
    /// but is NOT used on the encoded content, which is backwards compatible
    /// and hence uses 'hvc1' as its codec type.
    pub fn hevc_with_alpha() -> &'static CodecType {
        unsafe { AVVideoCodecTypeHEVCWithAlpha }
    }
}

extern "C" {
    static AVVideoCodecTypeHEVC: &'static CodecType;
    static AVVideoCodecTypeH264: &'static CodecType;
    static AVVideoCodecTypeJPEG: &'static CodecType;
    static AVVideoCodecTypeAppleProRes4444: &'static CodecType;
    static AVVideoCodecTypeAppleProRes422: &'static CodecType;
    static AVVideoCodecTypeAppleProRes422HQ: &'static CodecType;
    static AVVideoCodecTypeAppleProRes422LT: &'static CodecType;
    static AVVideoCodecTypeAppleProRes422Proxy: &'static CodecType;
    static AVVideoCodecTypeHEVCWithAlpha: &'static CodecType;
}
