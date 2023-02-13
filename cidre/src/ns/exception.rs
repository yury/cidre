use std::{ffi::c_void, intrinsics::transmute};

use crate::{define_obj_type, ns};

use super::objc_runtime::ExceptionName;

impl ExceptionName {
    /// A generic name for an exception.
    ///
    /// You should typically use a more specific exception name.
    pub fn generic() -> &'static Self {
        unsafe { NSGenericException }
    }

    /// Name of an exception that occurs when attempting to access
    /// outside the bounds of some data, such as beyond the end of a string.
    pub fn range() -> &'static Self {
        unsafe { NSRangeException }
    }

    /// Name of an exception that occurs when you pass an invalid argument
    /// to a method, such as a nil pointer where a non-nil object is required.
    pub fn invalid_argument() -> &'static Self {
        unsafe { NSInvalidArgumentException }
    }

    /// Name of an exception that occurs when an internal assertion fails and
    /// implies an unexpected condition within the called code.
    pub fn internal_inconsistency() -> &'static Self {
        unsafe { NSInternalInconsistencyException }
    }

    /// Obsolete; not currently used.
    pub fn malloc() -> &'static Self {
        unsafe { NSMallocException }
    }

    /// Name of an exception that occurs when a remote object is accessed
    /// from a thread that should not access it.
    ///
    /// See NSConnection’s enableMultipleThreads.
    pub fn object_inaccessible() -> &'static Self {
        unsafe { NSObjectInaccessibleException }
    }

    /// Name of an exception that occurs when the remote side of the NSConnection
    /// refused to send the message to the object because the object has never been
    /// vended.
    pub fn object_not_available() -> &'static Self {
        unsafe { NSObjectNotAvailableException }
    }

    /// Name of an exception that occurs when an internal assertion fails and implies
    /// an unexpected condition within the distributed objects.
    ///
    /// This is a distributed objects–specific exception.
    pub fn destination_invalid() -> &'static Self {
        unsafe { NSDestinationInvalidException }
    }

    /// Name of an exception that occurs when a timeout set on a port expires during
    /// a send or receive operation.
    ///
    /// This is a distributed objects–specific exception.
    pub fn port_timeout() -> &'static Self {
        unsafe { NSPortTimeoutException }
    }

    /// Name of an exception that occurs when the send port of an
    /// NSConnection has become invalid.
    ///
    /// This is a distributed objects–specific exception.
    pub fn invalid_send_port() -> &'static Self {
        unsafe { NSInvalidSendPortException }
    }

    /// Name of an exception that occurs when the receive port of
    /// an NSConnection has become invalid.
    ///
    /// This is a distributed objects–specific exception
    pub fn invalid_receive_port() -> &'static Self {
        unsafe { NSInvalidReceivePortException }
    }

    /// Generic error occurred on send.
    ///
    /// This is an NSPort-specific exception.
    pub fn port_send() -> &'static Self {
        unsafe { NSPortSendException }
    }

    /// Generic error occurred on receive.
    ///
    /// This is an NSPort-specific exception.
    pub fn port_receive() -> &'static Self {
        unsafe { NSPortReceiveException }
    }

    /// No longer used.
    pub fn old_style() -> &'static Self {
        unsafe { NSOldStyleException }
    }

    /// The name of an exception raised by NSArchiver if there are problems
    /// initializing or encoding.
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

define_obj_type!(Exception(ns::Id));

impl Exception {
    pub fn raise(message: &ns::String) -> ! {
        unsafe { cidre_raise_exception(message) }
    }
}

pub fn throw(message: &ns::Id) -> ! {
    unsafe { cidre_throw_exception(message) }
}

pub type UncaughtExceptionHandler = extern "C" fn(exception: &Exception);

pub fn get_uncaought_exception_handler() -> *const UncaughtExceptionHandler {
    unsafe { NSGetUncaughtExceptionHandler() }
}

pub fn set_uncaught_exception_handler(handler: *const UncaughtExceptionHandler) {
    unsafe { NSSetUncaughtExceptionHandler(handler) }
}

extern "C" {
    fn NSGetUncaughtExceptionHandler() -> *const UncaughtExceptionHandler;
    fn NSSetUncaughtExceptionHandler(handler: *const UncaughtExceptionHandler);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn cidre_raise_exception(message: &ns::String) -> !;
    fn cidre_throw_exception(message: &ns::Id) -> !;
    fn cidre_try_catch<'ar>(
        during: extern "C" fn(ctx: *mut c_void),
        ctx: *mut c_void,
    ) -> Option<&'ar ns::Id>;
}

pub fn try_catch_obj<'ar, F, R>(f: F) -> Result<R, &'ar ns::Id>
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
            Some(e) => Err(e),
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::{cf, ns};

    #[test]
    fn test_catch() {
        let x = ns::try_catch(|| 0).expect("result");
        assert_eq!(0, x);
    }

    #[test]
    fn test_exception_catch() {
        let err = ns::try_catch(|| {
            let msg = ns::String::with_str("test");
            ns::exception::Exception::raise(&msg);
        })
        .expect_err("result");

        assert_ne!(cf::String::type_id(), err.as_type_ref().get_type_id());

        println!("{:?} {:?}", err, err.as_type_ref().retain_count());
    }

    #[test]
    fn test_throw_catch() {
        let msg = ns::String::with_str("test");

        let exc = ns::try_catch_obj(|| {
            ns::exception::throw(&msg);
        })
        .expect_err("result");

        assert_eq!(cf::String::type_id(), exc.as_type_ref().get_type_id());
        assert!(msg.is_equal(&exc));

        println!("{:?} {:?}", exc, exc.as_type_ref().retain_count());
    }
}
