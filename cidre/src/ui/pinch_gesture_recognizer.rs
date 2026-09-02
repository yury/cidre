use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIPinchGestureRecognizer")]
    pub PinchGestureRecognizer(ui::GestureRecognizer),
    UI_PINCH_GESTURE_RECOGNIZER
);

impl arc::A<PinchGestureRecognizer> {
    #[objc::msg_send(initWithTarget:action:)]
    pub fn init_with_target_action(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<PinchGestureRecognizer>;
}

impl PinchGestureRecognizer {
    pub fn with_target_action(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target_action(target, action)
    }

    /// Scale relative to the touch points in screen coordinates.
    #[objc::msg_send(scale)]
    pub fn scale(&self) -> cg::Float;

    #[objc::msg_send(setScale:)]
    pub fn set_scale(&mut self, val: cg::Float);

    /// Velocity of the pinch in scale/second.
    #[objc::msg_send(velocity)]
    pub fn velocity(&self) -> cg::Float;
}

unsafe extern "C" {
    static UI_PINCH_GESTURE_RECOGNIZER: &'static objc::Class<PinchGestureRecognizer>;
}
