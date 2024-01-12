use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_resolver_config")]
    #[doc(alias = "nw_resolver_config_t")]
    pub ResolverCfg(ns::Id)
);
