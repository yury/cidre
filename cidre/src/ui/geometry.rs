use crate::cg;

#[doc(alias = "UIEdgeInsets")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct EdgeInsets {
    pub top: cg::Float,
    pub left: cg::Float,
    pub bottom: cg::Float,
    pub right: cg::Float,
}

#[doc(alias = "UIOffset")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Offset {
    pub horizontal: cg::Float,
    pub vertical: cg::Float,
}
