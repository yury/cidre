use crate::{define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITabBarControllerSidebar")]
    pub TabBarControllerSidebar(ns::Id)
);

impl TabBarControllerSidebar {
    #[objc::msg_send(setBottomBarView:)]
    #[objc::available(ios = 18.0, visionos = 2.0)]
    pub fn set_bottom_bar_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(isAvailable)]
    #[objc::available(ios = 27.0)]
    pub fn is_available(&self) -> bool;

    #[objc::msg_send(isHidden)]
    #[objc::available(ios = 18.0)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    #[objc::available(ios = 18.0)]
    pub fn set_hidden(&mut self, val: bool);
}
