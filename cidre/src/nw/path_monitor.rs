use crate::{api, arc, define_obj_type, dispatch, ns, nw};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "nw_path_monitor")]
    #[doc(alias = "nw_path_monitor_t")]
    pub PathMonitor(ns::Id)
);

unsafe impl Send for PathMonitor {}
unsafe impl Sync for PathMonitor {}

#[cfg(feature = "blocks")]
#[doc(alias = "nw_path_monitor_cancel_handler_t")]
pub type CancelHandler = blocks::SyncBlock<fn()>;

#[cfg(feature = "blocks")]
#[doc(alias = "nw_path_monitor_update_handler_t")]
pub type UpdateHandler = blocks::SyncBlock<fn(path: &nw::Path)>;

impl PathMonitor {
    #[doc(alias = "nw_path_monitor_create")]
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { nw_path_monitor_create() }
    }

    #[doc(alias = "nw_path_monitor_create_with_type")]
    #[inline]
    pub fn with_iface_type(required_iface_type: nw::IfaceType) -> arc::R<Self> {
        unsafe { nw_path_monitor_create_with_type(required_iface_type) }
    }

    #[api::available(macos = 13.0)]
    pub fn for_ethernet_channel() -> arc::R<PathMonitor> {
        unsafe { nw_path_monitor_create_for_ethernet_channel() }
    }

    #[doc(alias = "nw_path_monitor_prohibit_interface_type")]
    #[inline]
    pub fn prohibit_iface_type(&mut self, val: nw::IfaceType) {
        unsafe {
            nw_path_monitor_prohibit_interface_type(self, val);
        }
    }

    #[doc(alias = "nw_path_monitor_set_queue")]
    #[inline]
    pub fn set_queue(&mut self, val: &dispatch::Queue) {
        unsafe {
            nw_path_monitor_set_queue(self, val);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_path_monitor_set_cancel_handler")]
    #[inline]
    pub fn set_cancel_handler_block(&mut self, block: Option<&mut CancelHandler>) {
        unsafe { nw_path_monitor_set_cancel_handler(self, block) }
    }

    #[cfg(feature = "blocks")]
    pub fn set_cancel_handler(&mut self, handler: impl FnMut() + 'static + Sync) {
        let mut block = CancelHandler::new0(handler);
        unsafe { nw_path_monitor_set_cancel_handler(self, Some(&mut block)) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_path_monitor_set_update_handler")]
    #[inline]
    pub fn set_update_handler_block(&mut self, block: &mut UpdateHandler) {
        unsafe {
            nw_path_monitor_set_update_handler(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_path_monitor_set_update_handler")]
    #[inline]
    pub fn set_update_handler(&mut self, handler: impl FnMut(&nw::Path) + 'static + Sync) {
        let mut block = UpdateHandler::new1(handler);
        unsafe {
            nw_path_monitor_set_update_handler(self, &mut block);
        }
    }

    #[doc(alias = "nw_path_monitor_start")]
    #[inline]
    pub fn start(&mut self) {
        unsafe {
            nw_path_monitor_start(self);
        }
    }

    #[doc(alias = "nw_path_monitor_cancel")]
    #[inline]
    pub fn cancel(&mut self) {
        unsafe {
            nw_path_monitor_cancel(self);
        }
    }
}

#[link(name = "Network", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    fn nw_path_monitor_create() -> arc::R<PathMonitor>;
    #[api::available(macos = 13.0)]
    fn nw_path_monitor_create_for_ethernet_channel() -> arc::R<PathMonitor>;
    fn nw_path_monitor_create_with_type(required_iface_type: nw::IfaceType) -> arc::R<PathMonitor>;
    fn nw_path_monitor_prohibit_interface_type(
        monitor: &mut PathMonitor,
        iface_type: nw::IfaceType,
    );
    fn nw_path_monitor_set_queue(monitor: &mut PathMonitor, queue: &dispatch::Queue);
    fn nw_path_monitor_start(monitor: &mut PathMonitor);
    fn nw_path_monitor_cancel(monitor: &mut PathMonitor);
    #[cfg(feature = "blocks")]
    fn nw_path_monitor_set_cancel_handler(
        monitor: &mut PathMonitor,
        block: Option<&mut CancelHandler>,
    );
    #[cfg(feature = "blocks")]
    fn nw_path_monitor_set_update_handler(monitor: &mut PathMonitor, block: &mut UpdateHandler);
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic};

    use crate::{dispatch, nw};

    #[test]
    fn basics() {
        let queue = dispatch::Queue::new();
        let cancelled = Arc::new(atomic::AtomicBool::new(false));
        {
            let mut monitor = nw::PathMonitor::new();
            monitor.set_queue(&queue);
            let block_cancelled = cancelled.clone();

            monitor
                .set_cancel_handler(move || block_cancelled.store(true, atomic::Ordering::SeqCst));
            monitor.start();
            assert_eq!(false, cancelled.load(atomic::Ordering::SeqCst));
            monitor.cancel();
            assert_eq!(false, cancelled.load(atomic::Ordering::SeqCst));
            monitor.set_cancel_handler_block(None);
            // cancel called on monitor drop actually
        }
        queue.sync(|| {});
        let cancelled =
            Arc::try_unwrap(cancelled).expect("Arc is not released in cancel handler block");
        assert_eq!(true, cancelled.load(atomic::Ordering::SeqCst));
    }
}
