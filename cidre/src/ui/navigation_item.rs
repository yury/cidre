use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UINavigationItem")]
    pub NavItem(ns::Id),
    UI_NAVIGATION_ITEM
);

impl NavItem {
    #[objc::msg_send(setRightBarButtonItems:)]
    pub fn set_right_bar_button_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);

    #[objc::msg_send(setLeftBarButtonItems:)]
    pub fn set_left_bar_button_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);
}

unsafe extern "C" {
    static UI_NAVIGATION_ITEM: &'static objc::Class<NavItem>;
}
