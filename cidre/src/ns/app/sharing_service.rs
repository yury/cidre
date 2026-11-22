use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSharingServicePicker")]
    pub SharingServicePicker(ns::Id), sym NSSharingServicePicker
);

impl SharingServicePicker {
    #[objc::init(initWithItems:)]
    pub fn init_with_items(self, items: &ns::Array<ns::Id>) -> arc::R<SharingServicePicker>;

    #[objc::msg_send(standardShareMenuItem)]
    #[objc::available(macos = 13.0)]
    pub fn standard_share_menu_item(&self) -> arc::R<ns::MenuItem>;
}
