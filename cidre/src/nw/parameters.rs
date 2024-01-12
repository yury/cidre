use std::ffi::c_void;

use crate::{arc, blocks, define_obj_type, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_parameters")]
    #[doc(alias = "nw_parameters_t")]
    pub Params(ns::Id)
);

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
    pub fn create_secure_tcp<TlsB, TcpB>(
        configure_tls: &TlsB,
        configure_tcp: &TcpB,
    ) -> Option<arc::R<Self>>
    where
        TlsB: ConfigureProtocolBlock,
        TcpB: ConfigureProtocolBlock,
    {
        unsafe {
            nw_parameters_create_secure_tcp(
                configure_tls.as_block_ptr(),
                configure_tcp.as_block_ptr(),
            )
        }
    }

    #[doc(alias = "nw_parameters_create_secure_udp")]
    #[inline]
    pub fn create_secure_udp<DtlsB, UdpB>(
        configure_dtls: &DtlsB,
        configure_udp: &UdpB,
    ) -> Option<arc::R<Self>>
    where
        DtlsB: ConfigureProtocolBlock,
        UdpB: ConfigureProtocolBlock,
    {
        unsafe {
            nw_parameters_create_secure_udp(
                configure_dtls.as_block_ptr(),
                configure_udp.as_block_ptr(),
            )
        }
    }

    #[doc(alias = "nw_parameters_create_custom_ip")]
    #[inline]
    pub fn create_secure_custom_ip<IpB>(
        custom_ip_protocol_number: u32,
        configure_ip: &IpB,
    ) -> Option<arc::R<Self>>
    where
        IpB: ConfigureProtocolBlock,
    {
        unsafe {
            nw_parameters_create_custom_ip(custom_ip_protocol_number, configure_ip.as_block_ptr())
        }
    }

    #[doc(alias = "nw_parameters_create_quic")]
    #[inline]
    pub fn create_quic<B>(configure_quic: &B) -> Option<arc::R<Self>>
    where
        B: ConfigureProtocolBlock,
    {
        unsafe { nw_parameters_create_quic(configure_quic.as_block_ptr()) }
    }

    #[doc(alias = "nw_parameters_create_application_service")]
    #[inline]
    pub fn create_app_service() -> arc::R<Self> {
        unsafe { nw_parameters_create_application_service() }
    }

    #[doc(alias = "NW_PARAMETERS_DEFAULT_CONFIGURATION")]
    #[inline]
    pub fn default_cfg() -> ConfigureProtocol {
        ConfigureProtocol(unsafe { _nw_parameters_configure_protocol_default_configuration })
    }

    #[doc(alias = "NW_PARAMETERS_DISABLE_PROTOCOL")]
    #[inline]
    pub fn disable_protocol() -> ConfigureProtocol {
        ConfigureProtocol(unsafe { _nw_parameters_configure_protocol_disable })
    }
}

pub trait ConfigureProtocolBlock {
    fn as_block_ptr(&self) -> *const c_void;
}

pub struct ConfigureProtocol(*const c_void);

impl ConfigureProtocolBlock for ConfigureProtocol {
    #[inline]
    fn as_block_ptr(&self) -> *const c_void {
        self.0
    }
}

impl<'a, F> ConfigureProtocolBlock for blocks::Block<F>
where
    F: FnOnce(&'a mut nw::ProtocolOpts),
{
    #[inline]
    fn as_block_ptr(&self) -> *const c_void {
        self.as_ptr()
    }
}

impl ConfigureProtocolBlock for blocks::Block<extern "C" fn(&mut nw::ProtocolOpts)> {
    #[inline]
    fn as_block_ptr(&self) -> *const c_void {
        self.as_ptr()
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
        configure_tls: *const c_void,
        configure_tcp: *const c_void,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_secure_udp(
        configure_dtls: *const c_void,
        configure_udp: *const c_void,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_custom_ip(
        custom_ip_protocol_number: u32,
        configure_ip: *const c_void,
    ) -> Option<arc::R<Params>>;

    fn nw_parameters_create_quic(configure_quic: *const c_void) -> Option<arc::R<Params>>;

    fn nw_parameters_create_application_service() -> arc::R<Params>;

    static _nw_parameters_configure_protocol_default_configuration: *const c_void;
    static _nw_parameters_configure_protocol_disable: *const c_void;
}

#[cfg(test)]
mod tests {
    use crate::{blocks, nw, objc::Obj};

    #[test]
    fn basics() {
        let params =
            nw::Params::create_secure_tcp(&nw::Params::default_cfg(), &nw::Params::default_cfg())
                .unwrap();
        eprintln!("{:?}", params.debug_desc());

        let b = blocks::once1(|opts| {
            eprintln!("{:?}", opts);
        });
        let params = nw::Params::create_quic(b.escape()).unwrap();
        eprintln!("{:?}", params.debug_desc());
    }
}
