use crate::{define_obj_type, ns};

#[doc(alias = "UIMenuElementState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum MenuElementState {
    Off = 0,
    On = 1,
    Mixed = 2,
}

define_obj_type!(
    #[doc(alias = "UIMenuElement")]
    pub MenuElement(ns::Id)
);
