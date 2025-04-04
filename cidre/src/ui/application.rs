use crate::{api, arc, define_obj_type, ns, objc, ui};

#[objc::protocol(UIApplicationDelegate)]
pub trait AppDelegate {
    #[objc::optional]
    #[objc::msg_send(applicationDidFinishLaunching:)]
    fn app_did_finish_launching(&mut self, app: &mut App);

    #[objc::optional]
    #[objc::msg_send(application:configurationForConnectingSceneSession:options:)]
    fn app_cfg_for_connecting_scene_session(
        &mut self,
        app: &App,
        session: &ui::SceneSession,
        options: &ui::SceneConnectionOpts,
    ) -> arc::Rar<ui::SceneCfg>;

    #[objc::optional]
    #[objc::msg_send(window)]
    fn window(&self) -> Option<&ui::Window>;

    #[objc::optional]
    #[objc::msg_send(setWindow:)]
    fn set_window(&mut self, val: Option<&ui::Window>);

    #[objc::optional]
    #[objc::msg_send(applicationDidBecomeActive:)]
    fn app_did_become_active(&mut self, app: &mut App);

    #[objc::optional]
    #[objc::msg_send(applicationWillResignActive:)]
    fn app_will_resign_active(&mut self, app: &mut App);

    // ...open url

    #[objc::optional]
    #[objc::msg_send(applicationDidReceiveMemoryWarning:)]
    fn app_did_receive_mem_warning(&mut self, app: &mut App);

    #[objc::optional]
    #[objc::msg_send(applicationWillTerminate:)]
    fn app_will_terminate(&mut self, app: &mut App);

    #[objc::optional]
    #[objc::msg_send(applicationSignificantTimeChange:)]
    fn app_significant_time_change(&mut self, app: &mut App);
}

define_obj_type!(
    pub App(ui::Responder),
    UI_APPLICATION
);

define_obj_type!(
    pub AnyAppDelegate(ns::Id)
);

impl AppDelegate for AnyAppDelegate {}

impl App {
    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<&AnyAppDelegate>;

    #[objc::msg_send(delegate)]
    pub fn delegate_mut(&self) -> Option<&mut AnyAppDelegate>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate(&mut self, val: Option<&AnyAppDelegate>);

    #[objc::msg_send(sharedApplication)]
    pub fn shared() -> &'static Self;

    #[objc::msg_send(sharedApplication)]
    pub fn shared_mut() -> &'static mut Self;

    #[objc::msg_send(isIdleTimerDisabled)]
    pub fn is_idle_timer_disabled(&self) -> bool;

    #[objc::msg_send(setIdleTimerDisabled:)]
    pub fn set_idle_timer_disabled(&mut self, val: bool);

    //- (void)openURL:(NSURL*)url options:(NSDictionary<UIApplicationOpenExternalURLOptionsKey, id> *)options completionHandler:(void (^ __nullable NS_SWIFT_UI_ACTOR)(BOOL success))completion API_AVAILABLE(ios(10.0)) API_UNAVAILABLE(watchos);
    //
    // pub fn open_url_ch_block(&self, url: &ns::Url, options: )

    #[doc(alias = "UIApplicationOpenSettingsURLString")]
    #[api::available(ios = 8.0)]
    pub fn open_settings_url_string() -> &'static ns::String {
        unsafe { UIApplicationOpenSettingsURLString }
    }

    #[doc(alias = "UIApplicationOpenDefaultApplicationsSettingsURLString")]
    #[api::available(ios = 18.3)]
    pub fn open_default_apps_settings_url_string() -> &'static ns::String {
        unsafe { UIApplicationOpenDefaultApplicationsSettingsURLString }
    }

    #[doc(alias = "UIApplicationOpenNotificationSettingsURLString")]
    #[api::available(ios = 15.4)]
    pub fn open_notifications_settings_url_string() -> &'static ns::String {
        unsafe { UIApplicationOpenNotificationSettingsURLString }
    }
}

unsafe extern "C" {
    static UI_APPLICATION: &'static objc::Class<App>;
}

#[link(name = "UIKit", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    #[api::available(ios = 8.0)]
    static UIApplicationOpenSettingsURLString: &'static ns::String;

    #[api::available(ios = 18.3)]
    static UIApplicationOpenDefaultApplicationsSettingsURLString: &'static ns::String;

    #[api::available(ios = 15.4)]
    static UIApplicationOpenNotificationSettingsURLString: &'static ns::String;
}
