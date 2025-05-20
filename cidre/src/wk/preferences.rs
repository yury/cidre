use crate::{arc, cg, define_obj_type, ns, objc, wk};

define_obj_type!(
    #[doc(alias = "WKPreferences")]
    pub Preferences(ns::Id),
    WK_PREFERENCES
);

impl Preferences {
    #[objc::msg_send(minimumFontSize)]
    pub fn min_font_size(&self) -> cg::Float;

    #[objc::msg_send(setMinimumFontSize:)]
    pub fn set_min_font_size(&mut self, val: cg::Float);

    #[objc::msg_send(javaScriptCanOpenWindowsAutomatically)]
    pub fn js_can_open_windows_automatically(&self) -> bool;

    #[objc::msg_send(setJavaScriptCanOpenWindowsAutomatically:)]
    pub fn set_js_can_open_windows_automatically(&mut self, val: bool);

    #[objc::msg_send(isFraudulentWebsiteWarningEnabled)]
    pub fn is_fraudulent_website_warning_enabled(&self) -> bool;

    #[objc::msg_send(setFraudulentWebsiteWarningEnabled:)]
    pub fn set_fraudulent_website_warning_enabled(&mut self, val: bool);

    #[objc::msg_send(shouldPrintBackgrounds)]
    #[objc::available(macos = 13.3, ios = 16.4)]
    pub fn should_print_backgrounds(&self) -> bool;

    #[objc::msg_send(setShouldPrintBackgrounds:)]
    #[objc::available(macos = 13.3, ios = 16.4)]
    pub fn set_should_print_backgrounds(&mut self, val: bool);

    #[cfg(not(target_os = "ios"))]
    #[objc::msg_send(tabFocusesLinks)]
    pub fn tab_focuses_links(&self) -> bool;

    #[cfg(not(target_os = "ios"))]
    #[objc::msg_send(setTabFocusesLinks:)]
    pub fn set_tab_focuses_links(&mut self, val: bool);

    #[objc::msg_send(isTextInteractionEnabled)]
    #[objc::available(macos = 11.3, ios = 14.5)]
    pub fn is_text_interaction_enabled(&self) -> bool;

    #[objc::msg_send(setTextInteractionEnabled:)]
    #[objc::available(macos = 11.3, ios = 14.5)]
    pub fn set_text_interaction_enabled(&mut self, val: bool);

    #[objc::msg_send(isSiteSpecificQuirksModeEnabled)]
    #[objc::available(macos = 12.3, ios = 15.4)]
    pub fn is_site_specific_quirks_node_enabled(&self) -> bool;

    #[objc::msg_send(setSiteSpecificQuirksModeEnabled:)]
    #[objc::available(macos = 12.3, ios = 15.4)]
    pub fn set_site_specific_quirks_node_enabled(&mut self, val: bool);

    #[objc::msg_send(isElementFullscreenEnabled)]
    #[objc::available(macos = 12.3, ios = 15.4, tvos = 17.0)]
    pub fn is_element_fullscreen_enabled(&self) -> bool;

    #[objc::msg_send(setElementFullscreenEnabled:)]
    #[objc::available(macos = 12.3, ios = 15.4, tvos = 17.0)]
    pub fn set_element_fullscreen_enabled(&mut self, val: bool);

    #[objc::msg_send(inactiveSchedulingPolicy)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn inactive_scheduling_policy(&self) -> wk::InactiveSchedulingPolicy;

    #[objc::msg_send(setInactiveSchedulingPolicy:)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn set_inactive_scheduling_policy(&mut self, val: wk::InactiveSchedulingPolicy);
}

#[doc(alias = "WKInactiveSchedulingPolicy")]
#[repr(isize)]
pub enum InactiveSchedulingPolicy {
    Suspend,
    Throttle,
    None,
}

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_PREFERENCES: &'static objc::Class<Preferences>;
}
