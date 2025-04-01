use crate::{arc, cf, define_cf_type};

define_cf_type!(Action(cf::String));

impl Action {
    #[inline]
    pub fn with_string(str: &cf::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<cf::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<Action> for cf::String {
    #[inline]
    fn as_ref(&self) -> &Action {
        Action::with_string(self)
    }
}

/// Many UiElements have a set of actions that they can perform. Actions are designed to be
/// simple. Actions roughly correspond to things you could do with a single click of the mouse
/// on the UiElement. Buttons and menu items, for example, have a single action: push or pick,
/// respectively. A scroll bar has several actions: page up, page down, up one line, down one line.
pub mod action {
    use crate::{ax::Action, cf};

    /// Simulate clicking the UiElement, such as a button.
    #[doc(alias = "kAXPressAction")]
    pub fn press() -> &'static Action {
        cf::str!(c"AXPress").as_ref()
    }

    /// Increment the value of the UiElement.
    #[doc(alias = "kAXIncrementAction")]
    pub fn increment() -> &'static Action {
        cf::str!(c"AXIncrement").as_ref()
    }

    /// Decrement the value of the UiElement.
    #[doc(alias = "kAXDecrementAction")]
    pub fn decrement() -> &'static Action {
        cf::str!(c"AXDecrement").as_ref()
    }

    /// Simulate pressing Return in the UiElement, such as a text field.
    #[doc(alias = "kAXConfirmAction")]
    pub fn confirm() -> &'static Action {
        cf::str!(c"AXConfirm").as_ref()
    }

    /// Simulate a Cancel action, such as hitting the Cancel button.
    #[doc(alias = "kAXCancelAction")]
    pub fn cancel() -> &'static Action {
        cf::str!(c"AXCancel").as_ref()
    }

    /// Show alternate or hidden UI.
    ///
    /// This is often used to trigger the same change that would occur on a mouse hover.
    #[doc(alias = "kAXShowAlternateUIAction")]
    pub fn show_alternate_ui() -> &'static Action {
        cf::str!(c"AXShowAlternateUI").as_ref()
    }

    /// Show default UI.
    ///
    /// This is often used to trigger the same change that would occur when a mouse hover ends.
    #[doc(alias = "kAXShowDefaultUIAction")]
    pub fn show_default_ui() -> &'static Action {
        cf::str!(c"AXShowDefaultUI").as_ref()
    }

    #[doc(alias = "kAXRaiseAction")]
    pub fn raise() -> &'static Action {
        cf::str!(c"AXRaise").as_ref()
    }

    #[doc(alias = "kAXShowMenuAction")]
    pub fn show_menu() -> &'static Action {
        cf::str!(c"AXShowMenu").as_ref()
    }

    #[doc(alias = "kAXPickAction")]
    pub fn pick() -> &'static Action {
        cf::str!(c"AXPick").as_ref()
    }
}
