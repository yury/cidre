use crate::{arc, cf, define_cf_type};

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
}

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    fn DADiskGetTypeID() -> cf::TypeId;
    fn DADiskCopyDescription(disk: &Disk)
        -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>>;
    fn DADiskCopyWholeDisk(disk: &Disk) -> Option<arc::R<Disk>>;
}
