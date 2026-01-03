use std::ffi::{CStr, c_char};

use crate::{arc, define_obj_type, dispatch, ns, nw};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "nw_listener_state_changed_handler_t")]
pub type StateChangedHandler =
    blocks::SyncBlock<fn(state: nw::ListenerState, err: Option<&nw::Error>)>;

#[doc(alias = "nw_listener_new_connection_handler_t")]
pub type NewConnectionHandler = blocks::SyncBlock<fn(connection: &mut nw::Connection)>;

#[doc(alias = "nw_listener_new_connection_group_handler_t")]
pub type NewConnectionGroupHandler =
    blocks::SyncBlock<fn(connection_group: &mut nw::ConnectionGroup)>;

#[doc(alias = "nw_listener_advertised_endpoint_changed_handler_t")]
pub type AdvertisedEndpointChangedHandler<'a> =
    blocks::SyncBlock<fn(advertised_endpoint: &nw::Endpoint, added: bool)>;

define_obj_type!(
    #[doc(alias = "nw_listener")]
    pub Listener(ns::Id)
);

unsafe impl Send for Listener {}
unsafe impl Sync for Listener {}

/// States progress forward and do not move backwards.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[doc(alias = "nw_listener_state_t")]
#[repr(i32)]
pub enum State {
    /// The state of the listener is not valid. This state
    /// will never be delivered in the listener's state update handler, and can be treated as
    /// an unexpected value.
    Invalid = 0,

    /// The listener is waiting for a usable network before being able to receive connections
    Waiting = 1,

    /// The listener is ready and able to accept incoming connections
    Ready = 2,

    /// The listener has irrecoverably closed or failed
    Failed = 3,

    /// The listener has been cancelled by the caller
    Cancelled = 4,
}

impl Listener {
    #[doc(alias = "NW_LISTENER_INFINITE_CONNECTION_LIMIT")]
    pub const INFINITE_CONNECTION_LIMIT: u32 = u32::MAX;

    #[doc(alias = "nw_listener_create")]
    #[inline]
    pub fn with_params(params: &nw::Params) -> Option<arc::R<Self>> {
        unsafe { nw_listener_create(params) }
    }

    /// Creates a networking listener bound to a specified local port.
    ///
    /// Arguments:
    /// - port - A port number as a C string, such as c"443", or a service name, such as "https".
    #[doc(alias = "nw_listener_create_with_port")]
    #[inline]
    pub fn with_port(port: impl AsRef<CStr>, params: &nw::Params) -> Option<arc::R<Self>> {
        unsafe { nw_listener_create_with_port(port.as_ref().as_ptr(), params) }
    }

    #[doc(alias = "nw_listener_create_with_connection")]
    #[inline]
    pub fn with_connection(
        connection: &nw::Connection,
        params: &nw::Params,
    ) -> Option<arc::R<Listener>> {
        unsafe { nw_listener_create_with_connection(connection, params) }
    }

    #[doc(alias = "nw_listener_set_queue")]
    #[inline]
    fn set_queue(&mut self, val: &dispatch::Queue) {
        unsafe { nw_listener_set_queue(self, val) }
    }

    #[doc(alias = "nw_listener_get_new_connection_limit")]
    #[inline]
    pub fn new_connection_limit(&self) -> u32 {
        unsafe { nw_listener_get_new_connection_limit(self) }
    }

    #[doc(alias = "nw_listener_set_new_connection_limit")]
    #[inline]
    pub fn set_new_connection_limit(&mut self, val: u32) {
        unsafe { nw_listener_set_new_connection_limit(self, val) }
    }

    #[doc(alias = "nw_listener_set_advertise_descriptor")]
    #[inline]
    pub fn set_advertise_desc(&mut self, val: Option<&nw::AdvertiseDesc>) {
        unsafe { nw_listener_set_advertise_descriptor(self, val) }
    }

    #[doc(alias = "nw_listener_get_port")]
    #[inline]
    pub fn port(&self) -> u16 {
        unsafe { nw_listener_get_port(self) }
    }

    pub fn start(&mut self, queue: &dispatch::Queue) {
        self.set_queue(queue);
        unsafe { nw_listener_start(self) }
    }

    #[doc(alias = "nw_listener_cancel")]
    #[inline]
    pub fn cancel(&mut self) {
        unsafe { nw_listener_cancel(self) }
    }

