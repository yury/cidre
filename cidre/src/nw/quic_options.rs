use std::ffi::CStr;

use crate::{arc, nw};

#[cfg(feature = "sec")]
use crate::sec;

impl nw::ProtocolDefinition {
    #[doc(alias = "nw_protocol_copy_quic_definition")]
    #[inline]
    pub fn quic() -> arc::R<Self> {
        unsafe { nw_protocol_copy_quic_definition() }
    }
}

impl nw::ProtocolMetadata {
    #[doc(alias = "nw_quic_copy_sec_protocol_metadata")]
    #[cfg(feature = "sec")]
    #[inline]
    pub fn quic_sec_protocol_metadata(&self) -> arc::R<sec::ProtocolMetadata> {
        unsafe { nw_quic_copy_sec_protocol_metadata(self) }
    }
}

impl nw::ProtocolOpts {
    #[doc(alias = "nw_quic_add_tls_application_protocol")]
    #[inline]
    pub fn quic_add_tls_application_protocol(&mut self, application_protocol: &CStr) {
        unsafe { nw_quic_add_tls_application_protocol(self, application_protocol.as_ptr()) }
    }

    #[doc(alias = "nw_quic_copy_sec_protocol_options")]
    #[inline]
    pub fn quic_sec_protocol_opts(&self) -> arc::R<nw::SecProtocolOpts> {
        unsafe { nw_quic_copy_sec_protocol_options(self) }
    }

    #[doc(alias = "nw_quic_set_idle_timeout")]
    #[inline]
    pub fn quic_set_idle_timeout(&mut self, idle_timeout: u32) {
        unsafe { nw_quic_set_idle_timeout(self, idle_timeout) }
    }

    #[doc(alias = "nw_quic_set_stream_is_unidirectional")]
    #[inline]
    pub fn quic_set_stream_is_unidirectional(&mut self, is_unidirectional: bool) {
        unsafe { nw_quic_set_stream_is_unidirectional(self, is_unidirectional) }
    }
}

unsafe extern "C-unwind" {
    fn nw_protocol_copy_quic_definition() -> arc::R<nw::ProtocolDefinition>;

    #[cfg(feature = "sec")]
    fn nw_quic_copy_sec_protocol_metadata(
        metadata: &nw::ProtocolMetadata,
    ) -> arc::R<sec::ProtocolMetadata>;

    fn nw_quic_add_tls_application_protocol(
        options: &mut nw::ProtocolOpts,
        application_protocol: *const std::ffi::c_char,
    );
    fn nw_quic_copy_sec_protocol_options(options: &nw::ProtocolOpts)
    -> arc::R<nw::SecProtocolOpts>;
    fn nw_quic_set_idle_timeout(options: &mut nw::ProtocolOpts, idle_timeout: u32);
    fn nw_quic_set_stream_is_unidirectional(
        options: &mut nw::ProtocolOpts,
        is_unidirectional: bool,
    );
}
