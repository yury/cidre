use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    pub ViewController(ns::Responder), NS_VIEW_CONTROLLER
);

impl ViewController {
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title_string(&mut self, val: Option<&ns::String>);

    #[inline]
    pub fn set_title<S: AsRef<ns::String>>(&mut self, val: Option<&S>) {
        self.set_title_string(val.map(|s| s.as_ref()));
    }

    #[objc::msg_send(view)]
    pub fn view(&self) -> arc::R<ns::View>;

    #[objc::msg_send(setView:)]
    pub fn set_view(&mut self, val: &ns::View);

    #[objc::msg_send(viewIfLoaded)]
    #[objc::available(macos = 14.0)]
    pub fn view_if_loaded(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(loadView)]
    pub fn load_view(&mut self);

    #[objc::msg_send(loadViewIfNeeded)]
    #[objc::available(macos = 14.0)]
    pub fn load_view_if_needed(&mut self);

    #[objc::msg_send(isViewLoaded)]
    pub fn is_view_loaded(&self) -> bool;
}

/// Presents and dismisses a view controller's view in a custom way.
#[objc::protocol(NSViewControllerPresentationAnimator)]
pub trait ViewControllerPresentationAnimator: objc::Obj {
    /// Called when `vc` is going to be presented by `from_vc`; the animator shows its view.
    #[objc::msg_send(animatePresentationOfViewController:fromViewController:)]
    fn animate_presentation_of_vc(
        &mut self,
        vc: &mut ns::ViewController,
        from_vc: &mut ns::ViewController,
    );

    /// Called to dismiss a previously shown `vc`; the animator removes its view.
    #[objc::msg_send(animateDismissalOfViewController:fromViewController:)]
    fn animate_dismissal_of_vc(
        &mut self,
        vc: &mut ns::ViewController,
        from_vc: &mut ns::ViewController,
    );
}

/// NSViewControllerPresentation
impl ViewController {
    /// Presents `vc` with `animator`, which is kept until `vc` is dismissed.
    #[objc::msg_send(presentViewController:animator:)]
    #[objc::available(macos = 10.10)]
    pub fn present_vc_animator<A: ViewControllerPresentationAnimator>(
        &mut self,
        vc: &ns::ViewController,
        animator: &A,
    );

    /// Dismisses the receiver. Does nothing if the receiver is not currently presented.
    #[objc::msg_send(dismissController:)]
    #[objc::available(macos = 10.10)]
    pub fn dismiss_controller(&mut self, sender: Option<&ns::Id>);

    /// Dismisses `vc`, previously presented by the receiver.
    #[objc::msg_send(dismissViewController:)]
    #[objc::available(macos = 10.10)]
    pub fn dismiss_vc(&mut self, vc: &ns::ViewController);

    /// Presents `vc` as a sheet.
    #[objc::msg_send(presentViewControllerAsSheet:)]
    #[objc::available(macos = 10.10)]
    pub fn present_vc_as_sheet(&mut self, vc: &ns::ViewController);

    /// Presents `vc` as a modal window (also known as an alert).
    #[objc::msg_send(presentViewControllerAsModalWindow:)]
    #[objc::available(macos = 10.10)]
    pub fn present_vc_as_modal_window(&mut self, vc: &ns::ViewController);

    #[objc::msg_send(presentedViewControllers)]
    #[objc::available(macos = 10.10)]
    pub fn presented_vcs(&self) -> Option<arc::R<ns::Array<ns::ViewController>>>;

    #[objc::msg_send(presentingViewController)]
    #[objc::available(macos = 10.10)]
    pub fn presenting_vc(&self) -> Option<arc::R<ns::ViewController>>;
}

/// NSViewControllerContainer
impl ViewController {
    #[objc::msg_send(parentViewController)]
    pub fn parent_vc(&self) -> Option<arc::R<ns::ViewController>>;

    #[objc::msg_send(childViewControllers)]
    pub fn child_vcs(&self) -> arc::R<ns::Array<ns::ViewController>>;

    #[objc::msg_send(addChildViewController:)]
    pub fn add_child_vc(&mut self, val: &ns::ViewController);

    #[objc::msg_send(removeFromParentViewController)]
    pub fn remove_from_parent_vc(&mut self);

    #[objc::msg_send(insertChildViewController:atIndex:)]
    pub fn insert_child_vc_at(&mut self, val: &ns::ViewController, index: usize);

    #[objc::msg_send(removeChildViewControllerAtIndex:)]
    pub fn remove_child_vc_at(&mut self, index: usize);

    #[objc::msg_send(preferredContentSizeDidChangeForViewController:)]
    pub fn preferred_content_size_did_change_for_vc(&mut self, val: &ns::ViewController);

    #[objc::msg_send(viewWillTransitionToSize:)]
    pub fn view_will_transition_to_size(&mut self, val: ns::Size);
}

unsafe extern "C" {
    static NS_VIEW_CONTROLLER: &'static objc::Class<ViewController>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut vc = ns::ViewController::new();
        assert_eq!(vc.is_view_loaded(), false);
        assert!(vc.view_if_loaded().is_none());
        vc.load_view();
        assert_eq!(vc.is_view_loaded(), true);
        vc.set_title(Some(ns::str!(c"Hello")));
        assert_eq!(vc.title().unwrap().as_ref(), "Hello");
        assert!(vc.child_vcs().is_empty());
        assert!(vc.parent_vc().is_none());
        assert!(vc.view_if_loaded().is_some());
    }
}
