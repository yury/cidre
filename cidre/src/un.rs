pub mod error;
pub use error::Code as ErrorCode;

mod notification;
pub use notification::Notification;

mod notification_action;
pub use notification_action::NotificationAction;
pub use notification_action::NotificationActionOpts;
#[cfg(not(target_os = "tvos"))]
pub use notification_action::TextInputNotificationAction;

mod notification_sound;
#[cfg(not(target_os = "tvos"))]
pub use notification_sound::NotificationSound;
#[cfg(not(target_os = "tvos"))]
pub use notification_sound::NotificationSoundName;

mod notification_content;
pub use notification_content::NotificationContent;
pub use notification_content::NotificationContentMut;
pub use notification_content::NotificationInterruptionLevel;

mod notification_request;
pub use notification_request::NotificationRequest;

mod notification_settings;
pub use notification_settings::AlertStyle;
pub use notification_settings::AuthorizationStatus;
pub use notification_settings::NotificationSetting;
pub use notification_settings::NotificationSettings;
pub use notification_settings::ShowPreviewsSetting;

mod user_notification_center;
pub use user_notification_center::AuthorizationOpts;
pub use user_notification_center::Center;

use crate::{arc, ns, objc};

/// UNUserNotificationCenterSupport
impl ns::String {
    #[objc::msg_send(localizedUserNotificationStringForKey:arguments:)]
    pub fn localized_user_notification_string_for_key(
        key: &ns::String,
        args: ns::Array<ns::Id>,
    ) -> arc::R<ns::String>;
}
