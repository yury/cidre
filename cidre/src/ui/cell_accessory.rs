use crate::{arc, define_obj_type, ns, objc};

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
    UI_CELL_ACCESSORY_OUTLINE_DISCLOSURE
);

impl CellAccessoryOutlineDisclosure {
    #[objc::msg_send(style)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn style(&self) -> CellAccessoryOutlineDisclosureStyle;

    #[objc::msg_send(setStyle:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_style(&mut self, val: CellAccessoryOutlineDisclosureStyle);
}

unsafe extern "C" {
    static UI_CELL_ACCESSORY_OUTLINE_DISCLOSURE:
        &'static objc::Class<CellAccessoryOutlineDisclosure>;
}
