use std::io::ErrorKind;

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

impl ns::Copying for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.localized_desc();
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
    pub fn domain(&self) -> arc::R<ns::ErrorDomain>;

    #[inline]
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
        Self::alloc().init_with_domain(ns::ErrorDomain::posix(), code, user_info)
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

impl From<crate::os::Error> for arc::R<Error> {
    fn from(value: crate::os::Error) -> Self {
        ns::Error::with_domain(ns::ErrorDomain::os_status(), value.0.get() as _, None)
    }
}

impl From<ErrorKind> for arc::R<Error> {
    fn from(value: ErrorKind) -> Self {
        ns::Error::with_posix(posix_code(value), None)
    }
}

impl From<std::io::Error> for arc::R<Error> {
    fn from(value: std::io::Error) -> Self {
        if let Some(code) = value.raw_os_error() {
            return ns::Error::with_posix(code as _, None);
        }

        let code = posix_code(value.kind());

        let debug_desc = format!("{value:?}");
        let debug_desc = ns::String::with_str(&debug_desc);

        let user_info = ns::Dictionary::with_keys_values(
            &[ns::error_user_info_keys::debug_desc()],
            &[debug_desc.as_id_ref()],
        );

        ns::Error::with_posix(code, Some(user_info.as_ref()))
    }
}

fn posix_code(value: ErrorKind) -> isize {
    // http://fxr.watson.org/fxr/source/sys/errno.h

    let code = match value {
        ErrorKind::NotFound => 2,                               // ENOENT,
        ErrorKind::PermissionDenied => 13,                      // EACCES,
        ErrorKind::ConnectionRefused => 61,                     // ECONNREFUSED,
        ErrorKind::ConnectionReset => 54,                       // ECONNRESET,
        ErrorKind::HostUnreachable => 65,                       // EHOSTUNREACH
        ErrorKind::NetworkUnreachable => 51,                    // ENETUNREACH
        ErrorKind::ConnectionAborted => 53,                     // ECONNABORTED
        ErrorKind::NotConnected => 57,                          // ENOTCONN
        ErrorKind::AddrInUse => 48,                             // EADDRINUSE
        ErrorKind::AddrNotAvailable => 49,                      // EADDRNOTAVAIL
        ErrorKind::NetworkDown => 50,                           // ENETDOWN
        ErrorKind::BrokenPipe => 32,                            // EPIPE
        ErrorKind::AlreadyExists => 17,                         // EEXIST
        ErrorKind::WouldBlock => 35,                            // EWOULDBLOCK
        ErrorKind::NotADirectory => 20,                         // ENOTDIR
        ErrorKind::IsADirectory => 21,                          // EISDIR
        ErrorKind::DirectoryNotEmpty => 66,                     // ENOTEMPTY
        ErrorKind::ReadOnlyFilesystem => 30,                    // EROFS
        ErrorKind::StaleNetworkFileHandle => 70,                // ESTALE
        ErrorKind::InvalidInput | ErrorKind::InvalidData => 22, // EINVAL
        ErrorKind::TimedOut => 60,                              // ETIMEDOUT
        ErrorKind::StorageFull => 28,                           // ENOSPC
        ErrorKind::NotSeekable => 29,                           // ESPIPE
        ErrorKind::QuotaExceeded => 69,                         // EDQUOT
        ErrorKind::FileTooLarge => 27,                          // EFBIG
        ErrorKind::ResourceBusy => 16,                          // EBUSY
        ErrorKind::ExecutableFileBusy => 26,                    // ETXTBSY
        ErrorKind::Deadlock => 11,                              // EDEADLK
        ErrorKind::CrossesDevices => 18,                        // EXDEV
        ErrorKind::TooManyLinks => 31,                          // EMLINK
        ErrorKind::InvalidFilename => 63,                       // ENAMETOOLONG
        ErrorKind::ArgumentListTooLong => 7,                    // E2BIG
        ErrorKind::Interrupted => 4,                            // EINTR
        ErrorKind::Unsupported => 78,                           // ENOSYS
        ErrorKind::OutOfMemory => 12,                           // ENOMEM
        //
        // ErrorKind::InProgress => 36, // EINPROGRESS
        // ErrorKind::Other => todo!(),
        // ErrorKind::UnexpectedEof => todo!(),
        // ErrorKind::WriteZero => todo!(),
        _ => 0, // realy don't want to use zero here
    };
    code
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
    }
}
