use crate::{define_obj_type, ns, objc};

#[objc::protocol(MLComputeDeviceProtocol)]
pub trait ComputeDevice {}

define_obj_type!(
    pub AnyComputeDevice(ns::Id)
);

impl ComputeDevice for AnyComputeDevice {}
