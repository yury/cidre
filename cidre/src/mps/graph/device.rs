use crate::{arc, define_cls, define_obj_type, mtl, ns, objc};

#[doc(alias = "MPSGraphDeviceType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DeviceType {
    Metal = 0,
}

define_obj_type!(pub Device(ns::Id));

impl Device {
    define_cls!(MPS_GRAPH_DEVICE);

    #[objc::msg_send(metalDevice)]
    pub fn metal_device(&self) -> Option<&mtl::Device>;

    #[objc::msg_send(type)]
    pub fn device_type(&self) -> DeviceType;

    #[objc::cls_msg_send(deviceWithMTLDevice:)]
    pub fn with_mtl_device_ar(mtl_device: &mtl::Device) -> arc::Rar<Device>;

    #[objc::cls_rar_retain()]
    pub fn with_mtl_device(mtl_device: &mtl::Device) -> arc::R<Device>;
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    static MPS_GRAPH_DEVICE: &'static objc::Class<Device>;
}

#[cfg(test)]
mod tests {
    use crate::{mps::graph, mtl};

    #[test]
    fn basics() {
        let device = graph::Device::with_mtl_device(&mtl::Device::default().unwrap());

        assert_eq!(device.device_type(), graph::DeviceType::Metal);
    }
}
