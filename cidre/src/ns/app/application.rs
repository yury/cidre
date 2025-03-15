use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSApplication")]
    pub App(ns::Id)
);

#[objc::protocol(NSApplicationDelegate)]
pub trait Delegate {
    #[objc::optional]
    #[objc::msg_send(applicationSupportsSecureRestorableState)]
    fn app_supports_secure_restorable_state(&mut self) -> bool;

    // Notifications

    #[objc::optional]
    #[objc::msg_send(applicationWillFinishLaunching:)]
    fn app_will_finish_launching(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidFinishLaunching:)]
    fn app_did_finish_launching(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillHide:)]
    fn app_will_hide(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidHide:)]
    fn app_did_hide(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillUnhide:)]
    fn app_will_unhide(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidUnhide:)]
    fn app_did_unhide(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillBecomeActive:)]
    fn app_will_become_active(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidBecomeActive:)]
    fn app_did_become_active(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillResignActive:)]
    fn app_will_resign_active(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidResignActive:)]
    fn app_did_resign_active(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillUpdate:)]
    fn app_will_update(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidUpdate:)]
    fn app_did_update(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillTerminate:)]
    fn app_will_terminate(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidChangeScreenParameters:)]
    fn app_did_change_screen_params(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidChangeOcclusionState:)]
    fn app_did_change_occlusion_state(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationProtectedDataWillBecomeUnavailable:)]
    fn app_protected_data_will_become_unavailable(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationProtectedDataDidBecomeAvailable:)]
    fn app_protected_data_did_become_available(&mut self, n: &ns::Notification);
}

impl App {
    define_cls!(NS_APPLICATION);

    #[objc::msg_send(sharedApplication)]
    pub fn shared() -> arc::R<Self>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, delegate: Option<&D>);

    #[objc::msg_send(run)]
    pub fn run(&mut self);

    #[objc::msg_send(terminate:)]
    pub fn terminate(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(dockTile)]
    pub fn dock_tile(&self) -> arc::R<ns::DockTile>;

    #[objc::msg_send(mainWindow)]
    pub fn main_window(&self) -> Option<arc::R<ns::Window>>;

    #[objc::msg_send(keyWindow)]
    pub fn key_window(&self) -> Option<arc::R<ns::Window>>;
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_APPLICATION: &'static objc::Class<App>;
}

pub mod notifications {
    use crate::{api, ns::NotificationName};

    #[doc(alias = "NSApplicationDidBecomeActiveNotification")]
    pub fn did_become_active() -> &'static NotificationName {
        unsafe { NSApplicationDidBecomeActiveNotification }
    }

    #[doc(alias = "NSApplicationDidHideNotification")]
    pub fn did_hide() -> &'static NotificationName {
        unsafe { NSApplicationDidHideNotification }
    }

    #[doc(alias = "NSApplicationDidFinishLaunchingNotification")]
    pub fn did_finish_launching() -> &'static NotificationName {
        unsafe { NSApplicationDidFinishLaunchingNotification }
    }

    #[doc(alias = "NSApplicationDidResignActiveNotification")]
    pub fn did_resign_active() -> &'static NotificationName {
        unsafe { NSApplicationDidResignActiveNotification }
    }

    #[doc(alias = "NSApplicationDidUnhideNotification")]
    pub fn did_unhide() -> &'static NotificationName {
        unsafe { NSApplicationDidUnhideNotification }
    }

    #[doc(alias = "NSApplicationDidUpdateNotification")]
    pub fn did_update() -> &'static NotificationName {
        unsafe { NSApplicationDidUpdateNotification }
    }

    #[doc(alias = "NSApplicationWillBecomeActiveNotification")]
    pub fn will_become_active() -> &'static NotificationName {
        unsafe { NSApplicationWillBecomeActiveNotification }
    }

    #[doc(alias = "NSApplicationWillHideNotification")]
    pub fn will_hide() -> &'static NotificationName {
        unsafe { NSApplicationWillHideNotification }
    }

    #[doc(alias = "NSApplicationWillFinishLaunchingNotification")]
    pub fn will_finish_launching() -> &'static NotificationName {
        unsafe { NSApplicationWillFinishLaunchingNotification }
    }

    #[doc(alias = "NSApplicationWillResignActiveNotification")]
    pub fn will_resign_active() -> &'static NotificationName {
        unsafe { NSApplicationWillResignActiveNotification }
    }
    #[doc(alias = "NSApplicationWillUnhideNotification")]
    pub fn will_unhide() -> &'static NotificationName {
        unsafe { NSApplicationWillUnhideNotification }
    }

    #[doc(alias = "NSApplicationWillUpdateNotification")]
    pub fn will_update() -> &'static NotificationName {
        unsafe { NSApplicationWillUpdateNotification }
    }

    #[doc(alias = "NSApplicationWillTerminateNotification")]
    pub fn will_terminate() -> &'static NotificationName {
        unsafe { NSApplicationWillTerminateNotification }
    }

    #[doc(alias = "NSApplicationDidChangeScreenParametersNotification")]
    pub fn did_change_screen_params() -> &'static NotificationName {
        unsafe { NSApplicationDidChangeScreenParametersNotification }
    }

    #[doc(alias = "NSApplicationProtectedDataWillBecomeUnavailableNotification")]
    #[api::available(macos = 12.0)]
    pub fn protected_data_will_become_unavailable() -> &'static NotificationName {
        unsafe { NSApplicationProtectedDataWillBecomeUnavailableNotification }
    }

    #[doc(alias = "NSApplicationProtectedDataDidBecomeAvailableNotification")]
    #[api::available(macos = 12.0)]
    pub fn protected_data_did_become_available() -> &'static NotificationName {
        unsafe { NSApplicationProtectedDataDidBecomeAvailableNotification }
    }

    #[link(name = "AppKit", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
        static NSApplicationDidBecomeActiveNotification: &'static NotificationName;
        static NSApplicationDidHideNotification: &'static NotificationName;
        static NSApplicationDidFinishLaunchingNotification: &'static NotificationName;
        static NSApplicationDidResignActiveNotification: &'static NotificationName;
        static NSApplicationDidUnhideNotification: &'static NotificationName;
        static NSApplicationDidUpdateNotification: &'static NotificationName;
        static NSApplicationWillBecomeActiveNotification: &'static NotificationName;
        static NSApplicationWillHideNotification: &'static NotificationName;
        static NSApplicationWillFinishLaunchingNotification: &'static NotificationName;
        static NSApplicationWillResignActiveNotification: &'static NotificationName;
        static NSApplicationWillUnhideNotification: &'static NotificationName;
        static NSApplicationWillUpdateNotification: &'static NotificationName;
        static NSApplicationWillTerminateNotification: &'static NotificationName;
        static NSApplicationDidChangeScreenParametersNotification: &'static NotificationName;
        #[api::available(macos = 12.0)]
        static NSApplicationProtectedDataWillBecomeUnavailableNotification:
            &'static NotificationName;
        #[api::available(macos = 12.0)]
        static NSApplicationProtectedDataDidBecomeAvailableNotification: &'static NotificationName;
    }
}
