use crate::{arc, define_obj_type, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_group_descriptor")]
    #[doc(alias = "nw_group_descriptor_t")]
    pub GroupDesc(ns::Id)
);

unsafe impl Send for GroupDesc {}
unsafe impl Sync for GroupDesc {}

define_obj_type!(
    #[doc(alias = "nw_group_descriptor")]
    #[doc(alias = "nw_group_descriptor_t")]
    pub MulticastGroupDesc(GroupDesc)
);

unsafe impl Send for MulticastGroupDesc {}
unsafe impl Sync for MulticastGroupDesc {}

#[cfg(feature = "blocks")]
pub type EnumerateEndpointsBlock = crate::blocks::NoEscBlock<fn(endpoint: &nw::Endpoint) -> bool>;

impl GroupDesc {
    #[doc(alias = "nw_group_descriptor_create_multiplex")]
    #[inline]
    pub fn mutliplex(remote_endpoint: &nw::Endpoint) -> Option<arc::R<Self>> {
        unsafe { nw_group_descriptor_create_multiplex(remote_endpoint) }
    }

    #[doc(alias = "nw_group_descriptor_create_multicast")]
    #[inline]
    pub fn mulitcast(multicast_group: &nw::Endpoint) -> Option<arc::R<MulticastGroupDesc>> {
        unsafe { nw_group_descriptor_create_multicast(multicast_group) }
    }

    #[doc(alias = "nw_group_descriptor_add_endpoint")]
    #[inline]
    pub fn add_endpoint(&mut self, endpoint: &nw::Endpoint) -> bool {
        unsafe { nw_group_descriptor_add_endpoint(self, endpoint) }
    }

    #[doc(alias = "nw_group_descriptor_enumerate_endpoints")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enumerate_endpoints_block(&self, block: &mut EnumerateEndpointsBlock) {
        unsafe {
            nw_group_descriptor_enumerate_endpoints(self, block);
        }
    }

    /// List all endpoints associated with the group descriptor.
    #[doc(alias = "nw_group_descriptor_enumerate_endpoints")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enumerate_endpoints(
        &self,
        mut block: impl FnMut(/* endpoint: */ &nw::Endpoint) -> bool,
    ) {
        let mut block = unsafe { EnumerateEndpointsBlock::stack1(&mut block) };
        self.enumerate_endpoints_block(&mut block);
    }
}

impl MulticastGroupDesc {
    /// Require a particular source for this multicast group descriptor.
    #[doc(alias = "nw_multicast_group_descriptor_set_specific_source")]
    #[inline]
    pub fn set_specific_src(&mut self, src: &nw::Endpoint) {
        unsafe {
            nw_multicast_group_descriptor_set_specific_source(self, src);
        }
    }

    #[doc(alias = "nw_multicast_group_descriptor_set_disable_unicast_traffic")]
    #[inline]
    pub fn set_disable_unicast_traffic(&mut self, disable_unicast_traffic: bool) {
        unsafe {
            nw_multicast_group_descriptor_set_disable_unicast_traffic(
                self,
                disable_unicast_traffic,
            );
        }
    }

    #[doc(alias = "nw_multicast_group_descriptor_get_disable_unicast_traffic")]
    #[inline]
    pub fn disable_unicast_traffic(&self) -> bool {
        unsafe { nw_multicast_group_descriptor_get_disable_unicast_traffic(self) }
    }
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    fn nw_group_descriptor_create_multiplex(
        remote_endpoint: &nw::Endpoint,
    ) -> Option<arc::R<GroupDesc>>;

    fn nw_group_descriptor_create_multicast(
        multicast_group: &nw::Endpoint,
    ) -> Option<arc::R<MulticastGroupDesc>>;

    fn nw_group_descriptor_add_endpoint(desc: &mut GroupDesc, endpoint: &nw::Endpoint) -> bool;

    #[cfg(feature = "blocks")]
    fn nw_group_descriptor_enumerate_endpoints(
        desc: &GroupDesc,
        block: &mut EnumerateEndpointsBlock,
    );

    fn nw_multicast_group_descriptor_set_specific_source(
        mutlicast_desc: &mut GroupDesc,
        src: &nw::Endpoint,
    );

    fn nw_multicast_group_descriptor_set_disable_unicast_traffic(
        mutlicast_desc: &mut GroupDesc,
        disable_unicast_traffic: bool,
    );

    fn nw_multicast_group_descriptor_get_disable_unicast_traffic(
        mutlicast_desc: &GroupDesc,
    ) -> bool;

}
