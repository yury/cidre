use crate::{arc, blocks, cg, define_obj_type, define_opts, ns, objc};

#[doc(alias = "NSEventType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum EventType {
    /// The user pressed the left mouse button
    #[doc(alias = "NSEventTypeLeftMouseDown")]
    LeftMouseDown = 1,

    /// The user released the left mouse button.
    #[doc(alias = "NSEventTypeLeftMouseUp")]
    LeftMouseUp = 2,

    /// The user pressed the right mouse button.
    #[doc(alias = "NSEventTypeRightMouseDown")]
    RightMouseDown = 3,

    /// The user released the right mouse button.
    #[doc(alias = "NSEventTypeRightMouseUp")]
    RightMouseUp = 4,

    /// The user moved the mouse in a way that caused the cursor to move onscreen.
    #[doc(alias = "NSEventTypeMouseMoved")]
    MouseMoved = 5,

    /// The user moved the mouse while holding down the left mouse button.
    #[doc(alias = "NSEventTypeLeftMouseDragged")]
    LeftMouseDragged = 6,

    /// The user moved the mouse while holding down the right mouse button.
    #[doc(alias = "NSEventTypeRightMouseDragged")]
    RightMouseDragged = 7,

    /// The cursor entered a well-defined area, such as a view.
    #[doc(alias = "NSEventTypeMouseEntered")]
    MouseEntered = 8,

    /// The cursor exited a well-defined area, such as a view.
    #[doc(alias = "NSEventTypeMouseExited")]
    MouseExited = 9,

    /// The user pressed a key on the keyboard.
    #[doc(alias = "NSEventTypeKeyDown")]
    KeyDown = 10,

    /// The user released a key on the keyboard.
    #[doc(alias = "NSEventTypeKeyUp")]
    KeyUp = 11,

    /// The event flags changed.
    #[doc(alias = "NSEventTypeFlagsChanged")]
    FlagsChanged = 12,

    /// An AppKit-related event occurred.
    #[doc(alias = "NSEventTypeAppKitDefined")]
    AppKitDefined = 13,

    /// A system-related event occurred.
    #[doc(alias = "NSEventTypeSystemDefined")]
    SysDefined = 14,

    /// An app-defined event occurred.
    #[doc(alias = "NSEventTypeApplicationDefined")]
    AppDefined = 15,

    /// An event that provides execution time to periodic tasks.
    #[doc(alias = "NSEventTypePeriodic")]
    Periodic = 16,

    /// An event that updates the cursor.
    #[doc(alias = "NSEventTypeCursorUpdate")]
    CursorUpdate = 17,

    /// The scroll wheel position changed.
    #[doc(alias = "NSEventTypeScrollWheel")]
    ScrollWheel = 22,

    /// The user touched a point on a tablet.
    ///
    /// Tablets generate tablet-point events between a mouse-down event and the first mouse drag event
    #[doc(alias = "NSEventTypeTabletPoint")]
    TabletPoint = 23,

    /// A pointing device is near, but not touching, the associated tablet.
    #[doc(alias = "NSEventTypeTabletProximity")]
    TabletProximity = 24,

    /// The user pressed a tertiary mouse button.
    #[doc(alias = "NSEventTypeOtherMouseDown")]
    OtherMouseDown = 25,

    /// The user released a tertiary mouse button.
    #[doc(alias = "NSEventTypeOtherMouseUp")]
    OtherMouseUp = 26,

    /// The user moved the mouse while holding down a tertiary mouse button.
    #[doc(alias = "NSEventTypeOtherMouseDragged")]
    OtherMouseDragged = 27,

    /// The user performed a nonspecific type of gesture.
    #[doc(alias = "NSEventTypeGesture")]
    Gesture = 29,

    /// The user performed a pinch-open or pinch-close gesture.
    #[doc(alias = "NSEventTypeMagnify")]
    Magnify = 30,

    /// The user performed a swipe gesture.
    #[doc(alias = "NSEventTypeSwipe")]
    Swipe = 31,

    /// The user performed a rotate gesture.
    #[doc(alias = "NSEventTypeRotate")]
    Rotate = 18,

    /// An event marking the beginning of a gesture.
    #[doc(alias = "NSEventTypeBeginGesture")]
    BeginGesture = 19,

    /// An event that marks the end of a gesture.
    #[doc(alias = "NSEventTypeEndGesture")]
    EndGesture = 20,

    /// The user performed a smart-zoom gesture.
    #[doc(alias = "NSEventTypeSmartMagnify")]
    SmartMagnify = 32,

    /// An event that initiates a Quick Look request.
    #[doc(alias = "NSEventTypeQuickLook")]
    QuickLook = 33,

    /// An event that reports a change in pressure on a pressure-sensitive device.
    #[doc(alias = "NSEventTypePressure")]
    Pressure = 34,

    /// The user touched a portion of the touch bar.
    #[doc(alias = "NSEventTypeDirectTouch")]
    DirectTouch = 37,

    /// The user changed the mode of a connected device.
    #[doc(alias = "NSEventTypeChangeMode")]
    ChangeMode = 38,
}

