use crate::{arc, define_cls, define_obj_type, ns, objc, un};

define_obj_type!(
    #[doc(alias = "UNNotificationRequest")]
    pub NotificationRequest(ns::Id)
);

impl NotificationRequest {
    define_cls!(UN_NOTIFICATION_REQUEST);

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(content)]
    pub fn content(&self) -> arc::R<un::NotificationContent>;

    #[objc::msg_send(requestWithIdentifier:content:trigger:)]
    pub fn with_trigger(
        id: &ns::String,
        content: &un::NotificationContent,
        trigger: Option<&un::NotificationTrigger>,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UN_NOTIFICATION_REQUEST: &'static objc::Class<NotificationRequest>;
}
