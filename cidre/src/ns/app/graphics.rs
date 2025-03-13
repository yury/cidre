use crate::cg;

#[doc(alias = "NSWindowOrderingMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum WindowOrderingMode {
    Above = 1,
    Below = -1,
    Out = 0,
}

#[doc(alias = "NSWindowDepth")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct WindowDepth(pub i32);

impl WindowDepth {
    pub const RGB_24_BIT: Self = Self(0x208);
    pub const RGB_64_BIT: Self = Self(0x210);
    pub const RGB_128_BIT: Self = Self(0x220);
}

#[doc(alias = "NSEdgeInsets")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct EdgeInsets {
    pub top: cg::Float,
    pub left: cg::Float,
    pub bottom: cg::Float,
    pub right: cg::Float,
}
