use crate::{define_options, os};

define_options!(Flags(u32));

/// AudioComponentFlags
impl Flags {
    /// When this bit in AudioComponentDescription's componentFlags is set, AudioComponentFindNext
    /// will only return this component when performing a specific, non-wildcard search for the
    /// component, i.e. with non-zero values of componentType, componentSubType, and
    /// componentManufacturer. This can be useful when privately registering a component.
    pub const UNSEARCHABLE: Self = Self(1);

    /// An AudioComponent sets this bit in its componentFlags to indicate to the system that the
    /// AudioComponent is safe to open in a sandboxed process.
    pub const SANDBOX_SAFE: Self = Self(2);

    /// The system sets this flag automatically when registering components which implement a version 3
    /// Audio Unit.
    pub const IS_V3_AUDIO_UNIT: Self = Self(4);

    /// The system sets this flag automatically when registering components which require asynchronous
    /// instantiation via AudioComponentInstantiate (v3 audio units with views).
    pub const REQUIRES_ASYNC_INSTANTIATION: Self = Self(8);

    /// The system sets this flag automatically when registering components which can be loaded into
    /// the current process. This is always true for V2 audio units; it depends on the packaging
    /// in the case of a V3 audio unit.
    pub const CAN_LOAD_IN_PROCESS: Self = Self(0x10);
}

define_options!(InstantiationOptions(u32));

/// AudioComponentInstantiationOptions
///
/// Most component instances are loaded into the calling process.
///
/// A version 3 audio unit, however, can be loaded into a separate extension service process,
/// and this is the default behavior for these components. To be able to load one in-process
/// requires that the developer package the audio unit in a bundle separate from the application
/// extension, since an extension's main binary cannot be dynamically loaded into another
/// process.
///
/// A macOS host may request in-process loading of such audio units using
/// LOAD_IN_PROCESS.
///
/// Flag::IS_V3_AUDIO_UNIT specifies whether an audio unit is implemented using API
/// version 3.
///
/// These options are just requests to the implementation. It may fail and fall back to the
/// default.
impl InstantiationOptions {
    /// Attempt to load the component into a separate extension process.
    pub const LOAD_OUT_OF_PROCESS: Self = Self(1);

    /// Attempt to load the component into the current process. Only available on macOS.
    pub const LOAD_IN_PROCESS: Self = Self(2);
    pub const LOADED_REMOTELY: Self = Self(1 << 31);
}

/// AudioComponentDescription
///
/// A structure describing the unique and identifying IDs of an audio component
///
#[repr(C, align(4))]
pub struct Description {
    /// A 4-char code identifying the generic type of an audio component.
    pub type_: os::Type,

    /// A 4-char code identifying the a specific individual component. type/
    /// subtype/manufacturer triples must be globally unique.
    pub sub_type: os::Type,

    ///  Vendor identification.
    pub manufacturer: os::Type,

    /// Must be set to zero unless a known specific value is requested.
    pub flags: u32,

    /// Must be set to zero unless a known specific value is requested.
    pub flags_mask: u32,
}
