use crate::{arc, cg, define_obj_type, ns, objc, ui};

/// What a drag and a drop session share: the items, and where the touch is.
#[objc::protocol(UIDropSession)]
pub trait DropSession: objc::Obj {
    #[objc::msg_send(items)]
    fn items(&self) -> arc::R<ns::Array<ui::DragItem>>;

    #[objc::msg_send(locationInView:)]
    fn location_in_view(&self, view: &ui::View) -> cg::Point;

    /// Whether any item has one of the type identifiers, or a type conforming to one.
    #[objc::msg_send(hasItemsConformingToTypeIdentifiers:)]
    fn has_items_conforming_to_type_ids(&self, type_ids: &ns::Array<ns::String>) -> bool;

    /// Whether the drag started in this app.
    #[objc::msg_send(localDragSession)]
    fn local_drag_session(&self) -> Option<arc::R<ui::AnyDragSession>>;
}

define_obj_type!(
    pub AnyDropSession(ns::Id)
);

impl DropSession for AnyDropSession {}
