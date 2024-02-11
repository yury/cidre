use crate::{define_cls, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum ActivationState {
    NotActivated = 0,
    Inactive = 1,
    Activated = 2,
}

define_obj_type!(
    #[doc(alias = "WCSession")]
    pub Session(ns::Id)
);

impl Session {
    define_cls!(WC_SESSION);

    #[objc::cls_msg_send(isSupported)]
    pub fn is_supported() -> bool;

    #[objc::cls_msg_send(defaultSession)]
    pub fn default() -> &'static mut Self;

    #[objc::msg_send(activationState)]
    pub fn activation_state(&self) -> ActivationState;

    #[objc::msg_send(hasContentPending)]
    pub fn has_content_pending(&self) -> bool;

    #[objc::msg_send(isPaired)]
    pub fn is_paired(&self) -> bool;

    #[objc::msg_send(isWatchAppInstalled)]
    pub fn is_watch_app_installed(&self) -> bool;

    #[objc::msg_send(isComplicationEnabled)]
    pub fn is_complication_enabled(&self) -> bool;

    #[objc::msg_send(remainingComplicationUserInfoTransfers)]
    pub fn remaining_complication_user_info_transfers(&self) -> usize;

    #[objc::msg_send(watchDirectoryURL)]
    pub fn watch_directory_url(&self) -> Option<&ns::Url>;

    #[objc::msg_send(isCompanionAppInstalled)]
    pub fn is_companion_app_installed(&self) -> bool;

    #[objc::msg_send(isReachable)]
    pub fn is_reachable(&self) -> bool;
}

#[link(name = "wc", kind = "static")]
extern "C" {
    static WC_SESSION: &'static objc::Class<Session>;
}
