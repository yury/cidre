use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITextView")]
    pub TextView(ui::ScrollView),
    UI_TEXT_VIEW
);

impl TextView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<TextView>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(text)]
    pub fn text(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setText:)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(attributedText)]
    pub fn attr_text(&self) -> arc::R<ns::AttrString>;

    #[objc::msg_send(setAttributedText:)]
    pub fn set_attr_text(&mut self, val: Option<&ns::AttrString>);

    #[objc::msg_send(font)]
    pub fn font(&self) -> Option<arc::R<ui::Font>>;

    #[objc::msg_send(setFont:)]
    pub fn set_font(&mut self, val: Option<&ui::Font>);

    #[objc::msg_send(textColor)]
    pub fn text_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setTextColor:)]
    pub fn set_text_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(isEditable)]
    pub fn is_editable(&self) -> bool;

    #[objc::msg_send(setEditable:)]
    pub fn set_editable(&mut self, val: bool);

    #[objc::msg_send(isSelectable)]
    pub fn is_selectable(&self) -> bool;

    #[objc::msg_send(setSelectable:)]
    pub fn set_selectable(&mut self, val: bool);

    #[objc::msg_send(isScrollEnabled)]
    pub fn is_scroll_enabled(&self) -> bool;

    #[objc::msg_send(setScrollEnabled:)]
    pub fn set_scroll_enabled(&mut self, val: bool);

    #[objc::msg_send(textContainerInset)]
    pub fn text_container_inset(&self) -> ui::EdgeInsets;

    #[objc::msg_send(setTextContainerInset:)]
    pub fn set_text_container_inset(&mut self, val: ui::EdgeInsets);

    #[objc::msg_send(textAlignment)]
    pub fn text_alignment(&self) -> ns::Integer;

    #[objc::msg_send(setTextAlignment:)]
    pub fn set_text_alignment(&mut self, val: ns::Integer);

    /// `UITextInputTraits`
    #[objc::msg_send(setAutocapitalizationType:)]
    pub fn set_autocapitalization_type(&mut self, val: ns::Integer);

    #[objc::msg_send(setAutocorrectionType:)]
    pub fn set_autocorrection_type(&mut self, val: ns::Integer);

    #[objc::msg_send(scrollRangeToVisible:)]
    pub fn scroll_range_to_visible(&mut self, range: ns::Range);
}

unsafe extern "C" {
    static UI_TEXT_VIEW: &'static objc::Class<TextView>;
}
