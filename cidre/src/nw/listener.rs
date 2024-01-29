use std::ffi::{c_char, CStr};

use crate::{arc, define_obj_type, dispatch, ns, nw};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "nw_listener_state_changed_handler_t")]
pub type StateChangedHandler = blocks::SyncBlock<fn(nw::ListenerState, Option<&nw::Error>)>;

#[doc(alias = "nw_listener_new_connection_handler_t")]
pub type NewConnectionHandler = blocks::SyncBlock<fn(&nw::Connection)>;

#[doc(alias = "nw_listener_new_connection_group_handler_t")]
pub type NewConnectionGroupHandler = blocks::SyncBlock<fn(&nw::ConnectionGroup)>;

#[doc(alias = "nw_listener_advertised_endpoint_changed_handler_t")]
pub type AdvertisedEndpointChangedHandler<'a> = blocks::SyncBlock<fn(&nw::Endpoint, bool)>;

define_obj_type!(
    #[doc(alias = "nw_listener")]
    pub Listener(ns::Id)
);

unsafe impl Send for Listener {}
unsafe impl Sync for Listener {}

#[doc(alias = "nw_listener_state_t")]
#[repr(i32)]
pub enum State {
    Invalid = 0,
    Waiting = 1,
    Ready = 2,
    Failed = 3,
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

    #[doc(alias = "nw_listener_create_with_port")]
    #[inline]
    pub fn with_port(port: &CStr, params: &nw::Params) -> Option<arc::R<Self>> {
        unsafe { nw_listener_create_with_port(port.as_ptr(), params) }
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
    pub fn set_new_connection_handler(
        &mut self,
        handler: Option<&mut nw::ListenerNewConnectionHandler>,
    ) {
        unsafe { nw_listener_set_new_connection_handler(self, handler) }
    }

    #[doc(alias = "nw_listener_set_state_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_state_changed_handler(
        &mut self,
        handler: Option<&mut nw::ListenerStateChangedHandler>,
    ) {
        unsafe { nw_listener_set_state_changed_handler(self, handler) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
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
