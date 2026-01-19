use crate::{cf, define_cf_type, define_opts};

define_cf_type!(
    #[doc(alias = "CGEventRef")]
    Event(cf::Type)
);

/// Constants that specify buttons on a one, two, or three-button mouse.
#[doc(alias = "CGMouseButton")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum MouseButton {
    #[doc(alias = "kCGMouseButtonLeft")]
    Left = 0,

    #[doc(alias = "kCGMouseButtonRight")]
    Right = 1,

    #[doc(alias = "kCGMouseButtonCenter")]
    Center = 2,
}

/// Constants that specify the unit of measurement for a scrolling event.
#[doc(alias = "CGScrollEventUnit")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ScrollEventUnit {
    #[doc(alias = "kCGScrollEventUnitPixel")]
    Pixel = 0,

    #[doc(alias = "kCGScrollEventUnitLine")]
    Line = 1,
}

/// Constants that specify momentum scroll phases.
#[doc(alias = "CGMomentumScrollPhase")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum MomentumScrollPhase {
    #[doc(alias = "kCGMomentumScrollPhaseNone")]
    None = 0,

    #[doc(alias = "kCGMomentumScrollPhaseBegin")]
    Begin = 1,

    #[doc(alias = "kCGMomentumScrollPhaseContinue")]
    Continue = 2,

    #[doc(alias = "kCGMomentumScrollPhaseEnd")]
    End = 3,
}

/// Constants that specify scroll phases.
#[doc(alias = "CGScrollPhase")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ScrollPhase {
    #[doc(alias = "kCGScrollPhaseBegan")]
    Began = 1,

    #[doc(alias = "kCGScrollPhaseChanged")]
    Changed = 2,

    #[doc(alias = "kCGScrollPhaseEnded")]
    Ended = 4,

    #[doc(alias = "kCGScrollPhaseCancelled")]
    Cancelled = 8,

    #[doc(alias = "kCGScrollPhaseMayBegin")]
    MayBegin = 128,
}

/// Constants that specify gesture phases.
#[doc(alias = "CGGesturePhase")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GesturePhase {
    #[doc(alias = "kCGGesturePhaseNone")]
    None = 0,

    #[doc(alias = "kCGGesturePhaseBegan")]
    Began = 1,

    #[doc(alias = "kCGGesturePhaseChanged")]
    Changed = 2,

    #[doc(alias = "kCGGesturePhaseEnded")]
    Ended = 4,

    #[doc(alias = "kCGGesturePhaseCancelled")]
    Cancelled = 8,

    #[doc(alias = "kCGGesturePhaseMayBegin")]
    MayBegin = 128,
}

define_opts!(
    #[doc(alias = "CGEventFlags")]
    pub EventFlags(u64)
);

impl EventFlags {
    #[doc(alias = "kCGEventFlagMaskAlphaShift")]
    pub const ALPHA_SHIFT: Self = Self(0x00010000);

    #[doc(alias = "kCGEventFlagMaskShift")]
    pub const SHIFT: Self = Self(0x00020000);

    #[doc(alias = "kCGEventFlagMaskControl")]
    pub const CTRL: Self = Self(0x00040000);

    #[doc(alias = "kCGEventFlagMaskAlternate")]
    pub const ALT: Self = Self(0x00080000);

    #[doc(alias = "kCGEventFlagMaskCommand")]
    pub const CMD: Self = Self(0x00100000);

    #[doc(alias = "kCGEventFlagMaskHelp")]
    pub const HELP: Self = Self(0x00400000);

    #[doc(alias = "kCGEventFlagMaskSecondaryFn")]
    pub const SECONDARY_FN: Self = Self(0x00800000);

    #[doc(alias = "kCGEventFlagMaskNumericPad")]
    pub const NUM_PAD: Self = Self(0x00200000);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
#[repr(C)]
pub struct EventType(pub u32);

impl EventType {
    #[doc(alias = "kCGEventNull")]
    pub const NULL: Self = Self(0);

    #[doc(alias = "kCGEventLeftMouseDown")]
    pub const LEFT_MOUSE_DOWN: Self = Self(1);

    #[doc(alias = "kCGEventLeftMouseUp")]
    pub const LEFT_MOUSE_UP: Self = Self(2);

    #[doc(alias = "kCGEventRightMouseDown")]
    pub const RIGHT_MOUSE_DOWN: Self = Self(3);

    #[doc(alias = "kCGEventRightMouseUp")]
    pub const RIGHT_MOUSE_UP: Self = Self(4);

    #[doc(alias = "kCGEventMouseMoved")]
    pub const MOUSE_MOVED: Self = Self(5);

