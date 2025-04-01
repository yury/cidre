pub type Error = crate::os::Status;

pub mod err {
    use crate::os::Error;

    /// A system error occurred, such as the failure to allocate an object.
    #[doc(alias = "kAXErrorFailure")]
    pub const FAILURE: Error = Error::new_unchecked(-25200);

    /// An illegal argument was passed to the function
    #[doc(alias = "kAXErrorIllegalArgument")]
    pub const ILLEGAL_ARGUMENT: Error = Error::new_unchecked(-25201);

    /// The ax::UiElement ref passed to the function is invalid.
    #[doc(alias = "kAXErrorInvalidUIElement")]
    pub const INVALID_UI_ELEMENT: Error = Error::new_unchecked(-25202);

    /// The ax::Observer ref passed to the function is not a valid observer.
    #[doc(alias = "kAXErrorInvalidUIElementObserver")]
    pub const INVALID_UI_ELEMENT_OBSERVER: Error = Error::new_unchecked(-25203);

    /// The function cannot complete because messaging failed in some way or because the application with which the function is communicating is busy or unresponsive.
    #[doc(alias = "kAXErrorCannotComplete")]
    pub const CANNOT_COMPLETE: Error = Error::new_unchecked(-25204);

    /// The attribute is not supported by the ax::UiElement ref.
    #[doc(alias = "kAXErrorAttributeUnsupported")]
    pub const ATTR_UNSUPPORTED: Error = Error::new_unchecked(-25205);

    /// The action is not supported by the ax::UiElement ref.
    #[doc(alias = "kAXErrorActionUnsupported")]
    pub const ACTION_UNSUPPORTED: Error = Error::new_unchecked(-25206);

    /// The notification is not supported by the ax::UiElement ref.
    #[doc(alias = "kAXErrorNotificationUnsupported")]
    pub const NOTIFICATION_UNSUPPORTED: Error = Error::new_unchecked(-25207);

    /// Indicates that the function or method is not implemented (this can be returned if a process does not support the accessibility API).
    #[doc(alias = "kAXErrorNotImplemented")]
    pub const NOT_IMPLEMENTED: Error = Error::new_unchecked(-25208);

    /// This notification has already been registered for.
    #[doc(alias = "kAXErrorNotificationAlreadyRegistered")]
    pub const NOTIFICATION_ALREADY_REGISTERED: Error = Error::new_unchecked(-25209);

    /// Indicates that a notification is not registered yet.
    #[doc(alias = "kAXErrorNotificationNotRegistered")]
    pub const NOTIFICATION_NOT_REGISTERED: Error = Error::new_unchecked(-25210);

    /// The accessibility API is disabled (as when, for example, the user deselects "Enable access for assistive devices" in Universal Access Preferences).
    #[doc(alias = "kAXErrorAPIDisabled")]
    pub const API_DISABLED: Error = Error::new_unchecked(-25211);

    /// The requested value or AXUIElementRef does not exist.
    #[doc(alias = "kAXErrorNoValue")]
    pub const NO_VALUE: Error = Error::new_unchecked(-25212);

    /// The parameterized attribute is not supported by the ax::UiElement ref.
    #[doc(alias = "kAXErrorParameterizedAttributeUnsupported")]
    pub const PARAMETERIZED_ATTR_UNSUPPORTED: Error = Error::new_unchecked(-25213);

    /// Not enough precision.
    #[doc(alias = "kAXErrorNotEnoughPrecision")]
    pub const NOT_ENOUGH_PRECISION: Error = Error::new_unchecked(-25214);
}
