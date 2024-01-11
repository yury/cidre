use crate::{define_obj_type, ns};

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
