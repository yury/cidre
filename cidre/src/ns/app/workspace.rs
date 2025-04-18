use crate::{arc, define_cls, define_obj_type, mach::CpuType, ns, objc};

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

    /// Open a URL, using the default handler for the URL's scheme.
    #[objc::msg_send(openURL:)]
    pub fn open_url(&self, url: &ns::Url) -> bool;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(openURL:configuration:completionHandler:)]
    pub fn open_url_with_cfg_ch_block(
        &self,
        url: &ns::Url,
        configuration: &WorkspaceOpenCfg,
        ch: Option<&mut blocks::ResultCh<ns::RunningApp>>,
    );

    #[cfg(feature = "blocks")]
    pub fn open_url_with_cfg_ch(
        &self,
        url: &ns::Url,
        configuration: &WorkspaceOpenCfg,
        ch: impl FnMut(Option<&ns::RunningApp>, Option<&ns::Error>) + 'static,
    ) {
        let mut block = blocks::ResultCh::new2(ch);
        self.open_url_with_cfg_ch_block(url, configuration, Some(&mut block));
    }

    #[cfg(feature = "async")]
    pub async fn open_url_with_cfg(
        &self,
        url: &ns::Url,
        configuration: &WorkspaceOpenCfg,
    ) -> Result<arc::R<ns::RunningApp>, arc::R<ns::Error>> {
        let (comp, mut block) = blocks::result();
        self.open_url_with_cfg_ch_block(url, configuration, Some(&mut block));
        comp.await
    }
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

define_obj_type!(
    #[doc(alias = "NSWorkspaceOpenConfiguration")]
    pub WorkspaceOpenCfg(ns::Id),
    NS_WORKSPACE_OPEN_CONFIGURATION
);

impl WorkspaceOpenCfg {
    /// Display user interface elements if needed, including errors and authentication. Defaults to true.
    ///
    /// The completion handler will not be invoked until the user dismisses any such UI.
    /// Gatekeeper UI is not affected and will always be presented if needed.
    #[objc::msg_send(promptsUserIfNeeded)]
    pub fn prompts_user_if_needed(&self) -> bool;

    #[objc::msg_send(setPromptsUserIfNeeded:)]
    pub fn set_prompts_user_if_needed(&mut self, val: bool);

    /// Add the application instance or documents to the Recent Items menu. Defaults to true.
    #[objc::msg_send(addsToRecentItems)]
    pub fn adds_to_recent_items(&self) -> bool;

    #[objc::msg_send(setAddsToRecentItems:)]
    pub fn set_adds_to_recent_items(&mut self, val: bool);

    /// Bring the application instance to the foreground. Defaults to true.
    #[objc::msg_send(activates)]
    pub fn activates(&self) -> bool;

    #[objc::msg_send(setActivates:)]
    pub fn set_activates(&mut self, val: bool);

    /// Hide the application instance. Defaults to false.
    #[objc::msg_send(hides)]
    pub fn hides(&self) -> bool;

    #[objc::msg_send(setHides:)]
    pub fn set_hides(&mut self, val: bool);

    /// Hide other application instances. Defaults to false.
    #[objc::msg_send(hidesOthers)]
    pub fn hides_others(&self) -> bool;

    #[objc::msg_send(setHidesOthers:)]
    pub fn set_hides_others(&mut self, val: bool);

    /// Print documents and URLs instead of opening them. Defaults to false.
    #[objc::msg_send(isForPrinting)]
    pub fn is_for_printing(&self) -> bool;

    #[objc::msg_send(setForPrinting:)]
    pub fn set_for_printing(&mut self, val: bool);

    /// gnore any running instance of the application and launch a new one. Defaults to false.
    ///
    /// (i.e. prefers to reuse a running instance). When createsNewApplicationInstance is YES, the value of
    /// allowsRunningApplicationSubstitution is not consulted
    #[objc::msg_send(createsNewApplicationInstance)]
    pub fn creates_new_app_inst(&self) -> bool;

    #[objc::msg_send(setCreatesNewApplicationInstance:)]
    pub fn set_creates_new_app_inst(&mut self, val: bool);

    // Command-line arguments to pass to a new application instance. Defaults to empty.
    #[objc::msg_send(arguments)]
    pub fn args(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(setArguments:)]
    pub fn set_args(&mut self, val: &ns::Array<ns::String>);

    /// Environment variables to set in a new application instance. Defaults to empty.
    ///
    /// Only applies when a new application instance is created.
    /// Additional environment variables may be included by the system.
    #[objc::msg_send(environment)]
    pub fn env(&self) -> arc::R<ns::Dictionary<ns::String, ns::String>>;

    #[objc::msg_send(setEnvironment:)]
    pub fn set_env(&self, val: &ns::Dictionary<ns::String, ns::String>);

    // todo: ...

    /// A cpu_type_t (from <mach/machine.h>) that specifies the architecture to launch.
    /// Ignored if a new instance is not launched. Defaults to CPU_TYPE_ANY. When CPU_TYPE_ANY,
    /// the system will decide the preferred architecture based on those present in the executable.
    #[objc::msg_send(architecture)]
    pub fn arch(&self) -> CpuType;

    #[objc::msg_send(setArchitecture:)]
    pub fn set_arch(&mut self, val: CpuType);

    /// Only open the provided URL if it is a valid universal link with an application configured to open it.
    ///
    /// If there is no application configured, or the user disabled using it to open the link, then the open will fail with an error. Defaults to false.
    #[objc::msg_send(requiresUniversalLinks)]
    pub fn requires_universal_links(&self) -> bool;

    #[objc::msg_send(setRequiresUniversalLinks:)]
    pub fn set_requires_universal_links(&mut self, val: bool);
}

