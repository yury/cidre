use crate::{cf, cg, define_cf_type};

define_cf_type!(Color(cf::Type));

impl Color {
    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_gray(0.5, 0.5);
    /// ```
    #[inline]
    pub fn generic_gray(gray: cg::Float, alpha: cg::Float) -> cf::Retained<Color> {
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
    ) -> cf::Retained<Color> {
        unsafe { CGColorCreateGenericRGB(red, green, blue, alpha) }
    }

    #[inline]
    pub fn alpha(&self) -> cg::Float {
        unsafe { CGColorGetAlpha(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGColorCreateGenericGray(gray: cg::Float, alpha: cg::Float) -> cf::Retained<Color>;
    fn CGColorCreateGenericRGB(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> cf::Retained<Color>;

    fn CGColorGetAlpha(color: &Color) -> cg::Float;
}
