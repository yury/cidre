use crate::{arc, define_obj_type, ns, objc};

impl arc::A<WindowController> {
    #[objc::msg_send(initWithWindow:)]
    pub fn init_with_window(self, window: Option<&ns::Window>) -> arc::R<WindowController>;
}

define_obj_type!(
    pub WindowController(ns::Responder),
    NS_WINDOW_CONTROLLER
);

impl WindowController {
    pub fn with_window(window: Option<&ns::Window>) -> arc::R<Self> {
        Self::alloc().init_with_window(window)
    }

    #[objc::msg_send(shouldCascadeWindows)]
    pub fn should_cascade_windows(&self) -> bool;

    #[objc::msg_send(setShouldCascadeWindows:)]
    pub fn set_should_cascade_windows(&mut self, val: bool);

    #[objc::msg_send(showWindow:)]
    pub fn show_window(&self, sender: Option<&ns::Id>);

    #[objc::msg_send(close)]
    pub fn close(&self);

    #[objc::msg_send(dismissController:)]
    pub fn dismiss_controller(&self, sender: Option<&ns::Id>);
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_WINDOW_CONTROLLER: &'static objc::Class<WindowController>;
}
