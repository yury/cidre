use crate::{arc, define_cls, define_obj_type, objc, ui};

#[doc(alias = "UIGlassEffectStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum GlassEffectStyle {
    /// A glass with a regular tint, this style is the standard glass effect.
    Regular = 0,
    /// A clear glass with a minimal tint.
    Clear = 1,
}

define_obj_type!(
    #[doc(alias = "UIGlassEffect")]
    pub GlassEffect(ui::VisualEffect)
);

impl GlassEffect {
    define_cls!(UI_GLASS_EFFECT);

    #[objc::msg_send(effectWithStyle:)]
    #[objc::available(ios = 26.0)]
    pub fn with_style(style: GlassEffectStyle) -> arc::R<Self>;

    /// Whether the glass reacts to touches. Default is false.
    #[objc::msg_send(isInteractive)]
    #[objc::available(ios = 26.0)]
    pub fn is_interactive(&self) -> bool;

    #[objc::msg_send(setInteractive:)]
    #[objc::available(ios = 26.0)]
    pub fn set_interactive(&mut self, val: bool);

    #[objc::msg_send(tintColor)]
    #[objc::available(ios = 26.0)]
    pub fn tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setTintColor:)]
    #[objc::available(ios = 26.0)]
    pub fn set_tint_color(&mut self, val: Option<&ui::Color>);
}

unsafe extern "C" {
    static UI_GLASS_EFFECT: &'static objc::Class<GlassEffect>;
}
