use crate::{arc, cf, define_cls, define_obj_type, ns, objc};

impl<'ear> From<&'ear ns::Error> for ns::ExErr<'ear> {
    fn from(value: &'ear ns::Error) -> Self {
        Self::Err(value)
    }
}

pub fn if_false<'ear, F>(f: F) -> ns::Result<'ear>
where
    F: FnOnce(*mut Option<&'ear ns::Error>) -> bool,
{
    let mut err = None;
    if f(&mut err) {
        Ok(())
    } else {
        unsafe { Err(err.unwrap_unchecked()) }
    }
}

pub fn if_none<'ear, F, R>(f: F) -> Result<R, &'ear ns::Error>
where
    F: FnOnce(*mut Option<&'ear ns::Error>) -> Option<R>,
{
    let mut err = None;
    f(&mut err).ok_or_else(|| unsafe { err.unwrap_unchecked() })
}

pub fn if_err<'ear, F>(f: F) -> ns::Result<'ear>
where
    F: FnOnce(*mut Option<&'ear ns::Error>),
{
    let mut err = None;
    f(&mut err);
    if let Some(err) = err {
        Err(err)
    } else {
        Ok(())
    }
}

define_obj_type!(
    #[doc(alias = "NSError")]
    pub Error(ns::Id)
);

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.localized_description();
        desc.fmt(f)
    }
}

impl arc::A<Error> {
    #[objc::msg_send(initWithDomain:code:userInfo:)]
    pub fn init_with_domain(
        self,
        domain: &ns::ErrorDomain,
        code: ns::Integer,
        user_info: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Error>;
}

impl Error {
    define_cls!(NS_ERROR);

    #[objc::msg_send(code)]
    pub fn code(&self) -> ns::Integer;

    #[objc::msg_send(domain)]
    pub fn domain(&self) -> &ns::ErrorDomain;

    #[inline]
    pub fn with_domain(
        domain: &ns::ErrorDomain,
        code: ns::Integer,
        user_info: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_domain(domain, code, user_info)
    }

    #[objc::msg_send(localizedDescription)]
    pub fn localized_description(&self) -> arc::R<ns::String>;

    #[objc::msg_send(localizedRecoveryOptions)]
    pub fn localized_recovery_options(&self) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(localizedRecoverySuggestion)]
    pub fn localized_recovery_suggestion(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(localizedFailureReason)]
    pub fn localized_failure_reason(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> arc::R<ns::Dictionary<ns::String, ns::Id>>;

    /// Toll-Free Bridged
    #[inline]
    pub fn as_cf(&self) -> &cf::Error {
        unsafe { std::mem::transmute(self) }
    }
}

define_obj_type!(
    #[doc(alias = "NSErrorDomain")]
    pub Domain(ns::String)
);

impl Domain {
    #[inline]
    pub fn cocoa() -> &'static Self {
        unsafe { NSCocoaErrorDomain }
    }

    #[inline]
    pub fn posix() -> &'static Self {
        unsafe { NSPOSIXErrorDomain }
    }

    #[inline]
    pub fn os_status() -> &'static Self {
        unsafe { NSOSStatusErrorDomain }
    }

    #[inline]
    pub fn mach() -> &'static Self {
        unsafe { NSMachErrorDomain }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_ERROR: &'static objc::Class<Error>;
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {

    static NSCocoaErrorDomain: &'static Domain;
    static NSPOSIXErrorDomain: &'static Domain;
    static NSOSStatusErrorDomain: &'static Domain;
    static NSMachErrorDomain: &'static Domain;
}
