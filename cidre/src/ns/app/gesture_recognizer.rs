use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum GestureRecognizerState {
    Possible,
    Began,
    Changed,
    Ended,
    Cancelled,
    Failed,
}

impl GestureRecognizerState {
    pub const RECOGNIZED: Self = Self::Ended;
}

impl arc::A<GestureRecognizer> {
    #[objc::msg_send(initWithTarget:action:)]
    pub fn init_with_target(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<GestureRecognizer>;
}

define_obj_type!(
    #[doc(alias = "NSGestureRecognizer")]
    pub GestureRecognizer(ns::Id),
    NS_GESTURE_RECOGNIZER
);

impl GestureRecognizer {
    pub fn with_target(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target(target, action)
    }

    #[objc::msg_send(target)]
    pub fn target(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setTarget:)]
    pub fn set_target(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(action)]
    pub fn action(&self) -> Option<&'static objc::Sel>;

    #[objc::msg_send(setAction:)]
    pub fn set_action(&mut self, val: Option<&objc::Sel>);

    #[objc::msg_send(state)]
    pub fn state(&self) -> GestureRecognizerState;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyGestureRecognizerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: GestureRecognizerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(view)]
    pub fn view(&self) -> Option<arc::R<ns::View>>;

    // pressureConfiguration

    #[objc::msg_send(delaysPrimaryMouseButtonEvents)]
    pub fn delays_primary_mouse_button_events(&self) -> bool;

    #[objc::msg_send(setDelaysPrimaryMouseButtonEvents:)]
    pub fn set_delays_primary_mouse_button_events(&mut self, val: bool);

    #[objc::msg_send(delaysSecondaryMouseButtonEvents)]
    pub fn delays_secondary_mouse_button_events(&self) -> bool;

    #[objc::msg_send(setDelaysSecondaryMouseButtonEvents:)]
    pub fn set_delays_secondary_mouse_button_events(&mut self, val: bool);

    #[objc::msg_send(delaysOtherMouseButtonEvents)]
    pub fn delays_other_mouse_button_events(&self) -> bool;

    #[objc::msg_send(setDelaysOtherMouseButtonEvents:)]
    pub fn set_delays_other_mouse_button_events(&mut self, val: bool);

    #[objc::msg_send(delaysKeyEvents)]
    pub fn delays_key_events(&self) -> bool;

    #[objc::msg_send(setDelaysKeyEvents:)]
    pub fn set_delays_key_events(&self, val: bool);

    #[objc::msg_send(delaysMagnificationEvents)]
    pub fn delays_magnification_events(&self) -> bool;

    #[objc::msg_send(setDelaysMagnificationEvents:)]
    pub fn set_delays_magnification_events(&mut self, val: bool);

    #[objc::msg_send(delaysRotationEvents)]
    pub fn delays_rotation_events(&self) -> bool;

    #[objc::msg_send(setDelaysRotationEvents:)]
    pub fn set_delays_rotation_events(&mut self, val: bool);

    #[objc::msg_send(locationInView:)]
    pub fn location_in_view(&self, view: Option<&ns::View>) -> ns::Point;
}

#[objc::protocol(NSGestureRecognizerDelegate)]
pub trait GestureRecognizerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldAttemptToRecognizeWithEvent:)]
    fn gr_should_attempt_to_recognize_with_event(
        &mut self,
        gr: &GestureRecognizer,
        event: &ns::Event,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizerShouldBegin:)]
    fn gr_should_begin(&mut self, gr: &GestureRecognizer) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldRecognizeSimultaneouslyWithGestureRecognizer:)]
    fn gr_should_recognize_simultaneously_with_gr(
        &mut self,
        gr: &GestureRecognizer,
        other_gr: &GestureRecognizer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldRequireFailureOfGestureRecognizer:)]
    fn gr_should_require_failure_of_gr(
        &mut self,
        gr: &GestureRecognizer,
        other_gr: &GestureRecognizer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldBeRequiredToFailByGestureRecognizer:)]
    fn gr_should_be_required_to_fail_by_gr(
        &mut self,
        gr: &GestureRecognizer,
        other_gr: &GestureRecognizer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(gestureRecognizer:shouldReceiveTouch:)]
    fn gr_should_receive_touch(&mut self, gr: &GestureRecognizer, touch: &ns::Touch) -> bool;
}

define_obj_type!(
    pub AnyGestureRecognizerDelegate(ns::Id)
);

impl GestureRecognizerDelegate for AnyGestureRecognizerDelegate {}

unsafe extern "C" {
    static NS_GESTURE_RECOGNIZER: &'static objc::Class<GestureRecognizer>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let gr = ns::GestureRecognizer::new();
        assert!(gr.is_enabled());
        assert!(gr.action().is_none());
        assert!(gr.target().is_none());
        assert!(gr.delegate().is_none());
        let gr = ns::GestureRecognizer::with_target(None, None);
        assert!(gr.is_enabled());
        assert!(gr.action().is_none());
        assert!(gr.target().is_none());
        assert!(gr.delegate().is_none());
    }
}
