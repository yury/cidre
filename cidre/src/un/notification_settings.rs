use crate::{define_obj_type, ns, objc};

#[doc(alias = "UNAuthorizationStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(isize)]
pub enum AuthorizationStatus {
    /// The user has not yet made a choice regarding whether the application may post user notifications.
    #[doc(alias = "UNAuthorizationStatusNotDetermined")]
    NotDetermined = 0,

    /// The application is not authorized to post user notifications.
    #[doc(alias = "UNAuthorizationStatusDenied")]
    Denied,

    /// The application is authorized to post user notifications.
    #[doc(alias = "UNAuthorizationStatusAuthorized")]
    Authorized,

    /// The application is authorized to post non-interruptive user notifications.
    #[doc(alias = "UNAuthorizationStatusProvisional")]
    Provisional,

    /// The application is temporarily authorized to post notifications. Only available to app clips.
    #[doc(alias = "UNAuthorizationStatusEphemeral")]
    Ephemeral,
}

#[doc(alias = "UIShowPreviewsSetting")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(isize)]
pub enum ShowPreviewsSetting {
    /// Notification previews are always shown.
    Always,

    /// Notifications previews are only shown when authenticated.
    WhenAuthenticated,

    /// Notifications previews are never shown.
    Never,
}

#[doc(alias = "UNNotificationSetting")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(isize)]
pub enum NotificationSetting {
    /// The application does not support this notification type
    NotSupported,

    /// The notification setting is turned off.
    Disabled,

    /// The notification setting is turned on.
    Enabled,
}

#[doc(alias = "UNAlertStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(isize)]
pub enum AlertStyle {
    None = 0,
    Banner,
    Alert,
}

define_obj_type!(
    #[doc(alias = "UNNotificationSettings")]
    pub NotificationSettings(ns::Id)
);

impl NotificationSettings {
    #[objc::msg_send(authorizationStatus)]
    pub fn authorization_status(&self) -> AuthorizationStatus;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(soundSetting)]
    pub fn sound_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(badgeSetting)]
    pub fn badge_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(alertSetting)]
    pub fn alert_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(notificationCenterSetting)]
    pub fn notification_center_setting(&self) -> NotificationSetting;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(lockScreenSetting)]
    pub fn lock_screen_setting(&self) -> NotificationSetting;

    #[cfg(target_os = "ios")]
    #[objc::msg_send(carPlaySetting)]
    pub fn car_play_setting(&self) -> NotificationSetting;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(alertStyle)]
    pub fn alert_style(&self) -> AlertStyle;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(showPreviewsSetting)]
    pub fn show_previews_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(criticalAlertSetting)]
    pub fn critical_alert_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(providesAppNotificationSettings)]
    pub fn provides_app_notification_settings(&self) -> bool;

    #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
    #[objc::msg_send(announcementSetting)]
    pub fn announcement_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(timeSensitiveSetting)]
    pub fn time_sensitive_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(scheduledDeliverySetting)]
    pub fn scheduled_delivery_setting(&self) -> NotificationSetting;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(directMessagesSetting)]
    pub fn direct_messages_setting(&self) -> NotificationSetting;
}
