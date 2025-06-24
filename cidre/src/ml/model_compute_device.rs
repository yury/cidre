use crate::{arc, ml, ns, objc};

/// MLComputeDevice
impl ml::Model {
    #[objc::msg_send(availableComputeDevices)]
    pub fn available_compute_devices() -> arc::R<ns::Array<ml::AnyComputeDevice>>;
}