impl EventType {
    #[doc(alias = "NSEventMaskFromType")]
    pub const fn as_mask(self) -> EventMask {
        EventMask(1usize << self as usize)
    }
}

define_opts!(
    #[doc(alias = "NSEventMask")]
    pub EventMask(usize)
);

impl EventMask {
    #[doc(alias = "NSEventMaskLeftMouseDown")]
    pub const LEFT_MOUSE_DOWN: Self = EventType::LeftMouseDown.as_mask();

    #[doc(alias = "NSEventMaskLeftMouseUp")]
    pub const LEFT_MOUSE_UP: Self = EventType::LeftMouseUp.as_mask();

    #[doc(alias = "NSEventMaskRightMouseDown")]
    pub const RIGHT_MOUSE_DOWN: Self = EventType::RightMouseDown.as_mask();

    #[doc(alias = "NSEventMaskRightMouseUp")]
    pub const RIGHT_MOUSE_UP: Self = EventType::RightMouseUp.as_mask();

    #[doc(alias = "NSEventMaskMouseMoved")]
    pub const MOUSE_MOVED: Self = EventType::MouseMoved.as_mask();

    #[doc(alias = "NSEventMaskLeftMouseDragged")]
    pub const LEFT_MOUSE_DRAGGED: Self = EventType::LeftMouseDragged.as_mask();

    #[doc(alias = "NSEventMaskRightMouseDragged")]
    pub const RIGHT_MOUSE_DRAGGED: Self = EventType::RightMouseDragged.as_mask();

    #[doc(alias = "NSEventMaskMouseEntered")]
    pub const MOUSE_ENTERED: Self = EventType::MouseEntered.as_mask();

    #[doc(alias = "NSEventMaskMouseExited")]
    pub const MOUSE_EXITED: Self = EventType::MouseExited.as_mask();

    #[doc(alias = "NSEventMaskKeyDown")]
    pub const KEY_DOWN: Self = EventType::KeyDown.as_mask();

    #[doc(alias = "NSEventMaskKeyUp")]
    pub const KEY_UP: Self = EventType::KeyUp.as_mask();

    #[doc(alias = "NSEventMaskFlagsChanged")]
    pub const FLAGS_CHANGED: Self = EventType::FlagsChanged.as_mask();

    #[doc(alias = "NSEventMaskAppKitDefined")]
    pub const APP_KIT_DEFINED: Self = EventType::AppKitDefined.as_mask();

    #[doc(alias = "NSEventMaskSystemDefined")]
    pub const SYS_DEFINED: Self = EventType::SysDefined.as_mask();

    #[doc(alias = "NSEventMaskApplicationDefined")]
    pub const APP_DEFINED: Self = EventType::AppDefined.as_mask();

