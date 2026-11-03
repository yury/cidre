use crate::{arc, objc, ui};

#[objc::protocol(UIViewControllerTransitioningDelegate)]
pub trait ViewControllerTransitioningDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(presentationControllerForPresentedViewController:presentingViewController:sourceViewController:)]
    fn presentation_controller(
        &mut self,
        presented: &ui::ViewController,
        presenting: Option<&ui::ViewController>,
        source: &ui::ViewController,
    ) -> Option<arc::R<ui::PresentationController>>;
}
