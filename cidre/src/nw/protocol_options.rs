use crate::{arc, define_obj_type, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_protocol_definition")]
    #[doc(alias = "nw_protocol_definition_t")]
    pub ProtocolDefinition(ns::Id)
);

define_obj_type!(
    #[doc(alias = "nw_protocol_options")]
    #[doc(alias = "nw_protocol_options_t")]
    pub ProtocolOpts(ns::Id)
);

define_obj_type!(
    #[doc(alias = "nw_protocol_metadata")]
    #[doc(alias = "nw_protocol_metadata_t")]
    pub ProtocolMetadata(ns::Id)
);

impl ProtocolDefinition {
    /// Compare two protocol definitions to check if they represent the same protocol.
    #[doc(alias = "nw_protocol_definition_is_equal")]
    #[inline]
    pub fn is_equal(&self, other: &Self) -> bool {
        unsafe { nw_protocol_definition_is_equal(self, other) }
    }
}

impl ProtocolOpts {
    #[doc(alias = "nw_protocol_options_copy_definition")]
    #[inline]
    pub fn definition(&self) -> arc::R<nw::ProtocolDefinition> {
        unsafe { nw_protocol_options_copy_definition(self) }
    }
}

impl ProtocolMetadata {
    #[doc(alias = "nw_protocol_metadata_copy_definition")]
    #[inline]
    pub fn definition(&self) -> arc::R<nw::ProtocolDefinition> {
        unsafe { nw_protocol_metadata_copy_definition(self) }
    }
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    fn nw_protocol_definition_is_equal(
        a: &nw::ProtocolDefinition,
        b: &nw::ProtocolDefinition,
    ) -> bool;
    fn nw_protocol_options_copy_definition(
        options: &nw::ProtocolOpts,
    ) -> arc::R<nw::ProtocolDefinition>;
    fn nw_protocol_metadata_copy_definition(
        metadata: &nw::ProtocolMetadata,
    ) -> arc::R<nw::ProtocolDefinition>;
}
