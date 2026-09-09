use crate::{api, arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UICornerConfiguration")]
    pub CornerCfg(ns::Id)
);

impl CornerCfg {
    #[api::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    crate::define_cls!(UI_CORNER_CONFIGURATION);

    /// The same radius on every corner.
    #[objc::msg_send(configurationWithRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_radius(radius: &ui::CornerRadius) -> arc::R<Self>;

    #[objc::msg_send(configurationWithTopLeftRadius:topRightRadius:bottomLeftRadius:bottomRightRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_radii(
        top_left: Option<&ui::CornerRadius>,
        top_right: Option<&ui::CornerRadius>,
        bottom_left: Option<&ui::CornerRadius>,
        bottom_right: Option<&ui::CornerRadius>,
    ) -> arc::R<Self>;

    /// A capsule: the radius is half the shorter side.
    #[objc::msg_send(capsuleConfiguration)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn capsule() -> arc::R<Self>;

    #[objc::msg_send(capsuleConfigurationWithMaximumRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn capsule_with_max_radius(max_radius: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(configurationWithUniformRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_uniform_radius(radius: &ui::CornerRadius) -> arc::R<Self>;

    #[objc::msg_send(configurationWithUniformTopRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_uniform_top_radius(radius: &ui::CornerRadius) -> arc::R<Self>;

    #[objc::msg_send(configurationWithUniformBottomRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_uniform_bottom_radius(radius: &ui::CornerRadius) -> arc::R<Self>;

    #[objc::msg_send(configurationWithUniformLeftRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_uniform_left_radius(radius: &ui::CornerRadius) -> arc::R<Self>;

    #[objc::msg_send(configurationWithUniformRightRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_uniform_right_radius(radius: &ui::CornerRadius) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_CORNER_CONFIGURATION: &'static objc::Class<CornerCfg>;
}
