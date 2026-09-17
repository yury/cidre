use crate::{cg, define_opts};

define_opts!(
    /// The edges of a rectangle, leading and trailing following the layout
    /// direction.
    #[doc(alias = "NSDirectionalRectEdge")]
    pub DirectionalRectEdge(usize)
);

impl DirectionalRectEdge {
    pub const NONE: Self = Self(0);
    pub const TOP: Self = Self(1 << 0);
    pub const LEADING: Self = Self(1 << 1);
    pub const BOTTOM: Self = Self(1 << 2);
    pub const TRAILING: Self = Self(1 << 3);
    pub const ALL: Self = Self(Self::TOP.0 | Self::LEADING.0 | Self::BOTTOM.0 | Self::TRAILING.0);
}

define_opts!(
    /// The axes of a layout.
    #[doc(alias = "UIAxis")]
    pub Axis(usize)
);

impl Axis {
    pub const NEITHER: Self = Self(0);
    pub const HORIZONTAL: Self = Self(1 << 0);
    pub const VERTICAL: Self = Self(1 << 1);
    pub const BOTH: Self = Self(Self::HORIZONTAL.0 | Self::VERTICAL.0);
}

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
    pub const fn new(
        top: cg::Float,
        leading: cg::Float,
        bottom: cg::Float,
        trailing: cg::Float,
    ) -> Self {
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
