use super::Id;

/// Processes an object before the Objective-C runtime throws it. May return a replacement.
#[doc(alias = "objc_exception_preprocessor")]
pub type ExceptionPreprocessor = extern "C" fn(exception: &Id) -> &Id;

/// Installs a process-wide exception preprocessor and returns the previous one.
///
/// # Safety
///
/// The callback must not panic or throw. Call the previous preprocessor to preserve the
/// runtime's exception preparation, including Foundation's original throw stack.
#[doc(alias = "objc_setExceptionPreprocessor")]
pub unsafe fn set_exception_preprocessor(callback: ExceptionPreprocessor) -> ExceptionPreprocessor {
    unsafe { objc_setExceptionPreprocessor(callback) }
}

unsafe extern "C" {
    fn objc_setExceptionPreprocessor(callback: ExceptionPreprocessor) -> ExceptionPreprocessor;
}
