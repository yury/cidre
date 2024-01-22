use crate::{arc, cf, da, define_cf_type};

define_cf_type!(
    #[doc(alias = "DADiskRef")]
    #[doc(alias = "DADisk")]
    Disk(cf::Type)
);

impl Disk {
    #[inline]
    pub fn get_type_id() -> cf::TypeId {
        unsafe { DADiskGetTypeID() }
    }

    #[inline]
    pub fn desc(&self) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
        unsafe { DADiskCopyDescription(self) }
    }

    #[inline]
    pub fn whole_disk(&self) -> Option<arc::R<Disk>> {
        unsafe { DADiskCopyWholeDisk(self) }
    }

    #[inline]
    pub fn from_bsd_name_in(
        session: &da::Session,
        name: &std::ffi::CStr,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { DADiskCreateFromBSDName(alloc, session, name.as_ptr()) }
    }

    #[inline]
    pub fn from_volume_path_in(
        session: &da::Session,
        path: &cf::Url,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { DADiskCreateFromVolumePath(alloc, session, path) }
    }

    #[inline]
    pub fn from_bsd_name(session: &da::Session, name: &std::ffi::CStr) -> Option<arc::R<Self>> {
        unsafe { DADiskCreateFromBSDName(None, session, name.as_ptr()) }
    }

    #[inline]
    pub fn from_volume_path(session: &da::Session, path: &cf::Url) -> Option<arc::R<Self>> {
        unsafe { DADiskCreateFromVolumePath(None, session, path) }
    }
}

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    fn DADiskGetTypeID() -> cf::TypeId;
    fn DADiskCopyDescription(disk: &Disk)
        -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>>;
    fn DADiskCopyWholeDisk(disk: &Disk) -> Option<arc::R<Disk>>;
    fn DADiskCreateFromBSDName(
        allocator: Option<&cf::Allocator>,
        session: &da::Session,
        name: *const std::ffi::c_char,
    ) -> Option<arc::R<Disk>>;
    fn DADiskCreateFromVolumePath(
        allocator: Option<&cf::Allocator>,
        session: &da::Session,
        path: &cf::Url,
    ) -> Option<arc::R<Disk>>;
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::{cf, da};

    #[test]
    fn basics() {
        let session = da::Session::new();
        let bsd_name = CString::new("test").unwrap();
        let disk = da::Disk::from_bsd_name(&session, &bsd_name).unwrap();
        disk.show();

        let path = cf::Url::from_str("/").unwrap();
        let disk = da::Disk::from_volume_path(&session, &path).unwrap();
        let desc = disk.desc().unwrap();
        desc.show();
        disk.show();
    }
}
