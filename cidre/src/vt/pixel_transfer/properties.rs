pub mod keys {
    use crate::cf;

    #[inline]
    pub fn scaling_mode() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_ScalingMode }
    }

    #[inline]
    pub fn desination_clean_aperture() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationCleanAperture }
    }

    #[inline]
    pub fn destination_pixel_aspect_ration() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationPixelAspectRatio }
    }

    #[inline]
    pub fn downsampling_mode() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DownsamplingMode }
    }

    #[inline]
    pub fn destination_color_primaries() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationColorPrimaries }
    }

    #[inline]
    pub fn destination_transfer_function() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationTransferFunction }
    }

    #[inline]
    pub fn destination_icc_profile() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationICCProfile }
    }

    #[inline]
    pub fn destination_ycbcr_matrix() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationYCbCrMatrix }
    }

    #[inline]
    pub fn real_time() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_RealTime }
    }

    extern "C" {
        static kVTPixelTransferPropertyKey_ScalingMode: &'static cf::String;
        static kVTPixelTransferPropertyKey_DestinationCleanAperture: &'static cf::String;
        static kVTPixelTransferPropertyKey_DestinationPixelAspectRatio: &'static cf::String;
        static kVTPixelTransferPropertyKey_DownsamplingMode: &'static cf::String;
        static kVTPixelTransferPropertyKey_DestinationColorPrimaries: &'static cf::String;
        static kVTPixelTransferPropertyKey_DestinationTransferFunction: &'static cf::String;
        static kVTPixelTransferPropertyKey_DestinationICCProfile: &'static cf::String;
        static kVTPixelTransferPropertyKey_DestinationYCbCrMatrix: &'static cf::String;
        static kVTPixelTransferPropertyKey_RealTime: &'static cf::String;

    }
}

pub mod scaling_mode {
    use crate::cf;

    #[inline]
    pub fn normal() -> &'static cf::String {
        unsafe { kVTScalingMode_Normal }
    }

    #[inline]
    pub fn crop_src_to_clean_aperture() -> &'static cf::String {
        unsafe { kVTScalingMode_CropSourceToCleanAperture }
    }

    #[inline]
    pub fn letter_box() -> &'static cf::String {
        unsafe { kVTScalingMode_Letterbox }
    }

    #[inline]
    pub fn trim() -> &'static cf::String {
        unsafe { kVTScalingMode_Trim }
    }

    extern "C" {
        static kVTScalingMode_Normal: &'static cf::String;
        static kVTScalingMode_CropSourceToCleanAperture: &'static cf::String;
        static kVTScalingMode_Letterbox: &'static cf::String;
        static kVTScalingMode_Trim: &'static cf::String;
    }
}

pub mod downsampling_mode {
    use crate::cf;

    // Default
    #[inline]
    pub fn decimate() -> &'static cf::String {
        unsafe { kVTDownsamplingMode_Decimate }
    }

    #[inline]
    pub fn average() -> &'static cf::String {
        unsafe { kVTDownsamplingMode_Average }
    }

    extern "C" {
        static kVTDownsamplingMode_Decimate: &'static cf::String;
        static kVTDownsamplingMode_Average: &'static cf::String;
    }
}
