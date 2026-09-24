//! NSAccessibility.h: the workspace display preferences, the views' accessibility
//! properties, and posting notifications to assistive technologies.
use crate::{arc, ns, objc};

impl ns::Workspace {
    /// Whether the user requests reduced motion in animations.
    #[objc::msg_send(accessibilityDisplayShouldReduceMotion)]
    pub fn accessibility_display_should_reduce_motion(&self) -> bool;

    /// Whether VoiceOver is on.
    #[objc::msg_send(isVoiceOverEnabled)]
    pub fn is_voice_over_enabled(&self) -> bool;
}

/// What an element is, as VoiceOver says it: `AXButton`, `AXStaticText`, ...
#[doc(alias = "NSAccessibilityRole")]
pub type AccessibilityRole = ns::String;

/// What changed, for [`accessibility_post_notification`].
#[doc(alias = "NSAccessibilityNotificationName")]
pub type AccessibilityNotification = ns::String;

/// The roles of the common controls.
pub mod accessibility_role {
    use super::AccessibilityRole;

    #[doc(alias = "NSAccessibilityButtonRole")]
    pub fn button() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityButtonRole }
    }

    #[doc(alias = "NSAccessibilityCheckBoxRole")]
    pub fn check_box() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityCheckBoxRole }
    }

    #[doc(alias = "NSAccessibilityStaticTextRole")]
    pub fn static_text() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityStaticTextRole }
    }

    #[doc(alias = "NSAccessibilityImageRole")]
    pub fn image() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityImageRole }
    }

    #[doc(alias = "NSAccessibilityGroupRole")]
    pub fn group() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityGroupRole }
    }

    #[doc(alias = "NSAccessibilitySliderRole")]
    pub fn slider() -> &'static AccessibilityRole {
        unsafe { NSAccessibilitySliderRole }
    }

    #[doc(alias = "NSAccessibilityPopUpButtonRole")]
    pub fn pop_up_button() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityPopUpButtonRole }
    }

    #[doc(alias = "NSAccessibilityTextFieldRole")]
    pub fn text_field() -> &'static AccessibilityRole {
        unsafe { NSAccessibilityTextFieldRole }
    }

    unsafe extern "C" {
        static NSAccessibilityButtonRole: &'static AccessibilityRole;
        static NSAccessibilityCheckBoxRole: &'static AccessibilityRole;
        static NSAccessibilityStaticTextRole: &'static AccessibilityRole;
        static NSAccessibilityImageRole: &'static AccessibilityRole;
        static NSAccessibilityGroupRole: &'static AccessibilityRole;
        static NSAccessibilitySliderRole: &'static AccessibilityRole;
        static NSAccessibilityPopUpButtonRole: &'static AccessibilityRole;
        static NSAccessibilityTextFieldRole: &'static AccessibilityRole;
    }
}

/// The notifications posted most.
pub mod accessibility_notification {
    use super::AccessibilityNotification;

    #[doc(alias = "NSAccessibilityValueChangedNotification")]
    pub fn value_changed() -> &'static AccessibilityNotification {
        unsafe { NSAccessibilityValueChangedNotification }
    }

    #[doc(alias = "NSAccessibilityLayoutChangedNotification")]
    pub fn layout_changed() -> &'static AccessibilityNotification {
        unsafe { NSAccessibilityLayoutChangedNotification }
    }

    #[doc(alias = "NSAccessibilityFocusedUIElementChangedNotification")]
    pub fn focused_ui_element_changed() -> &'static AccessibilityNotification {
        unsafe { NSAccessibilityFocusedUIElementChangedNotification }
    }

    #[doc(alias = "NSAccessibilitySelectedChildrenChangedNotification")]
    pub fn selected_children_changed() -> &'static AccessibilityNotification {
        unsafe { NSAccessibilitySelectedChildrenChangedNotification }
    }

    #[doc(alias = "NSAccessibilityTitleChangedNotification")]
    pub fn title_changed() -> &'static AccessibilityNotification {
        unsafe { NSAccessibilityTitleChangedNotification }
    }

    #[doc(alias = "NSAccessibilityAnnouncementRequestedNotification")]
    pub fn announcement_requested() -> &'static AccessibilityNotification {
        unsafe { NSAccessibilityAnnouncementRequestedNotification }
    }

    unsafe extern "C" {
        static NSAccessibilityValueChangedNotification: &'static AccessibilityNotification;
        static NSAccessibilityLayoutChangedNotification: &'static AccessibilityNotification;
        static NSAccessibilityFocusedUIElementChangedNotification:
            &'static AccessibilityNotification;
        static NSAccessibilitySelectedChildrenChangedNotification:
            &'static AccessibilityNotification;
        static NSAccessibilityTitleChangedNotification: &'static AccessibilityNotification;
        static NSAccessibilityAnnouncementRequestedNotification: &'static AccessibilityNotification;
    }
}

