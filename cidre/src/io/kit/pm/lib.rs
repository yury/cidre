use crate::{arc, cf, io};

pub mod assertions {
    use crate::cf;

    /// Prevents the system from sleeping automatically due to a lack of user activity.
    #[doc(alias = "kIOPMAssertPreventUserIdleSystemSleep")]
    pub fn prevent_user_idle_system_sleep() -> &'static cf::String {
        cf::str!(c"PreventUserIdleSystemSleep")
    }

    /// Prevents the display from dimming automatically.
    #[doc(alias = "kIOPMAssertPreventUserIdleDisplaySleep")]
    pub fn prevent_user_idle_display_sleep() -> &'static cf::String {
        cf::str!(c"PreventUserIdleDisplaySleep")
    }

    /// Prevent attached disks from idling into lower power states.
    #[doc(alias = "kIOPMAssertPreventDiskIdle")]
    pub fn prevent_disk_idle() -> &'static cf::String {
        cf::str!(c"PreventDiskIdle")
    }

    /// Keeps the system awake while OS X serves active network clients.
    #[doc(alias = "kIOPMAssertNetworkClientActive")]
    pub fn network_client_active() -> &'static cf::String {
        cf::str!(c"NetworkClientActive")
    }
}

#[derive(Debug, Eq, PartialEq)]
#[doc(alias = "IOPMAssertionID")]
#[repr(transparent)]
pub struct Assertion(pub u32);

impl Assertion {
    pub const NULL: Self = Self(0);

    #[doc(alias = "IOPMAssertionCreateWithName")]
    pub fn with_name(
        name: impl AsRef<cf::String>,
        assertion_type: &cf::String,
        level: AssertionLevel,
    ) -> Result<Self, io::Return> {
        let mut val = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            IOPMAssertionCreateWithName(assertion_type, level, name.as_ref(), val.as_mut_ptr())
                .result()?
        }
        Ok(unsafe { val.assume_init() })
    }

    #[doc(alias = "IOPMAssertionCopyProperties")]
    pub fn props(&self) -> arc::R<cf::DictionaryOf<cf::String, cf::Type>> {
        unsafe { IOPMAssertionCopyProperties(self.0) }
    }
}

impl Drop for Assertion {
    fn drop(&mut self) {
        unsafe {
            let r = IOPMAssertionRelease(self.0);
            debug_assert!(r.is_ok())
        }
    }
}

#[doc(alias = "IOPMAssertionLevel")]
#[repr(transparent)]
pub struct AssertionLevel(pub u32);

impl AssertionLevel {
    #[doc(alias = "kIOPMAssertionLevelOff")]
    pub const OFF: Self = Self(0);

    #[doc(alias = "kIOPMAssertionLevelOn")]
    pub const ON: Self = Self(255);
}

#[doc(alias = "IOPMUserActiveType")]
pub struct UserActiveType(pub i32);

impl UserActiveType {
    pub const LOCAL: Self = Self(0);
    pub const REMOTE: Self = Self(1);
}

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C-unwind" {
    fn IOPMAssertionCreateWithName(
        assertion_type: &cf::String,
        level: AssertionLevel,
        name: &cf::String,
        assertion: *mut Assertion,
    ) -> io::Return;

    // fn IOPMAssertionRetain(assertion_id: u32);
    fn IOPMAssertionRelease(assertion_id: u32) -> io::Return;

    fn IOPMAssertionCopyProperties(
        assertion_id: u32,
    ) -> arc::R<cf::DictionaryOf<cf::String, cf::Type>>;
}

#[cfg(test)]
mod tests {
    use crate::{cf, io::pm};

    #[test]
    fn basics() {
        let assertion = pm::Assertion::with_name(
            cf::str!(c"No Name"),
            pm::assertions::prevent_user_idle_display_sleep(),
            pm::AssertionLevel::ON,
        )
        .unwrap();

        let _props = assertion.props();
    }
}
