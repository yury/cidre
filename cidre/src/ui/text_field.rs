use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITextField")]
    pub TextField(ui::Control),
    UI_TEXT_FIELD
);

impl arc::A<TextField> {
    #[objc::msg_send(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<TextField>;
}

impl TextField {
    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(text)]
    pub fn text(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setText:)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(placeholder)]
    pub fn placeholder(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setPlaceholder:)]
    pub fn set_placeholder(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(isSecureTextEntry)]
    pub fn is_secure_text_entry(&self) -> bool;

    #[objc::msg_send(setSecureTextEntry:)]
    pub fn set_secure_text_entry(&mut self, val: bool);

    #[objc::msg_send(font)]
    pub fn font(&self) -> Option<arc::R<ui::Font>>;

    #[objc::msg_send(setFont:)]
    pub fn set_font(&mut self, val: Option<&ui::Font>);

    /// `UITextBorderStyle`: 0 none, 1 line, 2 bezel, 3 rounded rect.
    #[objc::msg_send(setBorderStyle:)]
    pub fn set_border_style(&mut self, val: ns::Integer);

    /// `UITextAutocapitalizationType`: 0 none.
    #[objc::msg_send(setAutocapitalizationType:)]
    pub fn set_autocapitalization_type(&mut self, val: ns::Integer);

    /// `UITextAutocorrectionType`: 1 no.
    #[objc::msg_send(setAutocorrectionType:)]
    pub fn set_autocorrection_type(&mut self, val: ns::Integer);
}

unsafe extern "C" {
    static UI_TEXT_FIELD: &'static objc::Class<TextField>;
}