    #[doc(alias = "NSEventMaskPeriodic")]
    pub const PERIODIC: Self = EventType::Periodic.as_mask();

    #[doc(alias = "NSEventMaskCursorUpdate")]
    pub const CURSOR_UPDATE: Self = EventType::CursorUpdate.as_mask();

    #[doc(alias = "NSEventMaskScrollWheel")]
    pub const SCROLL_WHEEL: Self = EventType::ScrollWheel.as_mask();

    #[doc(alias = "NSEventMaskTabletPoint")]
    pub const TABLET_POINT: Self = EventType::TabletPoint.as_mask();

    #[doc(alias = "NSEventMaskTabletProximity")]
    pub const TABLET_PROXIMITY: Self = EventType::TabletProximity.as_mask();

    #[doc(alias = "NSEventMaskOtherMouseDown")]
    pub const OTHER_MOUSE_DOWN: Self = EventType::OtherMouseDown.as_mask();

    #[doc(alias = "NSEventMaskOtherMouseUp")]
    pub const OTHER_MOUSE_UP: Self = EventType::OtherMouseUp.as_mask();

    #[doc(alias = "NSEventMaskOtherMouseDragged")]
    pub const OTHER_MOUSE_DRAGGED: Self = EventType::OtherMouseDragged.as_mask();

    #[doc(alias = "NSEventMaskGesture")]
    pub const GESTURE: Self = EventType::Gesture.as_mask();

    #[doc(alias = "NSEventMaskMagnify")]
    pub const MAGNIFY: Self = EventType::Magnify.as_mask();

    #[doc(alias = "NSEventMaskSwipe")]
    pub const SWIPE: Self = EventType::Swipe.as_mask();

    #[doc(alias = "NSEventMaskRotate")]
    pub const ROTATE: Self = EventType::Rotate.as_mask();

    #[doc(alias = "NSEventMaskBeginGesture")]
    pub const BEGIN_GESTURE: Self = EventType::BeginGesture.as_mask();

    #[doc(alias = "NSEventMaskEndGesture")]
    pub const END_GESTURE: Self = EventType::EndGesture.as_mask();

    #[doc(alias = "NSEventMaskSmartMagnify")]
    pub const SMART_MAGNIFY: Self = EventType::SmartMagnify.as_mask();

    #[doc(alias = "NSEventMaskPressure")]
    pub const PRESSURE: Self = EventType::Pressure.as_mask();

    #[doc(alias = "NSEventMaskDirectTouch")]
    pub const DIRECT_TOUCH: Self = EventType::DirectTouch.as_mask();

    #[doc(alias = "NSEventMaskChangeMode")]
    pub const CHANGE_MODE: Self = EventType::ChangeMode.as_mask();

    #[doc(alias = "NSEventMaskAny")]
    pub const ANY: Self = Self(usize::MAX);
}

define_opts!(
    #[doc(alias = "NSEventModifierFlags")]
    pub EventModifierFlags(usize)
);

impl EventModifierFlags {
    /// The Caps Lock key has been pressed.
    #[doc(alias = "NSEventModifierFlagCapsLock")]
    pub const CAPS_LOCK: Self = Self(1usize << 16);

    /// The Shift key has been pressed.
    #[doc(alias = "NSEventModifierFlagShift")]
    pub const SHIFT: Self = Self(1usize << 17);

    /// The Control key has been pressed.
    #[doc(alias = "NSEventModifierFlagControl")]
    pub const CTRL: Self = Self(1usize << 18);

    /// The Option or Alt key has been pressed.
    #[doc(alias = "NSEventModifierFlagOption")]
    pub const OPT: Self = Self(1usize << 19);

    /// The Command key has been pressed.
    #[doc(alias = "NSEventModifierFlagCommand")]
    pub const CMD: Self = Self(1usize << 20);

    /// A key in the numeric keypad or an arrow key has been pressed.
    #[doc(alias = "NSEventModifierFlagNumericPad")]
    pub const NUM_PAD: Self = Self(1usize << 21);

