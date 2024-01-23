use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSDockTile")]
    pub DockTile(ns::Id)
);

impl DockTile {
    #[objc::msg_send(size)]
    pub fn size(&self) -> ns::Size;

    #[objc::msg_send(contentView)]
    pub fn content_view(&self) -> Option<&ns::View>;

    #[objc::msg_send(setContentView:)]
    pub fn set_content_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(display)]
    pub fn display(&mut self);

    #[objc::msg_send(showsApplicationBadge)]
    pub fn shows_app_badge(&self) -> bool;

    #[objc::msg_send(setShowsApplicationBadge:)]
    pub fn set_shows_app_badge(&mut self, val: bool);

    #[objc::msg_send(badgeLabel)]
    pub fn badge_label(&self) -> Option<&ns::String>;

    #[objc::msg_send(setBadgeLabel:)]
    pub fn set_badge_label(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(owner)]
    pub fn owner(&self) -> &ns::Id;
}
