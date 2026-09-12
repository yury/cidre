use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    /// A toolbar item that shows a menu when clicked.
    #[doc(alias = "NSMenuToolbarItem")]
    pub MenuToolbarItem(ns::ToolbarItem),
    NS_MENU_TOOLBAR_ITEM
);

impl MenuToolbarItem {
    #[objc::init(initWithItemIdentifier:)]
    pub fn init_with_item_id(self, id: &ns::ToolbarItemId) -> arc::R<MenuToolbarItem>;

    pub fn with_id(id: &ns::ToolbarItemId) -> arc::R<Self> {
        Self::alloc().init_with_item_id(id)
    }

    #[objc::msg_send(menu)]
    pub fn menu(&self) -> arc::R<ns::Menu>;

    #[objc::msg_send(setMenu:)]
    pub fn set_menu(&mut self, val: &ns::Menu);

    /// Whether the item shows the menu indicator (a chevron). `true` by default.
    #[objc::msg_send(showsIndicator)]
    pub fn shows_indicator(&self) -> bool;

    #[objc::msg_send(setShowsIndicator:)]
    pub fn set_shows_indicator(&mut self, val: bool);
}

unsafe extern "C" {
    static NS_MENU_TOOLBAR_ITEM: &'static objc::Class<MenuToolbarItem>;
}