    #[doc(alias = "kCGEventLeftMouseDragged")]
    pub const LEFT_MOUSE_DRAGGED: Self = Self(6);

    #[doc(alias = "kCGEventRightMouseDragged")]
    pub const RIGHT_MOUSE_DRAGGED: Self = Self(7);

    #[doc(alias = "kCGEventKeyDown")]
    pub const KEY_DOWN: Self = Self(10);

    #[doc(alias = "kCGEventKeyUp")]
    pub const KEY_UP: Self = Self(11);

    #[doc(alias = "kCGEventFlagsChanged")]
    pub const FLAGS_CHANGED: Self = Self(12);

    pub const SYSTEM_DEFINED: Self = Self(14);

    #[doc(alias = "kCGEventScrollWheel")]
    pub const SCROLL_WHEEL: Self = Self(22);

    #[doc(alias = "kCGEventTabletPointer")]
    pub const TABLET_POINTER: Self = Self(23);

    #[doc(alias = "kCGEventTabletProximity")]
    pub const TABLET_PROXIMITY: Self = Self(24);

    #[doc(alias = "kCGEventOtherMouseDown")]
    pub const OHTER_MOUSE_DOWN: Self = Self(25);

    #[doc(alias = "kCGEventOtherMouseUp")]
    pub const OHTER_MOUSE_UP: Self = Self(26);

    #[doc(alias = "kCGEventOtherMouseDragged")]
    pub const OTHER_MOUSE_DRAGGED: Self = Self(27);

    #[doc(alias = "kCGEventTapDisabledByTimeout")]
    pub const TAP_DISABLED_BY_TIMEOUT: Self = Self(0xFFFFFFFE);

    #[doc(alias = "kCGEventTapDisabledByUserInput")]
    pub const TAP_DISABLED_BY_USER_INPUT: Self = Self(0xFFFFFFFF);

    #[doc(alias = "CGEventMaskBit")]
    pub const fn mask(&self) -> EventMask {
        1 << self.0
    }

    pub const KB_EVENTS_MASK: EventMask =
        Self::KEY_DOWN.mask() | Self::KEY_UP.mask() | Self::FLAGS_CHANGED.mask();

    pub const LEFT_BUTTON_MOUSE_EVENTS_MASK: EventMask =
        Self::LEFT_MOUSE_DOWN.mask() | Self::LEFT_MOUSE_UP.mask() | Self::LEFT_MOUSE_DRAGGED.mask();

    pub const RIGHT_BUTTON_MOUSE_EVENTS_MASK: EventMask = Self::RIGHT_MOUSE_DOWN.mask()
        | Self::RIGHT_MOUSE_UP.mask()
        | Self::RIGHT_MOUSE_DRAGGED.mask();

    pub const MOUSE_EVENTS_MASK: EventMask = Self::LEFT_BUTTON_MOUSE_EVENTS_MASK
        | Self::RIGHT_BUTTON_MOUSE_EVENTS_MASK
        | Self::MOUSE_MOVED.mask();

    pub const ALL_EVENTS_MASK: EventMask = !0;
}

/// Constants that specify possible tapping points for events.
#[doc(alias = "CGEventTapLocation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum EventTapLocation {
    #[doc(alias = "kCGHIDEventTap")]
    Hid = 0,

    #[doc(alias = "kCGSessionEventTap")]
    Session,

    #[doc(alias = "kCGAnnotatedSessionEventTap")]
    AnnotatedSession,
}

/// Constants that specify where a new event tap is inserted into the list of
/// active event taps.
#[doc(alias = "CGEventTapPlacement")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum EventTapPlacement {
    #[doc(alias = "kCGHeadInsertEventTap")]
    HeadInsert = 0,

    #[doc(alias = "kCGTailAppendEventTap")]
    TailAppend,
}

define_opts!(
    #[doc(alias = "CGEventTapOptions")]
    pub EventTapOpts(u32)
);

impl EventTapOpts {
    #[doc(alias = "kCGEventTapOptionDefault")]
    pub const DEFAULT: Self = Self(0);

    #[doc(alias = "kCGEventTapOptionListenOnly")]
    pub const LISTEN_ONLY: Self = Self(1);
}

define_cf_type!(
    #[doc(alias = "CGEventSourceRef")]
    EventSrc(cf::Type)
);

pub type EventSrcKeyboardType = u32;

#[doc(alias = "CGEventSourceStateID")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum EventSrcStateId {
    #[doc(alias = "kCGEventSourceStatePrivate")]
    Private = -1,

    #[doc(alias = "kCGEventSourceStateCombinedSessionState")]
    CombinedSession = 0,

    #[doc(alias = "kCGEventSourceStateHIDSystemState")]
    HidSystem = 1,
}

