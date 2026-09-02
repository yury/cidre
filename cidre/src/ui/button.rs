use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UIButtonType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ButtonType {
    Custom = 0,
    System = 1,
    DetailDisclosure = 2,
    InfoLight = 3,
    InfoDark = 4,
    ContactAdd = 5,
    Close = 7,
}

define_obj_type!(
    #[doc(alias = "UIButton")]
    pub Button(ui::Control),
    UI_BUTTON
);

impl Button {
    #[objc::msg_send(buttonWithType:)]
    pub fn with_type(val: ButtonType) -> arc::R<Self>;

    pub fn system() -> arc::R<Self> {
        Self::with_type(ButtonType::System)
    }

    #[objc::msg_send(setTitle:forState:)]
    pub fn set_title_for_state(&mut self, title: Option<&ns::String>, state: ui::ControlState);

    #[objc::msg_send(titleForState:)]
    pub fn title_for_state(&self, state: ui::ControlState) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitleColor:forState:)]
    pub fn set_title_color_for_state(&mut self, color: Option<&ui::Color>, state: ui::ControlState);

    #[objc::msg_send(titleLabel)]
    pub fn title_label(&self) -> Option<arc::R<ui::Label>>;

    #[objc::msg_send(sizeToFit)]
    pub fn size_to_fit(&mut self);
}

unsafe extern "C" {
    static UI_BUTTON: &'static objc::Class<Button>;
}
