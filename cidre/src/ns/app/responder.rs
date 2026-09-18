use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    pub Responder(ns::Id),
    NS_RESPONDER
);

impl Responder {
    #[objc::msg_send(nextResponder)]
    pub fn next_responder(&self) -> Option<arc::R<ns::Responder>>;

    /// Turns key events into commands (`insertText:`, `cancelOperation:`, ...)
    /// sent through `doCommandBySelector:`, up the responder chain.
    #[objc::msg_send(interpretKeyEvents:)]
    pub fn interpret_key_events(&mut self, events: &ns::Array<ns::Event>);

    /// Sends `action` to the first responder in the chain from this one
    /// that responds to it; whether one did.
    #[objc::msg_send(tryToPerform:with:)]
    pub fn try_to_perform_with(&mut self, action: &objc::Sel, obj: Option<&ns::Id>) -> bool;
}

unsafe extern "C" {
    static NS_RESPONDER: &'static objc::Class<Responder>;
}
