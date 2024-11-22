use crate::{arc, blocks, define_obj_type, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_parameters")]
    #[doc(alias = "nw_parameters_t")]
    pub Params(ns::Id)
);

#[doc(alias = "nw_parameters_configure_protocol_block_t")]
pub type ParamsCfgProtocolBlock = blocks::SyncBlock<fn(options: &mut nw::ProtocolOpts)>;

impl Params {
    #[doc(alias = "nw_parameters_create")]
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { nw_parameters_create() }
    }

    #[doc(alias = "nw_parameters_copy")]
    #[inline]
    pub fn copy(&self) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_copy(self) }
    }

    #[doc(alias = "nw_parameters_create_secure_tcp")]
    #[inline]
    pub fn create_secure_tcp(
        configure_tls: &mut ParamsCfgProtocolBlock,
        configure_tcp: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_secure_tcp(configure_tls, configure_tcp) }
    }

    #[inline]
    pub fn default_tcp() -> arc::R<Self> {
        let cfg1 = Self::default_cfg();
        let cfg2 = Self::default_cfg();
        unsafe { Self::create_secure_tcp(cfg1, cfg2).unwrap_unchecked() }
    }

    #[doc(alias = "nw_parameters_create_secure_udp")]
    #[inline]
    pub fn create_secure_udp(
        configure_dtls: &mut ParamsCfgProtocolBlock,
        configure_udp: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_secure_udp(configure_dtls, configure_udp) }
    }

    #[doc(alias = "nw_parameters_create_custom_ip")]
    #[inline]
    pub fn create_secure_custom_ip(
        custom_ip_protocol_number: u32,
        configure_ip: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_custom_ip(custom_ip_protocol_number, configure_ip) }
    }

    #[doc(alias = "nw_parameters_create_quic")]
    #[inline]
    pub fn create_quic(configure_quic: &mut ParamsCfgProtocolBlock) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_quic(configure_quic) }
    }

    #[doc(alias = "nw_parameters_create_application_service")]
    #[inline]
    pub fn create_app_service() -> arc::R<Self> {
        unsafe { nw_parameters_create_application_service() }
    }

    #[doc(alias = "NW_PARAMETERS_DEFAULT_CONFIGURATION")]
    #[inline]
    pub fn default_cfg() -> &'static mut ParamsCfgProtocolBlock {
        unsafe { _nw_parameters_configure_protocol_default_configuration }
    }

    #[doc(alias = "NW_PARAMETERS_DISABLE_PROTOCOL")]
    #[inline]
    pub fn disable_protocol() -> &'static mut ParamsCfgProtocolBlock {
        unsafe { _nw_parameters_configure_protocol_disable }
    }
}

define_obj_type!(
    #[doc(alias = "nw_protocol_stack")]
    #[doc(alias = "nw_protocol_stack_t")]
    pub ProtocolStack(ns::Id)
);

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_parameters_create() -> arc::R<Params>;
    fn nw_parameters_copy(parameters: &Params) -> Option<arc::R<Params>>;

    fn nw_parameters_create_secure_tcp(
        configure_tls: &mut ParamsCfgProtocolBlock,
        configure_tcp: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_secure_udp(
        configure_dtls: &mut ParamsCfgProtocolBlock,
        configure_udp: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_custom_ip(
        custom_ip_protocol_number: u32,
        configure_ip: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_quic(
        configure_quic: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_application_service() -> arc::R<Params>;

    static mut _nw_parameters_configure_protocol_default_configuration:
        &'static mut ParamsCfgProtocolBlock;
    static mut _nw_parameters_configure_protocol_disable: &'static mut ParamsCfgProtocolBlock;
}

#[cfg(test)]
mod tests {
    use crate::{nw, objc::Obj};

    struct Dr(Vec<u8>);

    impl Drop for Dr {
        fn drop(&mut self) {
            eprintln!("Dropping");
        }
    }

    #[test]
    fn basics() {
        let params =
            nw::Params::create_secure_tcp(nw::Params::default_cfg(), nw::Params::default_cfg())
                .unwrap();
        eprintln!("{:?}", params.debug_desc());

        let mut x = Dr(vec![]);
        {
            let mut b = nw::ParamsCfgProtocolBlock::new1(move |opts| {
                x.0.push(1);
                eprintln!("{:?}----------> {:?}", x.0, opts);
            });
            let params = nw::Params::create_quic(&mut b).unwrap();
            eprintln!("{:?}", params.debug_desc());
        }
        eprintln!("{:?}", nw::Params::default_cfg().debug_desc());
        eprintln!("{:?}", nw::Params::disable_protocol().debug_desc());
    }
}
