use crate::{arc, cg, define_obj_type, ns, objc};

#[doc(alias = "UIColorProminence")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum ColorProminence {
    Primary = 0,
    Secondary = 1,
    Tertiary = 2,
    Quaternary = 3,
}

define_obj_type!(
    #[doc(alias = "UIColor")]
    pub Color(ns::Id),
    UI_COLOR
);

unsafe impl Send for Color {}

impl arc::A<Color> {
    #[objc::msg_send(initWithWhite:alpha:)]
    pub fn init_with_white_alpha(self, white: cg::Float, alpha: cg::Float) -> arc::Retained<Color>;
}

impl Color {
    #[inline]
    pub fn with_white_alpha(white: cg::Float, alpha: cg::Float) -> arc::R<Self> {
        Self::alloc().init_with_white_alpha(white, alpha)
    }

    #[objc::msg_send(colorWithRed:green:blue:alpha:)]
    pub fn with_rgba(r: cg::Float, g: cg::Float, b: cg::Float, a: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(colorWithRed:green:blue:alpha:exposure:)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn with_exposure(
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
        exposure: cg::Float,
    ) -> arc::R<Self>;

    #[objc::msg_send(colorWithRed:green:blue:alpha:linearExposure:)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn with_linear_exposure(
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
        exposure: cg::Float,
    ) -> arc::R<Self>;

    /// Reinterpret the color by applying a new `content_headroom` without changing the color components.
    /// Changing the `content_headroom` redefines the color relative to a different peak white, changing its behavior
    /// under tone mapping and the result of calling `sdr`. The new color will have a `content_headroom` >= 1.0.
    #[objc::msg_send(colorByApplyingContentHeadroom:)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn applying_content_headroom(&self, content_headroom: cg::Float) -> arc::R<Self>;

    /// The linear brightness multiplier that was applied when generating this color.
    /// Colors created with an exposure by ui::Color create cg::Colors that are tagged with a contentHeadroom value.
    /// While cg::Colors created without a contentHeadroom tag will return 0 from cg::Color::content_headroom, ui::Colors generated in a similar
    /// fashion return a linear_exposure of 1.0.
    #[objc::msg_send(linearExposure)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn linear_exposure(&self) -> cg::Float;

    #[objc::msg_send(standardDynamicRangeColor)]
    #[objc::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        watchos = 26.0,
        visionos = 26.0
    )]
    pub fn sdr(&self) -> arc::R<Self>;

    #[cfg(feature = "cg")]
    #[objc::msg_send(CGColor)]
    pub fn cg_color(&self) -> Option<&crate::cg::Color>;

    #[objc::msg_send(colorNamed:)]
    pub fn color_named(name: &ns::String) -> Option<arc::R<Self>>;

    pub fn named(name: impl AsRef<ns::String>) -> Option<arc::R<Self>> {
        Self::color_named(name.as_ref())
    }
}

impl Color {
    #[objc::msg_send(blackColor)]
    pub fn black() -> arc::R<Self>;

    #[objc::msg_send(darkGrayColor)]
    pub fn dark_gray() -> arc::R<Self>;

    #[objc::msg_send(lightGrayColor)]
    pub fn light_gray() -> arc::R<Self>;

    #[objc::msg_send(whiteColor)]
    pub fn white() -> arc::R<Self>;

    #[objc::msg_send(grayColor)]
    pub fn gray() -> arc::R<Self>;

    #[objc::msg_send(redColor)]
    pub fn red() -> arc::R<Self>;

    #[objc::msg_send(greenColor)]
    pub fn green() -> arc::R<Self>;

    #[objc::msg_send(blueColor)]
    pub fn blue() -> arc::R<Self>;

    #[objc::msg_send(cyanColor)]
    pub fn cyan() -> arc::R<Self>;

    #[objc::msg_send(yellowColor)]
    pub fn yellow() -> arc::R<Self>;

    #[objc::msg_send(magentaColor)]
    pub fn magenta() -> arc::R<Self>;

    #[objc::msg_send(orangeColor)]
    pub fn orange() -> arc::R<Self>;

    #[objc::msg_send(purpleColor)]
    pub fn purple() -> arc::R<Self>;

    #[objc::msg_send(brownColor)]
    pub fn brown() -> arc::R<Self>;

    #[objc::msg_send(clearColor)]
    pub fn clear() -> arc::R<Self>;
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_COLOR: &'static objc::Class<Color>;
}
