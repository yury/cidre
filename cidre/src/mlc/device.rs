use crate::{arc, define_obj_type, mlc, mtl, ns, objc};

define_obj_type!(Device(ns::Id), MLC_DEVICE);
impl Device {
    #[objc::cls_msg_send(aneDevice)]
    pub fn ane_ar() -> Option<arc::Rar<Device>>;

    #[objc::cls_rar_retain]
    pub fn ane() -> Option<arc::R<Device>>;

    #[objc::cls_msg_send(gpuDevice)]
    pub fn gpu_ar() -> Option<arc::Rar<Device>>;

    #[objc::cls_rar_retain]
    pub fn gpu() -> Option<arc::R<Device>>;

    #[objc::cls_msg_send(cpuDevice)]
    pub fn cpu_ar() -> arc::Rar<Device>;

    #[objc::cls_rar_retain]
    pub fn cpu() -> arc::R<Device>;

    #[objc::cls_msg_send(deviceWithType:)]
    pub fn with_type_ar(type_: mlc::DeviceType) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_type(type_: mlc::DeviceType) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(deviceWithType:selectsMultipleComputeDevices:)]
    pub fn with_type_multiple_devices_ar(
        type_: mlc::DeviceType,
        selects_mutliple_devices: bool,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_type_multiple_devices(
        type_: mlc::DeviceType,
        selects_mutliple_devices: bool,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(deviceWithGPUDevices:)]
    pub fn with_gpus_ar(gpus: &ns::Array<mtl::Device>) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_gpus(gpus: &ns::Array<mtl::Device>) -> Option<arc::R<Self>>;

    #[objc::msg_send(gpuDevices)]
    pub fn gpus(&self) -> &ns::Array<mtl::Device>;

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
        assert_eq!(device.device_type(), mlc::DeviceType::ANE);
        println!("{device:?}");

        let device = mlc::Device::cpu();
        assert_eq!(device.device_type(), mlc::DeviceType::CPU);
        println!("{device:?}");

        let device = mlc::Device::gpu().unwrap();
        assert_eq!(device.device_type(), mlc::DeviceType::GPU);
        println!("{device:?}");

        assert!(!device.gpus().is_empty());
    }
}
