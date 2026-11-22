use crate::{define_obj_type, ns, objc};

#[objc::protocol(UIDragAnimating)]
pub trait DragAnimating: objc::Obj {}

define_obj_type!(
    pub AnyDragAnimating(ns::Id)
);

impl DragAnimating for AnyDragAnimating {}

#[cfg(target_os = "ios")]
mod source {
    use crate::{arc, define_obj_type, ns, objc, ui};

    define_obj_type!(#[doc(alias = "UIDragInteraction")] pub DragInteraction(ns::Id), sym UIDragInteraction);
    impl ui::Interaction for DragInteraction {}
    impl DragInteraction {
        #[objc::init(initWithDelegate:)]
        pub fn init_with_delegate<D: DragInteractionDelegate>(
            self,
            delegate: &D,
        ) -> arc::R<DragInteraction>;

        #[objc::msg_send(setEnabled:)]
        pub fn set_enabled(&mut self, enabled: bool);
    }

    #[objc::protocol(UIDragInteractionDelegate)]
    pub trait DragInteractionDelegate: objc::Obj {
        #[objc::msg_send(dragInteraction:itemsForBeginningSession:)]
        fn items_for_beginning_session(
            &mut self,
            interaction: &DragInteraction,
            session: &ui::AnyDragSession,
        ) -> arc::R<ns::Array<ui::DragItem>>;
    }
}
#[cfg(target_os = "ios")]
pub use source::*;
