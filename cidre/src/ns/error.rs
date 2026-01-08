use std::io::ErrorKind;

use crate::{arc, define_cls, define_obj_type, ns, objc, os};

#[cfg(feature = "cf")]
use crate::cf;

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

pub fn if_err<'ear>(f: impl FnOnce(*mut Option<&'ear ns::Error>)) -> ns::Result<'ear> {
    let mut err = None;
    f(&mut err);
    err.map_or(Ok(()), Err)
}

define_obj_type!(
    #[doc(alias = "NSError")]
    pub Error(ns::Id)
);

impl ns::Copying for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.localized_desc().fmt(f)
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
    pub fn domain(&self) -> arc::R<ns::ErrorDomain>;

    #[inline]
    #[cold]
    pub fn with_domain(
        domain: &ns::ErrorDomain,
        code: ns::Integer,
        user_info: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_domain(domain, code, user_info)
    }

    #[inline]
    pub fn with_posix(
        code: ns::Integer,
        user_info: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Self> {
        Self::with_domain(ns::ErrorDomain::posix(), code, user_info)
    }

    #[inline]
    pub fn with_os_status(
        code: ns::Integer,
        user_info: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Self> {
        Self::with_domain(ns::ErrorDomain::os_status(), code, user_info)
    }

    #[objc::msg_send(localizedDescription)]
    pub fn localized_desc(&self) -> arc::R<ns::String>;

    #[objc::msg_send(localizedRecoveryOptions)]
    pub fn localized_recovery_opts(&self) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(localizedRecoverySuggestion)]
    pub fn localized_recovery_suggestion(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(localizedFailureReason)]
    pub fn localized_failure_reason(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> arc::R<ns::Dictionary<ns::String, ns::Id>>;

    /// Toll-Free Bridged
    #[cfg(feature = "cf")]
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
    pub fn with_static(str: &'static ns::String) -> &'static Self {
        unsafe { std::mem::transmute(str) }
    }

    #[doc(alias = "NSCocoaErrorDomain")]
    #[inline]
    pub fn cocoa() -> &'static Self {
        unsafe { NSCocoaErrorDomain }
    }

    #[doc(alias = "NSPOSIXErrorDomain")]
    #[inline]
    pub fn posix() -> &'static Self {
        unsafe { NSPOSIXErrorDomain }
    }

    #[doc(alias = "NSOSStatusErrorDomain")]
    #[inline]
    pub fn os_status() -> &'static Self {
        unsafe { NSOSStatusErrorDomain }
    }

    #[doc(alias = "NSMachErrorDomain")]
    #[inline]
    pub fn mach() -> &'static Self {
        unsafe { NSMachErrorDomain }
    }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_ERROR: &'static objc::Class<Error>;
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSCocoaErrorDomain: &'static Domain;
    static NSPOSIXErrorDomain: &'static Domain;
    static NSOSStatusErrorDomain: &'static Domain;
    static NSMachErrorDomain: &'static Domain;
}

/// UserInfo Keys
pub mod user_info_keys {
    use crate::ns;

