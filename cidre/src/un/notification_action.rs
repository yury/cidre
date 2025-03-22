use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "UNNotificationActionOptions")]
    pub NotificationActionOpts(usize)
);

impl NotificationActionOpts {
    #[doc(alias = "UNNotificationActionOptionNone")]
    pub const NONE: Self = Self(0);

    /// Whether this action should require unlocking before being performed.
    #[doc(alias = "UNNotificationActionOptionAuthenticationRequired")]
    pub const AUTHENTICATION_REQUIRED: Self = Self(1 << 0);

    /// Whether this action should be indicated as destructive.
    #[doc(alias = "UNNotificationActionOptionDestructive")]
    pub const DESTRUCTIVE: Self = Self(1 << 1);

    /// Whether this action should cause the application to launch in the foreground.
    #[doc(alias = "UNNotificationActionOptionForeground")]
    pub const FOREGROUND: Self = Self(1 << 2);
}

define_obj_type!(
    #[doc(alias = "UNNotificationAction")]
    pub NotificationAction(ns::Id)
);

impl NotificationAction {
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(options)]
    pub fn options(&self) -> NotificationActionOpts;
}

#[cfg(not(target_os = "tvos"))]
define_obj_type!(
    #[doc(alias = "UNTextInputNotificationAction")]
    pub TextInputNotificationAction(NotificationAction)
);

#[cfg(not(target_os = "tvos"))]
impl TextInputNotificationAction {
    #[objc::msg_send(textInputButtonTitle)]
    pub fn text_input_button_title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(textInputPlaceholder)]
    pub fn text_input_placeholder(&self) -> arc::R<ns::String>;
}
