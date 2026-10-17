use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITextView")]
    pub TextView(ui::ScrollView),
    sym UITextView
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

    /// The selection, the caret where it is empty, in UTF-16 units.
    #[objc::msg_send(selectedRange)]
    pub fn selected_range(&self) -> ns::Range;

    #[objc::msg_send(setSelectedRange:)]
    pub fn set_selected_range(&mut self, val: ns::Range);

    /// `UIKeyInput`: inserts `text` over the selection, as typing it would.
    #[objc::msg_send(insertText:)]
    pub fn insert_text(&mut self, text: &ns::String);

    #[objc::msg_send(scrollRangeToVisible:)]
    pub fn scroll_range_to_visible(&mut self, range: ns::Range);

    /// The keyboard's return key: a `UIReturnKeyType` (7 is Send).
    #[objc::msg_send(returnKeyType)]
    pub fn return_key_type(&self) -> ns::Integer;

    #[objc::msg_send(setReturnKeyType:)]
    pub fn set_return_key_type(&mut self, val: ns::Integer);
}

pub mod notifications {
    use crate::ns;

    /// Posted as the text view's text changes; the object is the text view.
    #[doc(alias = "UITextViewTextDidChangeNotification")]
    pub fn text_did_change() -> &'static ns::NotificationName {
        unsafe { UITextViewTextDidChangeNotification }
    }

    unsafe extern "C" {
        static UITextViewTextDidChangeNotification: &'static ns::NotificationName;
    }
}
