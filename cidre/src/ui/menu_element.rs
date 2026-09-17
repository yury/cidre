use crate::{define_obj_type, define_opts, ns};

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
