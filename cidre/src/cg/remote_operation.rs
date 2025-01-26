use crate::define_opts;

#[doc(alias = "CGButtonCount")]
pub type ButtonCount = u32;

#[doc(alias = "CGWheelCount")]
pub type WheelCount = u32;

#[doc(alias = "CGCharCode")]
pub type CharCode = u16;

#[doc(alias = "CGKeyCode")]
pub type KeyCode = u16;

define_opts!(
    #[doc(alias = "CGEventFilterMask")]
    pub EventFilterMask(u32)
);

impl EventFilterMask {
    #[doc(alias = "kCGEventFilterMaskPermitLocalMouseEvents")]
    pub const PERMIT_LOCAL_MOUSE_EVENTS: Self = Self(0x00000001);

    #[doc(alias = "kCGEventFilterMaskPermitLocalKeyboardEvents")]
    pub const PERMIT_LOCAL_KEYBOARD_EVENTS: Self = Self(0x00000002);

    #[doc(alias = "kCGEventFilterMaskPermitSystemDefinedEvents")]
    pub const PERMIT_SYSTEM_DEFINED_EVENTS: Self = Self(0x00000004);

    #[doc(alias = "kCGEventFilterMaskPermitAllEvents")]
    pub const ALL_EVENTS: Self = Self(
        Self::PERMIT_LOCAL_MOUSE_EVENTS.0
            | Self::PERMIT_LOCAL_KEYBOARD_EVENTS.0
            | Self::PERMIT_SYSTEM_DEFINED_EVENTS.0,
    );
}

#[doc(alias = "CGEventSuppressionState")]
#[repr(u32)]
pub enum EventSuppressionState {
    #[doc(alias = "kCGEventSuppressionStateSuppressionInterval")]
    SuppressionInterval = 0,

    #[doc(alias = "kCGEventSuppressionStateRemoteMouseDrag")]
    RemoteMouseDrag,

    #[doc(alias = "kCGNumberOfEventSuppressionStates")]
    NumberOfEventSuppressionStates,
}
