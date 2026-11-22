use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSDraggingItem")]
    pub DraggingItem(ns::Id), sym NSDraggingItem
);

impl DraggingItem {
    #[objc::init(initWithPasteboardWriter:)]
    pub fn init_with_pasteboard_writer<W: ns::PasteboardWriting>(
        self,
        writer: &W,
    ) -> arc::R<DraggingItem>;

    #[objc::msg_send(setDraggingFrame:contents:)]
    pub fn set_dragging_frame_contents(&mut self, frame: ns::Rect, contents: Option<&ns::Id>);
}
