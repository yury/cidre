/// ```
/// use cidre::ca;
///
/// let frr = ca::FrameRateRange::default();
/// let zero = ca::FrameRateRange { min: 0.0, max: 0.0, preferred: 0.0 };
///
/// // assert_ne!(frr, zero);
/// ```
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FrameRateRange {
    pub min: f32,
    pub max: f32,
    pub preferred: f32,
}

impl FrameRateRange {
    pub fn new(fps: f32) -> Self {
        Self {
            min: fps,
            max: fps,
            preferred: fps,
        }
    }
}

impl Default for FrameRateRange {
    fn default() -> Self {
        unsafe { CAFrameRateRangeDefault }
    }
}

impl PartialEq<Self> for FrameRateRange {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { CAFrameRateRangeIsEqualToRange(*self, *other) }
    }
}

unsafe extern "C-unwind" {
    static CAFrameRateRangeDefault: FrameRateRange;
    fn CAFrameRateRangeIsEqualToRange(range: FrameRateRange, other: FrameRateRange) -> bool;
}
