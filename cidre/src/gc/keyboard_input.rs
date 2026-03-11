use crate::{arc, define_obj_type, gc, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "GCKeyboardInput")]
    pub KeyboardInput(gc::PhysicalInputProfile)
);

#[cfg(feature = "blocks")]
#[doc(alias = "GCKeyboardValueChangedHandler")]
pub type ValueChangedHandler = blocks::EscBlock<
    fn(keyboard: &KeyboardInput, key: &gc::DeviceButtonInput, key_code: gc::KeyCode, pressed: bool),
>;

impl KeyboardInput {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(keyChangedHandler)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn key_changed_handler(&self) -> Option<&ValueChangedHandler>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setKeyChangedHandler:)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn set_key_changed_handler(&mut self, val: Option<&mut ValueChangedHandler>);

    #[objc::msg_send(isAnyKeyPressed)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn is_any_key_pressed(&self) -> bool;

    #[objc::msg_send(buttonForKeyCode:)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn button_for_key_code(&self, code: gc::KeyCode) -> Option<arc::R<gc::DeviceButtonInput>>;
}
