use crate::{arc, blocks, define_cls, define_mtl, define_obj_type, dispatch, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLEvent")]
    pub Event(ns::Id)
);

impl Event {
    define_mtl!(device, label, set_label);
}

define_obj_type!(
    /// This class provides a simple interface for handling the dispatching of mtl::SharedEvent notifications from Metal.
    #[doc(alias = "MTLSharedEventListener")]
    pub SharedEventListener(ns::Id)
);

impl arc::A<SharedEventListener> {
    #[objc::msg_send(initWithDispatchQueue:)]
    pub fn init_with_dispatch_queue(self, queue: &dispatch::Queue) -> arc::R<SharedEventListener>;
}

/// A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
pub type SharedEventNotificationBlock =
    blocks::EscBlock<fn(event: &mut mtl::SharedEvent, value: u64)>;

impl SharedEventListener {
    define_cls!(MTL_SHARED_EVENT_LISTENER);

    /// Creates a new shareable event listener with a specific dispatch queue.
    #[inline]
    pub fn with_dispatch_queue(queue: &dispatch::Queue) -> arc::R<Self> {
        Self::alloc().init_with_dispatch_queue(queue)
    }

    #[objc::msg_send(dispatchQueue)]
    pub fn dispatch_queue(&self) -> arc::R<dispatch::Queue>;
}

define_obj_type!(
    /// mtl::SharedEventHandle objects may be passed between processes via XPC connections and then used to recreate
    /// a mtl::SharedEvent via an existing mtl::Device.
    #[doc(alias = "MTLSharedEventHandle")]
    pub SharedEventHandle(ns::Id)
);

impl SharedEventHandle {
    define_mtl!(label);
}

define_obj_type!(
    /// An object you use to synchronize access to Metal resources across multiple CPUs, GPUs, and processes.
    #[doc(alias = "MTLSharedEvent")]
    pub SharedEvent(Event)
);

impl SharedEvent {
    /// When the event's signaled value reaches value or higher, invoke the block on the dispatch queue owned by the listener.
    #[objc::msg_send(notifyListener:atValue:block:)]
    pub fn notify_listener_at_block(
        &self,
        listener: &mtl::SharedEventListener,
        at_value: u64,
        block: &mut SharedEventNotificationBlock,
    );

    pub fn notify_listener_at(
        &self,
        listener: &mtl::SharedEventListener,
        at_value: u64,
        block: impl FnMut(&mut mtl::SharedEvent, u64) + Send + 'static,
    ) {
        let mut block = SharedEventNotificationBlock::new2(block);
        self.notify_listener_at_block(listener, at_value, &mut block);
    }

    /// Convenience method for creating a shared event handle that may be passed to other processes via XPC.
    #[objc::msg_send(newSharedEventHandle)]
    pub fn new_shared_event_handle(&self) -> arc::R<mtl::SharedEventHandle>;

    /// Synchronously wait for the signaledValue to be greater than or equal to 'value', with a timeout
    /// specified in milliseconds. Returns true if the value was signaled before the timeout, otherwise NO.
    #[objc::msg_send(waitUntilSignaledValue:timeoutMS:)]
    pub fn wait_until_signaled_value(&self, value: u64, timeout_ms: u64) -> bool;

    #[objc::msg_send(signaledValue)]
    pub fn signaled_value(&self) -> u64;

    #[objc::msg_send(setSignaledValue:)]
    pub fn set_signaled_value(&self, val: u64);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_SHARED_EVENT_LISTENER: &'static objc::Class<SharedEventListener>;
}
