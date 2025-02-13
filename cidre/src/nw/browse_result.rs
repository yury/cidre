use crate::{arc, blocks, define_obj_type, define_opts, ns, nw};

#[doc(alias = "nw_browse_result_enumerate_interface_t")]
pub type EnumerateIface<Attr> = blocks::Block<fn(&nw::Iface) -> bool, Attr>;

define_obj_type!(
    #[doc(alias = "nw_browse_result")]
    #[doc(alias = "nw_browse_result_t")]
    pub BrowseResult(ns::Id)
);

define_opts!(
    #[doc(alias = "nw_browse_result_change_t")]
    pub Change(u64)
);

impl Change {
    /// The browse result change is invalid.
    #[doc(alias = "nw_browse_result_change_invalid")]
    pub const INVALID: Self = Self(0x00);

    /// The browse results being compared are identical.
    #[doc(alias = "nw_browse_result_change_identical")]
    pub const IDENTICAL: Self = Self(0x01);

    /// A browse result was added.
    #[doc(alias = "nw_browse_result_change_result_added")]
    pub const RESULT_ADDED: Self = Self(0x02);

    /// A browse result was removed.
    #[doc(alias = "nw_browse_result_change_result_removed")]
    pub const RESULT_REMOVED: Self = Self(0x04);

    /// An interface became available.
    #[doc(alias = "nw_browse_result_change_interface_added")]
    pub const IFACE_ADDED: Self = Self(0x08);

    /// An interface was removed
    #[doc(alias = "nw_browse_result_change_interface_removed")]
    pub const IFACE_REMOVED: Self = Self(0x10);

    /// The TXT record changed.
    #[doc(alias = "nw_browse_result_change_txt_record_changed")]
    pub const TXT_RECORD_CHANGED: Self = Self(0x20);
}

impl BrowseResult {
    #[doc(alias = "nw_browse_result_copy_endpoint")]
    #[inline]
    pub fn endpoint(&self) -> arc::R<nw::Endpoint> {
        unsafe { nw_browse_result_copy_endpoint(self) }
    }

    #[doc(alias = "nw_browse_result_get_changes")]
    #[inline]
    pub fn changes(old: Option<&Self>, new: Option<&Self>) -> Change {
        unsafe { nw_browse_result_get_changes(old, new) }
    }

    /// The number of interfaces available.
    #[doc(alias = "nw_browse_result_get_interfaces_count")]
    pub fn ifaces_count(&self) -> usize {
        unsafe { nw_browse_result_get_interfaces_count(self) }
    }

    #[doc(alias = "nw_browse_result_copy_txt_record_object")]
    #[inline]
    pub fn txt_record_obj(&self) -> Option<arc::R<nw::TxtRecord>> {
        unsafe { nw_browse_result_copy_txt_record_object(self) }
    }

    #[doc(alias = "nw_browse_result_copy_txt_record_object")]
    #[inline]
    pub fn enumerate_ifaces<'a>(&self, enumerator: &mut EnumerateIface<blocks::NoEsc>) {
        unsafe { nw_browse_result_enumerate_interfaces(self, enumerator) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C-unwind" {
    fn nw_browse_result_copy_endpoint(res: &BrowseResult) -> arc::R<nw::Endpoint>;
    fn nw_browse_result_get_changes(
        old: Option<&BrowseResult>,
        new: Option<&BrowseResult>,
    ) -> Change;

    fn nw_browse_result_get_interfaces_count(res: &BrowseResult) -> usize;
    fn nw_browse_result_copy_txt_record_object(res: &BrowseResult)
        -> Option<arc::R<nw::TxtRecord>>;

    fn nw_browse_result_enumerate_interfaces(
        res: &BrowseResult,
        enumerator: &mut EnumerateIface<blocks::NoEsc>,
    );
}
