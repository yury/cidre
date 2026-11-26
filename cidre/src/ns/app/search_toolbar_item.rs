use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    /// A toolbar item with a search field: a button the field expands from, as space and
    /// the search ask.
    #[doc(alias = "NSSearchToolbarItem")]
    pub SearchToolbarItem(ns::ToolbarItem),
    sym NSSearchToolbarItem
);

impl SearchToolbarItem {
    #[objc::init(initWithItemIdentifier:)]
    pub fn init_with_item_id(self, id: &ns::ToolbarItemId) -> arc::R<SearchToolbarItem>;

    pub fn with_id(id: &ns::ToolbarItemId) -> arc::R<Self> {
        Self::alloc().init_with_item_id(id)
    }

    #[objc::msg_send(searchField)]
    pub fn search_field(&self) -> arc::R<ns::SearchField>;

    #[objc::msg_send(setSearchField:)]
    pub fn set_search_field(&mut self, val: &ns::SearchField);

    /// The field's width while it shows.
    #[objc::msg_send(preferredWidthForSearchField)]
    pub fn preferred_width_for_search_field(&self) -> cg::Float;

    #[objc::msg_send(setPreferredWidthForSearchField:)]
    pub fn set_preferred_width_for_search_field(&mut self, val: cg::Float);

    /// Whether Cancel (the field's ⓧ or Escape) also ends the search and collapses the field.
    #[objc::msg_send(resignsFirstResponderWithCancel)]
    pub fn resigns_first_responder_with_cancel(&self) -> bool;

    #[objc::msg_send(setResignsFirstResponderWithCancel:)]
    pub fn set_resigns_first_responder_with_cancel(&mut self, val: bool);

    /// Expands the field and puts the cursor in it.
    #[objc::msg_send(beginSearchInteraction)]
    pub fn begin_search_interaction(&mut self);

    #[objc::msg_send(endSearchInteraction)]
    pub fn end_search_interaction(&mut self);
}
