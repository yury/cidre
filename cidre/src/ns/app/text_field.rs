use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTextField")]
    pub TextField(ns::Control),
    NS_TEXT_FIELD
);

impl TextField {
    #[objc::msg_send(placeholderString)]
    pub fn placeholder_string(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setPlaceholderString:)]
    pub fn set_placeholder_string(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(placeholderAttributedString)]
    pub fn placeholder_attr_string(&self) -> Option<arc::R<ns::AttrString>>;

    #[objc::msg_send(setPlaceholderAttributedString:)]
    pub fn set_placeholder_attr_string(&mut self, val: Option<&ns::AttrString>);

    #[objc::msg_send(backgroundColor)]
    pub fn bg_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_bg_color(&mut self, val: Option<&ns::Color>);

    #[objc::msg_send(drawsBackground)]
    pub fn draws_bg(&self) -> bool;

    #[objc::msg_send(setDrawsBackground:)]
    pub fn set_draws_bg(&mut self, val: bool);

    #[objc::msg_send(textColor)]
    pub fn text_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setTextColor:)]
    pub fn set_text_color(&mut self, val: Option<&ns::Color>);

    #[objc::msg_send(isBordered)]
    pub fn is_bordered(&self) -> bool;

    #[objc::msg_send(setBordered:)]
    pub fn set_bordered(&mut self, val: bool);

    #[objc::msg_send(isBezeled)]
    pub fn is_bezeled(&self) -> bool;

    #[objc::msg_send(setBezeled:)]
    pub fn set_bezeled(&mut self, val: bool);

    #[objc::msg_send(isEditable)]
    pub fn is_editable(&self) -> bool;

    #[objc::msg_send(setEditable:)]
    pub fn set_editable(&mut self, val: bool);

    #[objc::msg_send(isSelectable)]
    pub fn is_selectable(&self) -> bool;

    #[objc::msg_send(setSelectable:)]
    pub fn set_selectable(&mut self, val: bool);

    #[objc::msg_send(selectText:)]
    pub fn select_text(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyTextFieldDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: TextFieldDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(textShouldBeginEditing:)]
    pub fn text_should_begin_editing(&self, text_object: &ns::Text) -> bool;

    #[objc::msg_send(textShouldEndEditing:)]
    pub fn text_should_end_editing(&self, text_object: &ns::Text) -> bool;

    #[objc::msg_send(textDidBeginEditing:)]
    pub fn text_did_begin_editing(&self, n: &ns::Notification);

    #[objc::msg_send(textDidEndEditing:)]
    pub fn text_did_end_editing(&self, n: &ns::Notification);

    #[objc::msg_send(textDidChange:)]
    pub fn text_did_change(&self, n: &ns::Notification);

    #[objc::msg_send(acceptsFirstResponder)]
    pub fn accepts_first_responder(&self) -> bool;

    #[objc::msg_send(bezelStyle)]
    pub fn bezel_style(&self) -> ns::TextFieldBezelStyle;

    #[objc::msg_send(setBezelStyle:)]
    pub fn set_bezel_style(&mut self, val: ns::TextFieldBezelStyle);

    #[objc::msg_send(preferredMaxLayoutWidth)]
    pub fn preferred_max_layout_width(&self) -> cg::Float;

    #[objc::msg_send(setPreferredMaxLayoutWidth:)]
    pub fn set_preferred_max_layout_width(&mut self, val: cg::Float);

    #[objc::msg_send(maximumNumberOfLines)]
    pub fn max_num_lines(&self) -> isize;

    #[objc::msg_send(setMaximumNumberOfLines:)]
    pub fn set_max_num_lines(&mut self, val: isize);

    #[objc::msg_send(allowsDefaultTighteningForTruncation)]
    pub fn allows_default_tightening_for_truncation(&self) -> bool;

    #[objc::msg_send(setAllowsDefaultTighteningForTruncation:)]
    pub fn set_allows_default_tightening_for_truncation(&mut self, val: bool);

    #[objc::msg_send(lineBreakStrategy)]
    pub fn line_break_strategy(&self) -> ns::LineBreakStrategy;

    #[objc::msg_send(setLineBreakStrategy:)]
    pub fn set_line_break_strategy(&mut self, val: ns::LineBreakStrategy);

    #[objc::msg_send(allowsWritingTools)]
    #[objc::available(macos = 15.2)]
    pub fn allows_writing_tools(&self) -> bool;

    #[objc::msg_send(setAllowsWritingTools:)]
    #[objc::available(macos = 15.2)]
    pub fn set_allows_writing_tools(&mut self, val: bool);

    #[objc::msg_send(allowsWritingToolsAffordance)]
    #[objc::available(macos = 15.4)]
    pub fn allows_writing_tools_affordance(&self) -> bool;

    #[objc::msg_send(setAllowsWritingToolsAffordance:)]
    #[objc::available(macos = 15.4)]
    pub fn set_allows_writing_tools_affordance(&mut self, val: bool);
}

/// NSTextFieldConvenience
impl TextField {
    #[objc::msg_send(labelWithString:)]
    pub fn label(val: &ns::String) -> arc::R<Self>;

    #[objc::msg_send(wrappingLabelWithString:)]
    pub fn wrapping_label(val: &ns::String) -> arc::R<Self>;

    #[objc::msg_send(labelWithAttributedString:)]
    pub fn label_with_attr_string(val: &ns::AttrString) -> arc::R<Self>;

    #[objc::msg_send(textFieldWithString:)]
    pub fn with_string(val: &ns::String) -> arc::R<Self>;
}

/// NSTextFieldAttributedStringMethods
impl TextField {
    #[objc::msg_send(allowsEditingTextAttributes)]
    pub fn allows_editing_text_attrs(&self) -> bool;

    #[objc::msg_send(setAllowsEditingTextAttributes:)]
    pub fn set_allows_editing_text_attrs(&mut self, val: bool);

    #[objc::msg_send(importsGraphics)]
    pub fn imports_graphics(&self) -> bool;

    #[objc::msg_send(setImportsGraphics:)]
    pub fn set_imports_graphics(&mut self, val: bool);
}

#[objc::protocol(NSTextFieldDelegate)]
pub trait TextFieldDelegate {}

define_obj_type!(
    pub AnyTextFieldDelegate(ns::Id)
);

impl TextFieldDelegate for AnyTextFieldDelegate {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_TEXT_FIELD: &'static objc::Class<TextField>;
}

// #[cfg(test)]
// mod tests {
//     use crate::ns;

//     #[test]
//     fn basics() {
//         let text_field = ns::TextField::with_string(ns::str!(c"Hello World!"));
//         let size = text_field.size_that_fits(ns::Size::new(100.0, 100.0));
//         assert!(size.width > 50.0);
//         assert!(size.height > 20.0);
//         assert_eq!(text_field.string_value().as_ref(), "Hello World!");
//     }
// }
