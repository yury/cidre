use std::ffi::c_void;

use crate::{arc, define_obj_type, dispatch, ns, nw};

#[cfg(feature = "blocks")]
use crate::blocks;

/// Connection states sent by nw_connection_set_state_changed_handler.
/// States generally progress forward and do not move backwards, with the
/// exception of preparing and waiting, which may alternate before the connection
/// becomes ready or failed.
#[doc(alias = "nw_connection_state_t")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum State {
    /// The state of the connection is not valid. This state
    /// will never be delivered in the connection's state update handler, and can be treated as
    /// an unexpected value.
    #[doc(alias = "nw_connection_state_invalid")]
    Invalid = 0,

    /// The connection is waiting for a usable network before re-attempting
    #[doc(alias = "nw_connection_state_waiting")]
    Waiting = 1,

    /// The connection is in the process of establishing
    #[doc(alias = "nw_connection_state_preparing")]
    Preparing = 2,

    /// The connection is established and ready to send and receive data upon
    #[doc(alias = "nw_connection_state_ready")]
    Ready = 3,

    /// The connection has irrecoverably closed or failed
    #[doc(alias = "nw_connection_state_failed")]
    Failed = 4,

    /// The connection has been cancelled by the caller
    #[doc(alias = "nw_connection_state_cancelled")]
    Cancelled = 5,
}

define_obj_type!(
    #[doc(alias = "nw_connection")]
    pub Connection(ns::Id)
);

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

impl Connection {
    #[doc(alias = "nw_connection_create")]
    #[inline]
    pub fn with_endpoint(endpoint: &nw::Endpoint, params: &nw::Params) -> Option<arc::R<Self>> {
        unsafe { nw_connection_create(endpoint, params) }
    }

    #[doc(alias = "nw_connection_copy_endpoint")]
    #[inline]
    pub fn endpoint(&self) -> Option<arc::R<nw::Endpoint>> {
        unsafe { nw_connection_copy_endpoint(self) }
    }

    #[doc(alias = "nw_connection_copy_parameters")]
    #[inline]
    pub fn params(&self) -> Option<arc::R<nw::Params>> {
        unsafe { nw_connection_copy_parameters(self) }
    }

