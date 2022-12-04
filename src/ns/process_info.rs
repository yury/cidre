use crate::{define_obj_type, ns, msg_send};

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
    pub fn current() -> &'static ProcessInfo {
        unsafe { NSProcessInfo_processInfo() }
    }

    #[inline]
    pub fn thermal_state(&self) -> ThermalState {
        unsafe { rsel_thermalState(self) }
    }

    #[inline]
    pub fn is_low_power_mode_enabled(&self) -> bool {
        unsafe { rsel_isLowPowerModeEnabled(self) }
    }

    /// ```
    /// use cidre::ns;
    /// 
    /// let pi = ns::ProcessInfo::current();
    /// let count = pi.processor_count();
    /// ```
    #[inline]
    pub fn processor_count(&self) -> usize {
        unsafe {
            self.sel(crate::objc::sel_processor_count())
        }
    }

    #[inline]
    pub fn processor_count2(&self) -> usize {
        unsafe {
            rsel_processorCount(self)
        }
    }

    #[inline]
    pub fn processor_count3(&self) -> usize {
        unsafe {
            msg_send!("common", self, sel_processorCount)
        }
    }


    #[inline]
    pub fn active_processor_count(&self) -> usize {
        unsafe { rsel_activeProcessorCount(self) }
    }

    #[inline]
    pub fn is_mac_catalyst_app(&self) -> bool {
        unsafe { rsel_isMacCatalystApp(self) }
    }

    #[inline]
    pub fn is_ios_app_on_mac(&self) -> bool {
        unsafe { rsel_isiOSAppOnMac(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSProcessInfo_processInfo() -> &'static ProcessInfo;
    fn rsel_thermalState(id: &ns::Id) -> ThermalState;
    fn rsel_isLowPowerModeEnabled(id: &ns::Id) -> bool;
    fn rsel_processorCount(id: &ns::Id) -> usize;
    fn rsel_activeProcessorCount(id: &ns::Id) -> usize;

    fn rsel_isMacCatalystApp(id: &ns::Id) -> bool;
    fn rsel_isiOSAppOnMac(id: &ns::Id) -> bool;
}
