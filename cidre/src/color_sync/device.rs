//! Display identity utilities from `ColorSyncDevice.h`.

use crate::{arc, cf, cg};

/// The system UUID corresponding to the current display ID.
#[doc(alias = "CGDisplayCreateUUIDFromDisplayID")]
#[inline]
pub fn display_uuid(display: cg::DirectDisplayId) -> Option<arc::R<cf::Uuid>> {
    unsafe { CGDisplayCreateUUIDFromDisplayID(display) }
}

/// The current display ID corresponding to the system UUID.
#[doc(alias = "CGDisplayGetDisplayIDFromUUID")]
#[inline]
pub fn display_id(uuid: &cf::Uuid) -> cg::DirectDisplayId {
    unsafe { CGDisplayGetDisplayIDFromUUID(uuid) }
}

#[link(name = "ColorSync", kind = "framework")]
unsafe extern "C-unwind" {
    fn CGDisplayCreateUUIDFromDisplayID(display: cg::DirectDisplayId) -> Option<arc::R<cf::Uuid>>;
    fn CGDisplayGetDisplayIDFromUUID(uuid: &cf::Uuid) -> cg::DirectDisplayId;
}
