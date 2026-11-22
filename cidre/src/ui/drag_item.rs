use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIDragItem")]
    pub DragItem(ns::Id)
);

impl DragItem {
    #[cfg(target_os = "ios")]
    crate::define_cls!(sym UIDragItem);

    #[cfg(target_os = "ios")]
    #[objc::init(initWithItemProvider:)]
    pub fn init_with_item_provider(self, provider: &ns::ItemProvider) -> arc::R<DragItem>;

    /// What the item carries, loaded on demand.
    #[objc::msg_send(itemProvider)]
    pub fn item_provider(&self) -> arc::R<ns::ItemProvider>;
}