    #[doc(alias = "nw_listener_set_advertised_endpoint_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_advertised_endpoint_changed_handler(
        &mut self,
        handler: Option<&mut nw::ListenerAdvertisedEndpointChangedHandler>,
    ) {
        unsafe { nw_listener_set_advertised_endpoint_changed_handler(self, handler) }
    }

    #[doc(alias = "nw_listener_set_new_connection_group_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_new_connection_group_handler(
        &mut self,
        handler: Option<&mut nw::ListenerNewConnectionGroupHandler>,
    ) {
        unsafe { nw_listener_set_new_connection_group_handler(self, handler) }
    }

    #[doc(alias = "nw_listener_set_new_connection_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_new_connection_handler_block(
        &mut self,
        handler: Option<&mut nw::ListenerNewConnectionHandler>,
    ) {
        unsafe { nw_listener_set_new_connection_handler(self, handler) }
    }

    #[doc(alias = "nw_listener_set_new_connection_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_new_connection_handler(
        &mut self,
        handler: impl FnMut(&mut nw::Connection) + 'static + Send + Sync,
    ) {
        let mut block = NewConnectionHandler::new1(handler);
        self.set_new_connection_handler_block(Some(&mut block));
    }

    #[doc(alias = "nw_listener_set_state_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_state_changed_handler_block(
        &mut self,
        handler: Option<&mut nw::ListenerStateChangedHandler>,
    ) {
        unsafe { nw_listener_set_state_changed_handler(self, handler) }
    }

    #[doc(alias = "nw_listener_set_state_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_state_changed_handler(
        &mut self,
        handler: impl FnMut(nw::ListenerState, Option<&nw::Error>) + 'static + Send + Sync,
    ) {
        let mut block = nw::ListenerStateChangedHandler::new2(handler);
        self.set_state_changed_handler_block(Some(&mut block));
    }
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C-unwind" {
    fn nw_listener_create_with_port(
        port: *const c_char,
        params: &nw::Params,
    ) -> Option<arc::R<Listener>>;

    fn nw_listener_create(params: &nw::Params) -> Option<arc::R<Listener>>;

    fn nw_listener_create_with_connection(
        connection: &nw::Connection,
        params: &nw::Params,
    ) -> Option<arc::R<Listener>>;

    fn nw_listener_set_queue(listener: &mut Listener, queue: &dispatch::Queue);

    fn nw_listener_get_new_connection_limit(listener: &Listener) -> u32;
    fn nw_listener_set_new_connection_limit(listener: &mut Listener, val: u32);

    fn nw_listener_set_advertise_descriptor(
        listener: &mut Listener,
        val: Option<&nw::AdvertiseDesc>,
    );

    fn nw_listener_get_port(listener: &Listener) -> u16;
    fn nw_listener_start(listener: &mut Listener);
    fn nw_listener_cancel(listener: &mut Listener);

    #[cfg(feature = "blocks")]
    fn nw_listener_set_advertised_endpoint_changed_handler(
        listener: &mut Listener,
        handler: Option<&mut nw::ListenerAdvertisedEndpointChangedHandler>,
    );
    #[cfg(feature = "blocks")]
    fn nw_listener_set_new_connection_group_handler(
        listener: &mut Listener,
        handler: Option<&mut nw::ListenerNewConnectionGroupHandler>,
    );
    #[cfg(feature = "blocks")]
    fn nw_listener_set_new_connection_handler(
        listener: &mut Listener,
        handler: Option<&mut nw::ListenerNewConnectionHandler>,
    );
    #[cfg(feature = "blocks")]
    fn nw_listener_set_state_changed_handler(
        listener: &mut Listener,
        handler: Option<&mut nw::ListenerStateChangedHandler>,
    );
}

#[cfg(test)]
mod tests {
    use crate::{dispatch, nw};

    #[test]
    fn basics() {
        let sema = dispatch::Semaphore::new(0);
        let q = dispatch::Queue::new();
        let mut listener = nw::Listener::with_port(c"9089", &nw::Params::app_service()).unwrap();
        let mut sema_guard = Some(sema.guard());
        let block_listener = listener.retained();

        listener.set_state_changed_handler(move |state, err| {
            assert!(err.is_none());
            assert_eq!(state, nw::ListenerState::Ready);
            assert_eq!(block_listener.port(), 9089);

            sema_guard.take();
        });

        listener.set_new_connection_handler(|_conn| {});

        assert_eq!(listener.port(), 0);

        listener.start(&q);

        sema.wait_forever();
    }
}
