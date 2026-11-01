use crate::{define_obj_type, ns, objc, ui};

#[objc::protocol(UIAdaptivePresentationControllerDelegate)]
pub trait AdaptivePresentationControllerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(adaptivePresentationStyleForPresentationController:traitCollection:)]
    fn adaptive_presentation_style(
        &mut self,
        controller: &ui::PresentationController,
        traits: &ui::TraitCollection,
    ) -> ui::ModalPresentationStyle;
}

define_obj_type!(
    #[doc(alias = "UIPresentationController")]
    pub PresentationController(ns::Id)
);
