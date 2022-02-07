use crate::{cf, define_cf_type};

#[repr(i32)]
pub enum ColorRenderingIntent {
    Default = 0,
    AbsoluteColorimetric = 1,
    RelativeColorimetric = 2,
    Perceptual = 3,
    Saturation = 4,
}

#[repr(i32)]
pub enum ColorSpaceModel {
    Unknown = -1,
    Monochrome,
    RGB,
    CMYK,
    Lab,
    DeviceN,
    Indexed,
    Pattern,
    XYZ,
}

define_cf_type!(ColorSpace(cf::Type));

impl ColorSpace {
    #[inline]
    pub fn is_wide_gamut_rgb(&self) -> bool {
        unsafe { CGColorSpaceIsWideGamutRGB(self) }
    }

    #[inline]
    pub fn uses_itur_2100tf(&self) -> bool {
        unsafe { CGColorSpaceUsesITUR_2100TF(self) }
    }

    #[inline]
    pub fn uses_extended_range(&self) -> bool {
        unsafe { CGColorSpaceUsesExtendedRange(self) }
    }
}

extern "C" {
    fn CGColorSpaceIsWideGamutRGB(color_space: &ColorSpace) -> bool;
    fn CGColorSpaceUsesITUR_2100TF(color_space: &ColorSpace) -> bool;
    fn CGColorSpaceUsesExtendedRange(color_space: &ColorSpace) -> bool;
}

pub mod names {
    use crate::cf;

    #[inline]
    pub fn generic_gray() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericGray
      }
    }

    #[inline]
    pub fn generic_rgb() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericRGB
      }
    }

    #[inline]
    pub fn generic_cmyk() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericCMYK
      }
    }

    #[inline]
    pub fn display_p3() -> &'static cf::String {
      unsafe {
        kCGColorSpaceDisplayP3
      }
    }

    #[inline]
    pub fn generic_rgb_linear() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericRGBLinear
      }
    }

    #[inline]
    pub fn adobe_rgb_1998() -> &'static cf::String {
      unsafe {
        kCGColorSpaceAdobeRGB1998
      }
    }

    #[inline]
    pub fn srgb() -> &'static cf::String {
      unsafe {
        kCGColorSpaceSRGB
      }
    }

    #[inline]
    pub fn generic_gray_gamma_2_2() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericGrayGamma2_2
      }
    }

    #[inline]
    pub fn generic_xyz() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericXYZ
      }
    }

    #[inline]
    pub fn generic_lab() -> &'static cf::String {
      unsafe {
        kCGColorSpaceGenericLab
      }
    }

    extern "C" {
        static kCGColorSpaceGenericGray: &'static cf::String;
        static kCGColorSpaceGenericRGB: &'static cf::String;
        static kCGColorSpaceGenericCMYK: &'static cf::String;
        static kCGColorSpaceDisplayP3: &'static cf::String;
        static kCGColorSpaceGenericRGBLinear: &'static cf::String;
        static kCGColorSpaceAdobeRGB1998: &'static cf::String;
        static kCGColorSpaceSRGB: &'static cf::String;
        static kCGColorSpaceGenericGrayGamma2_2: &'static cf::String;
        static kCGColorSpaceGenericXYZ: &'static cf::String;
        static kCGColorSpaceGenericLab: &'static cf::String;
        static kCGColorSpaceACESCGLinear: &'static cf::String;
        static kCGColorSpaceITUR_709: &'static cf::String;
        static kCGColorSpaceITUR_709_PQ: &'static cf::String;
        static kCGColorSpaceITUR_2020: &'static cf::String;
        static kCGColorSpaceITUR_2020_sRGBGamma: &'static cf::String;
        static kCGColorSpaceROMMRGB: &'static cf::String;
        static kCGColorSpaceDCIP3: &'static cf::String;
        static kCGColorSpaceLinearITUR_2020: &'static cf::String;
        static kCGColorSpaceExtendedITUR_2020: &'static cf::String;
        static kCGColorSpaceExtendedLinearITUR_2020: &'static cf::String;
        static kCGColorSpaceLinearDisplayP3: &'static cf::String;
        static kCGColorSpaceExtendedDisplayP3: &'static cf::String;
        static kCGColorSpaceExtendedLinearDisplayP3: &'static cf::String;
        static kCGColorSpaceITUR_2100_PQ: &'static cf::String;
        static kCGColorSpaceITUR_2100_HLG: &'static cf::String;
        static kCGColorSpaceDisplayP3_PQ: &'static cf::String;
        static kCGColorSpaceDisplayP3_HLG: &'static cf::String;
        static kCGColorSpaceExtendedSRGB: &'static cf::String;
        static kCGColorSpaceLinearSRGB: &'static cf::String;
        static kCGColorSpaceExtendedLinearSRGB: &'static cf::String;
        static kCGColorSpaceExtendedGray: &'static cf::String;
        static kCGColorSpaceLinearGray: &'static cf::String;
        static kCGColorSpaceExtendedLinearGray: &'static cf::String;
    }
}
