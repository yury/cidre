use crate::ns;

pub type Domain = ns::ErrorDomain;

impl Domain {
    pub fn watch_connectivity() -> &'static Self {
        unsafe { WCErrorDomain }
    }
}

#[link(name = "WatchConnectivity", kind = "framework")]
unsafe extern "C" {
    static WCErrorDomain: &'static Domain;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum Code {
    GenericError = 7001,
    SessionNotSupported = 7002,
    SessionMissingDelegate = 7003,
    SessionNotActivated = 7004,
    DeviceNotPaired = 7005,
    WatchAppNotInstalled = 7006,
    NotReachable = 7007,
    InvalidParameter = 7008,
    PayloadTooLarge = 7009,
    PayloadUnsupportedTypes = 7010,
    MessageReplyFailed = 7011,
    MessageReplyTimedOut = 7012,
    FileAccessDenied = 7013,
    DeliveryFailed = 7014,
    InsufficientSpace = 7015,
    SessionInactive = 7016,
    TransferTimedOut = 7017,
    CompanionAppNotInstalled = 7018,
    WatchOnlyApp = 7019,
}
