use crate::{arc, cf, define_cf_type};

/// Color rendering intents.
#[doc(alias = "CGColorRenderingIntent")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ColorRenderingIntent {
    Default = 0,
    AbsoluteColorimetric = 1,
    RelativeColorimetric = 2,
    Perceptual = 3,
    Saturation = 4,
}

/// The model of a color space.
#[doc(alias = "CGColorSpaceModel")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ColorSpaceModel {
    Unknown = -1,
    Monochrome,
    Rgb,
    Cmyk,
    Lab,
    DeviceN,
    Indexed,
    Pattern,
    Xyz,
}

define_cf_type!(
    #[doc(alias = "CGColorSpaceRef")]
    ColorSpace(cf::Type)
);

impl ColorSpace {
    /// ```
    /// use cidre::{cf, cg};
    ///
    /// let type_id = cg::ColorSpace::type_id();
    ///
    /// unsafe {
    ///     let type_desc = cf::type_id_desc(type_id).unwrap();
    ///     assert_eq!("CGColorSpace", type_desc.to_string());
    /// }
    /// ```
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGColorSpaceGetTypeID() }
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

    pub fn model(&self) -> ColorSpaceModel {
        unsafe { CGColorSpaceGetModel(self) }
    }

    pub fn base_color_space(&self) -> Option<&ColorSpace> {
        unsafe { CGColorSpaceGetBaseColorSpace(self) }
    }

    pub fn color_table_count(&self) -> usize {
        unsafe { CGColorSpaceGetColorTableCount(self) }
    }

    pub fn name(&self) -> Option<&cf::String> {
        unsafe { CGColorSpaceGetName(self) }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let space = cg::ColorSpace::device_gray().unwrap();
    ///
    /// assert_eq!(space.get_type_id(), cg::ColorSpace::type_id());
    ///
    /// let name = space.name().unwrap();
    /// assert_eq!("kCGColorSpaceDeviceGray", name.to_string());
    ///
    /// assert_eq!(cg::ColorSpaceModel::Monochrome, space.model());
    /// ```
    #[inline]
    pub fn device_gray() -> Option<arc::R<ColorSpace>> {
        unsafe { CGColorSpaceCreateDeviceGray() }
    }

    /// Create a DeviceRGB color space.
    ///
    /// # Example
    ///
    /// ```
    /// use cidre::cg;
    ///
    /// let space = cg::ColorSpace::device_rgb().unwrap();
    ///
    /// assert_eq!(space.get_type_id(), cg::ColorSpace::type_id());
    ///
    /// let name = space.name().unwrap();
    /// assert_eq!("kCGColorSpaceDeviceRGB", name.to_string());
    ///
    /// assert_eq!(cg::ColorSpaceModel::Rgb, space.model());
    /// ```
    #[inline]
    pub fn device_rgb() -> Option<arc::R<ColorSpace>> {
        unsafe { CGColorSpaceCreateDeviceRGB() }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let space = cg::ColorSpace::device_cmyk().unwrap();
    ///
    /// assert_eq!(space.get_type_id(), cg::ColorSpace::type_id());
    ///
    /// let name = space.name().unwrap();
    /// assert_eq!("kCGColorSpaceDeviceCMYK", name.to_string());
    ///
    /// assert_eq!(cg::ColorSpaceModel::Cmyk, space.model());
    /// ```
    #[inline]
    pub fn device_cmyk() -> Option<arc::R<ColorSpace>> {
        unsafe { CGColorSpaceCreateDeviceCMYK() }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let name = cg::color_space::names::generic_gray();
    /// let space = cg::ColorSpace::with_name(name).unwrap();
    ///
    /// let actual_name = space.name().unwrap();
    ///
    /// assert!(actual_name.equal(name));
    /// ```
    #[inline]
    pub fn with_name(name: &cf::String) -> Option<arc::R<ColorSpace>> {
        unsafe { CGColorSpaceCreateWithName(name) }
    }
}

unsafe extern "C" {
    fn CGColorSpaceGetTypeID() -> cf::TypeId;
    fn CGColorSpaceGetName(space: &ColorSpace) -> Option<&cf::String>;
    fn CGColorSpaceIsWideGamutRGB(space: &ColorSpace) -> bool;
    fn CGColorSpaceUsesITUR_2100TF(space: &ColorSpace) -> bool;
    fn CGColorSpaceUsesExtendedRange(space: &ColorSpace) -> bool;
    fn CGColorSpaceGetModel(space: &ColorSpace) -> ColorSpaceModel;
    fn CGColorSpaceGetBaseColorSpace(space: &ColorSpace) -> Option<&ColorSpace>;
    fn CGColorSpaceGetColorTableCount(space: &ColorSpace) -> usize;

    fn CGColorSpaceCreateDeviceGray() -> Option<arc::R<ColorSpace>>;
    fn CGColorSpaceCreateDeviceRGB() -> Option<arc::R<ColorSpace>>;
    fn CGColorSpaceCreateDeviceCMYK() -> Option<arc::R<ColorSpace>>;

    fn CGColorSpaceCreateWithName(name: &cf::String) -> Option<arc::R<ColorSpace>>;
}

pub mod names {
    use crate::cf;

    /// The name of the "Generic" Gray color space. It is a legacy
    /// color space so use it only if you definitely know that you
    /// need it. If you need just a reasonable monochrome color space,
    /// please use Generic Gray Gamma 2.2, i.e. use
    /// generic_gray_gamma_2_2() name instead of
    /// generic_gray().
    #[doc(alias = "kCGColorSpaceGenericGray")]
    #[inline]
    pub fn generic_gray() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericGray }
    }

    #[doc(alias = "kCGColorSpaceGenericRGB")]
    #[inline]
    pub fn generic_rgb() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericRGB }
    }

    #[doc(alias = "kCGColorSpaceGenericCMYK")]
    #[inline]
    pub fn generic_cmyk() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericCMYK }
    }

    /// The Display P3 color space, created by Apple.
    ///
    /// This color space uses the DCI P3 primaries, a D65 white point, and the sRGB transfer function.
    #[doc(alias = "kCGColorSpaceDisplayP3")]
    #[inline]
    pub fn display_p3() -> &'static cf::String {
        unsafe { kCGColorSpaceDisplayP3 }
    }

    #[doc(alias = "kCGColorSpaceGenericRGBLinear")]
    #[inline]
    pub fn generic_rgb_linear() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericRGBLinear }
    }

    #[doc(alias = "kCGColorSpaceAdobeRGB1998")]
    #[inline]
    pub fn adobe_rgb_1998() -> &'static cf::String {
        unsafe { kCGColorSpaceAdobeRGB1998 }
    }

    /// The standard Red Green Blue (sRGB) color space.
    ///
    /// The sRGB colorimetry and non-linear transfer function are specified in IEC 61966-2-1.
    #[doc(alias = "kCGColorSpaceSRGB")]
    #[inline]
    pub fn srgb() -> &'static cf::String {
        unsafe { kCGColorSpaceSRGB }
    }

    #[doc(alias = "kCGColorSpaceGenericGrayGamma2_2")]
    #[inline]
    pub fn generic_gray_gamma_2_2() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericGrayGamma2_2 }
    }

    #[doc(alias = "kCGColorSpaceGenericXYZ")]
    #[inline]
    pub fn generic_xyz() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericXYZ }
    }

    #[doc(alias = "kCGColorSpaceGenericLab")]
    #[inline]
    pub fn generic_lab() -> &'static cf::String {
        unsafe { kCGColorSpaceGenericLab }
    }

    #[doc(alias = "kCGColorSpaceACESCGLinear")]
    #[inline]
    pub fn acescg_linear() -> &'static cf::String {
        unsafe { kCGColorSpaceACESCGLinear }
    }

    #[doc(alias = "kCGColorSpaceITUR_709")]
    #[inline]
    pub fn itur_709() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_709 }
    }

    #[doc(alias = "kCGColorSpaceITUR_709_PQ")]
    #[inline]
    pub fn itur_709_pq() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_709_PQ }
    }

    #[doc(alias = "kCGColorSpaceITUR_2020")]
    #[inline]
    pub fn itur_2020() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_2020 }
    }

    #[doc(alias = "kCGColorSpaceITUR_2020_sRGBGamma")]
    #[inline]
    pub fn itur_2020_srgb_gamma() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_2020_sRGBGamma }
    }

    #[doc(alias = "kCGColorSpaceROMMRGB")]
    #[inline]
    pub fn rommrgb() -> &'static cf::String {
        unsafe { kCGColorSpaceROMMRGB }
    }

    #[doc(alias = "kCGColorSpaceDCIP3")]
    #[inline]
    pub fn dci_p3() -> &'static cf::String {
        unsafe { kCGColorSpaceDCIP3 }
    }

    #[doc(alias = "kCGColorSpaceLinearITUR_2020")]
    #[inline]
    pub fn linerar_itur_2020() -> &'static cf::String {
        unsafe { kCGColorSpaceLinearITUR_2020 }
    }

    #[doc(alias = "kCGColorSpaceExtendedITUR_2020")]
    #[inline]
    pub fn extended_itur_2020() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedITUR_2020 }
    }

    #[doc(alias = "kCGColorSpaceExtendedLinearITUR_2020")]
    #[inline]
    pub fn extended_linear_itur_2020() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedLinearITUR_2020 }
    }

    #[doc(alias = "kCGColorSpaceLinearDisplayP3")]
    #[inline]
    pub fn linear_display_p3() -> &'static cf::String {
        unsafe { kCGColorSpaceLinearDisplayP3 }
    }

    #[doc(alias = "kCGColorSpaceExtendedDisplayP3")]
    #[inline]
    pub fn extended_display_p3() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedDisplayP3 }
    }

    #[doc(alias = "kCGColorSpaceExtendedLinearDisplayP3")]
    #[inline]
    pub fn extended_linear_display_p3() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedLinearDisplayP3 }
    }

    #[doc(alias = "kCGColorSpaceITUR_2100_PQ")]
    #[inline]
    pub fn itur_2100_pq() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_2100_PQ }
    }

    #[doc(alias = "kCGColorSpaceITUR_2100_HLG")]
    #[inline]
    pub fn itur_2100_hlg() -> &'static cf::String {
        unsafe { kCGColorSpaceITUR_2100_HLG }
    }

    #[doc(alias = "kCGColorSpaceDisplayP3_PQ")]
    #[inline]
    pub fn display_p3_pq() -> &'static cf::String {
        unsafe { kCGColorSpaceDisplayP3_PQ }
    }

    #[doc(alias = "kCGColorSpaceDisplayP3_HLG")]
    #[inline]
    pub fn display_p3_hlg() -> &'static cf::String {
        unsafe { kCGColorSpaceDisplayP3_HLG }
    }

    #[doc(alias = "kCGColorSpaceExtendedSRGB")]
    #[inline]
    pub fn extended_srgb() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedSRGB }
    }

    #[doc(alias = "kCGColorSpaceLinearSRGB")]
    #[inline]
    pub fn linear_srgb() -> &'static cf::String {
        unsafe { kCGColorSpaceLinearSRGB }
    }

    #[doc(alias = "kCGColorSpaceExtendedLinearSRGB")]
    #[inline]
    pub fn extended_linear_srgb() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedLinearSRGB }
    }

    #[doc(alias = "kCGColorSpaceExtendedGray")]
    #[inline]
    pub fn extended_gray() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedGray }
    }

    #[doc(alias = "kCGColorSpaceLinearGray")]
    #[inline]
    pub fn linear_gray() -> &'static cf::String {
        unsafe { kCGColorSpaceLinearGray }
    }

    #[doc(alias = "kCGColorSpaceExtendedLinearGray")]
    #[inline]
    pub fn extended_linear_gray() -> &'static cf::String {
        unsafe { kCGColorSpaceExtendedLinearGray }
    }

    unsafe extern "C" {
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
