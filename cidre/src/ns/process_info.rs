use crate::{arc, define_cls, define_obj_type, ns, objc, sys};

#[doc(alias = "NSProcessInfoThermalState")]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(isize)]
pub enum ThermalState {
    /// No corrective action is needed.
    Nominal,

    /// The system has reached a state where fans may become audible
    /// (on systems which have fans). Recommendation: Defer non-user-visible activity.
    Fair,

    /// Fans are running at maximum speed (on systems which have fans), system performance
    /// may be impacted. Recommendation: reduce application's usage of CPU, GPU and I/O,
    /// if possible. Switch to lower quality visual effects, reduce frame rates.
    Serious,

    /// System performance is significantly impacted and the system needs to cool down.
    /// Recommendation: reduce application's usage of CPU, GPU, and I/O to the minimum
    /// level needed to respond to user actions. Consider stopping use of camera and
    /// other peripherals if your application is using them.
    Critical,
}

define_obj_type!(
    #[doc(alias = "NSProcessInfo")]
    pub ProcessInfo(ns::Id)
);

unsafe impl Send for ProcessInfo {}

impl ns::KvObserverRegistration for ProcessInfo {}

#[doc(alias = "NSOperatingSystemVersion")]
pub type OsVersion = crate::api::OsVersion;

impl ProcessInfo {
    define_cls!(NS_PROCESS_INFO);
    /// ```
    /// use cidre::ns;
    ///
    /// let pi = ns::ProcessInfo::current();
    ///
    /// assert_ne!(pi.thermal_state(), ns::ProcessInfoThermalState::Critical);
    /// assert_eq!(pi.is_low_power_mode_enabled(), false);
    /// assert!(pi.processor_count() > 1);
    /// assert!(pi.active_processor_count() > 1);
    /// ```
    #[objc::msg_send(processInfo)]
    pub fn current() -> &'static mut ProcessInfo;

    #[objc::msg_send(environment)]
    pub fn env(&self) -> arc::R<ns::Dictionary<ns::String, ns::String>>;

    #[objc::msg_send(arguments)]
    pub fn args(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(hostName)]
    pub fn host_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(processName)]
    pub fn process_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(processIdentifier)]
    pub fn process_id(&self) -> sys::Pid;

    #[objc::msg_send(globallyUniqueString)]
    pub fn globally_unique_string(&self) -> arc::R<ns::String>;

    /// Human readable, localized; appropriate for displaying to user or using in bug emails and such;
    /// NOT appropriate for parsing
    #[objc::msg_send(operatingSystemVersionString)]
    pub fn os_version_string(&self) -> arc::R<ns::String>;

    #[objc::msg_send(operatingSystemVersion)]
    #[objc::available(macos = 10.10, ios = 8.0, watchos = 2.0, tvos = 9.0)]
    pub fn os_version(&self) -> OsVersion;

    #[objc::msg_send(thermalState)]
    // macosx(10.10.3), ios(11.0), watchos(4.0), tvos(11.0)
    // #[objc::available(macos = "10.10.3", ios = 11.0, watchos = 4.0, tvos = 11.0)]
    pub fn thermal_state(&self) -> ThermalState;

    #[objc::msg_send(isLowPowerModeEnabled)]
    #[objc::available(macos = 12.0, ios = 9.0, watchos = 2.0, tvos = 9.0)]
    pub fn is_low_power_mode_enabled(&self) -> bool;

    #[objc::msg_send(processorCount)]
    #[objc::available(macos = 10.5, ios = 2.0, watchos = 2.0, tvos = 9.0)]
    pub fn processor_count(&self) -> usize;

    #[objc::msg_send(activeProcessorCount)]
    #[objc::available(macos = 10.5, ios = 2.0, watchos = 2.0, tvos = 9.0)]
    pub fn active_processor_count(&self) -> usize;

    #[objc::msg_send(physicalMemory)]
    #[objc::available(macos = 10.5, ios = 2.0, watchos = 2.0, tvos = 9.0)]
    pub fn physical_mem(&self) -> usize;

    #[objc::msg_send(isOperatingSystemAtLeastVersion:)]
    #[objc::available(macos = 10.10, ios = 8.0, watchos = 2.0, tvos = 9.0)]
    pub fn is_os_at_least_version(&self, ver: OsVersion) -> bool;

    #[objc::msg_send(systemUptime)]
    #[objc::available(macos = 10.6, ios = 4.0, watchos = 2.0, tvos = 9.0)]
    pub fn system_uptime(&self) -> ns::TimeInterval;
}

/// NSProcessInfoPlatform
impl ProcessInfo {
    #[objc::msg_send(isMacCatalystApp)]
    #[objc::available(macos = 10.15, ios = 13.0, watchos = 6.0, tvos = 13.0)]
    pub fn is_mac_catalyst_app(&self) -> bool;

    #[objc::msg_send(isiOSAppOnMac)]
    #[objc::available(macos = 11.0, ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn is_ios_app_on_mac(&self) -> bool;

    #[objc::msg_send(isiOSAppOnVision)]
    #[objc::available(macos = 26.1, ios = 26.1, watchos = 26.1, tvos = 26.1, visionos = 26.1)]
    pub fn is_ios_app_on_vision(&self) -> bool;
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_PROCESS_INFO: &'static objc::Class<ProcessInfo>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let pi = ns::ProcessInfo::current();

        assert_ne!(pi.thermal_state(), ns::ProcessInfoThermalState::Critical);
        assert_eq!(pi.is_low_power_mode_enabled(), false);
        assert!(pi.processor_count() > 1);
        assert!(pi.active_processor_count() > 1);
        assert!(!pi.is_ios_app_on_mac());
        assert!(!pi.is_mac_catalyst_app());
    }
}
