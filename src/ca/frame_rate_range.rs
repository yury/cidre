/// ```
/// use cidre::ca;
///
/// let frr = ca::FrameRateRange::default();
/// let zero = ca::FrameRateRange { minium: 0.0, maximum: 0.0, preferred: 0.0 };
///
/// // assert_ne!(frr, zero);
/// ```
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FrameRateRange {
    pub minium: f32,
    pub maximum: f32,
    pub preferred: f32,
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

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {
    static CAFrameRateRangeDefault: FrameRateRange;
    fn CAFrameRateRangeIsEqualToRange(range: FrameRateRange, other: FrameRateRange) -> bool;
}
