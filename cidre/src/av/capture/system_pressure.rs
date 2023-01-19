#[cfg(not(target_os = "macos"))]
use crate::{cf, define_cf_type, define_obj_type, ns};

#[cfg(not(target_os = "macos"))]
define_cf_type!(Level(cf::String));

#[cfg(not(target_os = "macos"))]
impl Level {
    #[inline]
    pub fn nominal() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelNominal }
    }

    #[inline]
    pub fn fair() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelFair }
    }

    #[inline]
    pub fn serious() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelSerious }
    }

    #[inline]
    pub fn critical() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelCritical }
    }

    #[inline]
    pub fn shutdown() -> &'static Self {
        unsafe { AVCaptureSystemPressureLevelShutdown }
    }
}

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Factors {
    None = 0,
    SystemTemperature = 1 << 0,
    PeakPower = 1 << 1,
    DepthModuleTemperature = 1 << 2,
}

#[cfg(not(target_os = "macos"))]
define_obj_type!(State(ns::Id));

#[cfg(not(target_os = "macos"))]
impl State {
    pub fn level(&self) -> &Level {
        unsafe { rsel_state_level(self) }
    }

    pub fn factors(&self) -> Factors {
        unsafe { rsel_state_factors(self) }
    }
}

#[cfg(not(target_os = "macos"))]
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureSystemPressureLevelNominal: &'static Level;
    static AVCaptureSystemPressureLevelFair: &'static Level;
    static AVCaptureSystemPressureLevelSerious: &'static Level;
    static AVCaptureSystemPressureLevelCritical: &'static Level;
    static AVCaptureSystemPressureLevelShutdown: &'static Level;

    fn rsel_state_level(id: &ns::Id) -> &Level;
    fn rsel_state_factors(id: &ns::Id) -> Factors;
}