    #[doc(alias = "NSUnderlyingErrorKey")]
    #[inline]
    pub fn underling_err() -> &'static ns::String {
        unsafe { NSUnderlyingErrorKey }
    }

    #[doc(alias = "NSMultipleUnderlyingErrorsKey")]
    #[inline]
    pub fn underlying_errs() -> &'static ns::String {
        unsafe { NSMultipleUnderlyingErrorsKey }
    }

    #[doc(alias = "NSLocalizedDescriptionKey")]
    #[inline]
    pub fn localized_desc() -> &'static ns::String {
        unsafe { NSLocalizedDescriptionKey }
    }

    #[doc(alias = "NSLocalizedFailureReasonErrorKey")]
    #[inline]
    pub fn localized_failure_reason() -> &'static ns::String {
        unsafe { NSLocalizedFailureReasonErrorKey }
    }

    #[doc(alias = "NSLocalizedRecoverySuggestionErrorKey")]
    #[inline]
    pub fn localized_recovery_suggestion() -> &'static ns::String {
        unsafe { NSLocalizedRecoverySuggestionErrorKey }
    }

    #[doc(alias = "NSLocalizedRecoveryOptionsErrorKey")]
    #[inline]
    pub fn localized_recovery_options() -> &'static ns::String {
        unsafe { NSLocalizedRecoveryOptionsErrorKey }
    }

    #[doc(alias = "NSRecoveryAttempterErrorKey")]
    #[inline]
    pub fn recovery_attempter() -> &'static ns::String {
        unsafe { NSRecoveryAttempterErrorKey }
    }

    #[doc(alias = "NSHelpAnchorErrorKey")]
    #[inline]
    pub fn help_anchor() -> &'static ns::String {
        unsafe { NSHelpAnchorErrorKey }
    }

    #[doc(alias = "NSDebugDescriptionErrorKey")]
    #[inline]
    pub fn debug_desc() -> &'static ns::String {
        unsafe { NSDebugDescriptionErrorKey }
    }

    #[doc(alias = "NSLocalizedFailureErrorKey")]
    #[inline]
    pub fn localized_failure() -> &'static ns::String {
        unsafe { NSLocalizedFailureErrorKey }
    }

    #[doc(alias = "NSStringEncodingErrorKey")]
    #[inline]
    pub fn string_encoding() -> &'static ns::String {
        unsafe { NSStringEncodingErrorKey }
    }

    #[doc(alias = "NSURLErrorKey")]
    #[inline]
    pub fn url() -> &'static ns::String {
        unsafe { NSURLErrorKey }
    }

    #[doc(alias = "NSFilePathErrorKey")]
    #[inline]
    pub fn file_path() -> &'static ns::String {
        unsafe { NSFilePathErrorKey }
    }

    #[link(name = "Foundation", kind = "framework")]
    unsafe extern "C" {
        static NSUnderlyingErrorKey: &'static ns::String;
        static NSMultipleUnderlyingErrorsKey: &'static ns::String;
        static NSLocalizedDescriptionKey: &'static ns::String;
        static NSLocalizedFailureReasonErrorKey: &'static ns::String;
        static NSLocalizedRecoverySuggestionErrorKey: &'static ns::String;
        static NSLocalizedRecoveryOptionsErrorKey: &'static ns::String;
        static NSRecoveryAttempterErrorKey: &'static ns::String;
        static NSHelpAnchorErrorKey: &'static ns::String;
        static NSDebugDescriptionErrorKey: &'static ns::String;
        static NSLocalizedFailureErrorKey: &'static ns::String;
        static NSStringEncodingErrorKey: &'static ns::String;
        static NSURLErrorKey: &'static ns::String;
        static NSFilePathErrorKey: &'static ns::String;
    }
}

// for anyhow
impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for arc::R<Error> {
    #[cold]
    fn from(value: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        if let Some(err) = value.downcast_ref::<std::io::Error>() {
            return err.into();
        }
        let debug_desc = format!("{value:?}");
        let debug_desc = ns::String::with_str(&debug_desc);

        let user_info = ns::Dictionary::with_keys_values(
            &[ns::error_user_info_keys::debug_desc()],
            &[debug_desc.as_id_ref()],
        );

        ns::Error::with_posix(0, Some(user_info.as_ref()))
    }
}

impl From<crate::os::Error> for arc::R<Error> {
    #[cold]
    fn from(value: crate::os::Error) -> Self {
        ns::Error::with_domain(ns::ErrorDomain::os_status(), value.0.get() as _, None)
    }
}

impl From<ErrorKind> for arc::R<Error> {
    #[cold]
    fn from(value: ErrorKind) -> Self {
        ns::Error::with_posix(posix_code(value).0.get() as isize, None)
    }
}

impl From<std::io::Error> for arc::R<Error> {
    #[cold]
    fn from(value: std::io::Error) -> Self {
        (&value).into()
    }
}

impl From<&std::io::Error> for arc::R<Error> {
    #[cold]
    fn from(value: &std::io::Error) -> Self {
        if let Some(code) = value.raw_os_error() {
            return ns::Error::with_posix(code as _, None);
        }

        let code = posix_code(value.kind()).0.get() as isize;

        let debug_desc = format!("{value:?}");
        let debug_desc = ns::String::with_str(&debug_desc);

        let user_info = ns::Dictionary::with_keys_values(
            &[ns::error_user_info_keys::debug_desc()],
            &[debug_desc.as_id_ref()],
        );

        ns::Error::with_posix(code, Some(user_info.as_ref()))
    }
}

