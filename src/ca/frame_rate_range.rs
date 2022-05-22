#[repr(C)]
pub struct FrameRateRage {
    pub minium: f32,
    pub maximum: f32,
    pub preferred: f32,
}

impl Default for FrameRateRage {
    fn default() -> Self {
        unsafe { CAFrameRateRangeDefault() }
    }
}

extern "C" {
    fn CAFrameRateRangeDefault() -> FrameRateRage;
}
