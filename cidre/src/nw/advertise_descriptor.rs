use std::ffi::{c_char, c_void, CStr};

use crate::{arc, define_obj_type, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_advertise_descriptor")]
    #[doc(alias = "nw_advertise_descriptor_t")]
    pub AdvertiseDesc(ns::Id)
);

impl AdvertiseDesc {
    #[doc(alias = "nw_advertise_descriptor_create_bonjour_service")]
    #[inline]
    pub fn bonjour_service(
        name: Option<impl AsRef<CStr>>,
        type_: impl AsRef<CStr>,
        domain: Option<impl AsRef<CStr>>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            nw_advertise_descriptor_create_bonjour_service(
                name.map_or(std::ptr::null(), |s| s.as_ref().as_ptr()),
                type_.as_ref().as_ptr(),
                domain.map_or(std::ptr::null(), |s| s.as_ref().as_ptr()),
            )
        }
    }

    #[doc(alias = "nw_advertise_descriptor_set_txt_record")]
    #[inline]
    pub fn set_txt_record(&mut self, val: &[u8]) {
        unsafe { nw_advertise_descriptor_set_txt_record(self, val.as_ptr() as _, val.len()) }
    }

    #[doc(alias = "nw_advertise_descriptor_set_no_auto_rename")]
    #[inline]
    pub fn set_no_auto_rename(&mut self, val: bool) {
        unsafe { nw_advertise_descriptor_set_no_auto_rename(self, val) }
    }

    #[doc(alias = "nw_advertise_descriptor_get_no_auto_rename")]
    #[inline]
    pub fn no_auto_rename(&mut self) -> bool {
        unsafe { nw_advertise_descriptor_get_no_auto_rename(self) }
    }

    #[doc(alias = "nw_advertise_descriptor_set_txt_record_object")]
    #[inline]
    pub fn set_txt_record_obj(&mut self, val: Option<&nw::TxtRecord>) {
        unsafe { nw_advertise_descriptor_set_txt_record_object(self, val) }
    }

    #[doc(alias = "nw_advertise_descriptor_copy_txt_record_object")]
    #[inline]
    pub fn txt_record_obj(&self) -> Option<arc::R<nw::TxtRecord>> {
        unsafe { nw_advertise_descriptor_copy_txt_record_object(self) }
    }

    #[doc(alias = "nw_advertise_descriptor_create_application_service")]
    #[inline]
    pub fn app_service(name: impl AsRef<CStr>) -> arc::R<Self> {
        unsafe { nw_advertise_descriptor_create_application_service(name.as_ref().as_ptr()) }
    }

    #[doc(alias = "nw_advertise_descriptor_get_application_service_name")]
    #[inline]
    pub fn app_service_name(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_advertise_descriptor_get_application_service_name(self);
            if ptr.is_null() {
                return None;
            }
            Some(CStr::from_ptr(ptr))
        }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C-unwind" {
    fn nw_advertise_descriptor_create_bonjour_service(
        name: *const c_char,
        type_: *const c_char,
        domain: *const c_char,
    ) -> Option<arc::R<AdvertiseDesc>>;

    fn nw_advertise_descriptor_set_txt_record(
        advertise_descriptor: &mut AdvertiseDesc,
        txt_record: *const c_void,
        txt_length: usize,
    );

    fn nw_advertise_descriptor_set_no_auto_rename(
        advertise_descriptor: &mut AdvertiseDesc,
        no_auto_rename: bool,
    );

    fn nw_advertise_descriptor_get_no_auto_rename(advertise_descriptor: &AdvertiseDesc) -> bool;

    fn nw_advertise_descriptor_set_txt_record_object(
        advertise_descriptor: &mut AdvertiseDesc,
        txt_record: Option<&nw::TxtRecord>,
    );

    fn nw_advertise_descriptor_copy_txt_record_object(
        advertise_descriptor: &AdvertiseDesc,
    ) -> Option<arc::R<nw::TxtRecord>>;

    fn nw_advertise_descriptor_create_application_service(
        application_service_name: *const c_char,
    ) -> arc::R<AdvertiseDesc>;

    fn nw_advertise_descriptor_get_application_service_name(
        advertise_descriptor: &AdvertiseDesc,
    ) -> *const c_char;
}

#[cfg(test)]
mod tests {
    use crate::nw;

    #[test]
    fn basics() {
        let name = c"Test";
        let desc = nw::AdvertiseDesc::app_service(&name);
        let service_name = desc.app_service_name().unwrap().to_str().unwrap();
        assert!(service_name.starts_with("Test"));
    }
}
