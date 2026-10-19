use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UINavigationItemLargeTitleDisplayMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum NavItemLargeTitleDisplayMode {
    /// Inherits the large title display from the previous item in the navigation stack.
    Automatic = 0,
    /// Always displays a large title, when the navigation bar prefers large titles.
    Always = 1,
    /// Never displays a large title.
    Never = 2,
    /// The title in the bar inline, with no room kept for a large one.
    Inline = 3,
}

define_obj_type!(
    #[doc(alias = "UINavigationItem")]
    pub NavItem(ns::Id),
    sym UINavigationItem
);

impl NavItem {
    #[objc::msg_send(largeTitleDisplayMode)]
    #[objc::available(ios = 11.0)]
    pub fn large_title_display_mode(&self) -> NavItemLargeTitleDisplayMode;

    #[objc::msg_send(setLargeTitleDisplayMode:)]
    #[objc::available(ios = 11.0)]
    pub fn set_large_title_display_mode(&mut self, val: NavItemLargeTitleDisplayMode);

    #[objc::msg_send(setRightBarButtonItems:)]
    pub fn set_right_bar_button_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);

    #[objc::msg_send(setLeftBarButtonItems:)]
    pub fn set_left_bar_button_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);

    /// The line under the title in the navigation bar.
    #[objc::msg_send(subtitle)]
    #[objc::available(ios = 26.0)]
    pub fn subtitle(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setSubtitle:)]
    #[objc::available(ios = 26.0)]
    pub fn set_subtitle(&mut self, val: Option<&ns::String>);

    /// How the navigation bar minimizes in response to scrolling; an
    /// integrated top tab bar minimizes with it.
    #[objc::msg_send(navigationBarMinimization)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn nav_bar_minimization(&self) -> arc::R<ui::BarMinimization>;

    #[objc::msg_send(setNavigationBarMinimization:)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn set_nav_bar_minimization(&mut self, val: &ui::BarMinimization);
}
