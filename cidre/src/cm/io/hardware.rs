pub mod object;
pub use object::Class;
pub use object::Object;
pub use object::PropAddr as ObjectPropAddr;
pub use object::PropElement as ObjectPropElement;
pub use object::PropScope as ObjectPropScope;
pub use object::PropSelector as ObjectPropSelector;

use crate::os;

/// The error constants unique to the DAL.
///
/// These are the error constants that are unique to the DAL.
/// Note that the DAL's functions can and will return other
/// codes that are not listed here. While these constants
/// give a general idea of what might have gone wrong during
/// the execution of an API call, if an API call returns anything
/// other than cm::io::Hardware::NO_ERROR it is to be
/// viewed as the same failure regardless of what constant is actually returned.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct Error(pub i32);

impl Error {
    /// The function call completed successfully.
    pub const NO_ERROR: Self = Self(0);

    /// The function call requires that the hardware be stopped but it isn't.
    pub const NOT_STOPPED: Self = Self(i32::from_be_bytes(*b"run "));

    /// The function call requires that the hardware be running but it isn't.
    pub const NOT_RUNNING: Self = Self(i32::from_be_bytes(*b"stop"));

    /// The function call failed while doing something that doesn't provide any error messages.
    pub const UNSPECIFIED: Self = Self(i32::from_be_bytes(*b"what"));

    /// The CMIOObject doesn't know about the property at the given address.
    pub const UNKNOWN_PROPERTY: Self = Self(i32::from_be_bytes(*b"who?"));

    /// An improperly sized buffer was provided when accessing the data of a property.
    pub const BAD_PROPERTY_SIZE: Self = Self(i32::from_be_bytes(*b"!siz"));

    /// The requested operation couldn't be completed.
    pub const ILLEGAL_OPERATION: Self = Self(i32::from_be_bytes(*b"nope"));

    /// The 'cm::io::ObjectId' passed to the function doesn't map to a valid 'cm::io::Object'.
    pub const BAD_OBJECT: Self = Self(i32::from_be_bytes(*b"!obj"));

    /// The 'cm::io::ObjectId' passed to the function doesn't map to a valid cm::io::Device.
    pub const BAD_DEVICE: Self = Self(i32::from_be_bytes(*b"!dev"));

    /// The 'cm::io::StreamId' passed to the function doesn't map to a valid 'cm::io::Stream'.
    pub const BAD_STREAM: Self = Self(i32::from_be_bytes(*b"!str"));

    /// The 'cm::io::Object' doesn't support the requested operation.
    pub const UNSUPPORTED_OPERATION: Self = Self(i32::from_be_bytes(*b"unop"));

    /// The function call failed because because access been suspended by the system.
    pub const SUSPENDED_BY_SYSTEM: Self = Self(i32::from_be_bytes(*b"deny"));

    ///  The 'cm::io::Stream' doesn't support the requested format.
    pub const UNSUPPORTED_FORMAT: Self = Self(i32::from_be_bytes(*b"!dat"));

    /// The requested operation can't be completed because the process doesn't have permission.
    pub const PERMISSIONS: Self = Self(i32::from_be_bytes(*b"!hog"));
}

impl PartialEq<os::Status> for Error {
    fn eq(&self, other: &os::Status) -> bool {
        self.0 == other.0
    }
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
