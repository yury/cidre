use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSharingServicePicker")]
    pub SharingServicePicker(ns::Id), sym NSSharingServicePicker
);

impl SharingServicePicker {
    #[objc::init(initWithItems:)]
    pub fn init_with_items(self, items: &ns::Array<ns::Id>) -> arc::R<SharingServicePicker>;

    /// Shows the services for the items in a menu next to `rect` of `view`.
    #[objc::msg_send(showRelativeToRect:ofView:preferredEdge:)]
    pub fn show_relative_to_rect(&self, rect: ns::Rect, view: &ns::View, edge: ns::RectEdge);

    #[objc::msg_send(standardShareMenuItem)]
    #[objc::available(macos = 13.0)]
    pub fn standard_share_menu_item(&self) -> arc::R<ns::MenuItem>;
}