/// How urgently an announcement interrupts what VoiceOver is saying.
#[doc(alias = "NSAccessibilityPriorityLevel")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum AccessibilityPriority {
    Low = 10,
    Medium = 50,
    High = 90,
}

/// Tells assistive technologies that `notification` happened to `element`.
#[doc(alias = "NSAccessibilityPostNotification")]
pub fn accessibility_post_notification(element: &ns::Id, notification: &AccessibilityNotification) {
    unsafe { NSAccessibilityPostNotification(element, notification) }
}

/// Has VoiceOver speak `text`, for `element` (the app's main window will do).
pub fn accessibility_announce(
    element: &ns::Id,
    text: &ns::String,
    priority: AccessibilityPriority,
) {
    let priority = ns::Number::with_i64(priority as i64);
    let keys = unsafe { [NSAccessibilityAnnouncementKey, NSAccessibilityPriorityKey] };
    let info = ns::Dictionary::<ns::String, ns::Id>::with_keys_values(
        &keys,
        &[text.as_id_ref(), priority.as_id_ref()],
    );
    unsafe {
        NSAccessibilityPostNotificationWithUserInfo(
            element,
            accessibility_notification::announcement_requested(),
            &info,
        )
    }
}

// NSView (NSAccessibility): the properties of the protocol NSView adopts.
impl ns::View {
    /// Whether assistive technologies see this view as one element; a control is one.
    #[objc::msg_send(isAccessibilityElement)]
    pub fn is_accessibility_element(&self) -> bool;

    #[objc::msg_send(setAccessibilityElement:)]
    pub fn set_accessibility_element(&mut self, val: bool);

    /// What VoiceOver says for the view, as for a button with only a symbol.
    #[objc::msg_send(accessibilityLabel)]
    pub fn accessibility_label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityLabel:)]
    pub fn set_accessibility_label(&mut self, val: Option<&ns::String>);

    /// What using the view does, said after a pause.
    #[objc::msg_send(accessibilityHelp)]
    pub fn accessibility_help(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityHelp:)]
    pub fn set_accessibility_help(&mut self, val: Option<&ns::String>);

    /// The view's value when its label does not say it; a string, a number or a date.
    #[objc::msg_send(accessibilityValue)]
    pub fn accessibility_value(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setAccessibilityValue:)]
    pub fn set_accessibility_value(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(accessibilityRole)]
    pub fn accessibility_role(&self) -> Option<arc::R<AccessibilityRole>>;

    #[objc::msg_send(setAccessibilityRole:)]
    pub fn set_accessibility_role(&mut self, val: Option<&AccessibilityRole>);

    #[objc::msg_send(isAccessibilityEnabled)]
    pub fn is_accessibility_enabled(&self) -> bool;

    #[objc::msg_send(setAccessibilityEnabled:)]
    pub fn set_accessibility_enabled(&mut self, val: bool);

    /// A name for UI tests; never spoken.
    #[objc::msg_send(accessibilityIdentifier)]
    pub fn accessibility_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityIdentifier:)]
    pub fn set_accessibility_id(&mut self, val: Option<&ns::String>);
}

unsafe extern "C" {
    fn NSAccessibilityPostNotification(element: &ns::Id, notification: &AccessibilityNotification);
    fn NSAccessibilityPostNotificationWithUserInfo(
        element: &ns::Id,
        notification: &AccessibilityNotification,
        user_info: &ns::Dictionary<ns::String, ns::Id>,
    );

    static NSAccessibilityAnnouncementKey: &'static ns::String;
    static NSAccessibilityPriorityKey: &'static ns::String;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn names() {
        assert_eq!(ns::accessibility_role::button().to_string(), "AXButton");
        assert_eq!(
            ns::accessibility_notification::value_changed().to_string(),
            "AXValueChanged"
        );
        assert_eq!(ns::AccessibilityPriority::High as isize, 90);
    }
}
