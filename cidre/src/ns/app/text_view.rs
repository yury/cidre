use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTextView")]
    pub TextView(ns::Text),
    sym NSTextView
);

impl TextView {
    /// A text view sized to its scroll view and growing down with its text, in a scroll view
    /// with a vertical scroller: its document view.
    #[objc::msg_send(scrollableTextView)]
    pub fn scrollable_text_view() -> arc::R<ns::ScrollView>;

    #[objc::msg_send(textContainerInset)]
    pub fn text_container_inset(&self) -> ns::Size;

    #[objc::msg_send(setTextContainerInset:)]
    pub fn set_text_container_inset(&mut self, val: ns::Size);

    #[objc::msg_send(allowsUndo)]
    pub fn allows_undo(&self) -> bool;

    #[objc::msg_send(setAllowsUndo:)]
    pub fn set_allows_undo(&mut self, val: bool);

    /// `NSTextInputClient`: inserts `string` over `replacement_range` (the selection where its
    /// location is `ns::NOT_FOUND`), as typing it would: undoable, the delegate told.
    #[objc::msg_send(insertText:replacementRange:)]
    pub fn insert_text_replacement_range(
        &mut self,
        string: &ns::String,
        replacement_range: ns::Range,
    );

    /// `NSTextInputClient`: the screen rect of the first line of `range`'s characters; with
    /// `actual_range`, the part of `range` it covers.
    #[objc::msg_send(firstRectForCharacterRange:actualRange:)]
    pub fn first_rect_for_char_range(
        &self,
        range: ns::Range,
        actual_range: *mut ns::Range,
    ) -> ns::Rect;
}
