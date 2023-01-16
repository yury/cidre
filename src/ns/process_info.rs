use crate::{
    define_obj_type, ns,
    objc::{self, msg_send, Class, Obj},
};

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

define_obj_type!(ProcessInfo(ns::Id));

impl ProcessInfo {
    /// ```no_run
    /// use cidre::ns;
    ///
    /// let pi = ns::ProcessInfo::current();
    ///
    /// assert_ne!(pi.thermal_state(), ns::ProcessInfoThermalState::Critical);
    /// assert_eq!(pi.is_low_power_mode_enabled(), false);
    /// assert!(pi.processor_count() > 1);
    /// assert!(pi.active_processor_count() > 1);
    /// ```
    pub fn current() -> &'static ProcessInfo {
        unsafe { NS_PROCESS_INFO.call0(msg_send::process_info) }
    }

    #[inline]
    pub fn thermal_state(&self) -> ThermalState {
        unsafe { self.call0(objc::msg_send::thermal_state) }
    }

    #[inline]
    pub fn is_low_power_mode_enabled(&self) -> bool {
        unsafe { self.call0(msg_send::is_low_power_mode_enabled) }
    }

    #[inline]
    pub fn processor_count(&self) -> usize {
        unsafe { self.call0(msg_send::processor_count) }
    }

    #[inline]
    pub fn active_processor_count(&self) -> usize {
        unsafe { self.call0(msg_send::active_processor_count) }
    }

    #[inline]
    pub fn is_mac_catalyst_app(&self) -> bool {
        unsafe { self.call0(msg_send::is_mac_catalyst_app) }
    }

    #[inline]
    pub fn is_ios_app_on_mac(&self) -> bool {
        unsafe { self.call0(msg_send::is_ios_app_on_mac) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_PROCESS_INFO: &'static Class<ProcessInfo>;
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
    }
}