    #[doc(alias = "nw_connection_set_state_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_state_changed_handler<'a, F>(
        &mut self,
        handler: Option<&'static mut blocks::Block<F>>,
    ) where
        F: FnMut(nw::ConnectionState, Option<&'a nw::Error>),
    {
        unsafe {
            nw_connection_set_state_changed_handler(
                self,
                handler.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr()),
            )
        }
    }

    #[doc(alias = "nw_connection_set_viability_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_viability_changed_handler<F>(
        &mut self,
        handler: Option<&'static mut blocks::Block<F>>,
    ) where
        F: FnMut(bool),
    {
        unsafe {
            nw_connection_set_viability_changed_handler(
                self,
                handler.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr()),
            )
        }
    }

    #[doc(alias = "nw_connection_set_better_path_available_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_better_path_available_handler<F>(
        &mut self,
        handler: Option<&'static mut blocks::Block<F>>,
    ) where
        F: FnMut(bool),
    {
        unsafe {
            nw_connection_set_better_path_available_handler(
                self,
                handler.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr()),
            )
        }
    }
    #[doc(alias = "nw_connection_set_path_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_path_changed_handler<'a, F>(
        &mut self,
        handler: Option<&'static mut blocks::Block<F>>,
    ) where
        F: FnMut(&'a nw::Path),
    {
        unsafe {
            nw_connection_set_path_changed_handler(
                self,
                handler.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr()),
            )
        }
    }

    #[doc(alias = "nw_connection_set_queue")]
    #[inline]
    fn set_queue(&mut self, val: &dispatch::Queue) {
        unsafe { nw_connection_set_queue(self, val) }
    }

    #[doc(alias = "NWConnection.start")]
    #[doc(alias = "nw_connection_start")]
    #[inline]
    pub fn start(&mut self, queue: &dispatch::Queue) {
        self.set_queue(queue);
        unsafe {
            nw_connection_start(self);
        }
    }

    #[doc(alias = "nw_connection_restart")]
    #[inline]
    pub fn restart(&mut self) {
        unsafe {
            nw_connection_restart(self);
        }
    }

    #[doc(alias = "nw_connection_cancel")]
    #[inline]
    pub fn cancel(&mut self) {
        unsafe {
            nw_connection_cancel(self);
        }
    }

    #[doc(alias = "nw_connection_force_cancel")]
    #[inline]
    pub fn force_cancel(&mut self) {
        unsafe {
            nw_connection_force_cancel(self);
        }
    }

    #[doc(alias = "nw_connection_cancel_current_endpoint")]
    #[inline]
    pub fn cancel_current_endpoint(&mut self) {
        unsafe {
            nw_connection_cancel_current_endpoint(self);
        }
    }

    #[doc(alias = "nw_connection_receive")]
    #[inline]
    pub fn recieve_ch<'a, F>(
        &mut self,
        min_incomplete_len: u32,
        max_len: u32,
        completion: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(
            /* content */ Option<&'a dispatch::Data>,
            /* context */ Option<&'a nw::ContentCtx>,
            /* is_complete */ bool,
            /* error */ Option<&'a nw::Error>,
        ),
    {
        unsafe {
            nw_connection_receive(self, min_incomplete_len, max_len, completion.as_mut_ptr())
        };
    }

    #[doc(alias = "nw_connection_receive_message")]
    #[inline]
    pub fn recieve_msg_ch<'a, F>(&mut self, completion: &'static mut blocks::Block<F>)
    where
        F: FnOnce(
            /* content */ Option<&'a dispatch::Data>,
            /* context */ Option<&'a nw::ContentCtx>,
            /* is_complete */ bool,
            /* error */ Option<&'a nw::Error>,
        ),
    {
        unsafe { nw_connection_receive_message(self, completion.as_mut_ptr()) };
    }

    #[doc(alias = "nw_connection_send")]
    #[inline]
    pub fn send_ch<'a, F>(
        &mut self,
        content: Option<&dispatch::Data>,
        context: &nw::ContentCtx,
        is_complete: bool,
        completion: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(Option<&'a nw::Error>),
    {
        unsafe { nw_connection_send(self, content, context, is_complete, completion.as_mut_ptr()) }
    }

    #[doc(alias = "nw_connection_batch")]
    #[inline]
    pub fn batch<'a, F, B: dispatch::Block<'a, F>>(&mut self, batch_block: &mut B) {
        unsafe { nw_connection_batch(self, batch_block.ptr()) };
    }

    #[doc(alias = "nw_connection_copy_current_path")]
    #[inline]
    pub fn current_path(&self) -> Option<arc::R<nw::Endpoint>> {
        unsafe { nw_connection_copy_current_path(self) }
    }

    #[doc(alias = "nw_connection_copy_protocol_metadata")]
    #[inline]
    pub fn protocol_metadata(&self) -> Option<arc::R<nw::ProtocolMetadata>> {
        unsafe { nw_connection_copy_protocol_metadata(self) }
    }

    #[doc(alias = "nw_connection_get_maximum_datagram_size")]
    #[inline]
    pub fn maximum_datagram_size(&self) -> u32 {
        unsafe { nw_connection_get_maximum_datagram_size(self) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_connection_create(
        endpoint: &nw::Endpoint,
        params: &nw::Params,
    ) -> Option<arc::R<Connection>>;

    fn nw_connection_copy_endpoint(connection: &Connection) -> Option<arc::R<nw::Endpoint>>;
    fn nw_connection_copy_parameters(connection: &Connection) -> Option<arc::R<nw::Params>>;

    #[cfg(feature = "blocks")]
    fn nw_connection_set_state_changed_handler(connection: &mut Connection, handler: *mut c_void);

    #[cfg(feature = "blocks")]
    fn nw_connection_set_viability_changed_handler(
        connection: &mut Connection,
        handler: *mut c_void,
    );

    #[cfg(feature = "blocks")]
    fn nw_connection_set_better_path_available_handler(
        connection: &mut Connection,
        handler: *mut c_void,
    );

    #[cfg(feature = "blocks")]
    fn nw_connection_set_path_changed_handler(connection: &mut Connection, handler: *mut c_void);

    fn nw_connection_set_queue(connection: &mut Connection, queue: &dispatch::Queue);
    fn nw_connection_start(connection: &mut Connection);
    fn nw_connection_restart(connection: &mut Connection);
    fn nw_connection_cancel(connection: &mut Connection);
    fn nw_connection_force_cancel(connection: &mut Connection);
    fn nw_connection_cancel_current_endpoint(connection: &mut Connection);

    fn nw_connection_receive(
        connection: &mut Connection,
        min_incomplete_length: u32,
        max_length: u32,
        completion: *mut c_void,
    );
    fn nw_connection_receive_message(connection: &mut Connection, completion: *mut c_void);

    fn nw_connection_send(
        connection: &mut Connection,
        content: Option<&dispatch::Data>,
        context: &nw::ContentCtx,
        is_complete: bool,
        completion: *mut c_void,
    );

    fn nw_connection_batch(connection: &mut Connection, batch_block: *mut c_void);

    fn nw_connection_copy_current_path(connection: &Connection) -> Option<arc::R<nw::Endpoint>>;
    fn nw_connection_copy_protocol_metadata(
        connection: &Connection,
    ) -> Option<arc::R<nw::ProtocolMetadata>>;

    fn nw_connection_get_maximum_datagram_size(connection: &Connection) -> u32;

}
