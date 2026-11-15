use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSItemBadge")]
    pub ItemBadge(ns::Id)
);

impl ItemBadge {
    #[api::available(macos = 26.0)]
    crate::define_cls!(NS_ITEM_BADGE);

    #[objc::msg_send(badgeWithCount:)]
    #[objc::available(macos = 26.0)]
    pub fn with_count(count: ns::Integer) -> arc::R<Self>;

    #[objc::msg_send(badgeWithText:)]
    #[objc::available(macos = 26.0)]
    pub fn with_text(text: &ns::String) -> arc::R<Self>;

    #[objc::msg_send(indicatorBadge)]
    #[objc::available(macos = 26.0)]
    pub fn indicator() -> arc::R<Self>;

    #[objc::msg_send(text)]
    #[objc::available(macos = 26.0)]
    pub fn text(&self) -> arc::R<ns::String>;
}

unsafe extern "C" {
    static NS_ITEM_BADGE: &'static objc::Class<ItemBadge>;
}
