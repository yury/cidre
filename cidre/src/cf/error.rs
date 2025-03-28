use crate::{arc, cf, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

#[inline]
pub fn if_false<F>(f: F) -> Result<(), arc::R<Error>>
where
    F: FnOnce(*mut arc::R<Error>) -> bool,
{
    let mut err = std::mem::MaybeUninit::uninit();
    if f(err.as_mut_ptr()) {
        Ok(())
    } else {
        unsafe { Err(err.assume_init()) }
    }
}

#[inline]
pub fn if_none<R, F>(f: F) -> Result<R, arc::R<Error>>
where
    F: FnOnce(*mut arc::R<Error>) -> Option<R>,
{
    let mut err = std::mem::MaybeUninit::uninit();
    f(err.as_mut_ptr()).ok_or_else(|| unsafe { err.assume_init() })
}

#[inline]
pub fn if_none_maybe<R, F>(f: F) -> Result<R, Option<arc::R<Error>>>
where
    F: FnOnce(*mut arc::R<Error>) -> Option<R>,
{
    let mut err = None;
    f(unsafe { std::mem::transmute(&mut err) }).ok_or(err)
}

define_cf_type!(
    #[doc(alias = "CFErrorRef")]
    Error(cf::Type)
);

unsafe impl Send for Error {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.desc();
        s.fmt(f)
    }
}

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
    pub fn desc(&self) -> arc::R<cf::String> {
        unsafe { CFErrorCopyDescription(self) }
    }

    #[inline]
    pub fn failure_reason(&self) -> Option<arc::R<cf::String>> {
        unsafe { CFErrorCopyFailureReason(self) }
    }

    #[inline]
    pub fn recovery_suggestion(&self) -> Option<arc::R<cf::String>> {
        unsafe { CFErrorCopyRecoverySuggestion(self) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::Error {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    static kCFErrorDomainPOSIX: &'static Domain;
    static kCFErrorDomainOSStatus: &'static Domain;
    static kCFErrorDomainMach: &'static Domain;
    static kCFErrorDomainCocoa: &'static Domain;

    fn CFErrorGetDomain(err: &Error) -> &Domain;
    fn CFErrorGetCode(err: &Error) -> cf::Index;
    fn CFErrorCopyDescription(err: &Error) -> arc::R<cf::String>;
    fn CFErrorCopyFailureReason(err: &Error) -> Option<arc::R<cf::String>>;
    fn CFErrorCopyRecoverySuggestion(err: &Error) -> Option<arc::R<cf::String>>;
}
