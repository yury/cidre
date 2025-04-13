use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UINavigationController")]
    pub NavController(ui::ViewController),
    UI_NAVIGATION_CONTROLLER
);

impl arc::A<NavController> {
    #[objc::msg_send(initWithRootViewController:)]
    pub fn init_with_root_vc(self, root_vc: &ui::ViewController) -> arc::R<NavController>;
}

impl NavController {
    pub fn with_root_vc(root_vc: &ui::ViewController) -> arc::R<Self> {
        Self::alloc().init_with_root_vc(root_vc)
    }

    #[objc::msg_send(pushViewController:animated:)]
    pub fn push_vc(&mut self, vc: &ui::ViewController, animated: bool);

    #[objc::msg_send(popViewControllerAnimated)]
    pub fn pop_vc(&mut self) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(popToViewController:animated:)]
    pub fn pop_to_vc(
        &mut self,
        vc: &ui::ViewController,
        animated: bool,
    ) -> Option<arc::R<ns::Array<ui::ViewController>>>;

    #[objc::msg_send(popToRootViewControllerAnimated:)]
    pub fn pop_to_root_vc(
        &mut self,
        animated: bool,
    ) -> Option<arc::R<ns::Array<ui::ViewController>>>;

    #[objc::msg_send(topViewController)]
    pub fn top_vc(&self) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(visibleViewController)]
    pub fn visible_vc(&self) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(viewControllers)]
    pub fn vcs(&self) -> Option<arc::R<ns::Array<ui::ViewController>>>;

    #[objc::msg_send(setViewControllers:animated:)]
    pub fn set_vcs(&mut self, val: &ns::Array<ui::ViewController>, animated: bool);

    #[objc::msg_send(isNavigationBarHidden)]
    pub fn is_nav_bar_hidden(&self) -> bool;

    #[objc::msg_send(setNavigationBarHidden:)]
    pub fn set_nav_bar_hidden(&mut self, val: bool);

    #[objc::msg_send(navigationBar)]
    pub fn nav_bar(&self) -> arc::R<ui::NavBar>;

    #[objc::msg_send(isToolbarHidden)]
    pub fn is_toolbar_hidden(&self) -> bool;

    #[objc::msg_send(setToolbarHidden:)]
    pub fn set_toolbar_hidden(&mut self, val: bool);

    #[objc::msg_send(setToolbarHidden:animated:)]
    pub fn set_toolbar_hidden_animated(&mut self, val: bool, animated: bool);

    /// For use when presenting an action sheet.
    #[objc::msg_send(toolbar)]
    pub fn toolbar(&self) -> ui::Toolbar;

    #[objc::msg_send(setToolbar:)]
    pub fn set_toolbar(&mut self, val: Option<&ui::Toolbar>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyNavControllerDelegate>>;

    #[objc::msg_send(set_delegate:)]
    pub fn set_delegate<D: NavControllerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(interactivePopGestureRecognizer)]
    pub fn interactive_pop_gesture_recognizer(&self) -> Option<arc::R<ui::GestureRecognizer>>;

    /// Interpreted as pushViewController:animated:
    #[objc::msg_send(showViewController:sender:)]
    pub fn show_vc(&self, vc: &ui::ViewController, sender: Option<&ns::Id>);

    #[objc::msg_send(hidesBarsWhenKeyboardAppears)]
    pub fn hides_bars_when_keyboard_appears(&self) -> bool;

    /// When the keyboard appears, the navigation controller's navigationBar toolbar will be hidden. The bars will remain hidden
    /// when the keyboard dismisses, but a tap in the content area will show them.
    #[objc::msg_send(setHidesBarsWhenKeyboardAppears:)]
    pub fn set_hides_bars_when_keyboard_appears(&mut self, val: bool);

    #[objc::msg_send(hidesBarsOnSwipe)]
    pub fn hides_bars_on_swipe(&self) -> bool;

    /// When the user swipes, the navigation controller's navigationBar & toolbar will be hidden (on a swipe up) or shown (on a swipe down).
    /// The toolbar only participates if it has items.
    #[objc::msg_send(setHidesBarsOnSwipe:)]
    pub fn set_hides_bars_on_swipe(&mut self, val: bool);

    #[objc::msg_send(barHideOnSwipeGestureRecognizer)]
    pub fn bar_hide_on_swipe_gesture_recognizer(&self) -> arc::R<ui::PanGestureRecognizer>;

    #[objc::msg_send(hidesBarsWhenVerticallyCompact)]
    pub fn hides_bars_when_vertically_compact(&self) -> bool;

    /// When the ui::NavController's vertical size class is compact, hide the ui::NavBar and ui::Toolbar. Unhandled taps in the regions
    /// that would normally be occupied by these bars will reveal the bars.
    #[objc::msg_send(setHidesBarsWhenVerticallyCompact:)]
    pub fn set_hides_bars_when_vertically_compact(&mut self, val: bool);

    #[objc::msg_send(hidesBarsOnTap)]
    pub fn hides_bars_on_tap(&self) -> bool;

    #[objc::msg_send(setHidesBarsOnTap:)]
    pub fn set_hides_bars_on_tap(&mut self, val: bool);

    /// The gesture recognizer used to recognize if the bars will hide or show due to a tap in content.
    /// Do not change the delegate or attempt to replace this gesture by overriding this method.
    #[objc::msg_send(barHideOnTapGestureRecognizer)]
    pub fn bar_hide_on_tap_gesture_recognizer(&self) -> arc::R<ui::TapGestureRecognizer>;
}

#[objc::protocol(UINavigationControllerDelegate)]
pub trait NavControllerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(navigationController:willShowViewController:animated:)]
    fn nav_ctrl_will_show_vc(
        &mut self,
        nav_ctrl: &mut ui::NavController,
        vc: &ui::ViewController,
        animated: bool,
    );

    #[objc::optional]
    #[objc::msg_send(navigationController:didShowViewController:animated:)]
    fn nav_ctrl_did_show_vc(
        &mut self,
        nav_ctrl: &mut ui::NavController,
        vc: &ui::ViewController,
        animated: bool,
    );

    #[objc::optional]
    #[objc::msg_send(navigationControllerSupportedInterfaceOrientations:)]
    fn nav_ctrl_supported_orientations(
        &mut self,
        nav_ctrl: &mut ui::NavController,
    ) -> ui::OrientationMask;

    #[objc::optional]
    #[objc::msg_send(navigationControllerPreferredInterfaceOrientationForPresentation:)]
    fn nav_ctrl_preffered_orientation_for_presentation(
        &mut self,
        nav_ctrl: &mut ui::NavController,
    ) -> ui::Orientation;
}

define_obj_type!(
    pub AnyNavControllerDelegate(ns::Id)
);

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_NAVIGATION_CONTROLLER: &'static objc::Class<NavController>;
}