    /// The Help key has been pressed.
    #[doc(alias = "NSEventModifierFlagHelp")]
    pub const HELP: Self = Self(1usize << 22);

    /// A function key has been pressed.
    #[doc(alias = "NSEventModifierFlagFunction")]
    pub const FN: Self = Self(1usize << 23);

    /// Device-independent modifier flags are masked.
    #[doc(alias = "NSEventModifierFlagDeviceIndependentFlagsMask")]
    pub const DEVICE_INDEPENDENT_FLAGS_MASK: Self = Self(0xffff0000);
}

#[doc(alias = "NSPointingDeviceType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum PointingDeviceType {
    Unknown = 0,
    Pen = 1,
    Cursor = 2,
    Eraser = 3,
}

define_opts!(
    #[doc(alias = "NSEventButtonMask")]
    pub EventButtonMask(usize)
);

impl EventButtonMask {
    /// A mask that matches the pen tip.
    pub const PEN_TIP: Self = Self(0x0001);

    /// A mask that matches the button on the lower side of the device.
    pub const PEN_LOWER_SIDE: Self = Self(0x0002);

    /// A mask that matches the button on the upper side of the device.
    pub const PEN_UPPER_SIDE: Self = Self(0x0004);
}

define_opts!(
    #[doc(alias = "NSEventPhase")]
    pub EventPhase(usize)
);

impl EventPhase {
    /// Event not associated with a phase
    pub const NONE: Self = Self(0);

    /// An event phase has begun.
    pub const BEGAN: Self = Self(0x1 << 0);

    /// An event phase is in progress but hasn't moved since the previous event.
    pub const STATIONARY: Self = Self(0x1 << 1);

    /// An event phase has changed.
    pub const CHANGED: Self = Self(0x1 << 2);

    /// The event phase ended.
    pub const ENDED: Self = Self(0x1 << 3);

    /// The system canceled the event phase.
    pub const CANCELLED: Self = Self(0x1 << 4);

    /// The system event phase may begin.
    pub const MAY_BEGIN: Self = Self(0x1 << 5);
}

#[doc(alias = "NSEventGestureAxis")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum EventGestureAxis {
    None = 0,
    Horizontal,
    Vertical,
}

define_opts!(
    #[doc(alias = "NSEventSwipeTrackingOptions")]
    pub EventSwipeTrackingOpts(usize)
);

impl EventSwipeTrackingOpts {
    /// Clamp gestureAmount to 0 if the user starts to swipe in the opposite direction than they started.
    pub const LOCK_DIRECTION: Self = Self(0x1 << 0);

    /// Don't allow gestureAmount to go beyond +/-1.0
    pub const CLAMP_GESTURE_AMOUNT: Self = Self(0x1 << 1);
}

define_opts!(
    #[doc(alias = "NSEventSubtype")]
    pub EventSubtype(i16)
);

/// event subtypes for NSEventTypeAppKitDefined events
impl EventSubtype {
    /// An event that indicates a window’s contents are visible again.
    #[doc(alias = "NSEventSubtypeWindowExposed")]
    pub const WINDOW_EXPOSED: Self = Self(0);

    /// An app-activation event occurred.
    #[doc(alias = "NSEventSubtypeApplicationActivated")]
    pub const APP_ACTIVATED: Self = Self(1);

    /// An app-deactivation event occurred.
    #[doc(alias = "NSEventSubtypeApplicationDeactivated")]
    pub const APP_DEACTIVATED: Self = Self(2);

    /// An event that indicates a window moved.
    #[doc(alias = "NSEventSubtypeWindowMoved")]
    pub const WINDOW_MOVED: Self = Self(4);

    /// An event that indicates a window changed screens.
    #[doc(alias = "NSEventSubtypeScreenChanged")]
    pub const SCREEN_CHANGED: Self = Self(8);
}

/// event subtypes for NSEventTypeSystemDefined events
impl EventSubtype {
    pub const POWER_OFF: Self = Self(1);
}

