pub mod keys {
    use crate::cf;

    pub fn codec() -> &'static cf::String {
        unsafe { AVVideoCodecKey }
    }

    pub fn pixel_aspect_ratio() -> &'static cf::String {
        unsafe { AVVideoPixelAspectRatioKey }
    }

    pub fn clean_aperture() -> &'static cf::String {
        unsafe { AVVideoCleanApertureKey }
    }

    pub fn scaling_mode() -> &'static cf::String {
        unsafe { AVVideoScalingModeKey }
    }

    pub fn color_properites() -> &'static cf::String {
        unsafe { AVVideoColorPropertiesKey }
    }

    pub fn allow_wide_color() -> &'static cf::String {
        unsafe { AVVideoAllowWideColorKey }
    }

    pub fn compression_properties() -> &'static cf::String {
        unsafe { AVVideoCompressionPropertiesKey }
    }

    pub fn decompression_properties() -> &'static cf::String {
        unsafe { AVVideoDecompressionPropertiesKey }
    }

    pub fn encoder_specification() -> &'static cf::String {
        unsafe { AVVideoEncoderSpecificationKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
    pub fn width() -> &'static cf::String {
        unsafe { AVVideoWidthKey }
    }

    /// For best results, always use even number values for AVVideoWidthKey and
    /// AVVideoHeightKey when encoding to AVVideoCodecTypeH264 or any other
    /// format that uses 4:2:0 downsampling
    pub fn height() -> &'static cf::String {
        unsafe { AVVideoHeightKey }
    }

    extern "C" {
        static AVVideoWidthKey: &'static cf::String;
        static AVVideoHeightKey: &'static cf::String;

        static AVVideoCodecKey: &'static cf::String;
        static AVVideoPixelAspectRatioKey: &'static cf::String;
        static AVVideoCleanApertureKey: &'static cf::String;
        static AVVideoScalingModeKey: &'static cf::String;
        static AVVideoColorPropertiesKey: &'static cf::String;
        static AVVideoAllowWideColorKey: &'static cf::String;
        static AVVideoCompressionPropertiesKey: &'static cf::String;
        static AVVideoDecompressionPropertiesKey: &'static cf::String;
        static AVVideoEncoderSpecificationKey: &'static cf::String;
    }
}

pub mod codec_type {
    use crate::cf;

    pub fn hevc() -> &'static cf::String {
        unsafe { AVVideoCodecTypeHEVC }
    }

    pub fn h264() -> &'static cf::String {
        unsafe { AVVideoCodecTypeH264 }
    }

    pub fn jpeg() -> &'static cf::String {
        unsafe { AVVideoCodecTypeJPEG }
    }

    pub fn apple_pro_res_4444() -> &'static cf::String {
        unsafe { AVVideoCodecTypeAppleProRes4444 }
    }

    pub fn apple_pro_res_422() -> &'static cf::String {
        unsafe { AVVideoCodecTypeAppleProRes422 }
    }

    pub fn apple_pro_res_422_hq() -> &'static cf::String {
        unsafe { AVVideoCodecTypeAppleProRes422HQ }
    }

    pub fn apple_pro_res_422_lt() -> &'static cf::String {
        unsafe { AVVideoCodecTypeAppleProRes422LT }
    }

    pub fn apple_pro_res_422_proxy() -> &'static cf::String {
        unsafe { AVVideoCodecTypeAppleProRes422Proxy }
    }

    pub fn hevc_with_alpha() -> &'static cf::String {
        unsafe { AVVideoCodecTypeHEVCWithAlpha }
    }

    extern "C" {
        static AVVideoCodecTypeHEVC: &'static cf::String;
        static AVVideoCodecTypeH264: &'static cf::String;
        static AVVideoCodecTypeJPEG: &'static cf::String;
        static AVVideoCodecTypeAppleProRes4444: &'static cf::String;
        static AVVideoCodecTypeAppleProRes422: &'static cf::String;
        static AVVideoCodecTypeAppleProRes422HQ: &'static cf::String;
        static AVVideoCodecTypeAppleProRes422LT: &'static cf::String;
        static AVVideoCodecTypeAppleProRes422Proxy: &'static cf::String;

        static AVVideoCodecTypeHEVCWithAlpha: &'static cf::String;
    }
}
