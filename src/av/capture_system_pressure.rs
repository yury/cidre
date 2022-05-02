use crate::{cf, define_obj_type, objc::Id};

pub type Level = cf::String;

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


#[repr(usize)]
pub enum Factor {
    None = 0,
    SystemTemperature = 1 << 0,
    PeakPower = 1 << 1,
    DepthModuleTemperature = 1 << 2,
}

define_obj_type!(State(Id));

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureSystemPressureLevelNominal: &'static Level;
    static AVCaptureSystemPressureLevelFair: &'static Level;
    static AVCaptureSystemPressureLevelSerious: &'static Level;
    static AVCaptureSystemPressureLevelCritical: &'static Level;
    static AVCaptureSystemPressureLevelShutdown: &'static Level;
}