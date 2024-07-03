use crate::{define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UNNotificationRequest")]
    pub NotificationRequest(ns::Id)
);

impl NotificationRequest {
    define_cls!(UN_NOTIFICATION_REQUEST);

    #[objc::msg_send2(identifier)]
    pub fn id(&self) -> &ns::String;
}

extern "C" {
    static UN_NOTIFICATION_REQUEST: &'static objc::Class<NotificationRequest>;
}
