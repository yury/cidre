pub mod object;
pub use object::Class;
pub use object::Obj;
pub use object::PropAddr as ObjPropAddr;
pub use object::PropElement as ObjPropElement;
pub use object::PropScope as ObjPropScope;
pub use object::PropSelector as ObjPropSelector;

pub mod system;

/// The err module constants unique to the DAL.
///
/// These are the error constants that are unique to the DAL.
/// Note that the DAL's functions can and will return other
/// codes that are not listed here. While these constants
/// give a general idea of what might have gone wrong during
/// the execution of an API call.
pub mod err {
    use crate::os::Error;

    /// The function call requires that the hardware be stopped but it isn't.
    #[doc(alias = "kCMIOHardwareNotStoppedError")]
    pub const HW_NOT_STOPPED: Error = Error::from_be_bytes(*b"run ");

    /// The function call requires that the hardware be running but it isn't.
    #[doc(alias = "kCMIOHardwareNotRunningError")]
    pub const HW_NOT_RUNNING: Error = Error::from_be_bytes(*b"stop");

    /// The function call failed while doing something that doesn't provide any error messages.
    #[doc(alias = "kCMIOHardwareUnspecifiedError")]
    pub const HW_UNSPECIFIED: Error = Error::from_be_bytes(*b"what");

    /// The CMIOObject doesn't know about the property at the given address.
    #[doc(alias = "kCMIOHardwareUnknownPropertyError")]
    pub const HW_UNKNOWN_PROP: Error = Error::from_be_bytes(*b"who?");

    /// An improperly sized buffer was provided when accessing the data of a property.
    #[doc(alias = "kCMIOHardwareBadPropertySizeError")]
    pub const HW_BAD_PROP_SIZE: Error = Error::from_be_bytes(*b"!siz");

    /// The requested operation couldn't be completed.
    #[doc(alias = "kCMIOHardwareIllegalOperationError")]
    pub const HW_ILLEGAL_OPERATION: Error = Error::from_be_bytes(*b"nope");

    /// The 'cm::io::ObjectId' passed to the function doesn't map to a valid 'cm::io::Object'.
    #[doc(alias = "kCMIOHardwareBadObjectError")]
    pub const HW_BAD_OBJECT: Error = Error::from_be_bytes(*b"!obj");

    /// The 'cm::io::ObjectId' passed to the function doesn't map to a valid cm::io::Device.
    #[doc(alias = "kCMIOHardwareBadDeviceError")]
    pub const HW_BAD_DEVICE: Error = Error::from_be_bytes(*b"!dev");

    /// The 'cm::io::StreamId' passed to the function doesn't map to a valid 'cm::io::Stream'.
    #[doc(alias = "kCMIOHardwareBadStreamError")]
    pub const HW_BAD_STREAM: Error = Error::from_be_bytes(*b"!str");

    /// The 'cm::io::Object' doesn't support the requested operation.
    #[doc(alias = "kCMIOHardwareUnsupportedOperationError")]
    pub const HW_UNSUPPORTED_OPERATION: Error = Error::from_be_bytes(*b"unop");

    /// The function call failed because because access been suspended by the system.
    #[doc(alias = "kCMIOHardwareSuspendedBySystemError")]
    pub const HW_SUSPENDED_BY_SYSTEM: Error = Error::from_be_bytes(*b"deny");

    ///  The 'cm::io::Stream' doesn't support the requested format.
    #[doc(alias = "kCMIODeviceUnsupportedFormatError")]
    pub const DEVICE_UNSUPPORTED_FORMAT: Error = Error::from_be_bytes(*b"!dat");

    /// The requested operation can't be completed because the process doesn't have permission.
    #[doc(alias = "kCMIODevicePermissionsError")]
    pub const DEVICE_PERMISSIONS: Error = Error::from_be_bytes(*b"!hog");
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct PlugIn(pub i32);

impl PlugIn {
    /// The 'cm::io::ClassId' that identifies the 'cm::io::PlugIn' class.
    pub const CLASS_ID: Self = Self(i32::from_be_bytes(*b"aplg"));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct PlugInProp(pub i32);

impl PlugInProp {
    /// A 'cf::String' that contains the bundle identifier for the 'cm::io::PlugIn'.
    /// The caller is responsible for releasing the returned cf::Object.
    pub const BUNDLE_ID: Self = Self(i32::from_be_bytes(*b"piid"));
    pub const IS_EXTENSION: Self = Self(i32::from_be_bytes(*b"piie"));
}
