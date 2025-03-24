use crate::define_opts;
#[cfg(not(target_os = "tvos"))]
use crate::{arc, define_cls, define_obj_type, ns, objc, un};

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

#[cfg(not(target_os = "tvos"))]
define_obj_type!(
    #[doc(alias = "UNNotificationAction")]
    pub NotificationAction(ns::Id)
);

#[cfg(not(target_os = "tvos"))]
impl NotificationAction {
    define_cls!(UN_NOTIFICATION_ACTION);

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(options)]
    pub fn options(&self) -> NotificationActionOpts;

    #[objc::msg_send(icon)]
    pub fn icon(&self) -> Option<arc::R<un::NotificationActionIcon>>;

    #[objc::msg_send(actionWithIdentifier:title:options:)]
    pub fn with_id(
        id: &ns::String,
        title: &ns::String,
        opts: NotificationActionOpts,
    ) -> arc::R<Self>;

    #[objc::msg_send(actionWithIdentifier:title:options:icon:)]
    pub fn with_icon(
        id: &ns::String,
        title: &ns::String,
        opts: NotificationActionOpts,
        icon: Option<&un::NotificationActionIcon>,
    ) -> arc::R<Self>;
}

#[cfg(not(target_os = "tvos"))]
define_obj_type!(
    #[doc(alias = "UNTextInputNotificationAction")]
    pub TextInputNotificationAction(NotificationAction)
);

#[cfg(not(target_os = "tvos"))]
impl TextInputNotificationAction {
    define_cls!(UN_TEXT_INPUT_NOTIFICATION_ACTION);

    #[objc::msg_send(textInputButtonTitle)]
    pub fn text_input_button_title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(textInputPlaceholder)]
    pub fn text_input_placeholder(&self) -> arc::R<ns::String>;

    #[objc::msg_send(actionWithIdentifier:title:options:textInputButtonTitle:textInputPlaceholder:)]
    pub fn with_id(
        id: &ns::String,
        title: &ns::String,
        opts: NotificationActionOpts,
        text_input_button_title: &ns::String,
        text_input_placeholder: &ns::String,
    ) -> arc::R<Self>;

    #[objc::msg_send(actionWithIdentifier:title:options:icon:textInputButtonTitle:textInputPlaceholder:)]
    pub fn with_icon(
        id: &ns::String,
        title: &ns::String,
        opts: NotificationActionOpts,
        icon: Option<&un::NotificationActionIcon>,
        text_input_button_title: &ns::String,
        text_input_placeholder: &ns::String,
    ) -> arc::R<Self>;
}

#[cfg(not(target_os = "tvos"))]
unsafe extern "C" {
    static UN_NOTIFICATION_ACTION: &'static objc::Class<NotificationAction>;
    static UN_TEXT_INPUT_NOTIFICATION_ACTION: &'static objc::Class<TextInputNotificationAction>;
}
