use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UNNotificationAttachment")]
    pub NotificationAttach(ns::Id)
);

impl NotificationAttach {
    define_cls!(UN_NOTIFICATION_ATTACHMENT);
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(URL)]
    pub fn url(&self) -> arc::R<ns::Url>;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> arc::R<ns::String>;

    #[objc::msg_send(attachmentWithIdentifier:URL:options:error:)]
    pub unsafe fn with_url_err(
        id: &ns::String,
        url: &ns::Url,
        options: Option<&ns::Dictionary<NotificationAttachOpts, ns::Id>>,
        err: *mut Option<&ns::Error>,
    ) -> Option<arc::R<Self>>;

    pub fn with_url<'ear>(
        id: &ns::String,
        url: &ns::Url,
        options: Option<&ns::Dictionary<NotificationAttachOpts, ns::Id>>,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::with_url_err(id, url, options, err) })
    }
}

define_obj_type!(
    pub NotificationAttachOpts(ns::String)
);

impl NotificationAttachOpts {
    #[doc(alias = "UNNotificationAttachmentOptionsTypeHintKey")]
    pub fn type_hint() -> &'static Self {
        unsafe { UNNotificationAttachmentOptionsTypeHintKey }
    }

    #[doc(alias = "UNNotificationAttachmentOptionsThumbnailHiddenKey")]
    pub fn thumbnail_hidden() -> &'static Self {
        unsafe { UNNotificationAttachmentOptionsThumbnailHiddenKey }
    }

    #[doc(alias = "UNNotificationAttachmentOptionsThumbnailClippingRectKey")]
    pub fn thumbnail_clipping_rect() -> &'static Self {
        unsafe { UNNotificationAttachmentOptionsThumbnailClippingRectKey }
    }

    #[doc(alias = "UNNotificationAttachmentOptionsThumbnailTimeKey")]
    pub fn thumbnail_time() -> &'static Self {
        unsafe { UNNotificationAttachmentOptionsThumbnailTimeKey }
    }
}

#[link(name = "UserNotifications", kind = "framework")]
unsafe extern "C" {
    static UNNotificationAttachmentOptionsTypeHintKey: &'static NotificationAttachOpts;
    static UNNotificationAttachmentOptionsThumbnailHiddenKey: &'static NotificationAttachOpts;
    static UNNotificationAttachmentOptionsThumbnailClippingRectKey: &'static NotificationAttachOpts;
    static UNNotificationAttachmentOptionsThumbnailTimeKey: &'static NotificationAttachOpts;
}

unsafe extern "C" {
    static UN_NOTIFICATION_ATTACHMENT: &'static objc::Class<NotificationAttach>;
}
