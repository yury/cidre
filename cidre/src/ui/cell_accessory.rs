use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UICellAccessory")]
    pub CellAccessory(ns::Id)
);

#[doc(alias = "UICellAccessoryOutlineDisclosureStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CellAccessoryOutlineDisclosureStyle {
    /// Determined by whether the cell is configured to be a section header.
    Automatic = 0,
    /// A tap anywhere in the header toggles the expansion state; the cell cannot be selected.
    Header = 1,
    /// Only a tap on the accessory toggles the expansion state; the cell can be selected.
    Cell = 2,
}

define_obj_type!(
    #[doc(alias = "UICellAccessoryOutlineDisclosure")]
    pub CellAccessoryOutlineDisclosure(CellAccessory),
    sym UICellAccessoryOutlineDisclosure
);

impl CellAccessoryOutlineDisclosure {
    #[objc::msg_send(style)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn style(&self) -> CellAccessoryOutlineDisclosureStyle;

    #[objc::msg_send(setStyle:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_style(&mut self, val: CellAccessoryOutlineDisclosureStyle);
}

define_obj_type!(
    #[doc(alias = "UICellAccessoryDisclosureIndicator")]
    pub CellAccessoryDisclosureIndicator(CellAccessory),
    sym UICellAccessoryDisclosureIndicator
);

#[doc(alias = "UICellAccessoryDisplayedState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CellAccessoryDisplayedState {
    Always = 0,
    WhenEditing = 1,
    WhenNotEditing = 2,
}

impl CellAccessory {
    #[objc::msg_send(setDisplayedState:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_displayed_state(&mut self, val: CellAccessoryDisplayedState);

    #[objc::msg_send(setTintColor:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_tint_color(&mut self, val: Option<&ui::Color>);
}

define_obj_type!(
    #[doc(alias = "UICellAccessoryCheckmark")]
    pub CellAccessoryCheckmark(CellAccessory),
    sym UICellAccessoryCheckmark
);

define_obj_type!(
    #[doc(alias = "UICellAccessoryDelete")]
    pub CellAccessoryDelete(CellAccessory),
    sym UICellAccessoryDelete
);

define_obj_type!(
    #[doc(alias = "UICellAccessoryDetail")]
    pub CellAccessoryDetail(CellAccessory),
    sym UICellAccessoryDetail
);

impl CellAccessoryDetail {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(setActionHandler:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_action_handler(&mut self, val: Option<&mut crate::blocks::EscBlock<fn()>>);
}
