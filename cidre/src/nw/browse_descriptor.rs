use std::ffi::{c_char, CStr};

use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_browse_descriptor")]
    #[doc(alias = "nw_browse_descriptor_t")]
    pub BrowseDesc(ns::Id)
);

impl BrowseDesc {
    #[doc(alias = "nw_browse_descriptor_create_bonjour_service")]
    #[inline]
    pub fn create_bonjour_service(type_: &CStr, domain: Option<&CStr>) -> arc::R<Self> {
        unsafe {
            nw_browse_descriptor_create_bonjour_service(
                type_.as_ptr(),
                domain.map_or(std::ptr::null(), CStr::as_ptr),
            )
        }
    }

    #[doc(alias = "nw_browse_descriptor_get_bonjour_service_type")]
    #[inline]
    pub fn bonjour_service_type(&self) -> &CStr {
        unsafe { CStr::from_ptr(nw_browse_descriptor_get_bonjour_service_type(self)) }
    }

    #[doc(alias = "nw_browse_descriptor_get_bonjour_service_domain")]
    #[inline]
    pub fn bonjour_service_domain(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_browse_descriptor_get_bonjour_service_domain(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "nw_browse_descriptor_set_include_txt_record")]
    #[inline]
    pub fn set_include_txt_record(&mut self, val: bool) {
        unsafe { nw_browse_descriptor_set_include_txt_record(self, val) }
    }

    #[doc(alias = "nw_browse_descriptor_get_include_txt_record")]
    #[inline]
    pub fn include_txt_record(&self) -> bool {
        unsafe { nw_browse_descriptor_get_include_txt_record(self) }
    }

    #[doc(alias = "nw_browse_descriptor_create_application_service")]
    #[inline]
    pub fn create_app_service(name: &CStr) -> arc::R<Self> {
        unsafe { nw_browse_descriptor_create_application_service(name.as_ptr()) }
    }

    #[doc(alias = "nw_browse_descriptor_get_application_service_name")]
    #[inline]
    pub fn app_service_name(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_browse_descriptor_get_application_service_name(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C-unwind" {
    fn nw_browse_descriptor_create_bonjour_service(
        type_: *const c_char,
        domain: *const c_char,
    ) -> arc::R<BrowseDesc>;

    fn nw_browse_descriptor_get_bonjour_service_type(descriptor: &BrowseDesc) -> *const c_char;
    fn nw_browse_descriptor_get_bonjour_service_domain(descriptor: &BrowseDesc) -> *const c_char;

    fn nw_browse_descriptor_set_include_txt_record(descriptor: &mut BrowseDesc, val: bool);
    fn nw_browse_descriptor_get_include_txt_record(descriptor: &BrowseDesc) -> bool;

    fn nw_browse_descriptor_create_application_service(
        app_service_name: *const c_char,
    ) -> arc::R<BrowseDesc>;

    fn nw_browse_descriptor_get_application_service_name(descriptor: &BrowseDesc) -> *const c_char;
}
