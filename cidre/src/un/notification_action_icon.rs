use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UNNotificationActionIcon")]
    pub NotificationActionIcon(ns::Id)
);

impl NotificationActionIcon {
    define_cls!(sym UNNotificationActionIcon);

    #[objc::msg_send(iconWithTemplateImageName:)]
    pub fn with_template_image_name(template_image_name: &ns::String) -> arc::R<Self>;

    #[objc::msg_send(iconWithSystemImageName:)]
    pub fn with_sys_image_name(sys_image_name: &ns::String) -> arc::R<Self>;
}
