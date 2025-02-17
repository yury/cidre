use crate::{arc, define_obj_type, mlc, mtl, ns, objc};

define_obj_type!(pub Device(ns::Id), MLC_DEVICE);
impl Device {
    #[objc::msg_send(aneDevice)]
    pub fn ane() -> Option<arc::R<Device>>;

    #[objc::msg_send(gpuDevice)]
    pub fn gpu() -> Option<arc::R<Device>>;

    #[objc::msg_send(cpuDevice)]
    pub fn cpu() -> arc::R<Device>;

    #[objc::msg_send(deviceWithType:)]
    pub fn with_type(type_: mlc::DeviceType) -> Option<arc::R<Self>>;

    #[objc::msg_send(deviceWithType:selectsMultipleComputeDevices:)]
    pub fn with_type_multiple_devices(
        type_: mlc::DeviceType,
        selects_mutliple_devices: bool,
    ) -> Option<arc::R<Self>>;

    #[objc::msg_send(deviceWithGPUDevices:)]
    pub fn with_gpus(gpus: &ns::Array<mtl::Device>) -> Option<arc::R<Self>>;

    #[objc::msg_send(gpuDevices)]
    pub fn gpus(&self) -> arc::R<ns::Array<mtl::Device>>;

    #[objc::msg_send(type)]
    pub fn device_type(&self) -> mlc::DeviceType;

    #[objc::msg_send(actualDeviceType)]
    pub fn actual_device_type(&self) -> mlc::DeviceType;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_DEVICE: &'static objc::Class<Device>;
}

#[cfg(test)]
mod tests {
    use crate::mlc;
    #[test]
    pub fn basics() {
        let device = mlc::Device::ane().unwrap();
        assert_eq!(device.device_type(), mlc::DeviceType::Ane);
        println!("{device:?}");

        let device = mlc::Device::cpu();
        assert_eq!(device.device_type(), mlc::DeviceType::Cpu);
        println!("{device:?}");

        let device = mlc::Device::gpu().unwrap();
        assert_eq!(device.device_type(), mlc::DeviceType::Gpu);
        println!("{device:?}");

        assert!(!device.gpus().is_empty());
    }
}
