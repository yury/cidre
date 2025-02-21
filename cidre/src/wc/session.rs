use crate::{arc, blocks, define_cls, define_obj_type, ns, objc};

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

    #[objc::msg_send(isSupported)]
    pub fn is_supported() -> bool;

    #[objc::msg_send(defaultSession)]
    pub fn default() -> arc::R<Self>;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDelegate>>;

    #[objc::msg_send(set_delegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

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
    pub fn watch_directory_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(isCompanionAppInstalled)]
    pub fn is_companion_app_installed(&self) -> bool;

    #[objc::msg_send(isReachable)]
    pub fn is_reachable(&self) -> bool;

    #[objc::msg_send(sendMessage:replyHandler:errorHandler:)]
    pub fn send_msg_with_handlers(
        &self,
        msg: &ns::Dictionary<ns::String, ns::Id>,
        reply_handler: Option<&mut blocks::SendBlock<fn(&ns::Dictionary<ns::String, ns::Id>)>>,
        error_handler: Option<&mut blocks::SendBlock<fn(&ns::Error)>>,
    );

    #[cfg(feature = "async")]
    pub async fn send_msg(
        &self,
        msg: &ns::Dictionary<ns::String, ns::Id>,
    ) -> (
        blocks::Completion<arc::R<ns::Dictionary<ns::String, ns::Id>>>,
        blocks::Completion<arc::R<ns::Error>>,
    ) {
        let (reply_future, mut reply_block) = blocks::retained1();
        let (error_future, mut error_block) = blocks::retained1();
        self.send_msg_with_handlers(msg, Some(&mut reply_block), Some(&mut error_block));
        (reply_future, error_future)
    }

    #[objc::msg_send(sendMessageData:replyHandler:errorHandler:)]
    pub fn send_msg_data_with_handlers(
        &self,
        msg: &ns::Data,
        reply_handler: Option<&mut blocks::SendBlock<fn(&ns::Data)>>,
        error_handler: Option<&mut blocks::SendBlock<fn(&ns::Error)>>,
    );

    #[cfg(feature = "async")]
    pub async fn send_msg_data(
        &self,
        msg: &ns::Data,
    ) -> (
        blocks::Completion<arc::R<ns::Data>>,
        blocks::Completion<arc::R<ns::Error>>,
    ) {
        let (reply_future, mut reply_block) = blocks::retained1();
        let (error_future, mut error_block) = blocks::retained1();
        self.send_msg_data_with_handlers(msg, Some(&mut reply_block), Some(&mut error_block));
        (reply_future, error_future)
    }
}

#[objc::protocol(WCSessionDelegate)]
pub trait Delegate: objc::Obj {
    #[objc::msg_send(session:activationDidCompleteWithState:error:)]
    fn session_activation_did_complete_with_state(
        &mut self,
        session: &Session,
        state: ActivationState,
        error: Option<&ns::Error>,
    );

    #[objc::msg_send(sessionDidBecomeInactive:)]
    fn session_did_become_inactive(&mut self, session: &Session);

    #[objc::msg_send(sessionDidDeactivate:)]
    fn session_did_deactivate(&mut self, session: &Session);

    #[objc::optional]
    #[objc::msg_send(sessionWatchStateDidChange:)]
    fn session_watch_state_did_change(&mut self, session: &Session);
}

define_obj_type!(
    pub AnyDelegate(ns::Id)
);

impl Delegate for AnyDelegate {}

#[link(name = "wc", kind = "static")]
unsafe extern "C" {
    static WC_SESSION: &'static objc::Class<Session>;
}
