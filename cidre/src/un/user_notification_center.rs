use crate::{define_cls, define_obj_type, define_opts, ns, objc};

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

#[link(name = "un", kind = "static")]
unsafe extern "C" {
    static UN_USER_NOTIFICATION_CENTER: &'static objc::Class<Center>;
}
