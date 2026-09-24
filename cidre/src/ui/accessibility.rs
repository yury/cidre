//! UIAccessibility.h: the display preferences, the views' accessibility properties, and
//! posting notifications to assistive technologies.

use crate::{arc, define_opts, ns, objc, ui};

/// Whether the user requests reduced motion in animations.
#[doc(alias = "UIAccessibilityIsReduceMotionEnabled")]
pub fn is_reduce_motion_enabled() -> bool {
    unsafe { UIAccessibilityIsReduceMotionEnabled() }
}

/// Whether VoiceOver is on.
#[doc(alias = "UIAccessibilityIsVoiceOverRunning")]
pub fn is_voice_over_running() -> bool {
    unsafe { UIAccessibilityIsVoiceOverRunning() }
}

define_opts!(
    /// What an accessibility element is and does, combined.
    #[doc(alias = "UIAccessibilityTraits")]
    pub Traits(u64)
);

impl Traits {
    #[doc(alias = "UIAccessibilityTraitNone")]
    pub fn none() -> Self {
        unsafe { UIAccessibilityTraitNone }
    }

    #[doc(alias = "UIAccessibilityTraitButton")]
    pub fn button() -> Self {
        unsafe { UIAccessibilityTraitButton }
    }

    #[doc(alias = "UIAccessibilityTraitLink")]
    pub fn link() -> Self {
        unsafe { UIAccessibilityTraitLink }
    }

    #[doc(alias = "UIAccessibilityTraitHeader")]
    pub fn header() -> Self {
        unsafe { UIAccessibilityTraitHeader }
    }

    #[doc(alias = "UIAccessibilityTraitSearchField")]
    pub fn search_field() -> Self {
        unsafe { UIAccessibilityTraitSearchField }
    }

    #[doc(alias = "UIAccessibilityTraitImage")]
    pub fn image() -> Self {
        unsafe { UIAccessibilityTraitImage }
    }

    #[doc(alias = "UIAccessibilityTraitSelected")]
    pub fn selected() -> Self {
        unsafe { UIAccessibilityTraitSelected }
    }

    #[doc(alias = "UIAccessibilityTraitPlaysSound")]
    pub fn plays_sound() -> Self {
        unsafe { UIAccessibilityTraitPlaysSound }
    }

    #[doc(alias = "UIAccessibilityTraitKeyboardKey")]
    pub fn keyboard_key() -> Self {
        unsafe { UIAccessibilityTraitKeyboardKey }
    }

    #[doc(alias = "UIAccessibilityTraitStaticText")]
    pub fn static_text() -> Self {
        unsafe { UIAccessibilityTraitStaticText }
    }

    #[doc(alias = "UIAccessibilityTraitSummaryElement")]
    pub fn summary_element() -> Self {
        unsafe { UIAccessibilityTraitSummaryElement }
    }

    #[doc(alias = "UIAccessibilityTraitNotEnabled")]
    pub fn not_enabled() -> Self {
        unsafe { UIAccessibilityTraitNotEnabled }
    }

    #[doc(alias = "UIAccessibilityTraitUpdatesFrequently")]
    pub fn updates_frequently() -> Self {
        unsafe { UIAccessibilityTraitUpdatesFrequently }
    }

    #[doc(alias = "UIAccessibilityTraitStartsMediaSession")]
    pub fn starts_media_session() -> Self {
        unsafe { UIAccessibilityTraitStartsMediaSession }
    }

    /// Changes by increments: VoiceOver's swipe up and down call `accessibilityIncrement`
    /// and `accessibilityDecrement`.
    #[doc(alias = "UIAccessibilityTraitAdjustable")]
    pub fn adjustable() -> Self {
        unsafe { UIAccessibilityTraitAdjustable }
    }

    #[doc(alias = "UIAccessibilityTraitAllowsDirectInteraction")]
    pub fn allows_direct_interaction() -> Self {
        unsafe { UIAccessibilityTraitAllowsDirectInteraction }
    }

    #[doc(alias = "UIAccessibilityTraitCausesPageTurn")]
    pub fn causes_page_turn() -> Self {
        unsafe { UIAccessibilityTraitCausesPageTurn }
    }

    #[doc(alias = "UIAccessibilityTraitTabBar")]
    pub fn tab_bar() -> Self {
        unsafe { UIAccessibilityTraitTabBar }
    }

    /// iOS 17.
    #[doc(alias = "UIAccessibilityTraitToggleButton")]
    pub fn toggle_button() -> Self {
        unsafe { UIAccessibilityTraitToggleButton }
    }

    /// iOS 17.
    #[doc(alias = "UIAccessibilityTraitSupportsZoom")]
    pub fn supports_zoom() -> Self {
        unsafe { UIAccessibilityTraitSupportsZoom }
    }
}

/// What changed, for [`post_notification`].
#[doc(alias = "UIAccessibilityNotifications")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Notification(pub u32);

