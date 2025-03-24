use crate::{blocks, define_cls, define_obj_type, define_opts, ns, objc, un};

define_opts!(
    pub AuthorizationOpts(usize)
);

impl AuthorizationOpts {
    pub const NONE: Self = Self(0);

    /// The ability to update the app’s badge.
    pub const BADGE: Self = Self(1 << 0);

    /// The ability to play sounds.
    pub const SOUND: Self = Self(1 << 1);

    /// The ability to display alerts.
    pub const ALERT: Self = Self(1 << 2);

    /// The ability to display notifications in a CarPlay environment.
    pub const CAR_PLAY: Self = Self(1 << 3);

    /// The ability to play sounds for critical alerts.
    pub const CRITICAL_ALERT: Self = Self(1 << 4);

    /// An option indicating the system should display a button for in-app notification settings.
    pub const PROVIDES_APP_NOTIFICATION_SETTINGS: Self = Self(1 << 5);

    /// The ability to post noninterrupting notifications provisionally to the Notification Center.
    pub const PROVISIONAL: Self = Self(1 << 6);
}

define_opts!(
    #[doc(alias = "UNNotificationPresentationOptions")]
    pub NotificationPresentationOpts(usize)
);

impl NotificationPresentationOpts {
    pub const NONE: Self = Self(0);
    pub const BADGE: Self = Self(1 << 0);
    pub const SOUND: Self = Self(1 << 1);
    pub const LIST: Self = Self(1 << 3);
    pub const BANNER: Self = Self(1 << 4);
}

define_obj_type!(
    /// Should be used only from app with main bundle
    #[doc(alias = "UNUserNotificationCenter")]
    pub Center(ns::Id)
);

impl Center {
    define_cls!(UN_USER_NOTIFICATION_CENTER);

    #[objc::msg_send(supportsContentExtensions)]
    pub fn supports_content_extensions(&self) -> bool;

    #[objc::msg_send(currentNotificationCenter)]
    pub fn current() -> &'static mut Self;
}

#[objc::protocol(UNUserNotificationCenterDelegate)]
pub trait CenterDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(userNotificationCenter:willPresentNotification:withCompletionHandler:)]
    fn center_will_present_notification_ch_block(
        &mut self,
        center: &mut un::Center,
        notification: &un::Notification,
        ch: &mut blocks::EscBlock<fn(options: un::NotificationPresentationOpts)>,
    );

    #[objc::optional]
    #[objc::msg_send(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:)]
    fn center_did_receive_notification_response(
        &mut self,
        center: &mut un::Center,
        response: &un::NotificationResponse,
        ch: &mut blocks::CompletionBlock,
    );

    #[objc::optional]
    #[objc::msg_send(userNotificationCenter:openSettingsForNotification:)]
    fn center_open_settings_for_notification(
        &mut self,
        center: &mut un::Center,
        notification: &un::Notification,
    );
}

#[link(name = "un", kind = "static")]
unsafe extern "C" {
    static UN_USER_NOTIFICATION_CENTER: &'static objc::Class<Center>;
}
