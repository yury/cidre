pub mod keys {
    use crate::cf;

    #[doc(alias = "kVTPixelTransferPropertyKey_ScalingMode")]
    #[inline]
    pub fn scaling_mode() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_ScalingMode }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DestinationCleanAperture")]
    #[inline]
    pub fn dst_clean_aperture() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationCleanAperture }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DestinationPixelAspectRatio")]
    #[inline]
    pub fn dst_pixel_aspect_ratio() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationPixelAspectRatio }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DownsamplingMode")]
    #[inline]
    pub fn downsampling_mode() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DownsamplingMode }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DestinationColorPrimaries")]
    #[inline]
    pub fn dst_color_primaries() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationColorPrimaries }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DestinationTransferFunction")]
    #[inline]
    pub fn dst_transfer_function() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationTransferFunction }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DestinationICCProfile")]
    #[inline]
    pub fn dst_icc_profile() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationICCProfile }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_DestinationYCbCrMatrix")]
    #[inline]
    pub fn dst_ycbcr_matrix() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_DestinationYCbCrMatrix }
    }

    #[doc(alias = "kVTPixelTransferPropertyKey_RealTime")]
    #[inline]
    pub fn real_time() -> &'static cf::String {
        unsafe { kVTPixelTransferPropertyKey_RealTime }
    }

    unsafe extern "C" {
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

    #[doc(alias = "kVTScalingMode_Normal")]
    #[inline]
    pub fn normal() -> &'static cf::String {
        unsafe { kVTScalingMode_Normal }
    }

    #[doc(alias = "kVTScalingMode_CropSourceToCleanAperture")]
    #[inline]
    pub fn crop_src_to_clean_aperture() -> &'static cf::String {
        unsafe { kVTScalingMode_CropSourceToCleanAperture }
    }

    #[doc(alias = "kVTScalingMode_Letterbox")]
    #[inline]
    pub fn letter_box() -> &'static cf::String {
        unsafe { kVTScalingMode_Letterbox }
    }

    #[doc(alias = "kVTScalingMode_Trim")]
    #[inline]
    pub fn trim() -> &'static cf::String {
        unsafe { kVTScalingMode_Trim }
    }

    unsafe extern "C" {
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

    unsafe extern "C" {
        static kVTDownsamplingMode_Decimate: &'static cf::String;
        static kVTDownsamplingMode_Average: &'static cf::String;
    }
}
