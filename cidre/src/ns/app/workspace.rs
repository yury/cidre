use crate::{arc, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "NSWorkspace")]
    pub Workspace(ns::Id)
);

impl Workspace {
    define_cls!(NS_WORKSPACE);

    #[objc::msg_send(sharedWorkspace)]
    pub fn shared() -> arc::R<Self>;

    #[objc::msg_send(notificationCenter)]
    pub fn notification_center(&self) -> arc::R<ns::NotificationCenter>;
}

#[doc(alias = "NSWorkspaceAuthorizationType")]
#[repr(isize)]
pub enum AuthorizationType {
    #[doc(alias = "NSWorkspaceAuthorizationTypeCreateSymbolicLink")]
    CreateSymbolicLink,

    #[doc(alias = "NSWorkspaceAuthorizationTypeSetAttributes")]
    SetAttributes,

    #[doc(alias = "NSWorkspaceAuthorizationTypeReplaceFile")]
    ReplaceFile,
}

define_obj_type!(
    #[doc(alias = "NSWorkspaceAuthorization")]
    pub Authorization(ns::Id)
);

unsafe impl Send for Authorization {}

/// NSWorkspaceAuthorization
impl Workspace {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(requestAuthorizationOfType:completionHandler:)]
    pub fn request_authorization_of_type_ch_block(
        &self,
        type_: ns::WorkspaceAuthorizationType,
        ch: &mut blocks::ResultCh<ns::WorkspaceAuthorization>,
    );

    #[cfg(feature = "blocks")]
    pub fn request_authorization_of_type_ch<F>(&self, type_: ns::WorkspaceAuthorizationType, ch: F)
    where
        F: FnMut(Option<&ns::WorkspaceAuthorization>, Option<&ns::Error>)
            + 'static
            + std::marker::Sync,
    {
        let mut ch = blocks::ResultCh::new2(ch);
        self.request_authorization_of_type_ch_block(type_, &mut ch)
    }

    #[cfg(feature = "async")]
    pub async fn request_authorization_of_type(
        &self,
        type_: ns::WorkspaceAuthorizationType,
    ) -> Result<arc::R<ns::WorkspaceAuthorization>, arc::R<ns::Error>> {
        let (future, mut ch) = blocks::result();
        self.request_authorization_of_type_ch_block(type_, &mut ch);
        future.await
    }
}

impl ns::FileManager {
    #[objc::msg_send(fileManagerWithAuthorization:)]
    pub fn with_authorization(authorization: &ns::WorkspaceAuthorization) -> arc::R<Self>;
}

unsafe extern "C" {
    static NS_WORKSPACE: &'static objc::Class<Workspace>;
}
