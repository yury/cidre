use std::fmt::Debug;

use crate::{cf, define_cf_type};

use super::Retained;

define_cf_type!(Error(cf::Type));
pub type Domain = cf::String;

impl Domain {
    #[inline]
    pub fn posix() -> &'static Domain {
        unsafe { kCFErrorDomainPOSIX }
    }

    #[inline]
    pub fn os_status() -> &'static Domain {
        unsafe { kCFErrorDomainOSStatus }
    }

    #[inline]
    pub fn mach() -> &'static Domain {
        unsafe { kCFErrorDomainMach }
    }

    #[inline]
    pub fn cocoa() -> &'static Domain {
        unsafe { kCFErrorDomainCocoa }
    }
}

impl Error {
    pub fn get_domain(&self) -> &Domain {
        unsafe { CFErrorGetDomain(self) }
    }

    pub fn get_code(&self) -> cf::Index {
        unsafe { CFErrorGetCode(self) }
    }

    pub fn copy_description(&self) -> Retained<cf::String> {
        unsafe { CFErrorCopyDescription(self) }
    }

    pub fn copy_failure_reason(&self) -> Option<Retained<cf::String>> {
        unsafe { CFErrorCopyFailureReason(self) }
    }

    pub fn copy_recovery_suggestion(&self) -> Option<Retained<cf::String>> {
        unsafe { CFErrorCopyRecoverySuggestion(self) }
    }
}

// impl Debug for Error {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let desc: &cf::String = &self.copy_description();
//         fmt.debug_struct("Error")
//             .field("code", &self.get_code())
//             .field("domain", &Cow::from(self.get_domain()))
//             .field("desc", &Cow::from(desc))
//             .finish()
//     }
// }

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFErrorDomainPOSIX: &'static Domain;
    static kCFErrorDomainOSStatus: &'static Domain;
    static kCFErrorDomainMach: &'static Domain;
    static kCFErrorDomainCocoa: &'static Domain;

    fn CFErrorGetDomain(err: &Error) -> &Domain;
    fn CFErrorGetCode(err: &Error) -> cf::Index;
    fn CFErrorCopyDescription(err: &Error) -> Retained<cf::String>;
    fn CFErrorCopyFailureReason(err: &Error) -> Option<Retained<cf::String>>;
    fn CFErrorCopyRecoverySuggestion(err: &Error) -> Option<Retained<cf::String>>;
}
