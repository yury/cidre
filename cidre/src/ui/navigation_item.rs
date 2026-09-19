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

    /// How the navigation bar minimizes in response to scrolling; an
    /// integrated top tab bar minimizes with it.
    #[objc::msg_send(navigationBarMinimization)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn nav_bar_minimization(&self) -> arc::R<ui::BarMinimization>;

    #[objc::msg_send(setNavigationBarMinimization:)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn set_nav_bar_minimization(&mut self, val: &ui::BarMinimization);
}

unsafe extern "C" {
    static UI_NAVIGATION_ITEM: &'static objc::Class<NavItem>;
}
