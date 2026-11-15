#[cfg(feature = "blocks")]
use crate::blocks;
use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIViewControllerTransitionCoordinatorContext")]
    pub TransitionCoordinatorCtx(ns::Id)
);

impl TransitionCoordinatorCtx {
    #[objc::msg_send(isAnimated)]
    pub fn is_animated(&self) -> bool;

    #[objc::msg_send(isInteractive)]
    pub fn is_interactive(&self) -> bool;

    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;

    #[objc::msg_send(initiallyInteractive)]
    pub fn initially_interactive(&self) -> bool;

    #[objc::msg_send(transitionDuration)]
    pub fn transition_duration(&self) -> ns::TimeInterval;

    #[objc::msg_send(percentComplete)]
    pub fn percent_complete(&self) -> cg::Float;

    #[objc::msg_send(containerView)]
    pub fn container_view(&self) -> arc::R<ui::View>;

    #[objc::msg_send(targetTransform)]
    pub fn target_transform(&self) -> cg::AffineTransform;
}

define_obj_type!(
    #[doc(alias = "UIViewControllerTransitionCoordinator")]
    pub TransitionCoordinator(TransitionCoordinatorCtx)
);

#[cfg(feature = "blocks")]
pub type TransitionCoordinatorBlock = blocks::EscBlock<fn(&TransitionCoordinatorCtx)>;

impl TransitionCoordinator {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(animateAlongsideTransition:completion:)]
    pub fn animate_alongside_blocks(
        &self,
        animations: Option<&mut TransitionCoordinatorBlock>,
        completion: Option<&mut TransitionCoordinatorBlock>,
    ) -> bool;

    #[cfg(feature = "blocks")]
    pub fn animate_alongside(
        &self,
        animations: impl FnMut(&TransitionCoordinatorCtx) + 'static,
    ) -> bool {
        let mut animations = TransitionCoordinatorBlock::new1(animations);
        self.animate_alongside_blocks(Some(&mut animations), None)
    }

    #[cfg(feature = "blocks")]
    pub fn on_completion(
        &self,
        completion: impl FnMut(&TransitionCoordinatorCtx) + 'static,
    ) -> bool {
        let mut completion = TransitionCoordinatorBlock::new1(completion);
        self.animate_alongside_blocks(None, Some(&mut completion))
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(notifyWhenInteractionChangesUsingBlock:)]
    pub fn on_interaction_change_block(&self, handler: &mut TransitionCoordinatorBlock);

    #[cfg(feature = "blocks")]
    pub fn on_interaction_change(&self, handler: impl FnMut(&TransitionCoordinatorCtx) + 'static) {
        let mut handler = TransitionCoordinatorBlock::new1(handler);
        self.on_interaction_change_block(&mut handler);
    }
}

impl ui::ViewController {
    #[objc::msg_send(transitionCoordinator)]
    pub fn transition_coordinator(&self) -> Option<arc::R<TransitionCoordinator>>;
}
