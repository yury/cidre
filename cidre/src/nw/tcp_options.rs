use crate::{api, arc, nw};

impl nw::ProtocolDefinition {
    /// Access the definition of the default system protocol implementation
    /// of TCP (Transmission Control Protocol). This protocol can be used
    /// as part of a connection's protocol stack as the transport protocol.
    #[doc(alias = "nw_protocol_copy_tcp_definition")]
    #[inline]
    pub fn tcp() -> arc::R<Self> {
        unsafe { nw_protocol_copy_tcp_definition() }
    }
}

impl nw::ProtocolOpts {
    #[doc(alias = "nw_tcp_create_options")]
    #[inline]
    pub fn tcp() -> arc::R<Self> {
        unsafe { nw_tcp_create_options() }
    }

    /// Disable Nagle's algorithm to avoid coalescing small interactive writes.
    #[doc(alias = "nw_tcp_options_set_no_delay")]
    #[inline]
    pub fn tcp_set_no_delay(&mut self, val: bool) {
        unsafe { nw_tcp_options_set_no_delay(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_enable_keepalive")]
    #[inline]
    pub fn tcp_set_enable_keepalive(&mut self, val: bool) {
        unsafe { nw_tcp_options_set_enable_keepalive(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_keepalive_count")]
    #[inline]
    pub fn tcp_set_keepalive_count(&mut self, val: u32) {
        unsafe { nw_tcp_options_set_keepalive_count(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_keepalive_idle_time")]
    #[inline]
    pub fn tcp_set_keepalive_idle_time(&mut self, val: u32) {
        unsafe { nw_tcp_options_set_keepalive_idle_time(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_keepalive_interval")]
    #[inline]
    pub fn tcp_set_keepalive_interval(&mut self, val: u32) {
        unsafe { nw_tcp_options_set_keepalive_interval(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_connection_timeout")]
    #[inline]
    pub fn tcp_set_connection_timeout(&mut self, val: u32) {
        unsafe { nw_tcp_options_set_connection_timeout(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_enable_fast_open")]
    #[inline]
    pub fn tcp_set_enable_fast_open(&mut self, val: bool) {
        unsafe { nw_tcp_options_set_enable_fast_open(self, val) }
    }

    #[doc(alias = "nw_tcp_options_set_disable_ecn")]
    #[inline]
    pub fn tcp_set_disable_ecn(&mut self, val: bool) {
        unsafe { nw_tcp_options_set_disable_ecn(self, val) }
    }
}

impl nw::ProtocolMetadata {
    #[doc(alias = "nw_protocol_metadata_is_tcp")]
    #[inline]
    pub fn is_tcp(&self) -> bool {
        unsafe { nw_protocol_metadata_is_tcp(self) }
    }
}

#[api::weak]
unsafe extern "C-unwind" {
    fn nw_protocol_copy_tcp_definition() -> arc::R<nw::ProtocolDefinition>;
    fn nw_tcp_create_options() -> arc::R<nw::ProtocolOpts>;
    fn nw_tcp_options_set_no_delay(options: &mut nw::ProtocolOpts, val: bool);
    fn nw_tcp_options_set_enable_keepalive(options: &mut nw::ProtocolOpts, val: bool);
    fn nw_tcp_options_set_keepalive_count(options: &mut nw::ProtocolOpts, val: u32);
    fn nw_tcp_options_set_keepalive_idle_time(options: &mut nw::ProtocolOpts, val: u32);
    fn nw_tcp_options_set_keepalive_interval(options: &mut nw::ProtocolOpts, val: u32);
    fn nw_tcp_options_set_connection_timeout(options: &mut nw::ProtocolOpts, val: u32);
    fn nw_tcp_options_set_enable_fast_open(options: &mut nw::ProtocolOpts, val: bool);
    fn nw_tcp_options_set_disable_ecn(options: &mut nw::ProtocolOpts, val: bool);
    fn nw_protocol_metadata_is_tcp(metadata: &nw::ProtocolMetadata) -> bool;
}
