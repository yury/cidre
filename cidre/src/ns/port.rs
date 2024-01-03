use crate::{arc, cf, define_obj_type, mach, ns, objc};

define_obj_type!(
    #[doc(alias = "NSPort")]
    pub Port(ns::Id), NS_PORT
);

define_obj_type!(
    #[doc(alias = "NSMachPort")]
    pub MachPort(Port), NS_MACH_PORT
);

impl Port {
    #[objc::msg_send(invalidate)]
    pub fn invalidate(&mut self);

    #[objc::msg_send(isValid)]
    pub fn is_valid(&self) -> bool;
}

impl MachPort {
    #[objc::msg_send(machPort)]
    pub fn mach_port(&self) -> mach::Port;

    #[objc::msg_send(scheduleInRunLoop:forMode:)]
    pub fn schedule_in_runloop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);

    #[objc::msg_send(scheduleInRunLoop:forMode:)]
    pub fn remove_from_runloop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);

    #[objc::msg_send(setDelegate:)]
    fn set_delegate<D: MachPortDelegate>(&mut self, delegate: Option<&D>);

    #[objc::msg_send(delegate)]
    fn delegate(&self) -> Option<&objc::Any>;
}

#[objc::obj_trait]
pub trait MachPortDelegate {
    #[objc::optional]
    #[objc::msg_send(handleMachMessage:)]
    fn handle_mach_message(&mut self, msg: *mut std::ffi::c_void);
}

impl MachPortDelegate for objc::Any {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_PORT: &'static objc::Class<Port>;
    static NS_MACH_PORT: &'static objc::Class<MachPort>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn create() {
        let mut mach = ns::MachPort::new();
        let port = mach.mach_port();
        assert_ne!(port.0, 0);
        assert!(mach.is_valid());
        mach.invalidate();
        assert!(!mach.is_valid());
    }
}
