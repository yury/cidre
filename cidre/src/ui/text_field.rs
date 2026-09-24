use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITextField")]
    pub TextField(ui::Control),
    UI_TEXT_FIELD
);

impl TextField {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<TextField>;

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

    /// `NSTextAlignment`: 0 left, 1 center, 2 right, 3 justified, 4 natural.
    #[objc::msg_send(textAlignment)]
    pub fn text_alignment(&self) -> ns::Integer;

    #[objc::msg_send(setTextAlignment:)]
    pub fn set_text_alignment(&mut self, val: ns::Integer);

    /// `UIKeyboardType`: 0 default, 2 numbers and punctuation, 4 number pad, 8 decimal pad.
    #[objc::msg_send(keyboardType)]
    pub fn keyboard_type(&self) -> ns::Integer;

    #[objc::msg_send(setKeyboardType:)]
    pub fn set_keyboard_type(&mut self, val: ns::Integer);

    /// `UITextFieldViewMode`: 0 never, 1 while editing, 2 unless editing, 3 always.
    #[objc::msg_send(setClearButtonMode:)]
    pub fn set_clear_button_mode(&mut self, val: ns::Integer);

    #[objc::msg_send(setAdjustsFontSizeToFitWidth:)]
    pub fn set_adjusts_font_size_to_fit_width(&mut self, val: bool);

    /// `UITextAutocorrectionType`: 1 no.
    #[objc::msg_send(setAutocorrectionType:)]
    pub fn set_autocorrection_type(&mut self, val: ns::Integer);
}

unsafe extern "C" {
    static UI_TEXT_FIELD: &'static objc::Class<TextField>;
}

// UIView (UITextField), declared in UITextField.h.
impl ui::View {
    #[objc::msg_send(endEditing:)]
    pub fn end_editing(&mut self, force: bool) -> bool;
}
