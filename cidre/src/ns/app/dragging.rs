use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    /// What a drag may do to what it carries: copy, link, move and so on.
    #[doc(alias = "NSDragOperation")]
    pub DragOp(usize)
);

impl DragOp {
    #[doc(alias = "NSDragOperationNone")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "NSDragOperationCopy")]
    pub const COPY: Self = Self(1);
    #[doc(alias = "NSDragOperationLink")]
    pub const LINK: Self = Self(2);
    #[doc(alias = "NSDragOperationGeneric")]
    pub const GENERIC: Self = Self(4);
    #[doc(alias = "NSDragOperationPrivate")]
    pub const PRIVATE: Self = Self(8);
    #[doc(alias = "NSDragOperationMove")]
    pub const MOVE: Self = Self(16);
    #[doc(alias = "NSDragOperationDelete")]
    pub const DELETE: Self = Self(32);
    #[doc(alias = "NSDragOperationEvery")]
    pub const EVERY: Self = Self(usize::MAX);
}

/// A drag under way, as a destination sees it.
#[objc::protocol(NSDraggingInfo)]
pub trait DraggingInfo: objc::Obj {
    /// The pointer, in the destination window's base coordinates.
    #[objc::msg_send(draggingLocation)]
    fn dragging_location(&self) -> ns::Point;

    /// What the drag carries.
    #[objc::msg_send(draggingPasteboard)]
    fn dragging_pasteboard(&self) -> arc::R<ns::Pasteboard>;

    /// What the source allows.
    #[objc::msg_send(draggingSourceOperationMask)]
    fn dragging_src_op_mask(&self) -> DragOp;

    #[objc::msg_send(draggingSequenceNumber)]
    fn dragging_seq_number(&self) -> ns::Integer;
}

define_obj_type!(
    pub AnyDraggingInfo(ns::Id)
);

impl DraggingInfo for AnyDraggingInfo {}

/// What a view that takes drops answers; register the types it takes with
/// [`ns::View::register_for_dragged_types`].
#[objc::protocol(NSDraggingDestination)]
pub trait DraggingDst: objc::Obj {
    /// What a drop would do, as the drag comes in.
    #[objc::optional]
    #[objc::msg_send(draggingEntered:)]
    fn dragging_entered(&mut self, sender: &mut AnyDraggingInfo) -> DragOp;

    /// What a drop would do, as the drag moves.
    #[objc::optional]
    #[objc::msg_send(draggingUpdated:)]
    fn dragging_updated(&mut self, sender: &mut AnyDraggingInfo) -> DragOp;

    #[objc::optional]
    #[objc::msg_send(draggingExited:)]
    fn dragging_exited(&mut self, sender: Option<&mut AnyDraggingInfo>);

    #[objc::optional]
    #[objc::msg_send(prepareForDragOperation:)]
    fn prepare_for_drag_op(&mut self, sender: &mut AnyDraggingInfo) -> bool;

    /// Takes the drop; whether it did.
    #[objc::optional]
    #[objc::msg_send(performDragOperation:)]
    fn perform_drag_op(&mut self, sender: &mut AnyDraggingInfo) -> bool;

    #[objc::optional]
    #[objc::msg_send(concludeDragOperation:)]
    fn conclude_drag_op(&mut self, sender: Option<&mut AnyDraggingInfo>);
}

/// Drop targets
impl ns::View {
    /// The pasteboard types the view takes drops of.
    #[objc::msg_send(registerForDraggedTypes:)]
    pub fn register_for_dragged_types(&mut self, types: &ns::Array<ns::PasteboardType>);

    #[objc::msg_send(unregisterDraggedTypes)]
    pub fn unregister_dragged_types(&mut self);

    #[objc::msg_send(registeredDraggedTypes)]
    pub fn registered_dragged_types(&self) -> arc::R<ns::Array<ns::PasteboardType>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn register() {
        let mut view = ns::View::new();
        let types = ns::Array::from_slice(&[ns::pasteboard_types::file_url()]);
        view.register_for_dragged_types(&types);
        assert_eq!(view.registered_dragged_types().len(), 1);
        view.unregister_dragged_types();
        assert!(view.registered_dragged_types().is_empty());
    }
}
