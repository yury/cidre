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
    #[objc::msg_send(buttonWithType:primaryAction:)]
    #[objc::available(ios = 14.0)]
    pub fn with_type_primary_action(kind: ButtonType, action: Option<&ui::Action>) -> arc::R<Self>;

    #[objc::msg_send(setMenu:)]
    #[objc::available(ios = 14.0)]
    pub fn set_menu(&mut self, menu: Option<&ui::Menu>);

    /// A pop-up button: the menu's actions are a single selection, the selected one's title
    /// is the button's, and picking one performs it. Needs `shows_menu_as_primary_action`.
    #[objc::msg_send(changesSelectionAsPrimaryAction)]
    #[objc::available(ios = 15.0)]
    pub fn changes_selection_as_primary_action(&self) -> bool;

    #[objc::msg_send(setChangesSelectionAsPrimaryAction:)]
    #[objc::available(ios = 15.0)]
    pub fn set_changes_selection_as_primary_action(&mut self, val: bool);

    #[objc::msg_send(configuration)]
    #[objc::available(ios = 15.0)]
    pub fn cfg(&self) -> Option<arc::R<ui::ButtonCfg>>;

    #[objc::msg_send(setConfiguration:)]
    #[objc::available(ios = 15.0)]
    pub fn set_cfg(&mut self, val: Option<&ui::ButtonCfg>);

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
