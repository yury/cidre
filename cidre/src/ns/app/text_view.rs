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
}
