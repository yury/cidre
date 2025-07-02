use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSToolbarIdentifier")]
    pub ToolbarId(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSToolbarItemIdentifier")]
    pub ToolbarItemId(ns::String)
);

#[doc(alias = "NSToolbarDisplayMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(usize)]
pub enum ToolbarDisplayMode {
    Default,
    IconAndLabel,
    IconOnly,
    LabelOnly,
}

define_obj_type!(
    #[doc(alias = "NSToolbar")]
    pub Toolbar(ns::Id),
    NS_TOOLBAR
);

impl arc::A<Toolbar> {
    #[objc::msg_send(initWithIdentifier:)]
    pub fn init_with_id(self, id: &ns::ToolbarId) -> arc::R<Toolbar>;
}

impl Toolbar {
    pub fn with_id(id: &ns::ToolbarId) -> arc::R<Self> {
        Self::alloc().init_with_id(id)
    }

    #[objc::msg_send(insertItemWithItemIdentifier:atIndex:)]
    pub fn insert_item_with_id(&mut self, item_id: &ns::ToolbarItemId, index: isize);

    #[objc::msg_send(removeItemAtIndex:)]
    pub fn remove_item(&mut self, index: isize);

    #[objc::msg_send(removeItemWithItemIdentifier:)]
    pub fn remove_item_with_id(&mut self, item_id: &ns::ToolbarItemId);
}

#[objc::protocol(NSToolbarDelegate)]
pub trait ToolbarDelegate {
    #[objc::optional]
    #[objc::msg_send(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
    fn toolbar_item_for_id_will_be_inserted_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
        id: &ns::ToolbarItemId,
        flag: bool,
    ) -> Option<arc::Rar<ns::ToolbarItem>>;

    #[objc::optional]
    #[objc::msg_send(toolbarDefaultItemIdentifiers:)]
    fn toolbar_default_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbarAllowedItemIdentifiers:)]
    fn toolbar_allowed_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbarSelectableItemIdentifiers:)]
    fn toolbar_selectable_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbarImmovableItemIdentifiers:)]
    fn toolbar_immovable_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbar:itemIdentifier:canBeInsertedAtIndex:)]
    fn toolbar_item_id_can_be_inserted(
        &mut self,
        toolbar: &mut ns::Toolbar,
        item_id: &ns::ToolbarId,
        index: isize,
    ) -> bool;

    #[objc::msg_send(toolbarWillAddItem:)]
    fn toolbar_will_add_item(&mut self, n: &ns::Notification);

    #[objc::msg_send(toolbarDidRemoveItem:)]
    fn toolbar_did_remove_item(&mut self, n: &ns::Notification);
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_TOOLBAR: &'static objc::Class<Toolbar>;
}
