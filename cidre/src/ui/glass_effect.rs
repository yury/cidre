use crate::{api, arc, cg, define_obj_type, objc, ui};

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
    #[api::available(ios = 26.0, tvos = 26.0)]
    crate::define_cls!(UI_GLASS_EFFECT);

    #[objc::msg_send(effectWithStyle:)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn with_style(style: GlassEffectStyle) -> arc::R<Self>;

    /// Whether the glass reacts to touches. Default is false.
    #[objc::msg_send(isInteractive)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn is_interactive(&self) -> bool;

    #[objc::msg_send(setInteractive:)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn set_interactive(&mut self, val: bool);

    #[objc::msg_send(tintColor)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setTintColor:)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn set_tint_color(&mut self, val: Option<&ui::Color>);
}

unsafe extern "C" {
    static UI_GLASS_EFFECT: &'static objc::Class<GlassEffect>;
}

define_obj_type!(
    /// Renders descendant glass elements together behind its content view.
    #[doc(alias = "UIGlassContainerEffect")]
    pub GlassContainerEffect(ui::VisualEffect)
);

impl GlassContainerEffect {
    #[api::available(ios = 26.0, tvos = 26.0)]
    crate::define_cls!(UI_GLASS_CONTAINER_EFFECT);

    // NSObject's inherited initializer is available whenever allocation succeeds.
    #[objc::init(init)]
    pub fn init(self) -> arc::R<GlassContainerEffect>;

    #[api::available(ios = 26.0, tvos = 26.0)]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    /// Distance at which neighboring glass elements begin to merge.
    #[objc::msg_send(spacing)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn spacing(&self) -> cg::Float;

    #[objc::msg_send(setSpacing:)]
    #[objc::available(ios = 26.0, tvos = 26.0)]
    pub fn set_spacing(&mut self, val: cg::Float);
}

unsafe extern "C" {
    static UI_GLASS_CONTAINER_EFFECT: &'static objc::Class<GlassContainerEffect>;
}
