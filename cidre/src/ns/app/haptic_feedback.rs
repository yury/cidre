use crate::{arc, define_obj_type, ns, objc};

#[doc(alias = "NSHapticFeedbackPattern")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum HapticFeedbackPattern {
    Generic = 0,
    /// Something lined up or snapped into place.
    Alignment = 1,
    /// A value passed a discrete step.
    LevelChange = 2,
}

#[doc(alias = "NSHapticFeedbackPerformanceTime")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum HapticFeedbackPerformanceTime {
    Default = 0,
    Now = 1,
    /// With the next frame drawn, so it lines up with what shows.
    DrawCompleted = 2,
}

/// Plays haptics on a Force Touch trackpad; nothing where there is none.
#[objc::protocol(NSHapticFeedbackPerformer)]
pub trait HapticFeedbackPerformer: objc::Obj {
    #[objc::msg_send(performFeedbackPattern:performanceTime:)]
    fn perform(&self, pattern: HapticFeedbackPattern, time: HapticFeedbackPerformanceTime);
}

define_obj_type!(
    pub AnyHapticFeedbackPerformer(ns::Id)
);

impl HapticFeedbackPerformer for AnyHapticFeedbackPerformer {}

define_obj_type!(
    #[doc(alias = "NSHapticFeedbackManager")]
    pub HapticFeedbackManager(ns::Id),
    NS_HAPTIC_FEEDBACK_MANAGER
);

impl HapticFeedbackManager {
    /// The performer for the device the user is on.
    #[objc::msg_send(defaultPerformer)]
    pub fn default_performer() -> arc::R<AnyHapticFeedbackPerformer>;
}

unsafe extern "C" {
    static NS_HAPTIC_FEEDBACK_MANAGER: &'static objc::Class<HapticFeedbackManager>;
}

#[cfg(test)]
mod tests {
    use crate::ns::{self, HapticFeedbackPerformer as _};

    #[test]
    fn performer() {
        // No trackpad in a test runner, or none that clicks: a no-op either way.
        ns::HapticFeedbackManager::default_performer().perform(
            ns::HapticFeedbackPattern::Alignment,
            ns::HapticFeedbackPerformanceTime::Now,
        );
    }
}
