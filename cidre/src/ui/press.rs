use crate::{arc, cg, define_obj_type, ns, objc, ui};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PressPhase {
    /// Whenever a button press begins.
    Began,

    /// Whenever a button moves.
    Changed,

    /// Whenever a buttons was pressed and is still being held down.
    Stationary,

    /// Whenever a button is released.
    Ended,

    /// Whenever a button press doesn't end but we need to stop tracking.
    Canceled,
}

#[doc(alias = "UIPressType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PressType {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Select,
    Menu,
    PlayPause,
    PageUp = 30,
    PageDown = 31,
    TvRemote123 = 32,
    TVRemote4Colors = 33,
}

define_obj_type!(
    #[doc(alias = "UIPress")]
    pub Press(ns::Id)
);

impl Press {
    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> ns::TimeInterval;

    #[objc::msg_send(phase)]
    pub fn phase(&self) -> PressPhase;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> PressType;

    #[objc::msg_send(window)]
    pub fn window(&self) -> Option<arc::R<ui::Window>>;

    #[objc::msg_send(responder)]
    pub fn responder(&self) -> Option<arc::R<ui::Responder>>;

    #[objc::msg_send(gestureRecognizers)]
    pub fn gesture_recognizers(&self) -> Option<arc::R<ns::Array<ui::GestureRecognizer>>>;

    /// For analog buttons, returns a value between 0 and 1.  Digital buttons return 0 or 1.
    #[objc::msg_send(force)]
    pub fn force(&self) -> cg::Float;

    #[objc::msg_send(key)]
    pub fn key(&self) -> Option<arc::R<ui::Key>>;
}
