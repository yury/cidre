use crate::{arc, cf, cg, define_cf_type};

define_cf_type!(Color(cf::Type));

impl Color {
    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_gray(0.5, 0.5);
    /// ```
    #[inline]
    pub fn generic_gray(gray: cg::Float, alpha: cg::Float) -> arc::R<Color> {
        unsafe { CGColorCreateGenericGray(gray, alpha) }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_rgb(0.5, 0.5, 0.5, 0.5);
    ///
    /// assert_eq!(c.alpha(), 0.5);
    /// ```
    #[inline]
    pub fn generic_rgb(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> arc::R<Color> {
        unsafe { CGColorCreateGenericRGB(red, green, blue, alpha) }
    }

    #[inline]
    pub fn alpha(&self) -> cg::Float {
        unsafe { CGColorGetAlpha(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGColorCreateGenericGray(gray: cg::Float, alpha: cg::Float) -> arc::R<Color>;
    fn CGColorCreateGenericRGB(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> arc::R<Color>;

    fn CGColorGetAlpha(color: &Color) -> cg::Float;
}
