use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITabSidebarItemRequest")]
    pub TabSidebarItemRequest(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UITabSidebarItem")]
    pub TabSidebarItem(ns::Id),
    sym UITabSidebarItem
);

impl TabSidebarItem {
    #[objc::msg_send(itemFromRequest:)]
    #[objc::available(ios = 18.0, visionos = 2.0)]
    pub fn from_request(request: &TabSidebarItemRequest) -> arc::R<Self>;

    #[objc::msg_send(tab)]
    #[objc::available(ios = 18.0, visionos = 2.0)]
    pub fn tab(&self) -> Option<arc::R<ui::Tab>>;

    #[objc::msg_send(setAccessories:)]
    #[objc::available(ios = 18.0, visionos = 2.0)]
    pub fn set_accessories(&mut self, accessories: &ns::Array<ui::CellAccessory>);
}
