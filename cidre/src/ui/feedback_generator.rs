use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    /// Haptics for a value clicking into place, as a picker's wheel or a snap does.
    #[doc(alias = "UISelectionFeedbackGenerator")]
    pub SelectionFeedbackGenerator(ns::Id),
    sym UISelectionFeedbackGenerator
);

impl SelectionFeedbackGenerator {
    /// Wakes the Taptic Engine, so the next feedback plays without delay.
    #[objc::msg_send(prepare)]
    pub fn prepare(&self);

    #[objc::msg_send(selectionChanged)]
    pub fn selection_changed(&self);
}
