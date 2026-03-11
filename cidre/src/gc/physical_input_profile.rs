use crate::{define_obj_type, gc, ns};

define_obj_type!(
    #[doc(alias = "GCPhysicalInputProfile")]
    pub PhysicalInputProfile(ns::Id)
);

#[doc(alias = "GCDeviceElement")]
pub type DeviceElement = gc::ControllerElement;

#[doc(alias = "GCDeviceButtonInput")]
pub type DeviceButtonInput = gc::ControllerButtonInput;
