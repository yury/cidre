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
    #[objc::cls_msg_send(processInfo)]
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
