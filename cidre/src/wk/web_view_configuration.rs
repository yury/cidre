use crate::{arc, define_obj_type, define_opts, ns, objc, wk};

#[doc(alias = "WKSelectionGranularity")]
#[repr(isize)]
pub enum SelectionGranularity {
    Dynamic,
    Character,
}

#[doc(alias = "WKUserInterfaceDirectionPolicy")]
#[repr(isize)]
pub enum UiDirectionPolicy {
    Content,
    System,
}

define_opts!(
    #[doc(alias = "WKAudiovisualMediaTypes")]
    pub AudiovisualMediaTypes(usize)
);

impl AudiovisualMediaTypes {
    pub const NONE: Self = Self(0);
    pub const AUDIO: Self = Self(1 << 0);
    pub const VIDEO: Self = Self(1 << 1);
    pub const ALL: Self = Self(usize::MAX);
}

define_obj_type!(
    pub WebViewCfg(ns::Id),
    WK_WEB_VIEW_CONFIGURATION
);

impl WebViewCfg {
    #[objc::msg_send(processPool)]
    pub fn process_pool(&self) -> arc::R<wk::ProcessPool>;

    #[objc::msg_send(setProcessPool:)]
    pub fn set_process_pool(&mut self, val: &wk::ProcessPool);

    #[objc::msg_send(preferences)]
    pub fn preferences(&self) -> arc::R<wk::Preferences>;

    #[objc::msg_send(setPreferences:)]
    pub fn set_preferences(&mut self, val: &wk::Preferences);

    #[objc::msg_send(userContentController)]
    pub fn user_content_ctrl(&self) -> arc::R<wk::UserContentController>;

    #[objc::msg_send(setUserContentController:)]
    pub fn set_user_content_ctrl(&mut self, val: &wk::UserContentController);
}

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_WEB_VIEW_CONFIGURATION: &'static objc::Class<WebViewCfg>;
}
