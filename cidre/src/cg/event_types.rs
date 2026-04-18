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

impl From<i64> for GesturePhase {
    fn from(value: i64) -> Self {
        match value {
            1 => GesturePhase::Began,
            2 => GesturePhase::Changed,
            4 => GesturePhase::Ended,
            8 => GesturePhase::Cancelled,
            128 => GesturePhase::MayBegin,
            _ => GesturePhase::None,
        }
    }
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

    #[doc(alias = "kCGAnyInputEventType")]
    pub const ALL_EVENTS_MASK: EventMask = !0;
}

// Private API
impl EventType {
    #[doc(alias = "kCGSEventZoom")]
    pub const ZOOM: Self = Self(28);

    #[doc(alias = "kCGSEventGesture")]
    pub const GESTURE: Self = Self(29);

    #[doc(alias = "kCGSEventDockControl")]
    pub const DOCK_CONTROL: Self = Self(30);

    #[doc(alias = "kCGSEventFluidTouchGesture")]
    pub const FLUID_TOUCH_GESTURE: Self = Self(31);
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
pub enum EventSrcState {
    #[doc(alias = "kCGEventSourceStatePrivate")]
    Private = -1,

    #[doc(alias = "kCGEventSourceStateCombinedSessionState")]
    CombinedSession = 0,

    #[doc(alias = "kCGEventSourceStateHIDSystemState")]
    HidSys = 1,
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

    /// This field is not used.
    #[doc(alias = "kCGScrollWheelEventDeltaAxis3")]
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS3: Self = Self(13);

    /// Key to access a field that contains scrolling data. The scrolling data
    /// represents a line-based or pixel-based change in vertical position
    /// since the last scrolling event from a Mighty Mouse scroller or a
    /// single-wheel mouse scroller. The scrolling data uses a fixed-point
    /// 16.16 signed integer format. If this key is passed to
    /// `CGEventGetDoubleValueField`, the fixed-point value is converted to a
    /// double value.
    #[doc(alias = "kCGScrollWheelEventFixedPtDeltaAxis1")]
    pub const SCROLL_WHEEL_EVENT_FIXED_PT_DELTA_AXIS1: Self = Self(93);

    /// Key to access a field that contains scrolling data. The scrolling data
    /// represents a line-based or pixel-based change in horizontal position
    /// since the last scrolling event from a Mighty Mouse scroller. The
    /// scrolling data uses a fixed-point 16.16 signed integer format. If this
    /// key is passed to `CGEventGetDoubleValueField`, the fixed-point value is
    /// converted to a double value.
    #[doc(alias = "kCGScrollWheelEventFixedPtDeltaAxis2")]
    pub const SCROLL_WHEEL_EVENT_FIXED_PT_DELTA_AXIS2: Self = Self(94);

    /// This field is not used.
    #[doc(alias = "kCGScrollWheelEventFixedPtDeltaAxis3")]
    pub const SCROLL_WHEEL_EVENT_FIXED_PT_DELTA_AXIS3: Self = Self(95);

    /// Key to access an integer field that contains pixel-based scrolling
    /// data. The scrolling data represents the change in vertical position
    /// since the last scrolling event from a Mighty Mouse scroller or a
    /// single-wheel mouse scroller.
    #[doc(alias = "kCGScrollWheelEventPointDeltaAxis1")]
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS1: Self = Self(96);

    /// Key to access an integer field that contains pixel-based scrolling
    /// data. The scrolling data represents the change in horizontal position
    /// since the last scrolling event from a Mighty Mouse scroller.
    #[doc(alias = "kCGScrollWheelEventPointDeltaAxis2")]
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS2: Self = Self(97);

    /// This field is not used.
    #[doc(alias = "kCGScrollWheelEventPointDeltaAxis3")]
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS3: Self = Self(98);

    /// Key to access an integer field that contains the scroll phase.
    #[doc(alias = "kCGScrollWheelEventScrollPhase")]
    pub const SCROLL_WHEEL_EVENT_SCROLL_PHASE: Self = Self(99);

