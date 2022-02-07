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
    pub fn type_id() -> cf::TypeId {
      unsafe {
        CGColorSpaceGetTypeID()
      }
    }

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

    pub fn get_model(&self) -> ColorSpaceModel {
        unsafe { CGColorSpaceGetModel(self) }
    }

    pub fn get_base_color_space(&self) -> Option<&ColorSpace> {
        unsafe { CGColorSpaceGetBaseColorSpace(self) }
    }

    pub fn get_color_table_count(&self) -> usize {
      unsafe {
        CGColorSpaceGetColorTableCount(self)
      }
    }

    pub fn get_name(&self) -> Option<&cf::String> {
      unsafe {
        CGColorSpaceGetName(self)
      }
    }

    #[inline]
    pub fn create_device_gray<'a>() -> Option<cf::Retained<'a, ColorSpace>> {
      unsafe {
        CGColorSpaceCreateDeviceGray()
      }
    }

    /// ```
    /// use cidre::cg;
    /// 
    /// let color_space = cg::ColorSpace::create_device_rgb().unwrap();
    /// 
    /// assert_eq!(color_space.get_type_id(), cg::ColorSpace::type_id());
    /// 
    /// let name = color_space.get_name().unwrap();
    /// assert_eq!("kCGColorSpaceDeviceRGB", name.to_string());
    /// ```
    #[inline]
    pub fn create_device_rgb<'a>() -> Option<cf::Retained<'a, ColorSpace>> {
      unsafe {
        CGColorSpaceCreateDeviceRGB()
      }
    }

    /// ```
    /// use cidre::cg;
    /// 
    /// let color_space = cg::ColorSpace::create_device_cmyk().unwrap();
    /// 
    /// assert_eq!(color_space.get_type_id(), cg::ColorSpace::type_id());
    /// 
    /// let name = color_space.get_name().unwrap();
    /// assert_eq!("kCGColorSpaceDeviceCMYK", name.to_string());
    /// ```
    #[inline]
    pub fn create_device_cmyk<'a>() -> Option<cf::Retained<'a, ColorSpace>> {
      unsafe {
        CGColorSpaceCreateDeviceCMYK()
      }
    }
}

extern "C" {
    fn CGColorSpaceGetTypeID() -> cf::TypeId;
    fn CGColorSpaceGetName(space: &ColorSpace) -> Option<&cf::String>;
    fn CGColorSpaceIsWideGamutRGB(space: &ColorSpace) -> bool;
    fn CGColorSpaceUsesITUR_2100TF(space: &ColorSpace) -> bool;
    fn CGColorSpaceUsesExtendedRange(space: &ColorSpace) -> bool;
    fn CGColorSpaceGetModel(space: &ColorSpace) -> ColorSpaceModel;
    fn CGColorSpaceGetBaseColorSpace(space: &ColorSpace) -> Option<&ColorSpace>;
    fn CGColorSpaceGetColorTableCount(space: &ColorSpace) -> usize;

    fn CGColorSpaceCreateDeviceGray<'a>() -> Option<cf::Retained<'a, ColorSpace>>;
    fn CGColorSpaceCreateDeviceRGB<'a>() -> Option<cf::Retained<'a, ColorSpace>>;
    fn CGColorSpaceCreateDeviceCMYK<'a>() -> Option<cf::Retained<'a, ColorSpace>>;
}

pub mod names {
    use crate::cf;

    /// The name of the "Generic" Gray color space. It is a legacy
    /// color space so use it only if you definitely know that you
    /// need it. If you need just a reasonable monochrome color space,
    /// please use Generic Gray Gamma 2.2, i.e. use
    /// generic_gray_gamma_2_2() name instead of
    /// generic_gray().
    #[inline]
    pub fn generic_gray() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericGray }
    }

    #[inline]
    pub fn generic_rgb() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericRGB }
    }

    #[inline]
    pub fn generic_cmyk() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericCMYK }
    }

    #[inline]
    pub fn display_p3() -> &'static cf::String {
        unsafe { kCGColorSpaceDisplayP3 }
    }

    #[inline]
    pub fn generic_rgb_linear() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericRGBLinear }
    }

    #[inline]
    pub fn adobe_rgb_1998() -> &'static cf::String {
        unsafe { kCGColorSpaceAdobeRGB1998 }
    }

    #[inline]
    pub fn srgb() -> &'static cf::String {
        unsafe { kCGColorSpaceSRGB }
    }

    #[inline]
    pub fn generic_gray_gamma_2_2() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericGrayGamma2_2 }
    }

    #[inline]
    pub fn generic_xyz() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericXYZ }
    }

    #[inline]
    pub fn generic_lab() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericLab }
    }

    #[inline]
    pub fn acescg_linear() -> &'static cf::String {
        unsafe { kCGColorSpaceACESCGLinear }
    }

    #[inline]
    pub fn itur_709() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_709 }
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
