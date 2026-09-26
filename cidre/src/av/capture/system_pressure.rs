use crate::{define_obj_type, define_opts, ns, objc};

define_obj_type!(pub Level(ns::String));

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

define_opts!(
    pub Factors(usize)
);

impl Factors {
    pub const NONE: Self = Self(0);
    pub const SYSTEM_TEMPERATURE: Self = Self(1 << 0);
    pub const PEAK_POWER: Self = Self(1 << 1);
    pub const DEPTH_MODULE_TEMPERATURE: Self = Self(1 << 2);

    /// The camera module is operating at an elevated temperature.
    #[doc(alias = "AVCaptureSystemPressureFactorCameraTemperature")]
    pub const CAMERA_TEMPERATURE: Self = Self(1 << 3);

    /// Under the current battery conditions, the device will shut down within 30 seconds
    /// if system load is not reduced.
    #[doc(alias = "AVCaptureSystemPressureFactorBatteryStress")]
    pub const BATTERY_STRESS: Self = Self(1 << 4);
}

define_obj_type!(pub State(ns::Id));

impl State {
    #[objc::msg_send(level)]
    pub fn level(&self) -> &Level;

    #[objc::msg_send(factors)]
    pub fn factors(&self) -> Factors;
}

unsafe extern "C" {
    static AVCaptureSystemPressureLevelNominal: &'static Level;
    static AVCaptureSystemPressureLevelFair: &'static Level;
    static AVCaptureSystemPressureLevelSerious: &'static Level;
    static AVCaptureSystemPressureLevelCritical: &'static Level;
    static AVCaptureSystemPressureLevelShutdown: &'static Level;
}