    /// Key to access an integer field that contains the scroll count.
    #[doc(alias = "kCGScrollWheelEventScrollCount")]
    pub const SCROLL_WHEEL_EVENT_SCROLL_COUNT: Self = Self(100);

    /// Key to access an integer field that contains the momentum phase.
    #[doc(alias = "kCGScrollWheelEventMomentumPhase")]
    pub const SCROLL_WHEEL_EVENT_MOMENTUM_PHASE: Self = Self(123);

    /// Key to access an integer field that indicates whether the event should
    /// be ignored by the Inkwell subsystem. If the value is non-zero, the
    /// event should be ignored.
    #[doc(alias = "kCGScrollWheelEventInstantMouser")]
    pub const SCROLL_WHEEL_EVENT_INSTANT_MOUSER: Self = Self(14);

    /// Key to access an integer field that contains the absolute X coordinate
    /// in tablet space at full tablet resolution.
    #[doc(alias = "kCGTabletEventPointX")]
    pub const TABLET_EVENT_POINT_X: Self = Self(15);

    /// Key to access an integer field that contains the absolute Y coordinate
    /// in tablet space at full tablet resolution.
    #[doc(alias = "kCGTabletEventPointY")]
    pub const TABLET_EVENT_POINT_Y: Self = Self(16);

    /// Key to access an integer field that contains the absolute Z coordinate
    /// in tablet space at full tablet resolution.
    #[doc(alias = "kCGTabletEventPointZ")]
    pub const TABLET_EVENT_POINT_Z: Self = Self(17);

    /// Key to access an integer field that contains the tablet button state.
    /// Bit 0 is the first button, and a set bit represents a closed or pressed
    /// button. Up to 16 buttons are supported.
    #[doc(alias = "kCGTabletEventPointButtons")]
    pub const TABLET_EVENT_POINT_BUTTONS: Self = Self(18);

    /// Key to access a double field that contains the tablet pen pressure. A
    /// value of 0.0 represents no pressure, and 1.0 represents maximum
    /// pressure.
    #[doc(alias = "kCGTabletEventPointPressure")]
    pub const TABLET_EVENT_POINT_PRESSURE: Self = Self(19);

    /// Key to access a double field that contains the horizontal tablet pen
    /// tilt. A value of 0 represents no tilt, and 1 represents maximum tilt.
    #[doc(alias = "kCGTabletEventTiltX")]
    pub const TABLET_EVENT_TILT_X: Self = Self(20);

    /// Key to access a double field that contains the vertical tablet pen
    /// tilt. A value of 0 represents no tilt, and 1 represents maximum tilt.
    #[doc(alias = "kCGTabletEventTiltY")]
    pub const TABLET_EVENT_TILT_Y: Self = Self(21);

    /// Key to access a double field that contains the tablet pen rotation.
    #[doc(alias = "kCGTabletEventRotation")]
    pub const TABLET_EVENT_ROTATION: Self = Self(22);

    /// Key to access a double field that contains the tangential pressure on
    /// the device. A value of 0.0 represents no pressure, and 1.0 represents
    /// maximum pressure.
    #[doc(alias = "kCGTabletEventTangentialPressure")]
    pub const TABLET_EVENT_TANGENTIAL_PRESSURE: Self = Self(23);

    /// Key to access an integer field that contains the system-assigned unique
    /// device ID.
    #[doc(alias = "kCGTabletEventDeviceID")]
    pub const TABLET_EVENT_DEVICE_ID: Self = Self(24);

    /// Key to access an integer field that contains a vendor-specified value.
    #[doc(alias = "kCGTabletEventVendor1")]
    pub const TABLET_EVENT_VENDOR_1: Self = Self(25);

    /// Key to access an integer field that contains a vendor-specified value.
    #[doc(alias = "kCGTabletEventVendor2")]
    pub const TABLET_EVENT_VENDOR_2: Self = Self(26);

    /// Key to access an integer field that contains a vendor-specified value.
    #[doc(alias = "kCGTabletEventVendor3")]
    pub const TABLET_EVENT_VENDOR_3: Self = Self(27);

    /// Key to access an integer field that contains the vendor-defined ID,
    /// typically the USB vendor ID.
    #[doc(alias = "kCGTabletProximityEventVendorID")]
    pub const TABLET_PROXIMITY_EVENT_VENDOR_ID: Self = Self(28);

