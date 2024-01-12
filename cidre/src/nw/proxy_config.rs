use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_proxy_config")]
    #[doc(alias = "nw_proxy_config_t")]
    pub ProxyCfg(ns::Id)
);

define_obj_type!(
    #[doc(alias = "nw_relay_hop")]
    #[doc(alias = "nw_relay_hop_t")]
    pub RelayHop(ns::Id)
);
