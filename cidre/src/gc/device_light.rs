use crate::{arc, define_obj_type, gc, ns, objc};

define_obj_type!(
    #[doc(alias = "GCDeviceLight")]
    pub DeviceLight(ns::Id)
);

impl DeviceLight {
    #[objc::msg_send(color)]
    pub fn color_ar(&self) -> arc::Rar<gc::Color>;

    #[objc::rar_retain]
    pub fn color(&self) -> arc::R<gc::Color>;
}
