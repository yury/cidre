
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