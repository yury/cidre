use crate::{api, arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    /// A menu element that runs a selector.
    #[doc(alias = "UICommand")]
    pub Cmd(ui::MenuElement),
    UI_COMMAND
);

impl Cmd {
    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(action)]
    pub fn action(&self) -> &'static objc::Sel;
}

define_obj_type!(
    /// A key press that sends `action` up the responder chain.
    #[doc(alias = "UIKeyCommand")]
    pub KeyCmd(Cmd),
    UI_KEY_COMMAND
);

impl KeyCmd {
    /// A key command for `input` with `mod_flags`, sending `action` to the first responder that handles it.
    #[objc::msg_send(keyCommandWithInput:modifierFlags:action:)]
    pub fn with_input_mod_flags_action(
        input: &ns::String,
        mod_flags: ui::KeyModFlags,
        action: &objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(input)]
    pub fn input(&self) -> Option<arc::R<ns::String>>;

    /// The up arrow key, as an `input`.
    #[doc(alias = "UIKeyInputUpArrow")]
    #[api::available(ios = 7.0)]
    pub fn input_up_arrow() -> &'static ns::String {
        unsafe { UIKeyInputUpArrow }
    }

    #[doc(alias = "UIKeyInputDownArrow")]
    #[api::available(ios = 7.0)]
    pub fn input_down_arrow() -> &'static ns::String {
        unsafe { UIKeyInputDownArrow }
    }

    #[doc(alias = "UIKeyInputLeftArrow")]
    #[api::available(ios = 7.0)]
    pub fn input_left_arrow() -> &'static ns::String {
        unsafe { UIKeyInputLeftArrow }
    }

    #[doc(alias = "UIKeyInputRightArrow")]
    #[api::available(ios = 7.0)]
    pub fn input_right_arrow() -> &'static ns::String {
        unsafe { UIKeyInputRightArrow }
    }

    #[objc::msg_send(modifierFlags)]
    pub fn mod_flags(&self) -> ui::KeyModFlags;

    /// Whether the command beats the system's handling of the same key.
    #[objc::msg_send(wantsPriorityOverSystemBehavior)]
    #[objc::available(ios = 15.0)]
    pub fn wants_priority_over_sys_behavior(&self) -> bool;

    #[objc::msg_send(setWantsPriorityOverSystemBehavior:)]
    #[objc::available(ios = 15.0)]
    pub fn set_wants_priority_over_sys_behavior(&mut self, val: bool);
}

unsafe extern "C" {
    static UI_COMMAND: &'static objc::Class<Cmd>;
    static UI_KEY_COMMAND: &'static objc::Class<KeyCmd>;
}

#[api::weak]
unsafe extern "C" {
    #[api::available(ios = 7.0)]
    static UIKeyInputUpArrow: &'static ns::String;
    #[api::available(ios = 7.0)]
    static UIKeyInputDownArrow: &'static ns::String;
    #[api::available(ios = 7.0)]
    static UIKeyInputLeftArrow: &'static ns::String;
    #[api::available(ios = 7.0)]
    static UIKeyInputRightArrow: &'static ns::String;
}