/// event subtypes for mouse events
impl EventSubtype {
    pub const MOUSE_EVENT: Self = Self(0);

    pub const TABLET_POINT: Self = Self(1);

    pub const TABLET_PROXIMITY: Self = Self(2);

    pub const TOUCH: Self = Self(3);
}

#[doc(alias = "NSPressureBehavior")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum PressureBehavior {
    /// A pressure gesture’s behavior is not known, perhaps because the input device does not support pressure gestures.
    #[doc(alias = "NSPressureBehaviorUnknown")]
    Unknown = -1,

    /// This is the default behavior when a pressure gesture’s behavior has not been explicitly configured. In OS X 10.10.3, this behavior defaults to the behavior of NSPressureBehaviorPrimaryDeepClick.
    #[doc(alias = "NSPressureBehaviorPrimaryDefault")]
    PrimaryDefault = 0,

    /// A pressure gesture’s behavior begins on left mouse-down events. A maximum of one stage is supported,
    /// and a stage transition animation occurs when moving from stage 1 to stage 0. Actuations (haptic feedback the user feels)
    /// occur during mouse-down and mouse-up events when this behavior is configured. Note that the pressure gesture operates
    /// on a separate event stream from the mouse events.
    #[doc(alias = "NSPressureBehaviorPrimaryClick")]
    PrimaryClick = 1,

    #[doc(alias = "NSPressureBehaviorPrimaryGeneric")]
    PrimaryGeneric = 2,

    #[doc(alias = "NSPressureBehaviorPrimaryAccelerator")]
    PrimaryAccelerator = 3,

    #[doc(alias = "NSPressureBehaviorPrimaryDeepClick")]
    PrimaryDeepClick = 5,

    #[doc(alias = "NSPressureBehaviorPrimaryDeepDrag")]
    PrimaryDeepDrag = 6,
}

define_obj_type!(
    #[doc(alias = "NSEvent")]
    pub Event(ns::Id),
    NS_EVENT
);

impl Event {
    #[objc::msg_send(type)]
    pub fn type_(&self) -> EventType;

    #[objc::msg_send(modifierFlags)]
    pub fn modifier_flags(&self) -> EventModifierFlags;

    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> ns::TimeInterval;

    #[objc::msg_send(window)]
    pub fn window(&self) -> Option<arc::R<ns::Window>>;

    #[objc::msg_send(windowNumber)]
    pub fn window_number(&self) -> isize;

    #[objc::msg_send(clickCount)]
    pub fn click_count(&self) -> isize;

    #[objc::msg_send(buttonNumber)]
    pub fn button_number(&self) -> isize;

    #[objc::msg_send(eventNumber)]
    pub fn event_number(&self) -> isize;

    #[objc::msg_send(pressure)]
    pub fn pressure(&self) -> f32;

    #[objc::msg_send(locationInWindow)]
    pub fn location_in_window(&self) -> ns::Point;

    #[objc::msg_send(deltaX)]
    pub fn delta_x(&self) -> cg::Float;

    #[objc::msg_send(deltaY)]
    pub fn delta_y(&self) -> cg::Float;

    #[objc::msg_send(deltaZ)]
    pub fn delta_z(&self) -> cg::Float;

    #[objc::msg_send(hasPreciseScrollingDeltas)]
    pub fn has_precise_scrolling_deltas(&self) -> bool;

    #[objc::msg_send(scrollingDeltaX)]
    pub fn scrolling_delta_x(&self) -> cg::Float;

    #[objc::msg_send(scrollingDeltaY)]
    pub fn scrolling_delta_y(&self) -> cg::Float;

    #[objc::msg_send(momentumPhase)]
    pub fn momentum_phase(&self) -> EventPhase;

    #[objc::msg_send(isDirectionInvertedFromDevice)]
    pub fn is_direction_inverted_from_device(&self) -> bool;

