use std::ffi::c_void;

use crate::os;

#[doc(alias = "CMIOObjectPropertySelector")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropSelector(pub u32);

impl PropSelector {
    #[doc(alias = "kCMIOObjectPropertySelectorWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    /// A u32 where 1 means that screen capture devices will be presented to the process.
    /// A 0 means screen capture devices will be ignored. By default, this property is 1.
    #[doc(alias = "kCMIOHardwarePropertyAllowScreenCaptureDevices")]
    pub const ALLOW_SCREEN_CAPTURE_DEVICES: Self = Self(u32::from_be_bytes(*b"yes "));

    /// A u32 where 1 means that wireless screen capture devices will be presented to the process.
    /// A 0 means wireless screen capture devices will be ignored. By default, this property is 0.
    #[doc(alias = "kCMIOHardwarePropertyAllowWirelessScreenCaptureDevices")]
    pub const ALLOW_WIRELESS_SCREEN_CAPTURE_DEVICES: Self = Self(u32::from_be_bytes(*b"wscd"));
}

#[doc(alias = "CMIOObjectPropertyScope")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropScope(pub u32);

impl PropScope {
    #[doc(alias = "kCMIOObjectPropertyScopeWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    #[doc(alias = "kCMIOObjectPropertyScopeGlobal")]
    pub const GLOBAL: Self = Self(u32::from_be_bytes(*b"glob"));
}

#[doc(alias = "CMIOObjectPropertyElement")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[doc(alias = "CMIOObjectPropertyAddress")]
#[derive(Debug)]
#[repr(C)]
pub struct PropAddress {
    pub selector: PropSelector,
    pub scope: PropScope,
    pub element: PropElement,
}

#[doc(alias = "CMIOClassID")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Class(pub u32);

impl Class {
    #[doc(alias = "kCMIOSystemObjectClassID")]
    pub const SYSTEM_OBJECT: Self = Self(u32::from_be_bytes(*b"asys"));

    #[doc(alias = "kCMIOPlugInClassID")]
    pub const PLUG_IN: Self = Self(u32::from_be_bytes(*b"aplg"));
}

#[doc(alias = "CMIOObjectID")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Object(pub u32);

impl Object {
    #[doc(alias = "kCMIOObjectSystemObject")]
    pub const SYSTEM: Self = Self(1);

    pub fn show(&self) {
        unsafe {
            CMIOObjectShow(*self);
        }
    }

    pub unsafe fn set_property_data(
        &self,
        address: *const PropAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status {
        unsafe {
            CMIOObjectSetPropertyData(
                *self,
                address,
                qualifier_data_size,
                qualifier_data,
                data_size,
                data,
            )
        }
    }

    pub fn set_prop<T: Sized>(&self, address: &PropAddress, value: &T) -> Result<(), os::Status> {
        unsafe {
            self.set_property_data(
                address,
                0,
                std::ptr::null(),
                std::mem::size_of::<T>() as u32,
                value as *const T as _,
            )
            .result()
        }
    }

    pub fn allow_screen_capture_devices(&self, value: bool) -> Result<(), os::Status> {
        let value: u32 = if value { 1u32 } else { 0u32 };
        let address = PropAddress {
            selector: PropSelector::ALLOW_SCREEN_CAPTURE_DEVICES,
            scope: PropScope::GLOBAL,
            element: PropElement::MAIN,
        };
        self.set_prop(&address, &value)
    }

    pub fn allow_wireless_screen_capture_devices(&self, value: bool) -> Result<(), os::Status> {
        let value: u32 = if value { 1u32 } else { 0u32 };
        let address = PropAddress {
            selector: PropSelector::ALLOW_WIRELESS_SCREEN_CAPTURE_DEVICES,
            scope: PropScope::GLOBAL,
            element: PropElement::MAIN,
        };
        self.set_prop(&address, &value)
    }
}

#[link(name = "CoreMediaIO", kind = "framework")]
extern "C" {
    fn CMIOObjectSetPropertyData(
        object_id: Object,
        address: *const PropAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn CMIOObjectShow(object_id: Object);
}
