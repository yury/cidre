use crate::{api, arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UICornerRadius")]
    pub CornerRadius(ns::Id)
);

impl CornerRadius {
    #[api::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    crate::define_cls!(UI_CORNER_RADIUS);

    /// A fixed radius.
    #[objc::msg_send(fixedRadius:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn fixed(radius: cg::Float) -> arc::R<Self>;

    /// A radius concentric with the nearest container.
    #[objc::msg_send(containerConcentricRadius)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn container_concentric() -> arc::R<Self>;

    /// A radius concentric with the nearest container, never smaller than `min`.
    #[objc::msg_send(containerConcentricRadiusWithMinimum:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn container_concentric_with_min(min: cg::Float) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_CORNER_RADIUS: &'static objc::Class<CornerRadius>;
}
