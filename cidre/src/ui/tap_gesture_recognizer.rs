use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITapGestureRecognizer")]
    pub TapGestureRecognizer(ui::GestureRecognizer),
    UI_TAP_GESTURE_RECOGNIZER
);

impl arc::A<TapGestureRecognizer> {
    #[objc::msg_send(initWithTarget:action:)]
    pub fn init_with_target_action(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<TapGestureRecognizer>;
}

impl TapGestureRecognizer {
    pub fn with_target_action(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target_action(target, action)
    }

    #[objc::msg_send(numberOfTapsRequired)]
    pub fn number_of_taps_required(&self) -> usize;

    #[objc::msg_send(setNumberOfTapsRequired:)]
    pub fn set_number_of_taps_required(&mut self, val: usize);

    #[objc::msg_send(numberOfTouchesRequired)]
    pub fn number_of_touches_required(&self) -> usize;

    #[objc::msg_send(setNumberOfTouchesRequired:)]
    pub fn set_number_of_touches_required(&mut self, val: usize);
}

unsafe extern "C" {
    static UI_TAP_GESTURE_RECOGNIZER: &'static objc::Class<TapGestureRecognizer>;
}
