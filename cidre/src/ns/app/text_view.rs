use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTextView")]
    pub TextView(ns::Text),
    sym NSTextView
);

impl TextView {
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
