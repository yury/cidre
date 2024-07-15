use std::str::FromStr;

use crate::{define_cls, define_obj_type, ns, objc};

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

define_obj_type!(pub ProcessInfo(ns::Id));

impl ns::KVObserverRegistration for ProcessInfo {}

#[doc(alias = "NSOperatingSystemVersion")]
pub struct OsVersion {
    pub major: isize,
    pub minor: isize,
    pub patch: isize,
}

#[derive(Debug)]
pub struct VersionError;

impl FromStr for OsVersion {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((major, rest)) = s.split_once('.') else {
            let Ok(major) = s.parse::<isize>() else {
                return Err(VersionError);
            };
            return Ok(Self {
                major,
                minor: 0,
                patch: 0,
            });
        };
        let Ok(major) = major.parse::<isize>() else {
            return Err(VersionError);
        };
        if let Some((minor, patch)) = rest.split_once('.') {
            let Ok(minor) = minor.parse::<isize>() else {
                return Err(VersionError);
            };
            let Ok(patch) = patch.parse::<isize>() else {
                return Err(VersionError);
            };
            return Ok(Self {
                major,
                minor,
                patch,
            });
        };
        let Ok(minor) = rest.parse::<isize>() else {
            return Err(VersionError);
        };
        Ok(Self {
            major,
            minor,
            patch: 0,
        })
    }
}

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

    #[objc::msg_send(thermalState)]
    pub fn thermal_state(&self) -> ThermalState;

    #[objc::msg_send(isLowPowerModeEnabled)]
    pub fn is_low_power_mode_enabled(&self) -> bool;

    #[objc::msg_send(processorCount)]
    pub fn processor_count(&self) -> usize;

    #[objc::msg_send(activeProcessorCount)]
    pub fn active_processor_count(&self) -> usize;

    #[objc::msg_send(isMacCatalystApp)]
    pub fn is_mac_catalyst_app(&self) -> bool;

    #[objc::msg_send(isiOSAppOnMac)]
    pub fn is_ios_app_on_mac(&self) -> bool;

    #[objc::msg_send(isOperatingSystemAtLeastVersion:)]
    pub fn is_os_at_least_version(&self, ver: OsVersion) -> bool;

    #[objc::msg_send(systemUptime)]
    pub fn system_uptime(&self) -> ns::TimeInterval;
}

#[link(name = "ns", kind = "static")]
extern "C" {
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
