use crate::{arc, blocks, define_obj_type, ns, objc, ui};

#[doc(alias = "UIAlertControllerStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AlertControllerStyle {
    ActionSheet = 0,
    Alert = 1,
}

#[doc(alias = "UIAlertActionStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AlertActionStyle {
    Default = 0,
    Cancel = 1,
    Destructive = 2,
}

define_obj_type!(
    #[doc(alias = "UIAlertAction")]
    pub AlertAction(ns::Id),
    UI_ALERT_ACTION
);

impl AlertAction {
    #[objc::msg_send(actionWithTitle:style:handler:)]
    pub fn with_title_style_handler(
        title: Option<&ns::String>,
        style: AlertActionStyle,
        handler: Option<&mut blocks::EscBlock<fn(&AlertAction)>>,
    ) -> arc::R<Self>;

    pub fn with_title(
        title: &ns::String,
        style: AlertActionStyle,
        handler: impl FnMut(&AlertAction) + 'static,
    ) -> arc::R<Self> {
        let mut block = blocks::EscBlock::new1(handler);
        Self::with_title_style_handler(Some(title), style, Some(&mut block))
    }

    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(style)]
    pub fn style(&self) -> AlertActionStyle;

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);
}

define_obj_type!(
    #[doc(alias = "UIAlertController")]
    pub AlertController(ui::ViewController),
    UI_ALERT_CONTROLLER
);

impl AlertController {
    #[objc::msg_send(alertControllerWithTitle:message:preferredStyle:)]
    pub fn with_title_message_style(
        title: Option<&ns::String>,
        message: Option<&ns::String>,
        style: AlertControllerStyle,
    ) -> arc::R<Self>;

    #[objc::msg_send(addAction:)]
    pub fn add_action(&mut self, action: &AlertAction);

    #[objc::msg_send(actions)]
    pub fn actions(&self) -> arc::R<ns::Array<AlertAction>>;

    #[objc::msg_send(setPreferredAction:)]
    pub fn set_preferred_action(&mut self, val: Option<&AlertAction>);

    /// Alert style only. The handler configures the field before it is shown.
    #[objc::msg_send(addTextFieldWithConfigurationHandler:)]
    pub fn add_text_field_ch(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&mut ui::TextField)>>,
    );

    pub fn add_text_field(&mut self, configure: impl FnMut(&mut ui::TextField) + 'static) {
        let mut block = blocks::EscBlock::new1(configure);
        self.add_text_field_ch(Some(&mut block));
    }

    #[objc::msg_send(textFields)]
    pub fn text_fields(&self) -> Option<arc::R<ns::Array<ui::TextField>>>;

    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(message)]
    pub fn message(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setMessage:)]
    pub fn set_message(&mut self, val: Option<&ns::String>);
}

unsafe extern "C" {
    static UI_ALERT_ACTION: &'static objc::Class<AlertAction>;
    static UI_ALERT_CONTROLLER: &'static objc::Class<AlertController>;
}