    /// Key to access an integer field that contains the vendor-defined tablet
    /// ID, typically the USB product ID.
    #[doc(alias = "kCGTabletProximityEventTabletID")]
    pub const TABLET_PROXIMITY_EVENT_TABLET_ID: Self = Self(29);

    /// Key to access an integer field that contains the vendor-defined ID of
    /// the pointing device.
    #[doc(alias = "kCGTabletProximityEventPointerID")]
    pub const TABLET_PROXIMITY_EVENT_POINTER_ID: Self = Self(30);

    /// Key to access an integer field that contains the system-assigned device
    /// ID.
    #[doc(alias = "kCGTabletProximityEventDeviceID")]
    pub const TABLET_PROXIMITY_EVENT_DEVICE_ID: Self = Self(31);

    /// Key to access an integer field that contains the system-assigned unique
    /// tablet ID.
    #[doc(alias = "kCGTabletProximityEventSystemTabletID")]
    pub const TABLET_PROXIMITY_EVENT_SYSTEM_TABLET_ID: Self = Self(32);

    /// Key to access an integer field that contains the vendor-assigned
    /// pointer type.
    #[doc(alias = "kCGTabletProximityEventVendorPointerType")]
    pub const TABLET_PROXIMITY_EVENT_VENDOR_POINTER_TYPE: Self = Self(33);

    /// Key to access an integer field that contains the vendor-defined pointer
    /// serial number.
    #[doc(alias = "kCGTabletProximityEventVendorPointerSerialNumber")]
    pub const TABLET_PROXIMITY_EVENT_VENDOR_POINTER_SERIAL_NUMBER: Self = Self(34);

    /// Key to access an integer field that contains the vendor-defined unique
    /// ID.
    #[doc(alias = "kCGTabletProximityEventVendorUniqueID")]
    pub const TABLET_PROXIMITY_EVENT_VENDOR_UNIQUE_ID: Self = Self(35);

    /// Key to access an integer field that contains the device capabilities
    /// mask.
    #[doc(alias = "kCGTabletProximityEventCapabilityMask")]
    pub const TABLET_PROXIMITY_EVENT_CAPABILITY_MASK: Self = Self(36);

    /// Key to access an integer field that contains the pointer type.
    #[doc(alias = "kCGTabletProximityEventPointerType")]
    pub const TABLET_PROXIMITY_EVENT_POINTER_TYPE: Self = Self(37);

    /// Key to access an integer field that indicates whether the pen is in
    /// proximity to the tablet. The value is non-zero if the pen is in
    /// proximity to the tablet and zero when leaving the tablet.
    #[doc(alias = "kCGTabletProximityEventEnterProximity")]
    pub const TABLET_PROXIMITY_EVENT_ENTER_PROXIMITY: Self = Self(38);

    /// Key to access a field that contains the event target process serial
    /// number. The value is a 64-bit value.
    #[doc(alias = "kCGEventTargetProcessSerialNumber")]
    pub const EVENT_TARGET_PROCESS_SERIAL_NUMBER: Self = Self(39);

    /// Key to access a field that contains the event target Unix process ID.
    #[doc(alias = "kCGEventTargetUnixProcessID")]
    pub const EVENT_TARGET_UNIX_PROCESS_ID: Self = Self(40);

    /// Key to access a field that contains the event source Unix process ID.
    #[doc(alias = "kCGEventSourceUnixProcessID")]
    pub const EVENT_SOURCE_UNIX_PROCESS_ID: Self = Self(41);

    /// Key to access a field that contains the event source user-supplied
    /// data, up to 64 bits.
    #[doc(alias = "kCGEventSourceUserData")]
    pub const EVENT_SOURCE_USER_DATA: Self = Self(42);

    /// Key to access a field that contains the event source Unix effective
    /// UID.
    #[doc(alias = "kCGEventSourceUserID")]
    pub const EVENT_SOURCE_USER_ID: Self = Self(43);

