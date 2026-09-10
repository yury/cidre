use crate::ns;

/// Window-related priorities, on macOS only.
impl ns::LayoutPriority {
    #[doc(alias = "NSLayoutPriorityDragThatCanResizeWindow")]
    pub const DRAG_THAT_CAN_RESIZE_WINDOW: Self = Self(510.0);

    #[doc(alias = "NSLayoutPriorityWindowSizeStayPut")]
    pub const WINDOW_SIZE_STAY_PUT: Self = Self(500.0);

    #[doc(alias = "NSLayoutPriorityDragThatCannotResizeWindow")]
    pub const DRAG_THAT_CANNOT_RESIZE_WINDOW: Self = Self(490.0);
}
