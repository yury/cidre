use crate::{arc, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "NSAlertStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
#[non_exhaustive]
pub enum AlertStyle {
    /// Warns about a current or impending event. The default.
    #[doc(alias = "NSAlertStyleWarning")]
    Warning = 0,

    /// Informs about a current or impending event. Looks like a warning.
    #[doc(alias = "NSAlertStyleInformational")]
    Informational = 1,

    /// Informs about a critical event; the icon is badged with a caution icon.
    #[doc(alias = "NSAlertStyleCritical")]
    Critical = 2,
}

impl ns::ModalResponse {
    /// The first (rightmost) button of an alert.
    #[doc(alias = "NSAlertFirstButtonReturn")]
    pub const ALERT_FIRST_BUTTON: Self = Self(1000);

    #[doc(alias = "NSAlertSecondButtonReturn")]
    pub const ALERT_SECOND_BUTTON: Self = Self(1001);

    /// Buttons after the third return this plus their position past it.
    #[doc(alias = "NSAlertThirdButtonReturn")]
    pub const ALERT_THIRD_BUTTON: Self = Self(1002);
}

define_obj_type!(
    #[doc(alias = "NSAlert")]
    pub Alert(ns::Id),
    NS_ALERT
);

impl Alert {
    #[objc::msg_send(alertWithError:)]
    pub fn with_error(error: &ns::Error) -> arc::R<Self>;

    #[objc::msg_send(messageText)]
    pub fn message_text(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setMessageText:)]
    pub fn set_message_text(&mut self, val: &ns::String);

    #[objc::msg_send(informativeText)]
    pub fn informative_text(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setInformativeText:)]
    pub fn set_informative_text(&mut self, val: &ns::String);

    #[objc::msg_send(icon)]
    pub fn icon(&self) -> arc::R<ns::Image>;

    /// `None` restores the app's icon.
    #[objc::msg_send(setIcon:)]
    pub fn set_icon(&mut self, val: Option<&ns::Image>);

    /// Buttons are added from the right; the first one is the default button.
    #[objc::msg_send(addButtonWithTitle:)]
    pub fn add_button_with_title(&mut self, title: &ns::String) -> arc::R<ns::Button>;

    #[objc::msg_send(buttons)]
    pub fn buttons(&self) -> arc::R<ns::Array<ns::Button>>;

    #[objc::msg_send(alertStyle)]
    pub fn alert_style(&self) -> AlertStyle;

    #[objc::msg_send(setAlertStyle:)]
    pub fn set_alert_style(&mut self, val: AlertStyle);

    #[objc::msg_send(showsHelp)]
    pub fn shows_help(&self) -> bool;

    #[objc::msg_send(setShowsHelp:)]
    pub fn set_shows_help(&mut self, val: bool);

    #[objc::msg_send(accessoryView)]
    pub fn accessory_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setAccessoryView:)]
    pub fn set_accessory_view(&mut self, val: Option<&ns::View>);

    /// Lays the alert out now, to move its views before it shows.
    #[objc::msg_send(layout)]
    pub fn layout(&mut self);

    /// Runs the alert app-modal. With no buttons added the implicit OK returns 0.
    #[objc::msg_send(runModal)]
    pub fn run_modal(&mut self) -> ns::ModalResponse;

    #[objc::msg_send(showsSuppressionButton)]
    pub fn shows_suppression_button(&self) -> bool;

    #[objc::msg_send(setShowsSuppressionButton:)]
    pub fn set_shows_suppression_button(&mut self, val: bool);

    #[objc::msg_send(suppressionButton)]
    pub fn suppression_button(&self) -> Option<arc::R<ns::Button>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(beginSheetModalForWindow:completionHandler:)]
    pub fn begin_sheet_modal_for_window_ch_block(
        &mut self,
        sheet_window: &ns::Window,
        handler: Option<&mut blocks::EscBlock<fn(ns::ModalResponse)>>,
    );

    /// Runs the alert as a sheet on `sheet_window`.
    #[cfg(feature = "blocks")]
    pub fn begin_sheet_modal_for_window_ch(
        &mut self,
        sheet_window: &ns::Window,
        handler: impl FnMut(ns::ModalResponse) + 'static,
    ) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.begin_sheet_modal_for_window_ch_block(sheet_window, Some(&mut handler));
    }

    /// The alert's panel.
    #[objc::msg_send(window)]
    pub fn window(&self) -> arc::R<ns::Window>;
}

unsafe extern "C" {
    static NS_ALERT: &'static objc::Class<Alert>;
}
