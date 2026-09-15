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

#[doc(alias = "NSDirectionalEdgeInsets")]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[repr(C)]
pub struct DirectionalEdgeInsets {
    pub top: cg::Float,
    pub leading: cg::Float,
    pub bottom: cg::Float,
    pub trailing: cg::Float,
}

impl DirectionalEdgeInsets {
    pub const fn new(top: cg::Float, leading: cg::Float, bottom: cg::Float, trailing: cg::Float) -> Self {
        Self {
            top,
            leading,
            bottom,
            trailing,
        }
    }
}

#[doc(alias = "UIOffset")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Offset {
    pub horizontal: cg::Float,
    pub vertical: cg::Float,
}