pub mod notification {
    use crate::ns;

    #[doc(alias = "NSWorkspaceApplicationKey")]
    #[inline]
    pub fn app_key() -> &'static ns::String {
        unsafe { NSWorkspaceApplicationKey }
    }

    #[doc(alias = "NSWorkspaceWillLaunchApplicationNotification")]
    #[inline]
    pub fn will_launch_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceWillLaunchApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceDidLaunchApplicationNotification")]
    #[inline]
    pub fn did_launch_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidLaunchApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceDidTerminateApplicationNotification")]
    #[inline]
    pub fn did_terminate_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidTerminateApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceDidHideApplicationNotification")]
    #[inline]
    pub fn did_hide_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidHideApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceDidUnhideApplicationNotification")]
    #[inline]
    pub fn did_unhide_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidUnhideApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceDidActivateApplicationNotification")]
    #[inline]
    pub fn did_activate_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidActivateApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceDidDeactivateApplicationNotification")]
    #[inline]
    pub fn did_deactivate_app() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidDeactivateApplicationNotification }
    }

    #[doc(alias = "NSWorkspaceWillPowerOffNotification")]
    #[inline]
    pub fn will_power_off() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceWillPowerOffNotification }
    }

    #[doc(alias = "NSWorkspaceWillSleepNotification")]
    #[inline]
    pub fn will_sleep() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceWillSleepNotification }
    }

    #[doc(alias = "NSWorkspaceDidWakeNotification")]
    #[inline]
    pub fn did_wake() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidWakeNotification }
    }

    #[doc(alias = "NSWorkspaceScreensDidSleepNotification")]
    #[inline]
    pub fn screens_did_sleep() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceScreensDidSleepNotification }
    }

    #[doc(alias = "NSWorkspaceScreensDidWakeNotification")]
    #[inline]
    pub fn screens_did_wake() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceScreensDidWakeNotification }
    }

    #[doc(alias = "NSWorkspaceSessionDidBecomeActiveNotification")]
    #[inline]
    pub fn session_did_become_active() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceSessionDidBecomeActiveNotification }
    }

    #[doc(alias = "NSWorkspaceSessionDidResignActiveNotification")]
    #[inline]
    pub fn session_did_resign_active() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceSessionDidResignActiveNotification }
    }

    #[doc(alias = "NSWorkspaceDidChangeFileLabelsNotification")]
    #[inline]
    pub fn did_change_file_labels() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceDidChangeFileLabelsNotification }
    }

    #[doc(alias = "NSWorkspaceActiveSpaceDidChangeNotification")]
    #[inline]
    pub fn active_space_did_change() -> &'static ns::NotificationName {
        unsafe { NSWorkspaceActiveSpaceDidChangeNotification }
    }

    #[link(name = "AppKit", kind = "framework")]
    unsafe extern "C" {
        static NSWorkspaceApplicationKey: &'static ns::String;

        static NSWorkspaceWillLaunchApplicationNotification: &'static ns::NotificationName;
        static NSWorkspaceDidLaunchApplicationNotification: &'static ns::NotificationName;
        static NSWorkspaceDidTerminateApplicationNotification: &'static ns::NotificationName;
        static NSWorkspaceDidHideApplicationNotification: &'static ns::NotificationName;
        static NSWorkspaceDidUnhideApplicationNotification: &'static ns::NotificationName;
        static NSWorkspaceDidActivateApplicationNotification: &'static ns::NotificationName;
        static NSWorkspaceDidDeactivateApplicationNotification: &'static ns::NotificationName;

        static NSWorkspaceWillPowerOffNotification: &'static ns::NotificationName;
        static NSWorkspaceWillSleepNotification: &'static ns::NotificationName;
        static NSWorkspaceDidWakeNotification: &'static ns::NotificationName;
        static NSWorkspaceScreensDidSleepNotification: &'static ns::NotificationName;
        static NSWorkspaceScreensDidWakeNotification: &'static ns::NotificationName;
        static NSWorkspaceSessionDidBecomeActiveNotification: &'static ns::NotificationName;
        static NSWorkspaceSessionDidResignActiveNotification: &'static ns::NotificationName;

        static NSWorkspaceDidChangeFileLabelsNotification: &'static ns::NotificationName;
        static NSWorkspaceActiveSpaceDidChangeNotification: &'static ns::NotificationName;
    }
}

unsafe extern "C" {
    static NS_WORKSPACE: &'static objc::Class<Workspace>;
    static NS_WORKSPACE_OPEN_CONFIGURATION: &'static objc::Class<WorkspaceOpenCfg>;
}

#[cfg(test)]
mod tests {
    use crate::mach::CpuType;
    use crate::ns;

    #[test]
    fn workspace_open_cfg_basics() {
        let cfg = ns::WorkspaceOpenCfg::new();
        assert!(cfg.prompts_user_if_needed());
        assert!(cfg.adds_to_recent_items());
        assert!(cfg.activates());
        assert!(!cfg.hides());
        assert!(!cfg.hides_others());
        assert!(!cfg.is_for_printing());
        assert!(!cfg.creates_new_app_inst());
        assert!(cfg.args().is_empty());
        assert!(cfg.env().is_empty());
        assert_eq!(cfg.arch(), CpuType::ANY);
        assert!(!cfg.requires_universal_links());
    }

    #[tokio::test]
    async fn open_url() {
        let mut cfg = ns::WorkspaceOpenCfg::new();
        cfg.set_prompts_user_if_needed(false);

        let res = ns::Workspace::shared()
            .open_url_with_cfg(&ns::Url::with_str("unknwonwn_url").unwrap(), &cfg)
            .await;

        assert!(res.is_err());
    }
}
