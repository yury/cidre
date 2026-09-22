//! UIAccessibility.h display preferences.

/// Whether the user requests reduced motion in animations.
#[doc(alias = "UIAccessibilityIsReduceMotionEnabled")]
pub fn is_reduce_motion_enabled() -> bool {
    unsafe { UIAccessibilityIsReduceMotionEnabled() }
}

unsafe extern "C" {
    fn UIAccessibilityIsReduceMotionEnabled() -> bool;
}
