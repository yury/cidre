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
