use std::ffi::c_void;

use crate::os;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropertySelector(pub u32);

impl PropertySelector {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropertyScope(pub u32);

impl PropertyScope {
    #[doc(alias = "kCMIOObjectPropertyScopeWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    #[doc(alias = "kCMIOObjectPropertyScopeGlobal")]
    pub const GLOBAL: Self = Self(u32::from_be_bytes(*b"glob"));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PropertyElement(pub u32);

impl PropertyElement {
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

#[derive(Debug)]
#[repr(C)]
pub struct PropertyAddress {
    pub selector: PropertySelector,
    pub scope: PropertyScope,
    pub element: PropertyElement,
}

#[repr(transparent)]
pub struct ClassId(pub u32);

impl ClassId {
    pub const SYSTEM_OBJECT: Self = Self(u32::from_be_bytes(*b"asys"));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Object(pub u32);

impl Object {
    pub const SYSTEM: Self = Self(1);

    pub fn show(&self) {
        unsafe {
            CMIOObjectShow(*self);
        }
    }

    pub unsafe fn set_property_data(
        &self,
        address: *const PropertyAddress,
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

    pub fn set_prop<T: Sized>(
        &self,
        address: &PropertyAddress,
        value: &T,
    ) -> Result<(), os::Status> {
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

    pub fn set_allow_screen_capture_devices(&self, value: bool) -> Result<(), os::Status> {
        let value: u32 = if value { 1u32 } else { 0u32 };
        let address = PropertyAddress {
            selector: PropertySelector::ALLOW_SCREEN_CAPTURE_DEVICES,
            scope: PropertyScope::GLOBAL,
            element: PropertyElement::MAIN,
        };
        self.set_prop(&address, &value)
    }

    pub fn set_allow_wireless_screen_capture_devices(&self, value: bool) -> Result<(), os::Status> {
        let value: u32 = if value { 1u32 } else { 0u32 };
        let address = PropertyAddress {
            selector: PropertySelector::ALLOW_WIRELESS_SCREEN_CAPTURE_DEVICES,
            scope: PropertyScope::GLOBAL,
            element: PropertyElement::MAIN,
        };
        self.set_prop(&address, &value)
    }
}

#[link(name = "CoreMediaIO", kind = "framework")]
extern "C" {
    fn CMIOObjectSetPropertyData(
        object_id: Object,
        address: *const PropertyAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn CMIOObjectShow(object_id: Object);
}
