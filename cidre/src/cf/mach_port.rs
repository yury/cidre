#[cfg(feature = "ns")]
use crate::ns;
use crate::{arc, cf, define_cf_type};

define_cf_type!(
    #[doc(alias = "CFMachPortRef")]
    MachPort(cf::Type)
);

#[doc(alias = "CFMachPortContext")]
#[repr(C)]
pub struct MachPortCtx<T> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C-unwind" fn(*const T) -> *const T>,
    pub release: Option<extern "C-unwind" fn(*const T)>,
    pub desc: Option<extern "C-unwind" fn(*const T) -> arc::R<cf::String>>,
}

impl<T> MachPortCtx<T> {
    pub fn with_info(info: *mut T) -> Self {
        Self {
            info,
            ..Default::default()
        }
    }
}

impl<T> Default for MachPortCtx<T> {
    fn default() -> Self {
        Self {
            version: 0,
            info: std::ptr::null_mut(),
            retain: None,
            release: None,
            desc: None,
        }
    }
}

#[doc(alias = "CFMachPortCallBack")]
pub type MachPortCb<T = std::ffi::c_void> = extern "C-unwind" fn(
    port: &mut cf::MachPort,
    msg: *mut std::ffi::c_void,
    size: cf::Index,
    info: *mut T,
);

#[doc(alias = "CFMachPortInvalidationCallBack")]
pub type MachPortInvalidationCb<T = std::ffi::c_void> =
    extern "C-unwind" fn(port: &mut cf::MachPort, info: *mut T);

impl MachPort {
    #[doc(alias = "CFMachPortGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFMachPortGetTypeID() }
    }

    #[doc(alias = "CFMachPortInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { CFMachPortInvalidate(self) }
    }

    #[doc(alias = "CFMachPortIsValid")]
    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFMachPortIsValid(self) }
    }

    #[doc(alias = "CFMachPortCreateRunLoopSource")]
    #[inline]
    pub fn run_loop_src_in(
        &self,
        order: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::RunLoopSrc>> {
        unsafe { CFMachPortCreateRunLoopSource(allocator, self, order) }
    }

    #[doc(alias = "CFMachPortCreateRunLoopSource")]
    #[inline]
    pub fn run_loop_src(&self, order: cf::Index) -> Option<arc::R<cf::RunLoopSrc>> {
        unsafe { CFMachPortCreateRunLoopSource(None, self, order) }
    }

    #[doc(alias = "CFMachPortGetPort")]
    #[inline]
    pub fn port(&self) -> crate::mach::Port {
        unsafe { CFMachPortGetPort(self) }
    }

    #[doc(alias = "CFMachPortCreate")]
    #[inline]
    pub fn with_ctx_in<T>(
        ctx: Option<&mut MachPortCtx<T>>,
        cb: MachPortCb<T>,
        should_free_info: *mut bool,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CFMachPortCreate(
                allocator,
                std::mem::transmute(cb),
                std::mem::transmute(ctx),
                should_free_info,
            )
        }
    }

    #[doc(alias = "CFMachPortCreateWithPort")]
    #[inline]
    pub fn with_port_in<T>(
        ctx: Option<&mut MachPortCtx<T>>,
        port: crate::mach::Port,
        cb: MachPortCb<T>,
        should_free_info: *mut bool,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CFMachPortCreateWithPort(
                allocator,
                port,
                std::mem::transmute(cb),
                std::mem::transmute(ctx),
                should_free_info,
            )
        }
    }

    #[doc(alias = "CFMachPortCreate")]
    #[inline]
    pub fn with_ctx<T>(
        ctx: Option<&mut MachPortCtx<T>>,
        cb: MachPortCb<T>,
    ) -> Option<arc::R<Self>> {
        let mut should_free_info = false;
        Self::with_ctx_in(ctx, cb, &mut should_free_info, None)
    }

    #[doc(alias = "CFMachPortCreateWithPort")]
    #[inline]
    pub fn with_port<T>(
        ctx: Option<&mut MachPortCtx<T>>,
        port: crate::mach::Port,
        cb: MachPortCb<T>,
        should_free_info: *mut bool,
    ) -> Option<arc::R<Self>> {
        Self::with_port_in(ctx, port, cb, should_free_info, None)
    }

    #[doc(alias = "CFMachPortCreate")]
    #[inline]
    pub fn new(cb: MachPortCb) -> Option<arc::R<Self>> {
        let mut should_free_info = false;
        Self::with_ctx_in(None, cb, &mut should_free_info, None)
    }

    #[doc(alias = "CFMachPortGetInvalidationCallBack")]
    #[inline]
    pub fn invalidation_cb(&self) -> Option<MachPortInvalidationCb> {
        unsafe { CFMachPortGetInvalidationCallBack(self) }
    }

    #[doc(alias = "CFMachPortSetInvalidationCallBack")]
    #[inline]
    pub fn set_invalidation_cb(&mut self, val: Option<MachPortInvalidationCb>) {
        unsafe { CFMachPortSetInvalidationCallBack(self, val) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::MachPort {
        unsafe { std::mem::transmute(self) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns_mut(&mut self) -> &mut ns::MachPort {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn CFMachPortGetTypeID() -> cf::TypeId;

    fn CFMachPortCreate(
        allocator: Option<&cf::Allocator>,
        cb: MachPortCb,
        ctx: *mut MachPortCtx<std::ffi::c_void>,
        should_free_info: *mut bool,
    ) -> Option<arc::R<MachPort>>;

    fn CFMachPortCreateWithPort(
        allocator: Option<&cf::Allocator>,
        port: crate::mach::Port,
        cb: MachPortCb,
        ctx: *mut MachPortCtx<std::ffi::c_void>,
        should_free_info: *mut bool,
    ) -> Option<arc::R<MachPort>>;

    fn CFMachPortInvalidate(port: &mut MachPort);
    fn CFMachPortIsValid(port: &MachPort) -> bool;
    fn CFMachPortCreateRunLoopSource(
        allocator: Option<&cf::Allocator>,
        port: &MachPort,
        order: cf::Index,
    ) -> Option<arc::R<cf::RunLoopSrc>>;

    fn CFMachPortGetPort(port: &MachPort) -> crate::mach::Port;
    fn CFMachPortGetInvalidationCallBack(port: &MachPort) -> Option<MachPortInvalidationCb>;
    fn CFMachPortSetInvalidationCallBack(port: &mut MachPort, val: Option<MachPortInvalidationCb>);
}

#[cfg(test)]
mod tests {
    use crate::{cf, mach};

    #[test]
    fn basics() {
        extern "C-unwind" fn cb(
            _port: &mut cf::MachPort,
            _msg: *mut std::ffi::c_void,
            _size: cf::Index,
            _info: *mut std::ffi::c_void,
        ) {
        }

        let mut port0 = cf::MachPort::with_ctx(None, cb).unwrap();
        assert!(port0.is_valid());
        assert_ne!(mach::Port::NULL, port0.port());
        assert_ne!(mach::Port::DEAD, port0.port());

        let port1 = cf::MachPort::new(cb).unwrap();
        assert!(port1.is_valid());
        assert_ne!(mach::Port::NULL, port1.port());
        assert_ne!(mach::Port::DEAD, port1.port());
        assert_ne!(port0.port(), port1.port());

        let ns_port = port0.as_ns_mut();
        assert!(ns_port.is_valid());
        ns_port.invalidate();
        assert!(!ns_port.is_valid());
        assert!(!port0.is_valid());

        assert!(port0.invalidation_cb().is_none());
    }
}
