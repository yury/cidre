use crate::{arc, define_obj_type, ns, objc};

#[doc(alias = "NSControlSize")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ControlSize {
    Regular = 0,
    Small = 1,
    Mini = 2,
    Large = 3,
}

#[doc(alias = "NSControlTint")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
#[repr(usize)]
pub enum ControlTint {
    Default = 0,
    Blue = 1,
    Graphite = 6,
    Clear = 7,
}

#[doc(alias = "NSCellType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum CellType {
    Null = 0,
    Text = 1,
    Image = 2,
}

impl arc::A<Cell> {
    #[objc::msg_send(initTextCell:)]
    pub fn init_text_cell(self, string: &ns::String) -> arc::R<Cell>;

    #[objc::msg_send(initImageCell:)]
    pub fn init_image_cell(self, image: Option<&ns::Image>) -> arc::R<Cell>;
}

define_obj_type!(
    #[doc(alias = "NSCell")]
    pub Cell(ns::Id),
    NS_CELL
);

impl Cell {
    #[inline]
    pub fn with_text(text: &ns::String) -> arc::R<Self> {
        Self::alloc().init_text_cell(text)
    }

    #[inline]
    pub fn with_image(image: Option<&ns::Image>) -> arc::R<Self> {
        Self::alloc().init_image_cell(image)
    }

    #[objc::msg_send(type)]
    pub fn type_(&self) -> CellType;

    #[objc::msg_send(tag)]
    pub fn tag(&self) -> isize;

    #[objc::msg_send(setTag:)]
    pub fn set_tag(&mut self, val: isize);
}

#[doc(alias = "NSCellImagePosition")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum CellImagePos {
    #[doc(alias = "NSNoImage")]
    No = 0,

    #[doc(alias = "NSImageOnly")]
    Only = 1,

    #[doc(alias = "NSImageLeft")]
    Left = 2,

    #[doc(alias = "NSImageRight")]
    Right = 3,

    #[doc(alias = "NSImageBelow")]
    Below = 4,

    #[doc(alias = "NSImageAbove")]
    Above = 5,

    #[doc(alias = "NSImageOverlaps")]
    Overlaps = 6,

    #[doc(alias = "NSImageLeading")]
    Leading = 7,

    Trailing = 8,
}

#[doc = "NSImageScaling"]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ImageScaling {
    ProportionallyDown = 0,
    AxesIndependently = 1,
    None = 2,
    ProportionallyUpOrDown = 3,
}

#[doc(alias = "NSControlStateValue")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ControlStateValue(pub isize);

impl ControlStateValue {
    pub const MIXED: Self = Self(-1);
    pub const OFF: Self = Self(0);
    pub const ON: Self = Self(1);
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_CELL: &'static objc::Class<Cell>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let cell = ns::Cell::with_image(None);
        assert_eq!(ns::CellType::Null, cell.type_());

        let text = ns::String::with_str("text");

        let cell = ns::Cell::with_text(&text);
        assert_eq!(ns::CellType::Text, cell.type_());
    }
}
