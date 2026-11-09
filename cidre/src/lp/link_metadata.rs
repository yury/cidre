use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "LPLinkMetadata")]
    pub LinkMetadata(ns::Id),
    sym LPLinkMetadata
);

impl LinkMetadata {
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(iconProvider)]
    pub fn icon_provider(&self) -> Option<arc::R<ns::ItemProvider>>;

    #[objc::msg_send(setIconProvider:)]
    pub fn set_icon_provider(&mut self, val: Option<&ns::ItemProvider>);
}
