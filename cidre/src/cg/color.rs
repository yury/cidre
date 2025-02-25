use crate::{arc, cf, cg, define_cf_type};

define_cf_type!(
    #[doc(alias = "CGColorRef")]
    Color(cf::Type)
);

impl Color {
    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_gray(0.5, 0.5);
    /// ```
    #[doc(alias = "CGColorCreateGenericGray")]
    #[inline]
    pub fn generic_gray(gray: cg::Float, alpha: cg::Float) -> arc::R<Color> {
        unsafe { CGColorCreateGenericGray(gray, alpha) }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_rgba(0.5, 0.5, 0.5, 0.5);
    ///
    /// assert_eq!(c.alpha(), 0.5);
    /// ```
    #[doc(alias = "CGColorCreateGenericRGB")]
    #[inline]
    pub fn generic_rgba(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> arc::R<Color> {
        unsafe { CGColorCreateGenericRGB(red, green, blue, alpha) }
    }

    #[doc(alias = "CGColorGetAlpha")]
    #[inline]
    pub fn alpha(&self) -> cg::Float {
        unsafe { CGColorGetAlpha(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C-unwind" {
    fn CGColorCreateGenericGray(gray: cg::Float, alpha: cg::Float) -> arc::R<Color>;
    fn CGColorCreateGenericRGB(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> arc::R<Color>;

    fn CGColorGetAlpha(color: &Color) -> cg::Float;
}
