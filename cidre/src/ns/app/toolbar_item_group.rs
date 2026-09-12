use crate::{arc, define_obj_type, ns, objc};

/// How a toolbar item group presents its subitems.
#[doc(alias = "NSToolbarItemGroupControlRepresentation")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum ToolbarItemGroupControlRepresentation {
    Automatic,
    Expanded,
    Collapsed,
}

/// How the subitems of a toolbar item group are selected.
#[doc(alias = "NSToolbarItemGroupSelectionMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum ToolbarItemGroupSelectionMode {
    SelectOne,
    SelectAny,
    Momentary,
}

define_obj_type!(
    /// A toolbar item that presents several items as one group.
    #[doc(alias = "NSToolbarItemGroup")]
    pub ToolbarItemGroup(ns::ToolbarItem),
    NS_TOOLBAR_ITEM_GROUP
);

impl ToolbarItemGroup {
    #[objc::init(initWithItemIdentifier:)]
    pub fn init_with_item_id(self, id: &ns::ToolbarItemId) -> arc::R<ToolbarItemGroup>;

    pub fn with_id(id: &ns::ToolbarItemId) -> arc::R<Self> {
        Self::alloc().init_with_item_id(id)
    }

    #[objc::msg_send(subitems)]
    pub fn subitems(&self) -> arc::R<ns::Array<ns::ToolbarItem>>;

    #[objc::msg_send(setSubitems:)]
    pub fn set_subitems(&mut self, val: &ns::Array<ns::ToolbarItem>);

    #[objc::msg_send(controlRepresentation)]
    #[objc::available(macos = 10.15)]
    pub fn control_representation(&self) -> ToolbarItemGroupControlRepresentation;

    #[objc::msg_send(setControlRepresentation:)]
    #[objc::available(macos = 10.15)]
    pub fn set_control_representation(&mut self, val: ToolbarItemGroupControlRepresentation);

    #[objc::msg_send(selectionMode)]
    #[objc::available(macos = 10.15)]
    pub fn selection_mode(&self) -> ToolbarItemGroupSelectionMode;

    #[objc::msg_send(setSelectionMode:)]
    #[objc::available(macos = 10.15)]
    pub fn set_selection_mode(&mut self, val: ToolbarItemGroupSelectionMode);
}

unsafe extern "C" {
    static NS_TOOLBAR_ITEM_GROUP: &'static objc::Class<ToolbarItemGroup>;
}
