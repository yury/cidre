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
    pub fn secure_tcp(
        configure_tls: &mut ParamsCfgProtocolBlock,
        configure_tcp: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_secure_tcp(configure_tls, configure_tcp) }
    }

    #[inline]
    pub fn default_tcp() -> arc::R<Self> {
        let cfg1 = Self::default_cfg();
        let cfg2 = Self::default_cfg();
        unsafe { Self::secure_tcp(cfg1, cfg2).unwrap_unchecked() }
    }

    #[doc(alias = "nw_parameters_create_secure_udp")]
    #[inline]
    pub fn secure_udp(
        configure_dtls: &mut ParamsCfgProtocolBlock,
        configure_udp: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_secure_udp(configure_dtls, configure_udp) }
    }

    #[inline]
    pub fn udp() -> Option<arc::R<Self>> {
        unsafe {
            nw_parameters_create_secure_udp(
                nw::Params::disable_protocol(),
                nw::Params::default_cfg(),
            )
        }
    }

    #[doc(alias = "nw_parameters_create_custom_ip")]
    #[inline]
    pub fn secure_custom_ip(
        custom_ip_protocol_number: u32,
        configure_ip: &mut ParamsCfgProtocolBlock,
    ) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_custom_ip(custom_ip_protocol_number, configure_ip) }
    }

    #[doc(alias = "nw_parameters_create_quic")]
    #[inline]
    pub fn quic(configure_quic: &mut ParamsCfgProtocolBlock) -> Option<arc::R<Self>> {
        unsafe { nw_parameters_create_quic(configure_quic) }
    }

    #[doc(alias = "nw_parameters_create_application_service")]
    #[inline]
    pub fn app_service() -> arc::R<Self> {
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

#[cfg(feature = "blocks")]
pub type IterIfaceBlock = crate::blocks::NoEscBlock<fn(iface: &nw::Iface) -> bool>;

#[cfg(feature = "blocks")]
pub type IterIfaceTypeBlock = crate::blocks::NoEscBlock<fn(iface_type: nw::IfaceType) -> bool>;

/// Path Selection
impl Params {
    /// Require any connections or listeners using these parameters to use
    /// the provided network interface, or none if None is passed.
    #[doc(alias = "nw_parameters_require_interface")]
    pub fn set_required_iface(&mut self, iface: Option<&nw::Iface>) {
        unsafe {
            nw_parameters_require_interface(self, iface);
        }
    }

    #[doc(alias = "nw_parameters_copy_required_interface")]
    #[inline]
    pub fn required_iface(&self) -> Option<arc::R<nw::Iface>> {
        unsafe { nw_parameters_copy_required_interface(self) }
    }

    #[doc(alias = "nw_parameters_prohibit_interface")]
    #[inline]
    pub fn prohibit_iface(&mut self, iface: &nw::Iface) {
        unsafe { nw_parameters_prohibit_interface(self, iface) }
    }

    #[doc(alias = "nw_parameters_clear_prohibited_interfaces")]
    #[inline]
    pub fn clear_prohibit_ifaces(&mut self) {
        unsafe { nw_parameters_clear_prohibited_interfaces(self) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_parameters_iterate_prohibited_interfaces")]
    #[inline]
    pub fn iterate_prohibit_ifaces_block(&self, block: &mut IterIfaceBlock) {
        unsafe {
            nw_parameters_iterate_prohibited_interfaces(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_parameters_iterate_prohibited_interfaces")]
    #[inline]
    pub fn iterate_prohibit_ifaces(&self, mut f: impl FnMut(&nw::Iface) -> bool) {
        let mut block = unsafe { IterIfaceBlock::stack1(&mut f) };
        self.iterate_prohibit_ifaces_block(&mut block);
    }

    #[doc(alias = "nw_parameters_set_required_interface_type")]
    #[inline]
    pub fn set_required_iface_type(&mut self, val: nw::IfaceType) {
        unsafe {
            nw_parameters_set_required_interface_type(self, val);
        }
    }

    #[doc(alias = "nw_parameters_get_required_interface_type")]
    #[inline]
    pub fn required_iface_type(&self) -> nw::IfaceType {
        unsafe { nw_parameters_get_required_interface_type(self) }
    }

    #[doc(alias = "nw_parameters_prohibit_interface_type")]
    #[inline]
    pub fn prohibit_iface_type(&mut self, val: nw::IfaceType) {
        unsafe {
            nw_parameters_prohibit_interface_type(self, val);
        }
    }

    #[doc(alias = "nw_parameters_clear_prohibited_interface_types")]
    #[inline]
    pub fn clear_prohibited_iface_types(&mut self) {
        unsafe { nw_parameters_clear_prohibited_interface_types(self) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_parameters_iterate_prohibited_interface_types")]
    #[inline]
    pub fn iterate_prohibit_iface_types_block(&self, block: &mut IterIfaceTypeBlock) {
        unsafe {
            nw_parameters_iterate_prohibited_interface_types(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_parameters_iterate_prohibited_interface_types")]
    #[inline]
    pub fn iterate_prohibit_iface_types(&self, mut f: impl FnMut(nw::IfaceType) -> bool) {
        let mut block = unsafe { IterIfaceTypeBlock::stack1(&mut f) };
        self.iterate_prohibit_iface_types_block(&mut block);
    }

    #[doc(alias = "nw_parameters_set_prohibit_expensive")]
    #[inline]
    pub fn set_prohibit_expensive(&mut self, val: bool) {
        unsafe {
            nw_parameters_set_prohibit_expensive(self, val);
        }
    }

    /// Returns true if expensive interfaces are prohibited, or
    /// false otherwise.
    #[doc(alias = "nw_parameters_get_prohibit_expensive")]
    #[inline]
    pub fn prohibit_expensive(&self) -> bool {
        unsafe { nw_parameters_get_prohibit_expensive(self) }
    }

    #[doc(alias = "nw_parameters_get_prohibit_constrained")]
    #[inline]
    pub fn prohibit_constrained(&self) -> bool {
        unsafe { nw_parameters_get_prohibit_constrained(self) }
    }

    #[doc(alias = "nw_parameters_set_prohibit_constrained")]
    #[inline]
    pub fn set_prohibit_constrained(&mut self, val: bool) {
        unsafe { nw_parameters_set_prohibit_constrained(self, val) }
    }
}

define_obj_type!(
    #[doc(alias = "nw_protocol_stack")]
    #[doc(alias = "nw_protocol_stack_t")]
    pub ProtocolStack(ns::Id)
);

#[link(name = "Network", kind = "framework")]
unsafe extern "C-unwind" {
    fn nw_parameters_create() -> arc::R<Params>;
    fn nw_parameters_copy(params: &Params) -> Option<arc::R<Params>>;

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

    fn nw_parameters_require_interface(params: &mut Params, iface: Option<&nw::Iface>);
    fn nw_parameters_copy_required_interface(params: &Params) -> Option<arc::R<nw::Iface>>;
    fn nw_parameters_prohibit_interface(params: &mut Params, iface: &nw::Iface);
    fn nw_parameters_clear_prohibited_interfaces(params: &mut Params);
    #[cfg(feature = "blocks")]
    fn nw_parameters_iterate_prohibited_interfaces(params: &Params, block: &mut IterIfaceBlock);
    fn nw_parameters_set_required_interface_type(params: &mut Params, val: nw::IfaceType);
    fn nw_parameters_get_required_interface_type(params: &Params) -> nw::IfaceType;
    fn nw_parameters_prohibit_interface_type(params: &mut Params, val: nw::IfaceType);
    fn nw_parameters_clear_prohibited_interface_types(params: &mut Params);
    #[cfg(feature = "blocks")]
    fn nw_parameters_iterate_prohibited_interface_types(
        params: &Params,
        block: &mut IterIfaceTypeBlock,
    );

    fn nw_parameters_set_prohibit_expensive(params: &mut Params, prohibit_expensive: bool);

    fn nw_parameters_get_prohibit_expensive(params: &Params) -> bool;

    fn nw_parameters_set_prohibit_constrained(params: &mut Params, prohibit_constrained: bool);
    fn nw_parameters_get_prohibit_constrained(params: &Params) -> bool;

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
            nw::Params::secure_tcp(nw::Params::default_cfg(), nw::Params::default_cfg()).unwrap();
        eprintln!("{:?}", params.debug_desc());

        let mut x = Dr(vec![]);
        {
            let mut b = nw::ParamsCfgProtocolBlock::new1(move |opts| {
                x.0.push(1);
                eprintln!("{:?}----------> {:?}", x.0, opts);
            });
            let params = nw::Params::quic(&mut b).unwrap();
            eprintln!("{:?}", params.debug_desc());
        }
        eprintln!("{:?}", nw::Params::default_cfg().debug_desc());
        eprintln!("{:?}", nw::Params::disable_protocol().debug_desc());
    }

    #[test]
    fn paths() {
        let udp_params = nw::Params::udp().unwrap();
        let mut n = 0;
        udp_params.iterate_prohibit_ifaces(|_iface| {
            n += 1;
            true
        });

        assert_eq!(n, 0);
    }
}
