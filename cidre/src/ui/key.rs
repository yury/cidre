use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    pub Key(ns::Id)
);

impl Key {
    /// A string representing what would be inserted into a text field when this key is pressed.
    ///
    /// If a modifier key is held, this property will contain the modified characters according
    /// the rules for that particular modifier key (i.e., if shift is held on a Latin keyboard, this will
    /// contain capital letters).
    #[objc::msg_send(characters)]
    pub fn characters(&self) -> arc::R<ns::String>;

    /// A string representing which characters would be inserted into a text field when this key is
    /// pressed, not taking any held modifiers into account.
    ///
    /// For Latin based languages, expect this to be always in lowercase (unmodified meaning not
    /// taking shift key into account). If only a modifier key was pressed, this property will contain an empty string.
    #[objc::msg_send(charactersIgnoringModifiers)]
    pub fn characters_ignoring_modifiers(&self) -> arc::R<ns::String>;

    /// A bitfield representing which modifier keys are currently being held in addition to this key.
    #[objc::msg_send(modifierFlags)]
    pub fn mod_flags(&self) -> ui::KeyModFlags;

    /// The raw HID usage code for the pressed key.
    #[objc::msg_send(keyCode)]
    pub fn key_code(&self) -> ui::KeyboardHidUsage;
}
