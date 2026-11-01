use crate::{arc, define_obj_type, ns, objc, ui};

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

impl PresentationController {
    /// The view UIKit animates for this presentation, including any wrapper
    /// supplied by the presentation controller.
    #[objc::msg_send(presentedView)]
    pub fn presented_view(&self) -> Option<arc::R<ui::View>>;
}
