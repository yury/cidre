//! NSAccessibility.h workspace display preferences.
use crate::{ns, objc};

impl ns::Workspace {
    /// Whether the user requests reduced motion in animations.
    #[objc::msg_send(accessibilityDisplayShouldReduceMotion)]
    pub fn accessibility_display_should_reduce_motion(&self) -> bool;
}
