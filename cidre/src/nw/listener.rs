use std::ffi::{c_char, CStr};

use crate::{arc, define_obj_type, dispatch, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_listener")]
    pub Listener(ns::Id)
);

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

    fn nw_listener_get_port(listener: &Listener) -> u16;
    fn nw_listener_start(listener: &mut Listener);
    fn nw_listener_cancel(listener: &mut Listener);
}