    /// Key to access a field that contains the event source Unix effective
    /// GID.
    #[doc(alias = "kCGEventSourceGroupID")]
    pub const EVENT_SOURCE_GROUP_ID: Self = Self(44);

    /// Key to access a field that contains the event source state ID used to
    /// create this event.
    #[doc(alias = "kCGEventSourceStateID")]
    pub const EVENT_SOURCE_STATE_ID: Self = Self(45);

    /// Key to access an integer field that indicates whether a scrolling event
    /// contains continuous, pixel-based scrolling data. The value is non-zero
    /// when the scrolling data is pixel-based and zero when the scrolling data
    /// is line-based.
    #[doc(alias = "kCGScrollWheelEventIsContinuous")]
    pub const SCROLL_WHEEL_EVENT_IS_CONTINUOUS: Self = Self(88);

    /// Key to access an integer field that contains the window under the mouse
    /// pointer.
    #[doc(alias = "kCGMouseEventWindowUnderMousePointer")]
    pub const MOUSE_EVENT_WINDOW_UNDER_MOUSE_POINTER: Self = Self(91);

    /// Key to access an integer field that contains the window under the mouse
    /// pointer that can handle this event.
    #[doc(alias = "kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent")]
    pub const MOUSE_EVENT_WINDOW_UNDER_MOUSE_POINTER_THAT_CAN_HANDLE_THIS_EVENT: Self = Self(92);

    /// Key to access an integer field that contains unaccelerated pointer
    /// movement along the X axis.
    #[doc(alias = "kCGEventUnacceleratedPointerMovementX")]
    pub const EVENT_UNACCELERATED_POINTER_MOVEMENT_X: Self = Self(170);

    /// Key to access an integer field that contains unaccelerated pointer
    /// movement along the Y axis.
    #[doc(alias = "kCGEventUnacceleratedPointerMovementY")]
    pub const EVENT_UNACCELERATED_POINTER_MOVEMENT_Y: Self = Self(171);

    /// Key to access an integer field that contains the optional momentum
    /// phase.
    #[doc(alias = "kCGScrollWheelEventMomentumOptionPhase")]
    pub const SCROLL_WHEEL_EVENT_MOMENTUM_OPTION_PHASE: Self = Self(173);

    /// Key to access an integer field that contains the accelerated vertical
    /// scroll delta.
    #[doc(alias = "kCGScrollWheelEventAcceleratedDeltaAxis1")]
    pub const SCROLL_WHEEL_EVENT_ACCELERATED_DELTA_AXIS1: Self = Self(176);

    /// Key to access an integer field that contains the accelerated
    /// horizontal scroll delta.
    #[doc(alias = "kCGScrollWheelEventAcceleratedDeltaAxis2")]
    pub const SCROLL_WHEEL_EVENT_ACCELERATED_DELTA_AXIS2: Self = Self(175);

    /// Key to access an integer field that contains the raw vertical scroll
    /// delta.
    #[doc(alias = "kCGScrollWheelEventRawDeltaAxis1")]
    pub const SCROLL_WHEEL_EVENT_RAW_DELTA_AXIS1: Self = Self(178);

    /// Key to access an integer field that contains the raw horizontal scroll
    /// delta.
    #[doc(alias = "kCGScrollWheelEventRawDeltaAxis2")]
    pub const SCROLL_WHEEL_EVENT_RAW_DELTA_AXIS2: Self = Self(177);
}

// Private API
impl EventField {
    #[doc(alias = "kCGSEventWindowIDField")]
    pub const WINDOW_ID: Self = Self(51);

    #[doc(alias = "kCGSEventTypeField")]
    pub const TYPE: Self = Self(55);

    #[doc(alias = "kCGEventGestureHIDType")]
    pub const GESTURE_HID_TYPE: Self = Self(110);

    #[doc(alias = "kCGEventGestureIsPreflight")]
    pub const GESTURE_IS_PREFLIGHT: Self = Self(111);

    #[doc(alias = "kCGEventGestureZoomValue")]
    pub const GESTURE_ZOOM_VALUE: Self = Self(113);

    #[doc(alias = "kCGEventGestureRotationValue")]
    pub const GESTURE_ROTATION_VALUE: Self = Self(114);

