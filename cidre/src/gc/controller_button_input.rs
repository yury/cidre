#[allow(unused_imports)]
use crate::{define_obj_type, gc, objc};

define_obj_type!(
    #[doc(alias = "GCControllerButtonInput")]
    pub ControllerButtonInput(gc::ControllerElement)
);

impl ControllerButtonInput {
    #[objc::msg_send(value)]
    pub fn value(&self) -> f32;

    #[objc::msg_send(isPressed)]
    pub fn is_pressed(&self) -> bool;

    #[objc::msg_send(isTouched)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn is_touched(&self) -> bool;

    #[objc::msg_send(setValue:)]
    #[objc::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    pub fn set_value(&mut self, value: f32);
}
