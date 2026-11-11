use crate::{api, arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSGlassEffectViewStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum GlassEffectViewStyle {
    Regular = 0,
    Clear = 1,
}

define_obj_type!(
    #[doc(alias = "NSGlassEffectView")]
    pub GlassEffectView(ns::View)
);

impl GlassEffectView {
    #[api::available(macos = 26.0)]
    crate::define_cls!(NS_GLASS_EFFECT_VIEW);

    #[objc::init(initWithFrame:)]
    #[objc::available(macos = 26.0)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<GlassEffectView>;

    #[api::available(macos = 26.0)]
    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(contentView)]
    #[objc::available(macos = 26.0)]
    pub fn content_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setContentView:)]
    #[objc::available(macos = 26.0)]
    pub fn set_content_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(cornerRadius)]
    #[objc::available(macos = 26.0)]
    pub fn corner_radius(&self) -> cg::Float;

    #[objc::msg_send(setCornerRadius:)]
    #[objc::available(macos = 26.0)]
    pub fn set_corner_radius(&mut self, val: cg::Float);

    #[objc::msg_send(style)]
    #[objc::available(macos = 26.0)]
    pub fn style(&self) -> GlassEffectViewStyle;

    #[objc::msg_send(setStyle:)]
    #[objc::available(macos = 26.0)]
    pub fn set_style(&mut self, val: GlassEffectViewStyle);

    #[objc::msg_send(effectIsInteractive)]
    #[objc::available(macos = 27.0)]
    pub fn effect_is_interactive(&self) -> bool;

    #[objc::msg_send(setEffectIsInteractive:)]
    #[objc::available(macos = 27.0)]
    pub fn set_effect_is_interactive(&mut self, val: bool);
}

define_obj_type!(
    /// Batches eligible descendant glass views without adding a glass surface.
    #[doc(alias = "NSGlassEffectContainerView")]
    pub GlassEffectContainerView(ns::View)
);

impl GlassEffectContainerView {
    #[api::available(macos = 26.0)]
    crate::define_cls!(NS_GLASS_EFFECT_CONTAINER_VIEW);

    #[objc::init(initWithFrame:)]
    #[objc::available(macos = 26.0)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<GlassEffectContainerView>;

    #[api::available(macos = 26.0)]
    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(contentView)]
    #[objc::available(macos = 26.0)]
    pub fn content_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setContentView:)]
    #[objc::available(macos = 26.0)]
    pub fn set_content_view(&mut self, val: Option<&ns::View>);

    /// Zero batches similar glass views while keeping neighboring shapes separate.
    #[objc::msg_send(spacing)]
    #[objc::available(macos = 26.0)]
    pub fn spacing(&self) -> cg::Float;

    #[objc::msg_send(setSpacing:)]
    #[objc::available(macos = 26.0)]
    pub fn set_spacing(&mut self, val: cg::Float);
}

unsafe extern "C" {
    static NS_GLASS_EFFECT_VIEW: &'static objc::Class<GlassEffectView>;
    static NS_GLASS_EFFECT_CONTAINER_VIEW: &'static objc::Class<GlassEffectContainerView>;
}
