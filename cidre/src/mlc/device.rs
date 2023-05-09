use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(Device(ns::Id));
impl Device {
    define_cls!(MLC_DEVICE);

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
        println!("{device:?}");

        let device = mlc::Device::cpu();
        println!("{device:?}");

        let device = mlc::Device::gpu().unwrap();
        println!("{device:?}");
    }
}
