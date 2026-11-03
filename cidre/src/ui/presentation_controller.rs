use crate::{arc, cg, define_obj_type, ns, objc, ui};

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
    pub PresentationController(ns::Id), sym UIPresentationController
);

impl PresentationController {
    #[objc::init(initWithPresentedViewController:presentingViewController:)]
    pub fn init_with_presented_presenting(
        self,
        presented: &ui::ViewController,
        presenting: Option<&ui::ViewController>,
    ) -> arc::R<PresentationController>;

    #[objc::msg_send(containerView)]
    pub fn container_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(presentedViewController)]
    pub fn presented_vc(&self) -> arc::R<ui::ViewController>;

    #[objc::msg_send(frameOfPresentedViewInContainerView)]
    pub fn frame_of_presented_view_in_container_view(&self) -> cg::Rect;

    #[objc::msg_send(shouldPresentInFullscreen)]
    pub fn should_present_in_fullscreen(&self) -> bool;

    #[objc::msg_send(shouldRemovePresentersView)]
    pub fn should_remove_presenters_view(&self) -> bool;

    #[objc::msg_send(containerViewWillLayoutSubviews)]
    pub fn container_view_will_layout_subviews(&mut self);

    #[objc::msg_send(presentationTransitionWillBegin)]
    pub fn presentation_transition_will_begin(&mut self);

    #[objc::msg_send(preferredContentSizeDidChangeForChildContentContainer:)]
    pub fn preferred_content_size_did_change(&mut self, container: &ns::Id);

    /// The view UIKit animates for this presentation, including any wrapper
    /// supplied by the presentation controller.
    #[objc::msg_send(presentedView)]
    pub fn presented_view(&self) -> Option<arc::R<ui::View>>;
}
