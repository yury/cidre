use crate::{arc, cf, define_cls, define_obj_type, ns, objc};

impl<'ear> From<&'ear ns::Error> for ns::ExErr<'ear> {
    fn from(value: &'ear ns::Error) -> Self {
        Self::Err(value)
    }
}

pub fn if_false<'ear, F>(f: F) -> Result<(), &'ear ns::Error>
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
    if let Some(r) = f(&mut err) {
        Ok(r)
    } else {
        unsafe { Err(err.unwrap_unchecked()) }
    }
}

define_obj_type!(
    #[doc(alias = "NSError")]
    pub Error(ns::Id)
);

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

    static NSCocoaErrorDomain: &'static Domain;
    static NSPOSIXErrorDomain: &'static Domain;
    static NSOSStatusErrorDomain: &'static Domain;
    static NSMachErrorDomain: &'static Domain;
}
