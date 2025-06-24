use crate::{define_obj_type, ns, objc};

#[objc::protocol(MLComputeDeviceProtocol)]
pub trait ComputeDevice: objc::Obj {}

define_obj_type!(
    pub AnyComputeDevice(ns::Id)
);

impl ComputeDevice for AnyComputeDevice {}
