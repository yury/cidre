use crate::{api, arc, cf, cg, define_cf_type};

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

    #[doc(alias = "CGColorCreateWithContentHeadroom")]
    #[inline]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn with_content_headroom(
        headroom: f32,
        space: Option<&cg::ColorSpace>,
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
    ) -> Option<arc::R<Self>> {
        unsafe { CGColorCreateWithContentHeadroom(headroom, space, r, g, b, a) }
    }

    #[doc(alias = "CGColorGetContentHeadroom")]
    #[inline]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn content_headroom(&self) -> f32 {
        unsafe { CGColorGetContentHeadroom(Some(self)) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    fn CGColorCreateGenericGray(gray: cg::Float, alpha: cg::Float) -> arc::R<Color>;
    fn CGColorCreateGenericRGB(
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
    ) -> arc::R<Color>;

    fn CGColorGetAlpha(color: &Color) -> cg::Float;

    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    fn CGColorCreateWithContentHeadroom(
        headroom: f32,
        space: Option<&cg::ColorSpace>,
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
    ) -> Option<arc::R<cg::Color>>;

    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    fn CGColorGetContentHeadroom(color: Option<&cg::Color>) -> f32;
}

#[cfg(test)]
mod tests {
    use crate::{api, cg};

    #[test]
    fn hdr() {
        if api::macos_available("26.0") {
            let space =
                cg::ColorSpace::with_name(cg::color_space::names::extended_display_p3()).unwrap();
            let color =
                cg::Color::with_content_headroom(1.0, Some(&space), 1.0, 1.0, 0.0, 1.0).unwrap();
            assert_eq!(color.content_headroom(), 1.0);
            let color =
                cg::Color::with_content_headroom(2.0, Some(&space), 1.0, 1.0, 0.0, 1.0).unwrap();
            assert_eq!(color.content_headroom(), 2.0);
        }
    }
}
