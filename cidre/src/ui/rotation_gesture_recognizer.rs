use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIRotationGestureRecognizer")]
    pub RotationGestureRecognizer(ui::GestureRecognizer),
    UI_ROTATION_GESTURE_RECOGNIZER
);

impl RotationGestureRecognizer {
    #[objc::init(initWithTarget:action:)]
    pub fn init_with_target_action(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<RotationGestureRecognizer>;

    pub fn with_target_action(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target_action(target, action)
    }

    /// Rotation in radians since the gesture began.
    #[objc::msg_send(rotation)]
    pub fn rotation(&self) -> cg::Float;

    #[objc::msg_send(setRotation:)]
    pub fn set_rotation(&mut self, val: cg::Float);

    /// Velocity of the rotation in radians/second.
    #[objc::msg_send(velocity)]
    pub fn velocity(&self) -> cg::Float;
}

unsafe extern "C" {
    static UI_ROTATION_GESTURE_RECOGNIZER: &'static objc::Class<RotationGestureRecognizer>;
}
