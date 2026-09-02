use crate::{arc, define_obj_type, define_opts, ns, objc, ui};

define_opts!(
    #[doc(alias = "UIControlEvents")]
    pub ControlEvents(usize)
);

impl ControlEvents {
    pub const TOUCH_DOWN: Self = Self(1 << 0);
    pub const TOUCH_DOWN_REPEAT: Self = Self(1 << 1);
    pub const TOUCH_DRAG_INSIDE: Self = Self(1 << 2);
    pub const TOUCH_DRAG_OUTSIDE: Self = Self(1 << 3);
    pub const TOUCH_DRAG_ENTER: Self = Self(1 << 4);
    pub const TOUCH_DRAG_EXIT: Self = Self(1 << 5);
    pub const TOUCH_UP_INSIDE: Self = Self(1 << 6);
    pub const TOUCH_UP_OUTSIDE: Self = Self(1 << 7);
    pub const TOUCH_CANCEL: Self = Self(1 << 8);
    pub const VALUE_CHANGED: Self = Self(1 << 12);
    pub const PRIMARY_ACTION_TRIGGERED: Self = Self(1 << 13);
    pub const MENU_ACTION_TRIGGERED: Self = Self(1 << 14);
    pub const EDITING_DID_BEGIN: Self = Self(1 << 16);
    pub const EDITING_CHANGED: Self = Self(1 << 17);
    pub const EDITING_DID_END: Self = Self(1 << 18);
    pub const EDITING_DID_END_ON_EXIT: Self = Self(1 << 19);
    pub const ALL_TOUCH_EVENTS: Self = Self(0x0000_0FFF);
    pub const ALL_EDITING_EVENTS: Self = Self(0x000F_0000);
    pub const ALL_EVENTS: Self = Self(0xFFFF_FFFF);
}

define_opts!(
    #[doc(alias = "UIControlState")]
    pub ControlState(usize)
);

impl ControlState {
    pub const NORMAL: Self = Self(0);
    pub const HIGHLIGHTED: Self = Self(1 << 0);
    pub const DISABLED: Self = Self(1 << 1);
    pub const SELECTED: Self = Self(1 << 2);
    pub const FOCUSED: Self = Self(1 << 3);
}

define_obj_type!(
    #[doc(alias = "UIControl")]
    pub Control(ui::View),
    UI_CONTROL
);

impl Control {
    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(isSelected)]
    pub fn is_selected(&self) -> bool;

    #[objc::msg_send(setSelected:)]
    pub fn set_selected(&mut self, val: bool);

    #[objc::msg_send(isHighlighted)]
    pub fn is_highlighted(&self) -> bool;

    #[objc::msg_send(state)]
    pub fn state(&self) -> ControlState;

    /// Target/action: `action` is sent to `target` (or up the responder chain when `None`).
    #[objc::msg_send(addTarget:action:forControlEvents:)]
    pub fn add_target_action_for_events(
        &mut self,
        target: Option<&ns::Id>,
        action: &objc::Sel,
        events: ControlEvents,
    );

    #[objc::msg_send(removeTarget:action:forControlEvents:)]
    pub fn remove_target_action_for_events(
        &mut self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
        events: ControlEvents,
    );
}

unsafe extern "C" {
    static UI_CONTROL: &'static objc::Class<Control>;
}
