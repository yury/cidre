use crate::{arc, define_obj_type, define_opts, ns, objc, ui};

define_opts!(
    #[doc(alias = "UISwipeGestureRecognizerDirection")]
    pub SwipeGestureRecognizerDirection(usize)
);

impl SwipeGestureRecognizerDirection {
    pub const RIGHT: Self = Self(1 << 0);
    pub const LEFT: Self = Self(1 << 1);
    pub const UP: Self = Self(1 << 2);
    pub const DOWN: Self = Self(1 << 3);
}

define_obj_type!(
    #[doc(alias = "UISwipeGestureRecognizer")]
    pub SwipeGestureRecognizer(ui::GestureRecognizer),
    UI_SWIPE_GESTURE_RECOGNIZER
);

impl SwipeGestureRecognizer {
    #[objc::init(initWithTarget:action:)]
    pub fn init_with_target_action(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<SwipeGestureRecognizer>;

    pub fn with_target_action(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target_action(target, action)
    }

    /// Default is 1. The number of fingers that must swipe.
    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(numberOfTouchesRequired)]
    pub fn number_of_touches_required(&self) -> usize;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setNumberOfTouchesRequired:)]
    pub fn set_number_of_touches_required(&mut self, val: usize);

    /// Default is `SwipeGestureRecognizerDirection::RIGHT`. The desired direction of the swipe.
    /// Multiple directions may be specified if they will result in the same behavior.
    #[objc::msg_send(direction)]
    pub fn direction(&self) -> SwipeGestureRecognizerDirection;

    #[objc::msg_send(setDirection:)]
    pub fn set_direction(&mut self, val: SwipeGestureRecognizerDirection);
}

unsafe extern "C" {
    static UI_SWIPE_GESTURE_RECOGNIZER: &'static objc::Class<SwipeGestureRecognizer>;
}
