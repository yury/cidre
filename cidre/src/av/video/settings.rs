use crate::{api, define_obj_type, ns};

pub mod keys {
    use crate::{api, ns};

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
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, visionos = 1.0)]
    pub fn width() -> &'static ns::String {
        unsafe { AVVideoWidthKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
    #[doc(alias = "AVVideoHeightKey")]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, visionos = 1.0)]
    pub fn height() -> &'static ns::String {
        unsafe { AVVideoHeightKey }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
        #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, visionos = 1.0)]
        static AVVideoWidthKey: &'static ns::String;
        #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, visionos = 1.0)]
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

define_obj_type!(
    #[doc(alias = "AVVideoCodecType")]
    pub Codec(ns::String)
);

impl Codec {
    /// The HEVC video codec.
    #[doc(alias = "AVVideoCodecTypeHEVC")]
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)]
    pub fn hevc() -> &'static Self {
        unsafe { AVVideoCodecTypeHEVC }
    }

    /// The H.264 video codec.
    #[doc(alias = "AVVideoCodecTypeH264")]
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)]
    pub fn h264() -> &'static Self {
        unsafe { AVVideoCodecTypeH264 }
    }

    /// The JPEG video codec.
    #[doc(alias = "AVVideoCodecTypeJPEG")]
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)]
    pub fn jpeg() -> &'static Self {
        unsafe { AVVideoCodecTypeJPEG }
    }

    /// The Apple ProRes 4444 video codec.
    /// 'ap4h'
    #[doc(alias = "AVVideoCodecTypeAppleProRes4444")]
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    pub fn apple_pro_res_4444() -> &'static Self {
        unsafe { AVVideoCodecTypeAppleProRes4444 }
    }

    /// The Apple ProRes 4444 XQ video codec.
    /// 'ap4x'
    #[doc(alias = "AVVideoCodecTypeAppleProRes4444XQ")]
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0)]
    pub fn apple_pro_res_4444xq() -> &'static Self {
        unsafe { AVVideoCodecTypeAppleProRes4444XQ }
    }

    /// The Apple ProRes 422 video codec.
    /// 'apcn'
    #[doc(alias = "AVVideoCodecTypeAppleProRes422")]
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    pub fn apple_pro_res_422() -> &'static Self {
        unsafe { AVVideoCodecTypeAppleProRes422 }
    }

    /// The Apple ProRes 422 HQ video codec.
    /// 'apch'
    #[doc(alias = "AVVideoCodecTypeAppleProRes422HQ")]
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    pub fn apple_pro_res_422_hq() -> &'static Self {
        unsafe { AVVideoCodecTypeAppleProRes422HQ }
    }

    /// The Apple ProRes 422 LT video codec.
    /// 'apcs'
    #[doc(alias = "AVVideoCodecTypeAppleProRes422LT")]
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    pub fn apple_pro_res_422_lt() -> &'static Self {
        unsafe { AVVideoCodecTypeAppleProRes422LT }
    }

    /// The Apple ProRes 422 Proxy video codec.
    /// 'apco'
    #[doc(alias = "AVVideoCodecTypeAppleProRes422Proxy")]
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    pub fn apple_pro_res_422_proxy() -> &'static Self {
        unsafe { AVVideoCodecTypeAppleProRes422Proxy }
    }

    /// The HEVC video codec that supports an alpha channel.
    ///
    /// MPORTANT NOTE: this constant is used to select the appropriate encoder,
    /// but is NOT used on the encoded content, which is backwards compatible
    /// and hence uses 'hvc1' as its codec type.
    /// 'muxa'
    #[doc(alias = "AVVideoCodecTypeHEVCWithAlpha")]
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    pub fn hevc_with_alpha() -> &'static Self {
        unsafe { AVVideoCodecTypeHEVCWithAlpha }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)]
    static AVVideoCodecTypeHEVC: &'static Codec;
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)]
    static AVVideoCodecTypeH264: &'static Codec;
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)]
    static AVVideoCodecTypeJPEG: &'static Codec;
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    static AVVideoCodecTypeAppleProRes4444: &'static Codec;
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0)]
    static AVVideoCodecTypeAppleProRes4444XQ: &'static Codec;
    #[api::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    static AVVideoCodecTypeAppleProRes422: &'static Codec;
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    static AVVideoCodecTypeAppleProRes422HQ: &'static Codec;
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    static AVVideoCodecTypeAppleProRes422LT: &'static Codec;
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    static AVVideoCodecTypeAppleProRes422Proxy: &'static Codec;
    #[api::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    static AVVideoCodecTypeHEVCWithAlpha: &'static Codec;
}
