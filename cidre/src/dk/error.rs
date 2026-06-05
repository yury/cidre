use crate::{swift, swift::abi};

/// DockKit `DockKitError`.
#[doc(alias = "DockKitError")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Error(u32);

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0aB5ErrorO12notSupportedyA2CmFWC"]
    static DOCK_KIT_ERROR_NOT_SUPPORTED: u32;
    #[link_name = "$s7DockKit0aB5ErrorO12notConnectedyA2CmFWC"]
    static DOCK_KIT_ERROR_NOT_CONNECTED: u32;
    #[link_name = "$s7DockKit0aB5ErrorO20notSupportedByDeviceyA2CmFWC"]
    static DOCK_KIT_ERROR_NOT_SUPPORTED_BY_DEVICE: u32;
    #[link_name = "$s7DockKit0aB5ErrorO16invalidParameteryA2CmFWC"]
    static DOCK_KIT_ERROR_INVALID_PARAMETER: u32;
    #[link_name = "$s7DockKit0aB5ErrorO14noSubjectFoundyA2CmFWC"]
    static DOCK_KIT_ERROR_NO_SUBJECT_FOUND: u32;
    #[link_name = "$s7DockKit0aB5ErrorO15frameRateTooLowyA2CmFWC"]
    static DOCK_KIT_ERROR_FRAME_RATE_TOO_LOW: u32;
    #[link_name = "$s7DockKit0aB5ErrorO16cameraTCCMissingyA2CmFWC"]
    static DOCK_KIT_ERROR_CAMERA_TCC_MISSING: u32;
    #[link_name = "$s7DockKit0aB5ErrorO16frameRateTooHighyA2CmFWC"]
    static DOCK_KIT_ERROR_FRAME_RATE_TOO_HIGH: u32;

    #[link_name = "$s7DockKit0aB5ErrorO9hashValueSivg"]
    fn dock_kit_error_hash_value();
}

impl Error {
    #[inline]
    pub fn not_supported() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_NOT_SUPPORTED) }
    }

    #[inline]
    pub fn not_connected() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_NOT_CONNECTED) }
    }

    #[inline]
    pub fn not_supported_by_device() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_NOT_SUPPORTED_BY_DEVICE) }
    }

    #[inline]
    pub fn invalid_parameter() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_INVALID_PARAMETER) }
    }

    #[inline]
    pub fn no_subject_found() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_NO_SUBJECT_FOUND) }
    }

    #[inline]
    pub fn frame_rate_too_low() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_FRAME_RATE_TOO_LOW) }
    }

    #[inline]
    pub fn camera_tcc_missing() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_CAMERA_TCC_MISSING) }
    }

    #[inline]
    pub fn frame_rate_too_high() -> Self {
        unsafe { Self(DOCK_KIT_ERROR_FRAME_RATE_TOO_HIGH) }
    }

    #[inline]
    pub fn as_abi_ptr(&self) -> *const () {
        (self as *const Self).cast()
    }

    #[inline]
    pub fn hash_value(&self) -> swift::Int {
        unsafe { abi::call_value_to_int(dock_kit_error_hash_value as *const (), self.as_abi_ptr()) }
    }
}
