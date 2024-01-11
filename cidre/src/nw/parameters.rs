use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_parameters")]
    #[doc(alias = "nw_parameters_t")]
    pub Params(ns::Id)
);

define_obj_type!(
    #[doc(alias = "nw_protocol_stack")]
    #[doc(alias = "nw_protocol_stack_t")]
    pub ProtocolStack(ns::Id)
);
