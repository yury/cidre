use std::{ffi::c_void, intrinsics::transmute};

use crate::{define_obj_type, ns, objc};

use super::objc_runtime::ExceptionName;

impl ExceptionName {
    /// A generic name for an exception.
    ///
    /// You should typically use a more specific exception name.
    #[doc(alias = "NSGenericException")]
    pub fn generic() -> &'static Self {
        unsafe { NSGenericException }
    }

    /// Name of an exception that occurs when attempting to access
    /// outside the bounds of some data, such as beyond the end of a string.
    #[doc(alias = "NSRangeException")]
    pub fn range() -> &'static Self {
        unsafe { NSRangeException }
    }

    /// Name of an exception that occurs when you pass an invalid argument
    /// to a method, such as a nil pointer where a non-nil object is required.
    #[doc(alias = "NSInvalidArgumentException")]
    pub fn invalid_arg() -> &'static Self {
        unsafe { NSInvalidArgumentException }
    }

    /// Name of an exception that occurs when an internal assertion fails and
    /// implies an unexpected condition within the called code.
    #[doc(alias = "NSInternalInconsistencyException")]
    pub fn internal_inconsistency() -> &'static Self {
        unsafe { NSInternalInconsistencyException }
    }

    /// Obsolete; not currently used.
    #[doc(alias = "NSMallocException")]
    pub fn malloc() -> &'static Self {
        unsafe { NSMallocException }
    }

    /// Name of an exception that occurs when a remote object is accessed
    /// from a thread that should not access it.
    ///
    /// See NSConnection’s enableMultipleThreads.
    #[doc(alias = "NSObjectInaccessibleException")]
    pub fn object_inaccessible() -> &'static Self {
        unsafe { NSObjectInaccessibleException }
    }

    /// Name of an exception that occurs when the remote side of the NSConnection
    /// refused to send the message to the object because the object has never been
    /// vended.
    #[doc(alias = "NSObjectNotAvailableException")]
    pub fn object_not_available() -> &'static Self {
        unsafe { NSObjectNotAvailableException }
    }

    /// Name of an exception that occurs when an internal assertion fails and implies
    /// an unexpected condition within the distributed objects.
    ///
    /// This is a distributed objects–specific exception.
    #[doc(alias = "NSDestinationInvalidException")]
    pub fn destination_invalid() -> &'static Self {
        unsafe { NSDestinationInvalidException }
    }

    /// Name of an exception that occurs when a timeout set on a port expires during
    /// a send or receive operation.
    ///
    /// This is a distributed objects–specific exception.
    #[doc(alias = "NSPortTimeoutException")]
    pub fn port_timeout() -> &'static Self {
        unsafe { NSPortTimeoutException }
    }

    /// Name of an exception that occurs when the send port of an
    /// NSConnection has become invalid.
    ///
    /// This is a distributed objects–specific exception.
    #[doc(alias = "NSInvalidSendPortException")]
    pub fn invalid_send_port() -> &'static Self {
        unsafe { NSInvalidSendPortException }
    }

    /// Name of an exception that occurs when the receive port of
    /// an NSConnection has become invalid.
    ///
    /// This is a distributed objects–specific exception
    #[doc(alias = "NSInvalidReceivePortException")]
    pub fn invalid_receive_port() -> &'static Self {
        unsafe { NSInvalidReceivePortException }
    }

    /// Generic error occurred on send.
    ///
    /// This is an NSPort-specific exception.
    #[doc(alias = "NSPortSendException")]
    pub fn port_send() -> &'static Self {
        unsafe { NSPortSendException }
    }

    /// Generic error occurred on receive.
    ///
    /// This is an NSPort-specific exception.
    #[doc(alias = "NSPortReceiveException")]
    pub fn port_receive() -> &'static Self {
        unsafe { NSPortReceiveException }
    }

    /// No longer used.
    #[doc(alias = "NSOldStyleException")]
    pub fn old_style() -> &'static Self {
        unsafe { NSOldStyleException }
    }

    /// The name of an exception raised by NSArchiver if there are problems
    /// initializing or encoding.
    #[doc(alias = "NSInconsistentArchiveException")]
    pub fn inconsisten_archive() -> &'static Self {
        unsafe { NSInconsistentArchiveException }
    }
}

