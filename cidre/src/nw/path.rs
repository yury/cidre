use crate::{arc, define_obj_type, ns, nw};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "nw_path_enumerate_interfaces_block_t")]
pub type EnumerateIfaces = blocks::NoEscBlock<fn(&nw::Iface) -> bool>;

define_obj_type!(
    #[doc(alias = "nw_path")]
    #[doc(alias = "nw_path_t")]
    pub Path(ns::Id)
);

#[doc(alias = "nw_path_status_t")]
#[repr(i32)]
pub enum Status {
    /// The path is not valid
    #[doc(alias = "nw_path_status_invalid")]
    Invalid = 0,

    /// The path has a usable route upon which to send and receive data
    #[doc(alias = "nw_path_status_satisfied")]
    Satisfied = 1,

    /// The path does not have a usable route. This may be due to a network
    /// interface being down, or due to system policy.
    #[doc(alias = "nw_path_status_unsatisfied")]
    Unsatisfied = 2,

    /// The path does not currently have a usable route, but a connection attempt
    /// will trigger network attachment
    #[doc(alias = "nw_path_status_satisfiable")]
    Satisfieable = 3,
}

#[doc(alias = "nw_path_unsatisfied_reason_t")]
#[repr(i32)]
pub enum UnsatisfiedReason {
    /// No reason is given
    #[doc(alias = "nw_path_unsatisfied_reason_not_available")]
    NotAvailable = 0,

    /// The user has disabled cellular
    #[doc(alias = "nw_path_unsatisfied_reason_cellular_denied")]
    CellularDenied = 1,

    /// The user has disabled Wi-Fi
    #[doc(alias = "nw_path_unsatisfied_reason_wifi_denied")]
    WifiDenied = 2,

    /// The user has disabled local network access
    #[doc(alias = "nw_path_unsatisfied_reason_local_network_denied")]
    LocalNetworkDenied = 3,

    /// A required VPN is not active
    #[doc(alias = "nw_path_unsatisfied_reason_vpn_inactive")]
    VpnInactive = 4,
}

impl Path {
    #[doc(alias = "nw_path_get_status")]
    #[inline]
    pub fn status(&self) -> Status {
        unsafe { nw_path_get_status(self) }
    }

    #[doc(alias = "nw_path_get_unsatisfied_reason")]
    #[inline]
    pub fn unsatified_reason(&self) -> UnsatisfiedReason {
        unsafe { nw_path_get_unsatisfied_reason(self) }
    }

    #[cfg(feature = "blocks")]
    pub fn enumerate_ifaces(&self, block: &mut nw::PathEnumerateIfaces) {
        unsafe { nw_path_enumerate_interfaces(self, block) }
    }

    #[doc(alias = "nw_path_is_expensive")]
    #[inline]
    pub fn is_expensive(&self) -> bool {
        unsafe { nw_path_is_expensive(self) }
    }

    #[doc(alias = "nw_path_is_constrained")]
    #[inline]
    pub fn is_constrained(&self) -> bool {
        unsafe { nw_path_is_constrained(self) }
    }

    #[doc(alias = "nw_path_has_ipv4")]
    #[inline]
    pub fn has_ipv4(&self) -> bool {
        unsafe { nw_path_has_ipv4(self) }
    }

    #[doc(alias = "nw_path_has_ipv6")]
    #[inline]
    pub fn has_ipv6(&self) -> bool {
        unsafe { nw_path_has_ipv6(self) }
    }

    #[doc(alias = "nw_path_has_dns")]
    #[inline]
    pub fn has_dns(&self) -> bool {
        unsafe { nw_path_has_dns(self) }
    }

    #[doc(alias = "nw_interface_type_t")]
    #[inline]
    pub fn uses_iface_type(&self, val: nw::IfaceType) -> bool {
        unsafe { nw_path_uses_interface_type(self, val) }
    }

    #[doc(alias = "nw_path_copy_effective_local_endpoint")]
    #[inline]
    pub fn effective_local_endpoint(&self) -> Option<arc::R<nw::Endpoint>> {
        unsafe { nw_path_copy_effective_local_endpoint(self) }
    }

    #[doc(alias = "nw_path_copy_effective_remote_endpoint")]
    #[inline]
    pub fn effective_remote_endpoint(&self) -> Option<arc::R<nw::Endpoint>> {
        unsafe { nw_path_copy_effective_remote_endpoint(self) }
    }

    #[doc(alias = "nw_path_enumerate_gateways")]
    #[inline]
    pub fn enumerate_gateways(&self, block: &mut blocks::NoEscBlock<fn(&nw::Endpoint) -> bool>) {
        unsafe { nw_path_enumerate_gateways(self, block) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C-unwind" {
    fn nw_path_get_status(path: &Path) -> Status;
    fn nw_path_get_unsatisfied_reason(path: &Path) -> UnsatisfiedReason;

    #[cfg(feature = "blocks")]
    fn nw_path_enumerate_interfaces(path: &Path, block: &mut nw::PathEnumerateIfaces);

    fn nw_path_is_expensive(path: &Path) -> bool;
    fn nw_path_is_constrained(path: &Path) -> bool;
    fn nw_path_has_ipv4(path: &Path) -> bool;
    fn nw_path_has_ipv6(path: &Path) -> bool;
    fn nw_path_has_dns(path: &Path) -> bool;

    fn nw_path_uses_interface_type(path: &Path, iface_type: nw::IfaceType) -> bool;
    fn nw_path_copy_effective_local_endpoint(path: &Path) -> Option<arc::R<nw::Endpoint>>;
    fn nw_path_copy_effective_remote_endpoint(path: &Path) -> Option<arc::R<nw::Endpoint>>;

    #[cfg(feature = "blocks")]
    fn nw_path_enumerate_gateways(
        path: &Path,
        block: &mut blocks::NoEscBlock<fn(&nw::Endpoint) -> bool>,
    );
}
