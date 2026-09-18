use crate::{define_obj_type, ns};

/// The macOS values: right is 1 and center is 2, the other way round from iOS.
#[doc(alias = "NSTextAlignment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TextAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
    Justified = 3,
    Natural = 4,
}

define_obj_type!(
    #[doc(alias = "NSText")]
    pub Text(ns::View)
);