    #[doc(alias = "kCGEventGestureSwipeValue")]
    pub const GESTURE_SWIPE_VALUE: Self = Self(115);

    #[doc(alias = "kCGEventGesturePreflightProgress")]
    pub const GESTURE_PREFLIGHT_PROGRESS: Self = Self(116);

    #[doc(alias = "kCGEventGestureStartEndSeriesType")]
    pub const GESTURE_START_END_SERIES_TYPE: Self = Self(117);

    #[doc(alias = "kCGEventGestureScrollX")]
    pub const GESTURE_SCROLL_X: Self = Self(118);

    #[doc(alias = "kCGEventGestureScrollY")]
    pub const GESTURE_SCROLL_Y: Self = Self(119);

    #[doc(alias = "kCGEventGestureScrollZ")]
    pub const GESTURE_SCROLL_Z: Self = Self(120);

    #[doc(alias = "kCGEventGestureSwipeMotion")]
    pub const GESTURE_SWIPE_MOTION: Self = Self(123);

    #[doc(alias = "kCGEventGestureSwipeProgress")]
    pub const GESTURE_SWIPE_PROGRESS: Self = Self(124);

    #[doc(alias = "kCGEventGestureSwipePositionX")]
    pub const GESTURE_SWIPE_POSITION_X: Self = Self(125);

    #[doc(alias = "kCGEventGestureSwipePositionY")]
    pub const GESTURE_SWIPE_POSITION_Y: Self = Self(126);

    #[doc(alias = "kCGEventGestureSwipeVelocityX")]
    pub const GESTURE_SWIPE_VELOCITY_X: Self = Self(129);

    #[doc(alias = "kCGEventGestureSwipeVelocityY")]
    pub const GESTURE_SWIPE_VELOCITY_Y: Self = Self(130);

    #[doc(alias = "kCGEventGestureSwipeVelocityZ")]
    pub const GESTURE_SWIPE_VELOCITY_Z: Self = Self(131);

    #[doc(alias = "kCGEventGesturePhase")]
    pub const GESTURE_PHASE: Self = Self(132);

    #[doc(alias = "kCGEventGestureMask")]
    pub const GESTURE_MASK: Self = Self(133);

    #[doc(alias = "kCGEventGestureSwipeMask")]
    pub const GESTURE_SWIPE_MASK: Self = Self(134);

    #[doc(alias = "kCGEventScrollGestureFlagBits")]
    pub const GESTURE_SCROLL_FLAG_BITS: Self = Self(135);

    #[doc(alias = "kCGEventSwipeGestureFlagBits")]
    pub const GESTURE_SWIPE_FLAG_BITS: Self = Self(136);

    #[doc(alias = "kCGEventGestureFlavor")]
    pub const GESTURE_FLAVOR: Self = Self(138);

    #[doc(alias = "kCGEventGestureZoomDeltaX")]
    pub const GESTURE_ZOOM_DELTA_X: Self = Self(139);

    #[doc(alias = "kCGEventGestureZoomDeltaY")]
    pub const GESTURE_ZOOM_DELTA_Y: Self = Self(140);

    #[doc(alias = "kCGEventGestureProgress")]
    pub const GESTURE_PROGRESS: Self = Self(142);

    #[doc(alias = "kCGEventGestureStage")]
    pub const GESTURE_STAGE: Self = Self(143);

    #[doc(alias = "kCGEventGestureBehavior")]
    pub const GESTURE_BEHAVIOR: Self = Self(144);

    #[doc(alias = "kCGEventTransitionProgress")]
    pub const TRANSITION_PROGRESS: Self = Self(147);

    #[doc(alias = "kCGEventStagePressure")]
    pub const STAGE_PRESSURE: Self = Self(148);
}

#[doc(alias = "CGSHIDEventType")]
#[repr(transparent)]
pub struct HidEventType(pub u32);

impl HidEventType {
    #[doc(alias = "kCGHIDEventTypeGestureStarted")]
    pub const GESTURE_STARTED: Self = Self(61);

    #[doc(alias = "kCGHIDEventTypeGestureEnded")]
    pub const GESTURE_ENDED: Self = Self(61);
}
