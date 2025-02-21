use crate::{arc, cf, cg};

impl cg::EventSrc {
    #[doc(alias = "CGEventSourceGetTypeID")]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGEventSourceGetTypeID() }
    }

    /// Return a Quartz event source created with a specified source state.
    #[doc(alias = "CGEventSourceCreate")]
    pub fn with_state(state_id: cg::EventSrcStateId) -> Option<arc::R<Self>> {
        unsafe { CGEventSourceCreate(state_id) }
    }

    #[doc(alias = "CGEventSourceGetKeyboardType")]
    pub fn keyboard_type(&self) -> cg::EventSrcKeyboardType {
        unsafe { CGEventSourceGetKeyboardType(self) }
    }

    #[doc(alias = "CGEventSourceSetKeyboardType")]
    pub fn set_keyboard_type(&mut self, val: cg::EventSrcKeyboardType) {
        unsafe {
            CGEventSourceSetKeyboardType(self, val);
        }
    }

    #[doc(alias = "CGEventSourceGetPixelsPerLine")]
    pub fn pixels_per_line(&self) -> f64 {
        unsafe { CGEventSourceGetPixelsPerLine(self) }
    }

    #[doc(alias = "CGEventSourceSetPixelsPerLine")]
    pub fn set_pixel_per_line(&mut self, val: f64) {
        unsafe { CGEventSourceSetPixelsPerLine(self, val) }
    }

    #[doc(alias = "CGEventSourceGetSourceStateID")]
    pub fn state_id(&self) -> cg::EventSrcStateId {
        unsafe { CGEventSourceGetSourceStateID(self) }
    }

    #[doc(alias = "CGEventSourceSetUserData")]
    pub fn set_user_data(&mut self, val: i64) {
        unsafe {
            CGEventSourceSetUserData(self, val);
        }
    }

    #[doc(alias = "CGEventSourceGetUserData")]
    pub fn user_data(&self) -> i64 {
        unsafe { CGEventSourceGetUserData(self) }
    }

    #[doc(alias = "CGEventSourceSetLocalEventsFilterDuringSuppressionState")]
    pub fn set_local_events_filter_during_suppression_state(
        &mut self,
        filter: cg::EventFilterMask,
        state: cg::EventSuppressionState,
    ) {
        unsafe {
            CGEventSourceSetLocalEventsFilterDuringSuppressionState(self, filter, state);
        }
    }

    #[doc(alias = "CGEventSourceGetLocalEventsFilterDuringSuppressionState")]
    pub fn local_events_filter_during_suppression_state(
        &mut self,
        state: cg::EventSuppressionState,
    ) -> cg::EventFilterMask {
        unsafe { CGEventSourceGetLocalEventsFilterDuringSuppressionState(self, state) }
    }

    #[doc(alias = "CGEventSourceSetLocalEventsSuppressionInterval")]
    pub fn set_local_events_suppression_interval(&mut self, seconds: cf::TimeInterval) {
        unsafe {
            CGEventSourceSetLocalEventsSuppressionInterval(self, seconds);
        }
    }

    #[doc(alias = "CGEventSourceGetLocalEventsSuppressionInterval")]
    pub fn local_events_suppression_interval(&self) -> cf::TimeInterval {
        unsafe { CGEventSourceGetLocalEventsSuppressionInterval(self) }
    }
}

impl cg::EventSrcStateId {
    #[doc(alias = "CGEventSourceButtonState")]
    pub fn button_state(&self, button: cg::MouseButton) -> bool {
        unsafe { CGEventSourceButtonState(*self, button) }
    }

    #[doc(alias = "CGEventSourceKeyState")]
    pub fn key_state(&self, key_code: cg::KeyCode) -> bool {
        unsafe { CGEventSourceKeyState(*self, key_code) }
    }

    #[doc(alias = "CGEventSourceFlagsState")]
    pub fn flags_state(&self) -> cg::EventFlags {
        unsafe { CGEventSourceFlagsState(*self) }
    }

    #[doc(alias = "CGEventSourceSecondsSinceLastEventType")]
    pub fn secs_since_last_event_type(&self, event_type: cg::EventType) -> cf::TimeInterval {
        unsafe { CGEventSourceSecondsSinceLastEventType(*self, event_type) }
    }

    #[doc(alias = "CGEventSourceCounterForEventType")]
    pub fn counter_for_event_type(&self, event_type: cg::EventType) -> u32 {
        unsafe { CGEventSourceCounterForEventType(*self, event_type) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C-unwind" {
    fn CGEventSourceGetTypeID() -> cf::TypeId;
    fn CGEventSourceCreate(state_id: cg::EventSrcStateId) -> Option<arc::R<cg::EventSrc>>;
    fn CGEventSourceGetKeyboardType(src: *const cg::EventSrc) -> cg::EventSrcKeyboardType;
    fn CGEventSourceSetKeyboardType(src: *mut cg::EventSrc, val: cg::EventSrcKeyboardType);
    fn CGEventSourceGetPixelsPerLine(src: *const cg::EventSrc) -> f64;
    fn CGEventSourceSetPixelsPerLine(src: *mut cg::EventSrc, val: f64);
    fn CGEventSourceGetSourceStateID(src: *const cg::EventSrc) -> cg::EventSrcStateId;
    fn CGEventSourceButtonState(state_id: cg::EventSrcStateId, button: cg::MouseButton) -> bool;
    fn CGEventSourceKeyState(state_id: cg::EventSrcStateId, key_code: cg::KeyCode) -> bool;
    fn CGEventSourceFlagsState(state_id: cg::EventSrcStateId) -> cg::EventFlags;
    fn CGEventSourceSecondsSinceLastEventType(
        state_id: cg::EventSrcStateId,
        event_type: cg::EventType,
    ) -> cf::TimeInterval;
    fn CGEventSourceCounterForEventType(
        state_id: cg::EventSrcStateId,
        event_type: cg::EventType,
    ) -> u32;

    fn CGEventSourceSetUserData(src: *mut cg::EventSrc, user_data: i64);
    fn CGEventSourceGetUserData(src: *const cg::EventSrc) -> i64;
    fn CGEventSourceSetLocalEventsFilterDuringSuppressionState(
        src: *mut cg::EventSrc,
        filter: cg::EventFilterMask,
        state: cg::EventSuppressionState,
    );

    fn CGEventSourceGetLocalEventsFilterDuringSuppressionState(
        src: *const cg::EventSrc,
        state: cg::EventSuppressionState,
    ) -> cg::EventFilterMask;

    fn CGEventSourceSetLocalEventsSuppressionInterval(
        src: *mut cg::EventSrc,
        seconds: cf::TimeInterval,
    );

    fn CGEventSourceGetLocalEventsSuppressionInterval(src: *const cg::EventSrc)
    -> cf::TimeInterval;
}
