use std::ffi::c_void;

use crate::{four_cc_fmt_debug, os};

#[doc(alias = "CMIOObjectPropertySelector")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropSelector(pub u32);

impl PropSelector {
    #[doc(alias = "kCMIOObjectPropertySelectorWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));
}

impl PropSelector {
    pub const fn global_addr(self) -> PropAddr {
        PropAddr {
            selector: self,
            scope: PropScope::GLOBAL,
            element: PropElement::MAIN,
        }
    }
}

impl std::fmt::Debug for PropSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        four_cc_fmt_debug(self.0, "cm::io::PropSelector ", f)
    }
}

#[doc(alias = "CMIOObjectPropertyScope")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropScope(pub u32);

impl PropScope {
    #[doc(alias = "kCMIOObjectPropertyScopeWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    #[doc(alias = "kCMIOObjectPropertyScopeGlobal")]
    pub const GLOBAL: Self = Self(u32::from_be_bytes(*b"glob"));
}

impl std::fmt::Debug for PropScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        four_cc_fmt_debug(self.0, "cm::io::PropScope", f)
    }
}

#[doc(alias = "CMIOObjectPropertyElement")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropElement(pub u32);

impl PropElement {
    #[doc(alias = "kCMIOObjectPropertyElementWildcard")]
    pub const WILDCARD: Self = Self(u32::MAX);

    #[doc(alias = "kCMIOObjectPropertyElementMain")]
    pub const MAIN: Self = Self(0);

    #[doc(alias = "kCMIOObjectPropertyElementName")]
    pub const NAME: Self = Self(u32::from_be_bytes(*b"lchn"));

    #[doc(alias = "kCMIOObjectPropertyElementCategoryName")]
    pub const CATEGORY_NAME: Self = Self(u32::from_be_bytes(*b"lccn"));

    #[doc(alias = "kCMIOObjectPropertyElementNumberName")]
    pub const NUMBER_NAME: Self = Self(u32::from_be_bytes(*b"lcnn"));
}

impl std::fmt::Debug for PropElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        four_cc_fmt_debug(self.0, "cm::io::PropElement", f)
    }
}

#[doc(alias = "CMIOObjectPropertyAddress")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct PropAddr {
    pub selector: PropSelector,
    pub scope: PropScope,
    pub element: PropElement,
}

#[doc(alias = "CMIOClassID")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Class(pub u32);

impl Class {
    #[doc(alias = "kCMIOSystemObjectClassID")]
    pub const SYS_OBJECT: Self = Self(u32::from_be_bytes(*b"asys"));

    #[doc(alias = "kCMIOPlugInClassID")]
    pub const PLUG_IN: Self = Self(u32::from_be_bytes(*b"aplg"));
}

impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        four_cc_fmt_debug(self.0, "cm::io::Class", f)
    }
}

#[doc(alias = "CMIOObjectID")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Obj(pub u32);

impl Obj {
    #[doc(alias = "kCMIOObjectSystemObject")]
    pub const SYS: Self = Self(1);

    pub fn show(&self) {
        unsafe {
            CMIOObjectShow(*self);
        }
    }

    pub unsafe fn set_prop_data(
        &self,
        address: *const PropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Result {
        unsafe {
            CMIOObjectSetPropertyData(
                *self,
                address,
                qualifier_data_size,
                qualifier_data,
                data_size,
                data,
            )
            .result()
        }
    }

    pub fn set_prop<T: Sized>(&self, address: &PropAddr, val: &T) -> os::Result {
        unsafe {
            self.set_prop_data(
                address,
                0,
                std::ptr::null(),
                std::mem::size_of::<T>() as u32,
                val as *const T as _,
            )
        }
    }

    pub fn allow_screen_capture_devices(&self, val: bool) -> os::Result {
        let val: u32 = val as _;
        self.set_prop(
            &PropSelector::HW_ALLOW_SCREEN_CAPTURE_DEVICES.global_addr(),
            &val,
        )
    }

    pub fn allow_wireless_screen_capture_devices(&self, val: bool) -> os::Result {
        let val: u32 = val as _;
        self.set_prop(
            &PropSelector::HW_ALLOW_WIRELESS_SCREEN_CAPTURE_DEVICES.global_addr(),
            &val,
        )
    }
}

#[link(name = "CoreMediaIO", kind = "framework")]
extern "C-unwind" {
    fn CMIOObjectSetPropertyData(
        object_id: Obj,
        address: *const PropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn CMIOObjectShow(object_id: Obj);
}
