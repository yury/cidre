use crate::{
    cf::{self, Retained},
    cg, define_cf_type,
};

define_cf_type!(Color(cf::Type));

impl Color {
    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_gray(0.5, 0.5);
    /// ```
    pub fn generic_gray<'c>(gray: cg::Float, alpha: cg::Float) -> Retained<'c, Color> {
        unsafe { CGColorCreateGenericGray(gray, alpha) }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let c = cg::Color::generic_rgb(0.5, 0.5, 0.5, 0.5);
    ///
    /// assert_eq!(c.alpha(), 0.5);
    /// ```
    pub fn generic_rgb<'c>(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> Retained<'c, Color> {
        unsafe { CGColorCreateGenericRGB(red, green, blue, alpha) }
    }

    pub fn alpha(&self) -> cg::Float {
        unsafe { CGColorGetAlpha(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGColorCreateGenericGray<'c>(gray: cg::Float, alpha: cg::Float) -> Retained<'c, Color>;
    fn CGColorCreateGenericRGB<'c>(
        red: cg::Float,
        green: cg::Float,
        blue: cg::Float,
        alpha: cg::Float,
    ) -> Retained<'c, Color>;

    fn CGColorGetAlpha(color: &Color) -> cg::Float;
}