extern "C" {
    static NSGenericException: &'static ExceptionName;
    static NSRangeException: &'static ExceptionName;
    static NSInvalidArgumentException: &'static ExceptionName;
    static NSInternalInconsistencyException: &'static ExceptionName;
    static NSMallocException: &'static ExceptionName;
    static NSObjectInaccessibleException: &'static ExceptionName;
    static NSObjectNotAvailableException: &'static ExceptionName;
    static NSDestinationInvalidException: &'static ExceptionName;
    static NSPortTimeoutException: &'static ExceptionName;
    static NSInvalidSendPortException: &'static ExceptionName;
    static NSInvalidReceivePortException: &'static ExceptionName;
    static NSPortSendException: &'static ExceptionName;
    static NSPortReceiveException: &'static ExceptionName;
    static NSOldStyleException: &'static ExceptionName;
    static NSInconsistentArchiveException: &'static ExceptionName;
}

define_obj_type!(pub Exception(ns::Id));

impl Exception {
    pub fn raise(message: &ns::String) -> ! {
        unsafe { cidre_raise_exception(message) }
    }

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::ExceptionName;

    #[objc::msg_send(reason)]
    pub fn reason(&self) -> Option<&ns::String>;

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<&ns::Dictionary<ns::Id, ns::Id>>;
}

pub type UncaughtExceptionHandler = extern "C" fn(exception: &Exception);

pub fn uncaught_exception_handler() -> *const UncaughtExceptionHandler {
    unsafe { NSGetUncaughtExceptionHandler() }
}

pub unsafe fn set_uncaught_exception_handler(handler: *const UncaughtExceptionHandler) {
    unsafe { NSSetUncaughtExceptionHandler(handler) }
}

extern "C" {
    fn NSGetUncaughtExceptionHandler() -> *const UncaughtExceptionHandler;
    fn NSSetUncaughtExceptionHandler(handler: *const UncaughtExceptionHandler);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn cidre_raise_exception(message: &ns::String) -> !;
    fn cidre_try_catch<'ar>(
        during: extern "C" fn(ctx: *mut c_void),
        ctx: *mut c_void,
    ) -> Option<&'ar ns::Id>;
}

#[inline]
fn type_helper<F>(_t: &Option<F>) -> extern "C" fn(t: &mut Option<F>)
where
    F: FnOnce(),
{
    extern "C" fn during<F>(f: &mut Option<F>)
    where
        F: FnOnce(),
    {
        unsafe { f.take().unwrap_unchecked()() };
    }
    during
}

pub fn try_catch<'ar, F, R>(f: F) -> Result<R, &'ar ns::Exception>
where
    F: FnOnce() -> R,
{
    let mut result = None;
    let mut wrapper = Some(|| result = Some(f()));

    let f = type_helper(&wrapper);
    let ctx = &mut wrapper as *mut _ as *mut c_void;

    unsafe {
        match cidre_try_catch(transmute(f), ctx) {
            None => Ok(result.unwrap_unchecked()),
            Some(e) => Err(transmute(e)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{cf, ns, objc};

    #[test]
    fn catch() {
        let x = ns::try_catch(|| 0).expect("result");
        assert_eq!(0, x);
    }

    #[test]
    fn ns_exception_catch() {
        let reason = ns::String::with_str("test");
        let ex = ns::try_catch(|| ns::Exception::raise(&reason)).expect_err("result");

        assert!(ex.user_info().is_none());
        assert!(ex.name().eq(ns::ExceptionName::generic()));
        assert!(ex.reason().unwrap().eq(&reason));

        assert_ne!(cf::String::type_id(), ex.as_type_ref().get_type_id());

        println!("{:?} {:?}", ex, ex.as_type_ref().retain_count());
    }

    #[test]
    fn objc_throw_catch() {
        let msg = ns::String::with_str("this is longer string so it is not tagged ptr");

        let exc = objc::try_catch(|| objc::throw(&msg)).expect_err("result");

        assert_eq!(cf::String::type_id(), exc.as_type_ref().get_type_id());
        assert!(msg.is_equal(&exc));

        println!("{:?} {:?}", exc, exc.as_type_ref().retain_count());
    }
}
