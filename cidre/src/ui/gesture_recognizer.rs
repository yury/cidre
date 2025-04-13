use crate::{arc, cg, define_obj_type, ns, objc, ui};

#[doc(alias = "UIGestureRecognizerState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(isize)]
pub enum GestureRecognizerState {
    Possible,
    Began,
    Changed,
    Ended, // Recognized
    Cancelled,
    Failed,
}

define_obj_type!(
    #[doc(alias = "UIGestureRecognizer")]
    pub GestureRecognizer(ns::Id)
);

impl GestureRecognizer {
    #[objc::msg_send(state)]
    pub fn state(&self) -> GestureRecognizerState;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyGestureRecognizerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: GestureRecognizerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    /// Default is true. disabled gesture recognizers will not receive touches. when changed to false the gesture recognizer will be
    /// cancelled if it's currently recognizing a gesture
    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(view)]
    pub fn view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(cancelsTouchesInView)]
    pub fn cancels_touches_in_view(&self) -> bool;

    #[objc::msg_send(setCancelsTouchesInView:)]
    pub fn set_cancels_touches_in_view(&mut self, val: bool);

    #[objc::msg_send(delaysTouchesBegan)]
    pub fn delays_touches_began(&self) -> bool;

    #[objc::msg_send(setDelaysTouchesBegan:)]
    pub fn set_delays_touches_began(&mut self, val: bool);

    #[objc::msg_send(delaysTouchesEnded)]
    pub fn delays_touches_ended(&self) -> bool;

    #[objc::msg_send(setDelaysTouchesEnded:)]
    pub fn set_delays_touches_ended(&mut self, val: bool);

    #[objc::msg_send(allowedTouchTypes)]
    pub fn allowed_touch_types(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(setAllowedTouchTypes:)]
    pub fn set_allowed_touch_types(&mut self, val: &ns::Array<ns::Number>);

    #[objc::msg_send(allowedPressTypes)]
    pub fn allowed_press_types(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(setAllowedPressTypes:)]
    pub fn set_allowed_press_types(&mut self, val: &ns::Array<ns::Number>);

    #[objc::msg_send(requiresExclusiveTouchType)]
    pub fn requires_exclusive_touch_type(&self) -> bool;

    #[objc::msg_send(setRequiresExclusiveTouchType:)]
    pub fn set_requires_exclusive_touch_type(&mut self, val: bool);

    #[objc::msg_send(requireGestureRecognizerToFail:)]
    pub fn require_gesture_recognizer_to_fail(&mut self, val: &ui::GestureRecognizer);

    #[objc::msg_send(locationInView:)]
    pub fn location_in_view(&self, view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(numberOfTouches)]
    pub fn number_of_touches(&self) -> usize;

    #[objc::msg_send(locationOfTouch:inView:)]
    pub fn location_of_touch(&self, touch_index: usize, in_view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    // #[objc::msg_send(modifierFlags)]
    // pub fn modifier_flags(&self) -> ui::KeyModifierFlags;
}

#[objc::protocol(UIGestureRecognizerDelegate)]
pub trait GestureRecognizerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(gestureRecognizerShouldBegin:)]
    fn gr_should_begin(&mut self, gr: &ui::GestureRecognizer) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldRecognizeSimultaneouslyWithGestureRecognizer:)]
    fn gr_should_recognize_simultaneously_with_gr(
        &mut self,
        gr: &ui::GestureRecognizer,
        other_gr: &ui::GestureRecognizer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldRequireFailureOfGestureRecognizer:)]
    fn gr_should_require_failure_of_gr(
        &mut self,
        gr: &ui::GestureRecognizer,
        other_gr: &ui::GestureRecognizer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldBeRequiredToFailByGestureRecognizer:)]
    fn gr_should_be_required_to_fail_by_gr(
        &mut self,
        gr: &ui::GestureRecognizer,
        other_gr: &ui::GestureRecognizer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldReceiveTouch:)]
    fn gr_should_receive_touch(&mut self, gr: &ui::GestureRecognizer, touch: &ui::Touch) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldReceivePress:)]
    fn gr_should_receive_press(&mut self, gr: &ui::GestureRecognizer, touch: &ui::Press) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldReceiveEvent:)]
    fn gr_should_receive_event(&mut self, gr: &ui::GestureRecognizer, touch: &ui::Event) -> bool;
}

define_obj_type!(
    pub AnyGestureRecognizerDelegate(ns::Id)
);

impl GestureRecognizerDelegate for AnyGestureRecognizerDelegate {}
