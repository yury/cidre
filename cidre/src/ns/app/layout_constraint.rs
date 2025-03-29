#[doc(alias = "NSLayoutPriority")]
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
#[repr(transparent)]
pub struct LayoutPriority(pub f32);

impl LayoutPriority {
    #[doc(alias = "NSLayoutPriorityRequired")]
    pub const REQUIRED: Self = Self(1000.0);

    #[doc(alias = "NSLayoutPriorityDefaultHigh")]
    pub const DEFAULT_HIGH: Self = Self(750.0);

    #[doc(alias = "NSLayoutPriorityDragThatCanResizeWindow")]
    pub const DRAG_THAT_CAN_RESIZE_WINDOW: Self = Self(510.0);

    #[doc(alias = "NSLayoutPriorityWindowSizeStayPut")]
    pub const WINDOW_SIZE_STAY_PUT: Self = Self(500.0);

    #[doc(alias = "NSLayoutPriorityDragThatCannotResizeWindow")]
    pub const DRAG_THAT_CANNOT_RESIZE_WINDOW: Self = Self(490.0);

    #[doc(alias = "NSLayoutPriorityDefaultLow")]
    pub const DEFAULT_LOW: Self = Self(250.0);

    #[doc(alias = "NSLayoutPriorityFittingSizeCompression")]
    pub const FITTING_SIZE_COMPRESSION: Self = Self(50.0);
}

#[doc(alias = "NSLayoutAttribute")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum LayoutAttr {
    Left = 1,
    Right,
    Top,
    Bottom,
    Leading,
    Trailing,
    Width,
    Height,
    CenterX,
    CenterY,
    LastBaseline,
    _Unused,
    FirstBaseline,
    LeftMargin,
    RightMargin,
    TopMargin,
    BottomMargin,
    LeadingMargin,
    TrailingMargin,
    CenterXWithinMargins,
    CenterYWithinMargins,
    NotAnAttribute = 0,
}
