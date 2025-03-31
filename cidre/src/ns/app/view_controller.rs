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

#[link(name = "app", kind = "static")]
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
