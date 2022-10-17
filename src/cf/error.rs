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
    #[inline]
    pub fn domain(&self) -> &Domain {
        unsafe { CFErrorGetDomain(self) }
    }

    #[inline]
    pub fn code(&self) -> cf::Index {
        unsafe { CFErrorGetCode(self) }
    }

    #[inline]
    pub fn description(&self) -> Retained<cf::String> {
        unsafe { CFErrorCopyDescription(self) }
    }

    #[inline]
    pub fn failure_reason(&self) -> Option<Retained<cf::String>> {
        unsafe { CFErrorCopyFailureReason(self) }
    }

    #[inline]
    pub fn recovery_suggestion(&self) -> Option<Retained<cf::String>> {
        unsafe { CFErrorCopyRecoverySuggestion(self) }
    }
}

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
