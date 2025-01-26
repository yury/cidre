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

impl EventFlags {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum EventType {
    Null = 0,
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    MouseMoved = 5,
    LeftMouseDragged = 6,
    RightMouseDragged = 7,

    KeyDown = 10,
    KeyUp = 11,
    FlagsChanged = 12,

    ScrollWheel = 22,
    TabletPointer = 23,
    TabletProximity = 24,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    OtherMouseDragged = 27,

    TapDisabledByTimeout = 0xFFFFFFFE,
    TapDisabledByUserInput = 0xFFFFFFFF,
}

impl EventType {
    #[doc(alias = "CGEventMaskBit")]
    pub fn mask(&self) -> EventMask {
        1 << *self as u32
    }

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
