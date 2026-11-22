use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIListSeparatorConfiguration")]
    pub ListSeparatorCfg(ns::Id)
);

#[doc(alias = "UIListSeparatorVisibility")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ListSeparatorVisibility {
    Automatic,
    Visible,
    Hidden,
}

impl ListSeparatorCfg {
    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<Self>;

    #[objc::msg_send(setTopSeparatorVisibility:)]
    #[objc::available(ios = 14.5)]
    pub fn set_top_separator_visibility(&mut self, val: ListSeparatorVisibility);

    #[objc::msg_send(setBottomSeparatorVisibility:)]
    #[objc::available(ios = 14.5)]
    pub fn set_bottom_separator_visibility(&mut self, val: ListSeparatorVisibility);
}
