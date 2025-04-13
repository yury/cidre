use crate::{define_obj_type, define_opts, ns, objc};

#[doc(alias = "UIEventType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
#[non_exhaustive]
pub enum EventType {
    Touches,
    Motion,
    RemoteControl,
    Presses,
    Scroll = 10,
    Hover = 11,
    Transform = 14,
}

#[doc(alias = "UIEventSubtype")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
#[non_exhaustive]
pub enum EventSubType {
    None = 0,
    MotionSnake = 1,
    RemoteControlPlay = 100,
    RemoteControlPause = 101,
    RemoteControlStop = 102,
    RemoteControlTogglePlayPause = 103,
    RemoteControlNextTrack = 104,
    RemoteControlPrevTrack = 105,
    RemoteControlBeginSeekingBackward = 106,
    RemoteControlEndSeekingBackward = 107,
    RemoteControlBeginSeekingForward = 108,
    RemoteControlEndSeekingForward = 109,
}

define_opts!(
    pub EventButtonMask(isize)
);

impl EventButtonMask {
    pub const PRIMARY: Self = Self(1 << 0);
    pub const SECONDARY: Self = Self(1 << 1);
}

define_obj_type!(
    pub Event(ns::Id)
);

impl Event {
    #[objc::msg_send(type)]
    pub fn type_(&self) -> EventType;

    #[objc::msg_send(subtype)]
    pub fn sub_type(&self) -> EventSubType;

    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> ns::TimeInterval;
}
