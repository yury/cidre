use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIDragItem")]
    pub DragItem(ns::Id)
);

impl DragItem {
    /// What the item carries, loaded on demand.
    #[objc::msg_send(itemProvider)]
    pub fn item_provider(&self) -> arc::R<ns::ItemProvider>;
}
