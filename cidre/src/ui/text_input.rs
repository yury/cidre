use crate::{ns, objc, ui};

/// The text input operations shared by UIKit text fields and text views.
#[objc::protocol(UIKeyInput)]
pub trait KeyInput: objc::Obj {
    #[objc::msg_send(hasText)]
    fn has_text(&self) -> bool;

    #[objc::msg_send(insertText:)]
    fn insert_text(&mut self, text: &ns::String);

    #[objc::msg_send(deleteBackward)]
    fn delete_backward(&mut self);
}

impl KeyInput for ui::TextField {}
impl KeyInput for ui::TextView {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
impl KeyInput for ui::SearchTextField {}
