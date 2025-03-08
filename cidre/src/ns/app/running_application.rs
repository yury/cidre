use crate::{arc, define_obj_type, ns, objc, sys};

define_obj_type!(
    #[doc(alias = "NSRunningApplication")]
    pub RunningApp(ns::Id),
    NS_RUNNING_APPLICATION
);

impl RunningApp {
    #[objc::msg_send(isTerminated)]
    pub fn is_terminated(&self) -> bool;

    #[objc::msg_send(isFinishedLaunching)]
    pub fn is_finished_launching(&self) -> bool;

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;

    #[objc::msg_send(ownsMenuBar)]
    pub fn owns_menu_bar(&self) -> bool;

    // activationPolicy

    #[objc::msg_send(localizedName)]
    pub fn localized_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(bundleIdentifier)]
    pub fn bundle_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(bundleURL)]
    pub fn bundle_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(executableURL)]
    pub fn executable_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(processIdentifier)]
    pub fn pid(&self) -> sys::Pid;

    #[objc::msg_send(launchDate)]
    pub fn launch_date(&self) -> Option<arc::R<ns::Date>>;

    #[objc::msg_send(icon)]
    pub fn icon(&self) -> Option<arc::R<ns::Image>>;

    #[objc::msg_send(executableArchitecture)]
    pub fn executable_arch(&self) -> ns::Integer;

    #[objc::msg_send(hide)]
    pub fn hide(&self) -> bool;

    #[objc::msg_send(unhide)]
    pub fn unhide(&self) -> bool;

    #[objc::msg_send(terminate)]
    pub fn terminate(&self) -> bool;

    #[objc::msg_send(forceTerminate)]
    pub fn force_terminate(&self) -> bool;

    #[objc::msg_send(currentApplication)]
    pub fn current() -> arc::R<Self>;

    #[objc::msg_send(runningApplicationWithProcessIdentifier:)]
    pub fn with_pid(pid: sys::Pid) -> Option<arc::R<Self>>;

    /// An array of currently running applications with the given bundle identifier, or an empty array if no apps match.
    #[objc::msg_send(runningApplicationsWithBundleIdentifier:)]
    pub fn with_bundle_id(bundle_id: &ns::String) -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(terminateAutomaticallyTerminableApplications)]
    pub fn terminate_automatically_terminable_apps();
}

/// NSWorkspaceRunningApplications
impl ns::Workspace {
    #[objc::msg_send(runningApplications)]
    pub fn running_apps(&self) -> arc::R<ns::Array<RunningApp>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;
    #[test]
    fn basics() {
        let apps = ns::Workspace::shared().running_apps();
        assert!(!apps.is_empty());

        let app = apps.get(0).unwrap();
        assert!(!app.is_terminated());

        let _app = ns::RunningApp::new();

        let _app = ns::RunningApp::current();
    }
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_RUNNING_APPLICATION: &'static objc::Class<RunningApp>;
}
