use crate::{arc, define_obj_type, ns, objc, un};

define_obj_type!(
    #[doc(alias = "UNNotificationResponse")]
    pub NotificationResponse(ns::Id)
);

impl NotificationResponse {
    #[objc::msg_send(notification)]
    pub fn notification(&self) -> arc::R<un::Notification>;

    #[objc::msg_send(actionIdentifier)]
    pub fn action_id(&self) -> arc::R<ns::String>;
}

define_obj_type!(
    #[doc(alias = "UNTextInputNotificationResponse")]
    pub TextInputNotificationResponse(NotificationResponse)
);

impl TextInputNotificationResponse {
    #[objc::msg_send(userText)]
    pub fn user_text(&self) -> arc::R<ns::String>;
}