impl Notification {
    /// A new screen: VoiceOver moves to the argument, a view, or to the first element.
    #[doc(alias = "UIAccessibilityScreenChangedNotification")]
    pub fn screen_changed() -> Self {
        unsafe { UIAccessibilityScreenChangedNotification }
    }

    /// Elements came or went: VoiceOver moves to the argument, a view, if there is one.
    #[doc(alias = "UIAccessibilityLayoutChangedNotification")]
    pub fn layout_changed() -> Self {
        unsafe { UIAccessibilityLayoutChangedNotification }
    }

    /// Speaks the argument, an `ns::String`.
    #[doc(alias = "UIAccessibilityAnnouncementNotification")]
    pub fn announcement() -> Self {
        unsafe { UIAccessibilityAnnouncementNotification }
    }

    /// A scroll by pages: speaks the argument, an `ns::String` such as "Page 2 of 5".
    #[doc(alias = "UIAccessibilityPageScrolledNotification")]
    pub fn page_scrolled() -> Self {
        unsafe { UIAccessibilityPageScrolledNotification }
    }
}

/// Tells assistive technologies about `notification`, with its argument.
#[doc(alias = "UIAccessibilityPostNotification")]
pub fn post_notification(notification: Notification, argument: Option<&ns::Id>) {
    unsafe { UIAccessibilityPostNotification(notification, argument) }
}

/// Has VoiceOver speak `text`.
pub fn announce(text: &ns::String) {
    post_notification(Notification::announcement(), Some(text));
}

// NSObject (UIAccessibility) and UIAccessibilityIdentification, for the views that carry them.
impl ui::View {
    /// Whether assistive technologies see this view as one element; a control is one.
    #[objc::msg_send(isAccessibilityElement)]
    pub fn is_accessibility_element(&self) -> bool;

    #[objc::msg_send(setIsAccessibilityElement:)]
    pub fn set_is_accessibility_element(&mut self, val: bool);

    /// What VoiceOver says for the view, as for a button with only a symbol.
    #[objc::msg_send(accessibilityLabel)]
    pub fn accessibility_label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityLabel:)]
    pub fn set_accessibility_label(&mut self, val: Option<&ns::String>);

    /// What using the view does, said after a pause.
    #[objc::msg_send(accessibilityHint)]
    pub fn accessibility_hint(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityHint:)]
    pub fn set_accessibility_hint(&mut self, val: Option<&ns::String>);

    /// The view's value when its label does not say it, as a slider's.
    #[objc::msg_send(accessibilityValue)]
    pub fn accessibility_value(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityValue:)]
    pub fn set_accessibility_value(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(accessibilityTraits)]
    pub fn accessibility_traits(&self) -> Traits;

    #[objc::msg_send(setAccessibilityTraits:)]
    pub fn set_accessibility_traits(&mut self, val: Traits);

    /// Hides the view and its subviews from assistive technologies.
    #[objc::msg_send(accessibilityElementsHidden)]
    pub fn accessibility_elements_hidden(&self) -> bool;

    #[objc::msg_send(setAccessibilityElementsHidden:)]
    pub fn set_accessibility_elements_hidden(&mut self, val: bool);

    /// A name for UI tests; never spoken.
    #[objc::msg_send(accessibilityIdentifier)]
    pub fn accessibility_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityIdentifier:)]
    pub fn set_accessibility_id(&mut self, val: Option<&ns::String>);
}

unsafe extern "C" {
    fn UIAccessibilityIsReduceMotionEnabled() -> bool;
    fn UIAccessibilityIsVoiceOverRunning() -> bool;
    fn UIAccessibilityPostNotification(notification: Notification, argument: Option<&ns::Id>);

    static UIAccessibilityTraitNone: Traits;
    static UIAccessibilityTraitButton: Traits;
    static UIAccessibilityTraitLink: Traits;
    static UIAccessibilityTraitHeader: Traits;
    static UIAccessibilityTraitSearchField: Traits;
    static UIAccessibilityTraitImage: Traits;
    static UIAccessibilityTraitSelected: Traits;
    static UIAccessibilityTraitPlaysSound: Traits;
    static UIAccessibilityTraitKeyboardKey: Traits;
    static UIAccessibilityTraitStaticText: Traits;
    static UIAccessibilityTraitSummaryElement: Traits;
    static UIAccessibilityTraitNotEnabled: Traits;
    static UIAccessibilityTraitUpdatesFrequently: Traits;
    static UIAccessibilityTraitStartsMediaSession: Traits;
    static UIAccessibilityTraitAdjustable: Traits;
    static UIAccessibilityTraitAllowsDirectInteraction: Traits;
    static UIAccessibilityTraitCausesPageTurn: Traits;
    static UIAccessibilityTraitTabBar: Traits;
    static UIAccessibilityTraitToggleButton: Traits;
    static UIAccessibilityTraitSupportsZoom: Traits;

    static UIAccessibilityScreenChangedNotification: Notification;
    static UIAccessibilityLayoutChangedNotification: Notification;
    static UIAccessibilityAnnouncementNotification: Notification;
    static UIAccessibilityPageScrolledNotification: Notification;
}
