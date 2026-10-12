use crate::{arc, blocks, define_cls, define_obj_type, ns, objc};

#[doc(alias = "PHAccessLevel")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AccessLevel {
    /// Adding to the library, nothing read from it.
    AddOnly = 1,
    ReadWrite = 2,
}

#[doc(alias = "PHAuthorizationStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AuthorizationStatus {
    /// The user has not chosen yet.
    NotDetermined = 0,
    /// The user cannot change it (parental controls, say).
    Restricted = 1,
    Denied = 2,
    Authorized = 3,
    /// Some of the library (read-write access only).
    Limited = 4,
}

define_obj_type!(
    #[doc(alias = "PHPhotoLibrary")]
    pub PhotoLib(ns::Id)
);

impl PhotoLib {
    define_cls!(sym PHPhotoLibrary);

    #[objc::msg_send(sharedPhotoLibrary)]
    pub fn shared() -> arc::R<Self>;

    #[objc::msg_send(authorizationStatusForAccessLevel:)]
    pub fn authorization_status(level: AccessLevel) -> AuthorizationStatus;

    /// `handler` runs on an arbitrary queue once the user has answered.
    #[objc::msg_send(requestAuthorizationForAccessLevel:handler:)]
    pub fn request_authorization_ch(
        level: AccessLevel,
        handler: &mut blocks::SendBlock<fn(AuthorizationStatus)>,
    );

    /// Runs `changes` (asset creation requests made in it are applied) on an arbitrary serial
    /// queue, then `ch` with whether they were applied. Asks the user for access first, if it
    /// is not decided yet.
    #[objc::msg_send(performChanges:completionHandler:)]
    pub fn perform_changes_ch(
        &mut self,
        changes: &mut blocks::SendBlock<fn()>,
        ch: &mut blocks::SendBlock<fn(bool, Option<&ns::Error>)>,
    );
}
