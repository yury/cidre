use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_proxy_config")]
    #[doc(alias = "nw_proxy_config_t")]
    pub ProxyCfg(ns::Id)
);
