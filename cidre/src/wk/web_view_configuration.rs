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

    // ...

    #[objc::msg_send(suppressesIncrementalRendering)]
    pub fn suppresses_incremental_rendering(&self) -> bool;

    #[objc::msg_send(setSuppressesIncrementalRendering:)]
    pub fn set_suppresses_incremental_rendering(&mut self, val: bool);

    #[objc::msg_send(applicationNameForUserAgent)]
    pub fn app_name_for_user_agent(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setApplicationNameForUserAgent:)]
    pub fn set_app_name_for_user_agent(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(allowsAirPlayForMediaPlayback)]
    pub fn allows_air_play_for_media_playback(&self) -> bool;

    #[objc::msg_send(setAllowsAirPlayForMediaPlayback:)]
    pub fn set_allows_air_play_for_media_playback(&mut self, val: bool);

    #[objc::msg_send(upgradeKnownHostsToHTTPS)]
    pub fn upgrade_known_hosts_to_https(&self) -> bool;

    #[objc::msg_send(setUpgradeKnownHostsToHTTPS:)]
    pub fn set_upgrade_known_hosts_to_https(&mut self, val: bool);

    #[objc::msg_send(mediaTypesRequiringUserActionForPlayback)]
    pub fn media_types_requiring_user_action_for_playback(&self) -> wk::AudiovisualMediaTypes;

    #[objc::msg_send(setMediaTypesRequiringUserActionForPlayback:)]
    pub fn set_media_types_requiring_user_action_for_playback(
        &mut self,
        val: wk::AudiovisualMediaTypes,
    );

    // ...

    #[objc::msg_send(limitsNavigationsToAppBoundDomains)]
    pub fn limits_navigations_to_app_bound_domains(&self) -> bool;

    #[objc::msg_send(setLimitsNavigationsToAppBoundDomains:)]
    pub fn set_limits_navigations_to_app_bound_domains(&mut self, val: bool);

    #[objc::msg_send(allowsInlinePredictions)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn allows_inline_predictions(&self) -> bool;

    #[objc::msg_send(setAllowsInlinePredictions:)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn set_allows_inline_predictions(&mut self, val: bool);

    #[objc::msg_send(supportsAdaptiveImageGlyph)]
    #[objc::available(macos = 15.0, ios = 18.0, visionos = 2.0)]
    pub fn supports_adaptive_image_glyph(&self) -> bool;

    #[objc::msg_send(setSupportsAdaptiveImageGlyph:)]
    #[objc::available(macos = 15.0, ios = 18.0, visionos = 2.0)]
    pub fn set_supports_adaptive_image_glyph(&mut self, val: bool);
}

#[cfg(target_os = "ios")]
impl WebViewCfg {
    #[objc::msg_send(allowsInlineMediaPlayback)]
    pub fn allows_inline_media_playback(&self) -> bool;

    #[objc::msg_send(setAllowsInlineMediaPlayback:)]
    pub fn set_allows_inline_media_playback(&mut self, val: bool);

    #[objc::msg_send(selectionGranularity)]
    pub fn selection_granularity(&self) -> wk::SelectionGranularity;

    #[objc::msg_send(setSelectionGranularity:)]
    pub fn set_selection_granularity(&mut self, val: wk::SelectionGranularity);

    #[objc::msg_send(allowsPictureInPictureMediaPlayback)]
    pub fn allows_picture_in_picture_media_playback(&self) -> bool;

    #[objc::msg_send(setAllowsPictureInPictureMediaPlayback:)]
    pub fn set_allows_picture_in_picture_media_playback(&mut self, val: bool);

    #[objc::msg_send(ignoresViewportScaleLimits)]
    pub fn ignores_viewport_scale_limits(&self) -> bool;

    #[objc::msg_send(setIgnoresViewportScaleLimits:)]
    pub fn set_ignores_viewport_scale_limits(&mut self, val: bool);
}

#[cfg(not(target_os = "ios"))]
impl WebViewCfg {
    #[objc::msg_send(userInterfaceDirectionPolicy)]
    pub fn ui_direction_policy(&self) -> wk::UiDirectionPolicy;

    #[objc::msg_send(setUserInterfaceDirectionPolicy:)]
    pub fn set_ui_direction_policy(&self, val: wk::UiDirectionPolicy);
}

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_WEB_VIEW_CONFIGURATION: &'static objc::Class<WebViewCfg>;
}
