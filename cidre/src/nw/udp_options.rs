use crate::{arc, nw};

impl nw::ProtocolDefinition {
    /// Access the definition of the default system protocol implementation
    /// of UDP (User Datagram Protocol). This protocol can be used
    /// as part of a connection's protocol stack as the transport protocol.
    #[doc(alias = "nw_protocol_copy_udp_definition")]
    #[inline]
    pub fn udp() -> arc::R<Self> {
        unsafe { nw_protocol_copy_udp_definition() }
    }
}

impl nw::ProtocolOpts {
    #[doc(alias = "nw_udp_create_options")]
    #[inline]
    pub fn udp() -> arc::R<Self> {
        unsafe { nw_udp_create_options() }
    }

    #[doc(alias = "nw_udp_options_set_prefer_no_checksum")]
    #[inline]
    pub fn udp_set_prefer_no_checksum(&mut self, prefere_no_checksum: bool) {
        unsafe {
            nw_udp_options_set_prefer_no_checksum(self, prefere_no_checksum);
        }
    }
}

impl nw::ProtocolMetadata {
    #[doc(alias = "nw_udp_create_metadata")]
    #[inline]
    pub fn udp() -> arc::R<Self> {
        unsafe { nw_udp_create_metadata() }
    }

    #[doc(alias = "nw_protocol_metadata_is_udp")]
    #[inline]
    pub fn is_udp(&self) -> bool {
        unsafe { nw_protocol_metadata_is_udp(self) }
    }
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    fn nw_protocol_copy_udp_definition() -> arc::R<nw::ProtocolDefinition>;
    fn nw_udp_create_options() -> arc::R<nw::ProtocolOpts>;
    fn nw_udp_options_set_prefer_no_checksum(
        opts: &mut nw::ProtocolOpts,
        prefere_no_checksum: bool,
    );
    fn nw_udp_create_metadata() -> arc::R<nw::ProtocolMetadata>;
    fn nw_protocol_metadata_is_udp(metadata: &nw::ProtocolMetadata) -> bool;
}
