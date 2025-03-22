use crate::ns;

impl ns::ErrorDomain {
    #[doc(alias = "UNErrorDomain")]
    pub fn user_notifications() -> &'static Self {
        unsafe { UNErrorDomain }
    }
}

#[link(name = "UserNotifications", kind = "framework")]
unsafe extern "C" {
    static UNErrorDomain: &'static ns::ErrorDomain;
}

#[doc(alias = "UNErrorCode")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    NotificationsNotAllowed = 1,
    AttachInvalidUrl = 100,
    AttachUnrecognizedType,
    AttachInvalidFileSize,
    AttachNotInDataStore,
    AttachMoveIntoDataStoreFailed,
    AttachCorrupt,

    NotificationInvalidNoDate = 1400,
    NotificationInvalidNoContent,

    ContentProvidingObjectNotAllowed = 1500,
    ContentProvidingInvalid,

    CodeBadgeInputInvalid = 1600,
}
