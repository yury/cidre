use crate::{define_obj_type, mlc, ns, objc};

define_obj_type!(Layer(ns::Id));

impl Layer {
    #[objc::msg_send(layerID)]
    pub fn layer_id(&self) -> usize;

    #[objc::msg_send(label)]
    pub fn label(&self) -> &ns::String;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, value: &ns::String);

    #[objc::msg_send(isDebuggingEnabled)]
    pub fn is_debug_enabled(&self) -> bool;

    #[objc::msg_send(setIsDebuggingEnabled:)] // TODO: check setter signature
    pub fn set_debug_enabled(&mut self, value: bool);

    #[objc::msg_send(deviceType)]
    pub fn device_type(&self) -> mlc::DeviceType;
}
