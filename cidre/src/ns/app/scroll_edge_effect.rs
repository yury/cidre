use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSScrollEdgeEffectStyle")]
    pub ScrollEdgeEffectStyle(ns::Id)
);

impl ScrollEdgeEffectStyle {
    #[api::available(macos = 26.1)]
    crate::define_cls!(NS_SCROLL_EDGE_EFFECT_STYLE);

    #[objc::msg_send(automaticStyle)]
    #[objc::available(macos = 26.1)]
    pub fn automatic() -> arc::R<Self>;

    #[objc::msg_send(softStyle)]
    #[objc::available(macos = 26.1)]
    pub fn soft() -> arc::R<Self>;

    #[objc::msg_send(hardStyle)]
    #[objc::available(macos = 26.1)]
    pub fn hard() -> arc::R<Self>;
}

unsafe extern "C" {
    static NS_SCROLL_EDGE_EFFECT_STYLE: &'static objc::Class<ScrollEdgeEffectStyle>;
}