    #[objc::msg_send(characters)]
    pub fn characters(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(charactersIgnoringModifiers)]
    pub fn characters_ignoring_modifiers(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(charactersByApplyingModifiers:)]
    pub fn characters_by_applying_modifiers(
        &self,
        modifiers: EventModifierFlags,
    ) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(isARepeat)]
    pub fn is_a_repeat(&self) -> bool;

    #[objc::msg_send(keyCode)]
    pub fn key_code(&self) -> u16;

    #[objc::msg_send(trackingNumber)]
    pub fn tracking_number(&self) -> isize;

    #[objc::msg_send(userData)]
    pub fn user_data(&self) -> *const std::ffi::c_void;

    // #[objc::msg_send(trackingArea)]
    // pub fn tracking_area(&self) -> Option<arc::R<ns::TrackingArea>>;

    #[objc::msg_send(subtype)]
    pub fn subtype(&self) -> EventSubtype;

    #[objc::msg_send(data1)]
    pub fn data1(&self) -> isize;

    #[objc::msg_send(data2)]
    pub fn data2(&self) -> isize;

    #[objc::msg_send(eventRef)]
    pub fn event_ref(&self) -> *const std::ffi::c_void;

    // #[objc::msg_send(CGEvent)]
    // pub fn cg_event(&self) -> Option<cg::Event>;

    #[objc::msg_send(isMouseCoalescingEnabled)]
    pub fn is_mouse_coalescing_enabled(&self) -> bool;

    #[objc::msg_send(magnification)]
    pub fn magnification(&self) -> cg::Float;

    #[objc::msg_send(deviceID)]
    pub fn device_id(&self) -> usize;

    #[objc::msg_send(rotation)]
    pub fn rotation(&self) -> f32;

    #[objc::msg_send(absoluteX)]
    pub fn absolute_x(&self) -> isize;

    #[objc::msg_send(absoluteY)]
    pub fn absolute_y(&self) -> isize;

    #[objc::msg_send(absoluteZ)]
    pub fn absolute_z(&self) -> isize;

    #[objc::msg_send(buttonMask)]
    pub fn button_mask(&self) -> EventButtonMask;

    #[objc::msg_send(tilt)]
    pub fn tilt(&self) -> ns::Point;

    #[objc::msg_send(tangentialPressure)]
    pub fn tangential_pressure(&self) -> f32;

    #[objc::msg_send(vendorDefined)]
    pub fn vendor_defined(&self) -> arc::R<ns::Id>;

    #[objc::msg_send(vendorID)]
    pub fn vendor_id(&self) -> usize;

    #[objc::msg_send(tabletID)]
    pub fn tablet_id(&self) -> usize;

    #[objc::msg_send(pointingDeviceID)]
    pub fn pointing_device_id(&self) -> usize;

    #[objc::msg_send(systemTabletID)]
    pub fn system_tablet_id(&self) -> usize;

    #[objc::msg_send(vendorPointingDeviceType)]
    pub fn vendor_pointing_device_type(&self) -> usize;

    #[objc::msg_send(pointingDeviceSerialNumber)]
    pub fn pointing_device_serial_number(&self) -> usize;

    #[objc::msg_send(uniqueID)]
    pub fn unique_id(&self) -> usize;

    #[objc::msg_send(capabilityMask)]
    pub fn capability_mask(&self) -> usize;

    #[objc::msg_send(pointingDeviceType)]
    pub fn pointing_device_type(&self) -> PointingDeviceType;

    #[objc::msg_send(isEnteringProximity)]
    pub fn is_entering_proximity(&self) -> bool;

    #[objc::msg_send(touchesMatchingPhase:inView:)]
    pub fn touches_matching_phase_in_view(
        &self,
        phase: ns::TouchPhase,
        view: Option<&ns::View>,
    ) -> arc::R<ns::Set<ns::Touch>>;

    #[objc::msg_send(allTouches)]
    pub fn all_touches(&self) -> arc::R<ns::Set<ns::Touch>>;

    #[objc::msg_send(touchesForView:)]
    pub fn touches_for_view(&self, view: &ns::View) -> arc::R<ns::Set<ns::Touch>>;

    #[objc::msg_send(coalescedTouchesForTouch:)]
    pub fn coalesced_touches_for_touch(&self, touch: &ns::Touch) -> arc::R<ns::Array<ns::Touch>>;

    #[objc::msg_send(phase)]
    pub fn phase(&self) -> EventPhase;

    #[objc::msg_send(stage)]
    pub fn stage(&self) -> isize;

    #[objc::msg_send(stageTransition)]
    pub fn stage_transition(&self) -> cg::Float;

    #[objc::msg_send(associatedEventsMask)]
    pub fn associated_events_mask(&self) -> EventMask;

    #[objc::msg_send(pressureBehavior)]
    pub fn pressure_behavior(&self) -> PressureBehavior;

    #[objc::msg_send(isSwipeTrackingFromScrollEventsEnabled)]
    pub fn is_swipe_tracking_from_scroll_events_enabled(&self) -> bool;

    #[objc::msg_send(trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler:)]
    pub fn track_swipe_event(
        &self,
        options: EventSwipeTrackingOpts,
        dampen_threshold_min: cg::Float,
        dampen_threshold_max: cg::Float,
        handler: &mut blocks::EscBlock<
            fn(gesture_amount: cg::Float, phase: EventPhase, is_complete: bool, stop: &mut bool),
        >,
    );

    #[objc::msg_send(startPeriodicEventsAfterDelay:withPeriod:)]
    pub fn start_periodic_events_after_delay(
        &self,
        delay: ns::TimeInterval,
        period: ns::TimeInterval,
    );

    #[objc::msg_send(stopPeriodicEvents)]
    pub fn stop_periodic_events(&self);

    #[objc::msg_send(mouseLocation)]
    pub fn mouse_location() -> ns::Point;

    #[objc::msg_send(modifierFlags)]
    pub fn get_modifier_flags() -> EventModifierFlags;

    #[objc::msg_send(pressedMouseButtons)]
    pub fn pressed_mouse_buttons() -> usize;

    #[objc::msg_send(doubleClickInterval)]
    pub fn double_click_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(keyRepeatDelay)]
    pub fn key_repeat_delay(&self) -> ns::TimeInterval;

    #[objc::msg_send(keyRepeatInterval)]
    pub fn key_repeat_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(addGlobalMonitorForEventsMatchingMask:handler:)]
    pub fn add_global_monitor_for_events_matching_mask_handler(
        mask: EventMask,
        handler: &mut blocks::EscBlock<fn(event: &ns::Event)>,
    ) -> Option<arc::R<ns::Id>>;

    pub fn add_global_monitor_for_events_matching_mask(
        mask: EventMask,
        handler: impl FnMut(&ns::Event) + 'static,
    ) -> Option<arc::R<ns::Id>> {
        let mut block = blocks::EscBlock::new1(handler);
        Self::add_global_monitor_for_events_matching_mask_handler(mask, &mut block)
    }

    #[objc::msg_send(addLocalMonitorForEventsMatchingMask:handler:)]
    pub fn add_local_monitor_for_events_matching_mask_handler(
        mask: EventMask,
        handler: &mut blocks::EscBlock<fn(event: &ns::Event)>,
    ) -> Option<arc::R<ns::Id>>;

    pub fn add_local_monitor_for_events_matching_mask(
        mask: EventMask,
        handler: impl FnMut(&ns::Event) + 'static,
    ) -> Option<arc::R<ns::Id>> {
        let mut block = blocks::EscBlock::new1(handler);
        Self::add_local_monitor_for_events_matching_mask_handler(mask, &mut block)
    }

    #[objc::msg_send(removeMonitor:)]
    pub fn remove_monitor(event_monitor: &ns::Id);
}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_EVENT: &'static objc::Class<Event>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let _loc = ns::Event::mouse_location();
    }
}
