use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    /// A menu element that runs a selector.
    #[doc(alias = "UICommand")]
    pub Command(ui::MenuElement),
    UI_COMMAND
);

impl Command {
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
    pub KeyCommand(Command),
    UI_KEY_COMMAND
);

impl KeyCommand {
    /// A key command for `input` with `mod_flags`, sending `action` to the first responder that handles it.
    #[objc::msg_send(keyCommandWithInput:modifierFlags:action:)]
    pub fn with_input_mod_flags_action(
        input: &ns::String,
        mod_flags: ui::KeyModFlags,
        action: &objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(input)]
    pub fn input(&self) -> Option<arc::R<ns::String>>;

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
    static UI_COMMAND: &'static objc::Class<Command>;
    static UI_KEY_COMMAND: &'static objc::Class<KeyCommand>;
}
