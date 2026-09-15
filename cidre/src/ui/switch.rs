use crate::{arc, cg, define_obj_type, ns, objc, ui};

#[doc(alias = "UISwitchStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum SwitchStyle {
    Automatic = 0,
    Checkbox = 1,
    Sliding = 2,
}

define_obj_type!(
    #[doc(alias = "UISwitch")]
    pub Switch(ui::Control),
    UI_SWITCH
);

impl Switch {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<Switch>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::init(initWithFrame:primaryAction:)]
    #[objc::available(ios = 14.0)]
    pub fn init_with_frame_primary_action(
        self,
        frame: cg::Rect,
        action: Option<&ui::Action>,
    ) -> arc::R<Switch>;

    #[objc::available(ios = 14.0)]
    pub fn with_frame_primary_action(frame: cg::Rect, action: Option<&ui::Action>) -> arc::R<Self> {
        Self::alloc().init_with_frame_primary_action(frame, action)
    }

    #[objc::msg_send(isOn)]
    pub fn is_on(&self) -> bool;

    #[objc::msg_send(setOn:)]
    pub fn set_on(&mut self, val: bool);

    #[objc::msg_send(setOn:animated:)]
    pub fn set_on_animated(&mut self, val: bool, animated: bool);

    #[objc::msg_send(onTintColor)]
    pub fn on_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setOnTintColor:)]
    pub fn set_on_tint_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(thumbTintColor)]
    pub fn thumb_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setThumbTintColor:)]
    pub fn set_thumb_tint_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(preferredStyle)]
    #[objc::available(ios = 14.0)]
    pub fn preferred_style(&self) -> SwitchStyle;

    #[objc::msg_send(setPreferredStyle:)]
    #[objc::available(ios = 14.0)]
    pub fn set_preferred_style(&mut self, val: SwitchStyle);

    #[objc::msg_send(style)]
    #[objc::available(ios = 14.0)]
    pub fn style(&self) -> SwitchStyle;

    #[objc::msg_send(title)]
    #[objc::available(ios = 14.0)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    #[objc::available(ios = 14.0)]
    pub fn set_title(&mut self, val: Option<&ns::String>);
}

unsafe extern "C" {
    static UI_SWITCH: &'static objc::Class<Switch>;
}
