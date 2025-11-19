pub mod error;
pub use error::Code as ErrorCode;

mod notification;
pub use notification::Notification;

#[cfg(not(target_os = "tvos"))]
mod notification_attachment;
#[cfg(not(target_os = "tvos"))]
pub use notification_attachment::NotificationAttach;
#[cfg(not(target_os = "tvos"))]
pub use notification_attachment::NotificationAttachOpts;

mod notification_action;
#[cfg(not(target_os = "tvos"))]
pub use notification_action::NotificationAction;
pub use notification_action::NotificationActionOpts;
#[cfg(not(target_os = "tvos"))]
pub use notification_action::TextInputNotificationAction;

mod notification_action_icon;
pub use notification_action_icon::NotificationActionIcon;

mod notification_sound;
#[cfg(not(target_os = "tvos"))]
pub use notification_sound::NotificationSound;
#[cfg(not(target_os = "tvos"))]
pub use notification_sound::NotificationSoundName;

mod notification_trigger;
pub use notification_trigger::CalendarNotificationTrigger;
pub use notification_trigger::LocationNotificationTrigger;
pub use notification_trigger::NotificationTrigger;
pub use notification_trigger::PushNotificationTrigger;
pub use notification_trigger::TimeIntervalNotificationTrigger;

mod notification_content;
pub use notification_content::NotificationContent;
pub use notification_content::NotificationContentMut;
pub use notification_content::NotificationInterruptionLevel;

mod notification_request;
pub use notification_request::NotificationRequest;

mod notification_response;
pub use notification_response::NotificationResponse;
pub use notification_response::TextInputNotificationResponse;

mod notification_settings;
pub use notification_settings::AlertStyle;
pub use notification_settings::AuthorizationStatus;
pub use notification_settings::NotificationSetting;
pub use notification_settings::NotificationSettings;
pub use notification_settings::ShowPreviewsSetting;

mod user_notification_center;
pub use user_notification_center::AuthorizationOpts;
pub use user_notification_center::Center;
pub use user_notification_center::CenterDelegate;
pub use user_notification_center::CenterDelegateImpl;
pub use user_notification_center::NotificationPresentationOpts;

use crate::{arc, ns, objc};

/// UNUserNotificationCenterSupport
impl ns::String {
    #[objc::msg_send(localizedUserNotificationStringForKey:arguments:)]
    pub fn localized_user_notification_string_for_key(
        key: &ns::String,
        args: ns::Array<ns::Id>,
    ) -> arc::R<ns::String>;
}
