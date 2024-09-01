use std::str::FromStr;

use crate::{arc, define_cls, define_obj_type, ns, objc};

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

impl ns::KVObserverRegistration for ProcessInfo {}

#[doc(alias = "NSOperatingSystemVersion")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OsVersion {
    pub major: isize,
    pub minor: isize,
    pub patch: isize,
}

#[repr(u32)]
pub enum Platform {
    MacOs = 1,
    IOs = 2,
    TvOs = 3,
    WatchOs = 4,
    // DriverKit = 10,
    VisionOs = 11,
}

extern "C" {
    fn __isPlatformVersionAtLeast(platform: u32, major: u32, minor: u32, patch: u32) -> i32;
}

impl OsVersion {
    #[inline]
    pub fn platform_at_least(&self, platform: Platform) -> bool {
        unsafe {
            __isPlatformVersionAtLeast(
                platform as _,
                self.major as _,
                self.minor as _,
                self.patch as _,
            ) != 0
        }
    }

    #[cfg(target_os = "macos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::MacOs)
    }

    #[cfg(target_os = "ios")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::IOs)
    }

    #[cfg(target_os = "tvos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::TvOs)
    }

    #[cfg(target_os = "watchos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::WatchOs)
    }

    #[cfg(target_os = "visionos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::VisionOs)
    }
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

    #[objc::msg_send(environment)]
    pub fn env(&self) -> arc::R<ns::Dictionary<ns::String, ns::String>>;

    #[objc::msg_send(arguments)]
    pub fn args(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(hostName)]
    pub fn host_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(processName)]
    pub fn process_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(processIdentifier)]
    pub fn process_id(&self) -> i32;

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

    #[objc::msg_send(isMacCatalystApp)]
    #[objc::available(macos = 10.15, ios = 13.0, watchos = 6.0, tvos = 13.0)]
    pub fn is_mac_catalyst_app(&self) -> bool;

    #[objc::msg_send(isiOSAppOnMac)]
    #[objc::available(macos = 11.0, ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn is_ios_app_on_mac(&self) -> bool;

    #[objc::msg_send(isOperatingSystemAtLeastVersion:)]
    #[objc::available(macos = 10.10, ios = 8.0, watchos = 2.0, tvos = 9.0)]
    pub fn is_os_at_least_version(&self, ver: OsVersion) -> bool;

    #[objc::msg_send(systemUptime)]
    #[objc::available(macos = 10.6, ios = 4.0, watchos = 2.0, tvos = 9.0)]
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
