use crate::define_opts;

define_opts!(
    pub PowerFlags(i32)
);

impl PowerFlags {
    /// Indicates the device is on, requires power, and provides power. Useful as a: Capability, InputPowerRequirement, OutputPowerCharacter
    #[doc(alias = "kIOPMPowerOn")]
    pub const POWER_ON: Self = Self(0x00000002);

    /// Indicates the device is usable in this state. Useful only as a Capability
    #[doc(alias = "kIOPMDeviceUsable")]
    pub const DEVICE_USABLE: Self = Self(0x00008000);

    /// Indicates device is in a low power state. May be bitwis-OR'd together
    /// with DEVICE_USABLE flag, to indicate the device is still usable.
    #[doc(alias = "kIOPMLowPower")]
    pub const LOW_POWER: Self = Self(0x00010000);

    /// In the capability field of a power state, disallows idle system sleep while the device is in that state.
    #[doc(alias = "kIOPMPreventIdleSleep")]
    pub const PREVENT_IDLE_SLEEP: Self = Self(0x00000040);

    /// Used only by certain IOKit Families (USB). Not defined or used by generic Power Management.
    /// Read your family documentation to see if you should define a powerstate using these capabilities.
    #[doc(alias = "kIOPMSleepCapability")]
    pub const SLEEP_CAPABILITY: Self = Self(0x00000004);

    /// Used only by certain IOKit Families (USB). Not defined or used by generic Power Management.
    /// Read your family documentation to see if you should define a powerstate using these capabilities.
    #[doc(alias = "kIOPMRestartCapability")]
    pub const RESTART_CAPABILITY: Self = Self(0x00000080);

    /// Used only by certain IOKit Families (USB). Not defined or used by generic Power Management.
    /// Read your family documentation to see if you should define a powerstate using these capabilities.
    #[doc(alias = "kIOPMSleep")]
    pub const SLEEP: Self = Self(0x00000001);

    /// Used only by certain IOKit Families (USB). Not defined or used by generic Power Management.
    /// Read your family documentation to see if you should define a powerstate using these capabilities.
    #[doc(alias = "kIOPMRestart")]
    pub const RESTART: Self = Self(0x00000080);

    #[doc(alias = "kIOPMInitialDeviceState")]
    pub const INITIAL_DEVICE_STATE: Self = Self(0x00000100);

    #[doc(alias = "kIOPMRootDomainState")]
    pub const ROOT_DOMAIN_STATE: Self = Self(0x00000200);
}

pub mod err {
    use crate::os::Error;

    #[doc(alias = "kIOPMBadSpecification")]
    pub const BAD_SPEC: Error = Error::new_unchecked(4);

    #[doc(alias = "kIOPMNoSuchState")]
    pub const NO_SUCH_STATE: Error = Error::new_unchecked(5);

    #[doc(alias = "kIOPMCannotRaisePower")]
    pub const CANNOT_RAISE_POWER: Error = Error::new_unchecked(6);

    #[doc(alias = "kIOPMParameterError")]
    pub const PARAM_ERROR: Error = Error::new_unchecked(7);

    #[doc(alias = "kIOPMNotYetInitialized")]
    pub const NOT_YET_INITIALIZED: Error = Error::new_unchecked(8);
}

mod lib;

pub use lib::Assertion;
pub use lib::AssertionLevel;
pub use lib::UserActiveType;
pub use lib::assertions;
