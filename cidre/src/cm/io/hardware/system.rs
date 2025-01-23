use crate::cm;

/// CMIOSystemObject Properties
/// CMIOObjectPropertySelector values that apply to the CMIOSystemObject.
///
/// The CMIOSystemObject has one scope, kCMIOObjectPropertyScopeGlobal, and only a main element.
impl cm::io::ObjPropSelector {
    /// A u32 where 1 means that the current process contains the main instance of the DAL.
    /// The main instance of the DAL is the only instance in which plug-ins should save/restore their devices' settings.
    #[doc(alias = "kCMIOHardwarePropertyProcessIsMain")]
    pub const HW_PROCESS_IS_MAIN: Self = Self(u32::from_be_bytes(*b"main"));

    /// A u32 whose value will be non-zero if the DAL is either in the midst of initializing or in the midst of exiting the process.
    #[doc(alias = "kCMIOHardwarePropertyIsInitingOrExiting")]
    pub const HW_IS_INITING_OR_EXITING: Self = Self(u32::from_be_bytes(*b"inot"));

    /// An array of the CMIODeviceIDs that represent all the devices currently available to the system.
    #[doc(alias = "kCMIOHardwarePropertyDevices")]
    pub const HW_DEVICES: Self = Self(u32::from_be_bytes(*b"dev#"));

    /// The CMIODeviceID of the default input CMIODevice.
    #[doc(alias = "kCMIOHardwarePropertyDefaultInputDevice")]
    pub const HW_DEFAULT_INPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dIn "));

    /// The CMIODeviceID of the default output CMIODevice.
    #[doc(alias = "kCMIOHardwarePropertyDefaultOutputDevice")]
    pub const HW_DEFAULT_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dOut"));

    /// Using a AudioValueTranslation structure, this property translates the input CFStringRef containing a UID into the CMIODeviceID
    /// that refers to the CMIODevice with that UID. This property will return kCMIODeviceUnknown if the given UID does not match
    /// any currently available CMIODevice.
    #[doc(alias = "kCMIOHardwarePropertyDeviceForUID")]
    pub const HW_DEVICE_FOR_UID: Self = Self(u32::from_be_bytes(*b"duid"));

    /// A u32 where 1 means that the process will allow the CPU to idle sleep even if there is IO in progress.
    /// A 0 means that the CPU will not be allowed to idle
    #[doc(alias = "kCMIOHardwarePropertySleepingIsAllowed")]
    pub const HW_SLEEPING_IS_ALLOWED: Self = Self(u32::from_be_bytes(*b"slep"));

    /// A u32 where 1 means that this process wants the DAL to unload itself after a period of inactivity where there are
    /// no streams active and no listeners registered with any CMIOObject.
    #[doc(alias = "kCMIOHardwarePropertyUnloadingIsAllowed")]
    pub const HW_UNLOADING_IS_ALLOWED: Self = Self(u32::from_be_bytes(*b"unld"));

    /// Using a AudioValueTranslation structure, this property translates the input CFString containing a bundle ID into the CMIOObjectID of the CMIOPlugIn that
    /// corresponds to it. This property will return kCMIOObjectUnkown if the given bundle ID doesn't match any CMIOPlugIns.
    #[doc(alias = "kCMIOHardwarePropertyPlugInForBundleID")]
    pub const HW_PLUG_IN_FOR_BUNDLE_ID: Self = Self(u32::from_be_bytes(*b"pibi"));

    /// A u32 where a value other than 0 indicates that the login session of the user of the process is either an active console session
    /// or a headless session.
    #[doc(alias = "kCMIOHardwarePropertyUserSessionIsActiveOrHeadless")]
    pub const HW_USER_SESSION_IS_ACTIVE_OR_HEADLESS: Self = Self(u32::from_be_bytes(*b"user"));

    /// A u32 where a value of 0 indicates the hardware is not suspended due to a system action, and a value of 1 means that it is.
    /// For example, if a fast user switch occurs, the system will suspend all devices. While suspended, no operartions can be performed on any devices.
    /// This property is never settable.
    #[doc(alias = "kCMIOHardwarePropertySuspendedBySystem")]
    pub const HW_SUSPENDED_BY_SYSTEM: Self = Self(u32::from_be_bytes(*b"sbys"));

    /// A u32 where 1 means that screen capture devices will be presented to the process.
    /// A 0 means screen capture devices will be ignored. By default, this property is 1.
    #[doc(alias = "kCMIOHardwarePropertyAllowScreenCaptureDevices")]
    pub const HW_ALLOW_SCREEN_CAPTURE_DEVICES: Self = Self(u32::from_be_bytes(*b"yes "));

    /// A u32 where 1 means that wireless screen capture devices will be presented to the process.
    /// A 0 means wireless screen capture devices will be ignored. By default, this property is 0.
    #[doc(alias = "kCMIOHardwarePropertyAllowWirelessScreenCaptureDevices")]
    pub const HW_ALLOW_WIRELESS_SCREEN_CAPTURE_DEVICES: Self = Self(u32::from_be_bytes(*b"wscd"));
}
