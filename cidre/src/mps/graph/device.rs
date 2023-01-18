use crate::{arc, define_obj_type, msg_send, mtl, ns};

#[doc(alias = "MPSGraphDeviceType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DeviceType {
    Metal = 0,
}

define_obj_type!(Device(ns::Id));

impl Device {
    #[inline]
    pub fn metal_device(&self) -> Option<&mtl::Device> {
        unsafe { rsel_metalDevice(self) }
    }

    #[inline]
    pub fn device_type(&self) -> DeviceType {
        msg_send!("mpsg", self, sel_type)
    }

    #[inline]
    pub fn with_mtl_device(mtl_device: &mtl::Device) -> arc::R<Device> {
        unsafe { MPSGraphDevice_deviceWithMTLDevice(mtl_device) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_metalDevice(device: &Device) -> Option<&mtl::Device>;
    fn MPSGraphDevice_deviceWithMTLDevice(mtl_device: &mtl::Device) -> arc::R<Device>;
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
