use crate::{arc, define_obj_type, define_opts, ns, objc};

#[doc(alias = "UIMenuElementState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum MenuElementState {
    Off = 0,
    On = 1,
    Mixed = 2,
}

define_opts!(
    #[doc(alias = "UIMenuElementAttributes")]
    pub MenuElementAttrs(usize)
);

impl MenuElementAttrs {
    pub const DISABLED: Self = Self(1 << 0);
    pub const DESTRUCTIVE: Self = Self(1 << 1);
    pub const HIDDEN: Self = Self(1 << 2);
    /// The menu stays open after the element is tapped.
    pub const KEEPS_MENU_PRESENTED: Self = Self(1 << 3);
}

define_obj_type!(
    #[doc(alias = "UIMenuElement")]
    pub MenuElement(ns::Id)
);

impl MenuElement {
    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    /// Shown under the title; for a menu, under its title where the menu shows one.
    #[objc::msg_send(subtitle)]
    #[objc::available(ios = 15.0)]
    pub fn subtitle(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setSubtitle:)]
    #[objc::available(ios = 15.0)]
    pub fn set_subtitle(&mut self, val: Option<&ns::String>);
}
