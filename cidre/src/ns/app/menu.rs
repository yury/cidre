use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSMenu")]
    pub Menu(ns::Id),
    NS_MENU
);

impl arc::A<Menu> {
    #[objc::msg_send(initWithTitle:)]
    pub fn init_with_title(self, title: &ns::String) -> arc::R<Menu>;
}

impl Menu {
    pub fn with_title(title: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_title(title)
    }

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(insertItem:atIndex:)]
    pub fn insert_item_at(&mut self, item: &ns::MenuItem, index: ns::Integer);

    #[objc::msg_send(addItem:)]
    pub fn add_item(&mut self, item: &ns::MenuItem);

    /// Creates, adds and returns an item; `action` is sent up the responder chain when
    /// the item has no target.
    #[objc::msg_send(addItemWithTitle:action:keyEquivalent:)]
    pub fn add_item_with_title_action_key_equivalent(
        &mut self,
        title: &ns::String,
        action: Option<&objc::Sel>,
        key_equivalent: &ns::String,
    ) -> arc::R<ns::MenuItem>;

    #[objc::msg_send(removeItem:)]
    pub fn remove_item(&mut self, item: &ns::MenuItem);

    #[objc::msg_send(removeItemAtIndex:)]
    pub fn remove_item_at(&mut self, index: ns::Integer);

    #[objc::msg_send(removeAllItems)]
    pub fn remove_all_items(&mut self);

    #[objc::msg_send(setSubmenu:forItem:)]
    pub fn set_submenu_for_item(&mut self, submenu: Option<&ns::Menu>, item: &ns::MenuItem);

    #[objc::msg_send(itemArray)]
    pub fn item_array(&self) -> arc::R<ns::Array<ns::MenuItem>>;

    #[objc::msg_send(numberOfItems)]
    pub fn number_of_items(&self) -> ns::Integer;

    #[objc::msg_send(itemAtIndex:)]
    pub fn item_at(&self, index: ns::Integer) -> Option<arc::R<ns::MenuItem>>;

    #[objc::msg_send(itemWithTag:)]
    pub fn item_with_tag(&self, tag: ns::Integer) -> Option<arc::R<ns::MenuItem>>;

    #[objc::msg_send(autoenablesItems)]
    pub fn autoenables_items(&self) -> bool;

    #[objc::msg_send(setAutoenablesItems:)]
    pub fn set_autoenables_items(&mut self, val: bool);

    #[objc::msg_send(supermenu)]
    pub fn supermenu(&self) -> Option<arc::R<ns::Menu>>;
}

unsafe extern "C" {
    static NS_MENU: &'static objc::Class<Menu>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut menu = ns::Menu::with_title(ns::str!(c"Copy"));
        assert_eq!(menu.title().as_ref(), "Copy");

        menu.set_title(ns::str!(c""));
        assert!(menu.title().is_empty());

        let quit =
            menu.add_item_with_title_action_key_equivalent(ns::str!(c"Quit"), None, ns::str!(c"q"));
        assert_eq!(menu.number_of_items(), 1);
        assert_eq!(quit.title().as_ref(), "Quit");
        assert_eq!(quit.key_equivalent().as_ref(), "q");
        assert!(quit.menu().is_some());

        let sep = ns::MenuItem::separator_item();
        assert!(sep.is_separator_item());
        menu.add_item(&sep);
        assert_eq!(menu.number_of_items(), 2);

        let mut sub = ns::Menu::with_title(ns::str!(c"Sub"));
        let item =
            ns::MenuItem::with_title_action_key_equivalent(ns::str!(c"Sub"), None, ns::str!(c""));
        sub.add_item(&item);
        assert_eq!(sub.item_array().len(), 1);
        menu.remove_all_items();
        assert_eq!(menu.number_of_items(), 0);
    }
}
