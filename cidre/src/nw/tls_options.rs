use crate::{api, arc, define_obj_type, dispatch, ns, nw};

#[cfg(feature = "sec")]
use crate::sec;

define_obj_type!(
    #[doc(alias = "sec_protocol_options")]
    #[doc(alias = "sec_protocol_options_t")]
    pub SecProtocolOpts(ns::Id)
);

impl nw::ProtocolDefinition {
    #[doc(alias = "nw_protocol_copy_tls_definition")]
    #[inline]
    pub fn tls() -> arc::R<Self> {
        unsafe { nw_protocol_copy_tls_definition() }
    }
}

impl nw::ProtocolOpts {
    #[doc(alias = "nw_tls_create_options")]
    #[inline]
    pub fn tls() -> arc::R<Self> {
        unsafe { nw_tls_create_options() }
    }

    #[doc(alias = "nw_tls_copy_sec_protocol_options")]
    #[inline]
    pub fn tls_sec_protocol_opts(&self) -> arc::R<SecProtocolOpts> {
        unsafe { nw_tls_copy_sec_protocol_options(self) }
    }
}

impl SecProtocolOpts {
    #[doc(alias = "sec_protocol_options_add_pre_shared_key")]
    #[inline]
    pub fn add_pre_shared_key(&mut self, psk: &dispatch::Data, psk_identity: &dispatch::Data) {
        unsafe { sec_protocol_options_add_pre_shared_key(self, psk, psk_identity) }
    }

    #[doc(alias = "sec_protocol_options_set_peer_authentication_required")]
    #[inline]
    pub fn set_peer_authentication_required(&mut self, required: bool) {
        unsafe { sec_protocol_options_set_peer_authentication_required(self, required) }
    }

    #[doc(alias = "sec_protocol_options_set_min_tls_protocol_version")]
    #[inline]
    pub fn set_min_tls_protocol_version(&mut self, version: TlsProtocolVersion) {
        unsafe { sec_protocol_options_set_min_tls_protocol_version(self, version) }
    }

    #[doc(alias = "sec_protocol_options_set_max_tls_protocol_version")]
    #[inline]
    pub fn set_max_tls_protocol_version(&mut self, version: TlsProtocolVersion) {
        unsafe { sec_protocol_options_set_max_tls_protocol_version(self, version) }
    }

    #[doc(alias = "sec_protocol_options_append_tls_ciphersuite")]
    #[inline]
    pub fn append_tls_ciphersuite(&mut self, ciphersuite: TlsCiphersuite) {
        unsafe { sec_protocol_options_append_tls_ciphersuite(self, ciphersuite) }
    }

    #[doc(alias = "sec_protocol_options_set_local_identity")]
    #[cfg(feature = "sec")]
    #[inline]
    pub fn set_local_identity(&mut self, identity: &sec::ProtocolIdentity) {
        unsafe { sec_protocol_options_set_local_identity(self, identity) }
    }

    #[doc(alias = "sec_protocol_options_set_verify_block")]
    #[cfg(all(feature = "blocks", feature = "sec"))]
    #[inline]
    pub fn set_verify_block(
        &mut self,
        verify_block: &mut sec::ProtocolVerify,
        verify_queue: &dispatch::Queue,
    ) {
        unsafe { sec_protocol_options_set_verify_block(self, verify_block, verify_queue) }
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TlsProtocolVersion {
    #[doc(alias = "tls_protocol_version_TLSv13")]
    Tls13 = 0x0304,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TlsCiphersuite {
    #[doc(alias = "TLS_AES_128_GCM_SHA256")]
    Aes128GcmSha256 = 0x1301,
    #[doc(alias = "TLS_PSK_WITH_AES_128_GCM_SHA256")]
    PskWithAes128GcmSha256 = 0x00A8,
}

#[api::weak]
unsafe extern "C-unwind" {
    fn nw_protocol_copy_tls_definition() -> arc::R<nw::ProtocolDefinition>;
    fn nw_tls_create_options() -> arc::R<nw::ProtocolOpts>;
    fn nw_tls_copy_sec_protocol_options(options: &nw::ProtocolOpts) -> arc::R<SecProtocolOpts>;

    fn sec_protocol_options_add_pre_shared_key(
        options: &mut SecProtocolOpts,
        psk: &dispatch::Data,
        psk_identity: &dispatch::Data,
    );
    fn sec_protocol_options_set_peer_authentication_required(
        options: &mut SecProtocolOpts,
        required: bool,
    );
    fn sec_protocol_options_set_min_tls_protocol_version(
        options: &mut SecProtocolOpts,
        version: TlsProtocolVersion,
    );
    fn sec_protocol_options_set_max_tls_protocol_version(
        options: &mut SecProtocolOpts,
        version: TlsProtocolVersion,
    );
    fn sec_protocol_options_append_tls_ciphersuite(
        options: &mut SecProtocolOpts,
        ciphersuite: TlsCiphersuite,
    );
    #[cfg(feature = "sec")]
    fn sec_protocol_options_set_local_identity(
        options: &mut SecProtocolOpts,
        identity: &sec::ProtocolIdentity,
    );
    #[cfg(all(feature = "blocks", feature = "sec"))]
    fn sec_protocol_options_set_verify_block(
        options: &mut SecProtocolOpts,
        verify_block: &mut sec::ProtocolVerify,
        verify_queue: &dispatch::Queue,
    );
}

#[link(name = "Security", kind = "framework")]
unsafe extern "C" {}
