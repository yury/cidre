use std::{ffi::c_void, ops::Deref};

use crate::{arc, cf, define_cls_init, define_obj_type, mach, ns, objc, objc::Delegate};

define_obj_type!(Port(ns::Id), NS_PORT);
define_obj_type!(MachPort(Port), NS_MACH_PORT);

impl Port {
    #[objc::msg_send(invalidate)]
    pub fn invalidate(&self);

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
    fn _set_delegate(&self, delegate: Option<&ns::Id>);

    pub fn set_delegate<D>(&mut self, delegate: &Delegate<D>)
    where
        D: MachPortDelegate,
    {
        let obj = delegate.obj.deref();
        self._set_delegate(Some(obj))
    }

    pub fn remove_delegate(&mut self) {
        self._set_delegate(None)
    }
}

pub trait MachPortDelegate {
    extern "C" fn handle_mach_message(msg: *mut c_void);

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] =
            [b.as_ref() as *const _ as _, Self::handle_mach_message as _];

        let ptr = table.as_ptr();

        let obj = unsafe { make_mach_port_delegate(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_PORT: &'static objc::Class<Port>;
    static NS_MACH_PORT: &'static objc::Class<MachPort>;
    fn make_mach_port_delegate(vtable: *const *const c_void) -> arc::R<ns::Id>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn create() {
        let mach = ns::MachPort::new();
        let port = mach.mach_port();
        assert_ne!(port.0, 0);
        assert!(mach.is_valid());
        mach.invalidate();
        assert!(!mach.is_valid());

        // TODO: investigate why invalidate decrease refs count
        std::mem::forget(mach);
    }
}
