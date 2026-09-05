use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIViewController")]
    pub ViewController(ui::Responder), UI_VIEW_CONTROLLER
);

impl ViewController {
    #[objc::msg_send(setView:)]
    pub fn set_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(navigationItem)]
    pub fn nav_item(&self) -> arc::R<ui::NavItem>;

    #[objc::msg_send(view)]
    pub fn view(&self) -> arc::R<ui::View>;

    /// Is used for any container laying out a child view controller.
    #[objc::msg_send(preferredContentSize)]
    pub fn preferred_content_size(&self) -> cg::Size;

    #[objc::msg_send(setPreferredContentSize:)]
    pub fn set_preferred_content_size(&mut self, val: cg::Size);

    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title_string(&mut self, val: Option<&ns::String>);

    #[inline]
    pub fn set_title<S: AsRef<ns::String>>(&mut self, val: Option<&S>) {
        self.set_title_string(val.map(|s| s.as_ref()));
    }

    /// Preferred system provided transition to use when displaying this
    /// view controller. Note that this only indicates a preference.
    /// The provided transition may be ignored if not supported by the
    /// current context. For example, `ui::NavController` supports
    /// the .zoom transition, but not the .cover_vertical transition.
    #[objc::msg_send(preferredTransition)]
    pub fn preferred_transition(&self) -> Option<arc::R<ui::ViewControllerTransition>>;

    #[objc::msg_send(setPreferredTransition:)]
    pub fn set_preferred_transition(&mut self, val: Option<&ui::ViewControllerTransition>);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(presentViewController:animated:completion:)]
    pub fn present_vc_ch(
        &mut self,
        vc: &ui::ViewController,
        animated: bool,
        completion: Option<&mut crate::blocks::EscBlock<fn()>>,
    );

    #[cfg(feature = "blocks")]
    pub fn present_vc(&mut self, vc: &ui::ViewController, animated: bool) {
        self.present_vc_ch(vc, animated, None);
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(dismissViewControllerAnimated:completion:)]
    pub fn dismiss_vc_ch(
        &mut self,
        animated: bool,
        completion: Option<&mut crate::blocks::EscBlock<fn()>>,
    );

    #[cfg(feature = "blocks")]
    pub fn dismiss_vc(&mut self, animated: bool) {
        self.dismiss_vc_ch(animated, None);
    }

    #[objc::msg_send(presentedViewController)]
    pub fn presented_vc(&self) -> Option<arc::R<ui::ViewController>>;
}

unsafe extern "C" {
    static UI_VIEW_CONTROLLER: &'static objc::Class<ViewController>;
}
