use crate::{arc, define_obj_type, ns, objc, un};

define_obj_type!(
    #[doc(alias = "UNNotification")]
    pub Notification(ns::Id)
);

impl Notification {
    /// The date displayed on the notification.
    #[objc::msg_send(date)]
    pub fn date(&self) -> arc::R<ns::Data>;

    /// The notification request that caused the notification to be delivered.
    #[objc::msg_send(request)]
    pub fn request(&self) -> arc::R<un::NotificationRequest>;
}