fn posix_code(value: ErrorKind) -> os::Error {
    // http://fxr.watson.org/fxr/source/sys/errno.h

    use crate::sys::errno;

    match value {
        ErrorKind::NotFound => errno::ENOENT,
        ErrorKind::PermissionDenied => errno::EACCES,
        ErrorKind::ConnectionRefused => errno::ECONNREFUSED,
        ErrorKind::ConnectionReset => errno::ECONNRESET,
        ErrorKind::HostUnreachable => errno::EHOSTUNREACH,
        ErrorKind::NetworkUnreachable => errno::ENETUNREACH,
        ErrorKind::ConnectionAborted => errno::ECONNABORTED,
        ErrorKind::NotConnected => errno::ENOTCONN,
        ErrorKind::AddrInUse => errno::EADDRINUSE,
        ErrorKind::AddrNotAvailable => errno::EADDRNOTAVAIL,
        ErrorKind::NetworkDown => errno::ENETDOWN,
        ErrorKind::BrokenPipe => errno::EPIPE,
        ErrorKind::AlreadyExists => errno::EEXIST,
        ErrorKind::WouldBlock => errno::EWOULDBLOCK,
        ErrorKind::NotADirectory => errno::ENOTDIR,
        ErrorKind::IsADirectory => errno::EISDIR,
        ErrorKind::DirectoryNotEmpty => errno::ENOTEMPTY,
        ErrorKind::ReadOnlyFilesystem => errno::EROFS,
        ErrorKind::StaleNetworkFileHandle => errno::ESTALE,
        ErrorKind::InvalidInput | ErrorKind::InvalidData => errno::EINVAL,
        ErrorKind::TimedOut => errno::ETIMEDOUT,
        ErrorKind::StorageFull => errno::ENOSPC,
        ErrorKind::NotSeekable => errno::ESPIPE,
        ErrorKind::QuotaExceeded => errno::EDQUOT,
        ErrorKind::FileTooLarge => errno::EFBIG,
        ErrorKind::ResourceBusy => errno::EBUSY,
        ErrorKind::ExecutableFileBusy => errno::ETXTBSY,
        ErrorKind::Deadlock => errno::EDEADLK,
        ErrorKind::CrossesDevices => errno::EXDEV,
        ErrorKind::TooManyLinks => errno::EMLINK,
        ErrorKind::InvalidFilename => errno::ENAMETOOLONG,
        ErrorKind::ArgumentListTooLong => errno::E2BIG,
        ErrorKind::Interrupted => errno::EINTR,
        ErrorKind::Unsupported => errno::ENOSYS,
        ErrorKind::OutOfMemory => errno::ENOMEM,
        // ErrorKind::FilesystemLoop => todo!(),
        // ErrorKind::WriteZero => todo!(),
        // ErrorKind::UnexpectedEof => todo!(),
        // ErrorKind::InProgress => todo!(),
        // ErrorKind::Other => todo!(),
        // _ => todo!(),
        _ => os::Error::new_unchecked(errno::ELAST.0.get() + 1),
    }
}

#[cfg(test)]
mod tests {
    use crate::{arc, ns, objc::Obj};

    #[test]
    fn basics() {
        let domain = ns::ErrorDomain::with_static(ns::str!(c"cidre"));
        let error = ns::Error::with_domain(domain, 36, None);
        let user_info = error.user_info();
        assert!(user_info.is_empty());
    }

    #[test]
    fn into() {
        let error: arc::R<_> = std::io::Error::from_raw_os_error(20).into();
        assert_eq!(error.code(), 20);
        assert_eq!(error.domain().as_ref(), ns::ErrorDomain::posix());

        let error = std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "oh no!");
        let error_debug_str = format!("{error:?}");
        let error: arc::R<_> = error.into();
        assert_eq!(error.code(), 53);
        assert_eq!(error.domain().as_ref(), ns::ErrorDomain::posix());
        let debug_desc = error
            .user_info()
            .value_for_key(ns::error_user_info_keys::debug_desc())
            .unwrap()
            .unwrap();

        let debug_desc = debug_desc.try_cast(ns::String::cls()).unwrap();
        assert_eq!(debug_desc, error_debug_str.as_str());

        let error: Box<dyn std::error::Error + Send + Sync + 'static> =
            Box::new(std::io::Error::from_raw_os_error(20));
        let error: arc::R<ns::Error> = error.into();
        assert_eq!(error.code(), 20);

        let error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
        let _error: arc::R<ns::Error> = error.into();
    }
}
