use crate::{arc, define_cls, define_obj_type, ns, objc, un};

#[cfg(all(feature = "cl", any(target_os = "ios", target_os = "watchos")))]
use crate::cl;

define_obj_type!(
    #[doc(alias = "UNNotificationTrigger")]
    pub NotificationTrigger(ns::Id)
);

impl NotificationTrigger {
    define_cls!(sym UNNotificationTrigger);

    #[objc::msg_send(repeats)]
    pub fn repeats(&self) -> bool;
}

define_obj_type!(
    #[doc(alias = "UNPushNotificationTrigger")]
    pub PushNotificationTrigger(un::NotificationTrigger)
);

impl PushNotificationTrigger {
    define_cls!(sym UNPushNotificationTrigger);
}

define_obj_type!(
    #[doc(alias = "UNTimeIntervalNotificationTrigger")]
    pub TimeIntervalNotificationTrigger(un::NotificationTrigger)
);

impl TimeIntervalNotificationTrigger {
    define_cls!(sym UNTimeIntervalNotificationTrigger);

    #[objc::msg_send(timeInterval)]
    pub fn time_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(triggerWithTimeInterval:repeats:)]
    pub fn with_interval(time_interval: ns::TimeInterval, repeats: bool) -> arc::R<Self>;

    #[objc::msg_send(nextTriggerDate)]
    pub fn next_trigger_date(&self) -> Option<arc::R<ns::Date>>;
}

define_obj_type!(
    #[doc(alias = "UNCalendarNotificationTrigger")]
    pub CalendarNotificationTrigger(un::NotificationTrigger)
);

impl CalendarNotificationTrigger {
    define_cls!(sym UNCalendarNotificationTrigger);
}

define_obj_type!(
    #[doc(alias = "UNLocationNotificationTrigger")]
    pub LocationNotificationTrigger(un::NotificationTrigger)
);

#[cfg(all(feature = "cl", any(target_os = "ios", target_os = "watchos")))]
impl LocationNotificationTrigger {
    define_cls!(sym UNLocationNotificationTrigger);

    #[objc::msg_send(region)]
    pub fn region(&self) -> arc::R<cl::Region>;

    #[objc::msg_send(triggerWithRegion:repeats:)]
    pub fn with_region(region: &cl::Region, repeats: bool) -> arc::R<Self>;
}