#[doc(alias = "CGEventMask")]
pub type EventMask = u64;

#[doc(alias = "CGEventTapProxy")]
pub type EventTapProxy = std::ffi::c_void;

/// The callback is passed a proxy for the tap, the event type, the incoming
/// event, and the user-defined data specified when the event tap was
/// created. The function should return the (possibly modified) passed-in
/// event, a newly constructed event, or None if the event is to be deleted.
///
/// The event passed to the callback is retained by the calling code, and is
/// released after the callback returns and the data is passed back to the
/// event system. If a different event is returned by the callback function,
/// then that event will be released by the calling code along with the
/// original event, after the event data has been passed back to the event
/// system.
#[doc(alias = "CGEventTapCallBack")]
pub type EventTapCb<U = std::ffi::c_void> = extern "C" fn(
    proxy: *mut EventTapProxy,
    event_type: EventType,
    event: &mut Event,
    user_info: *mut U,
) -> Option<&Event>;

#[doc(alias = "CGEventField")]
#[repr(transparent)]
pub struct EventField(pub u32);

impl EventField {
    /// Key to access an integer field that contains the mouse button event
    /// number. Matching mouse-down and mouse-up events will have the same
    /// event number.
    #[doc(alias = "kCGMouseEventNumber")]
    pub const MOUSE_EVENT_NUMBER: Self = Self(0);

    /// Key to access an integer field that contains the mouse button click
    /// state. A click state of 1 represents a single click. A click state of 2
    /// represents a double-click. A click state of 3 represents a
    /// triple-click.
    #[doc(alias = "kCGMouseEventClickState")]
    pub const MOUSE_EVENT_CLICK_STATE: Self = Self(1);

    /// Key to access a double field that contains the mouse button pressure.
    /// The pressure value may range from 0 to 1, with 0 representing the mouse
    /// being up. This value is commonly set by tablet pens mimicking a mouse.
    #[doc(alias = "kCGMouseEventPressure")]
    pub const MOUSE_EVENT_PRESSURE: Self = Self(2);

    /// Key to access an integer field that contains the mouse button number.
    #[doc(alias = "kCGMouseEventButtonNumber")]
    pub const MOUSE_EVENT_BUTTON_NUMBER: Self = Self(3);

    /// Key to access an integer field that contains the horizontal mouse delta
    /// since the last mouse movement event.
    #[doc(alias = "kCGMouseEventDeltaX")]
    pub const MOUSE_EVENT_DELTA_X: Self = Self(4);

    /// Key to access an integer field that contains the vertical mouse delta
    /// since the last mouse movement event.
    #[doc(alias = "kCGMouseEventDeltaY")]
    pub const MOUSE_EVENT_DELTA_Y: Self = Self(5);

    /// Key to access an integer field. The value is non-zero if the event
    /// should be ignored by the Inkwell subsystem.
    #[doc(alias = "kCGMouseEventInstantMouser")]
    pub const MOUSE_EVENT_INSTANT_MOUSER: Self = Self(6);

    /// Key to access an integer field that encodes the mouse event subtype as a `kCFNumberIntType'
    #[doc(alias = "kCGMouseEventSubtype")]
    pub const MOUSE_EVENT_SUBTYPE: Self = Self(7);

    /// Key to access an integer field, non-zero when this is an autorepeat of
    /// a key-down, and zero otherwise.
    #[doc(alias = "kCGKeyboardEventAutorepeat")]
    pub const KEYBOARD_EVENT_AUTOREPEAT: Self = Self(8);

    /// Key to access an integer field that contains the virtual keycode of the
    /// key-down or key-up event.
    #[doc(alias = "kCGKeyboardEventKeycode")]
    pub const KEYBOARD_EVENT_KEYCODE: Self = Self(9);

    /// Key to access an integer field that contains the keyboard type identifier.
    #[doc(alias = "kCGKeyboardEventKeyboardType")]
    pub const KEYBOARD_EVENT_KEYBOARD_TYPE: Self = Self(10);

    /// Key to access an integer field that contains scrolling data. This field
    /// typically contains the change in vertical position since the last
    /// scrolling event from a Mighty Mouse scroller or a single-wheel mouse scroller.
    #[doc(alias = "kCGScrollWheelEventDeltaAxis1")]
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS1: Self = Self(11);

    /// Key to access an integer field that contains scrolling data. This field
    /// typically contains the change in horizontal position since the last
    /// scrolling event from a Mighty Mouse scroller.
    #[doc(alias = "kCGScrollWheelEventDeltaAxis2")]
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS2: Self = Self(12);

    // ...
}
