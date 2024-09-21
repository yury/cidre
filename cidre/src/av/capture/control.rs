use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVCaptureControl")]
    pub Control(ns::Id)
);

impl Control {
    #[objc::msg_send(isEnabled)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_enabled(&mut self, val: bool);
}
