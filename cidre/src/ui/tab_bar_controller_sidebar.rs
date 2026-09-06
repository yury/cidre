use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITabBarControllerSidebar")]
    pub TabBarControllerSidebar(ns::Id)
);

#[doc(alias = "UITabBarControllerSidebarPlacement")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum Placement {
    Automatic = 0,
    Sidebar = 1,
    TabBar = 2,
}

impl TabBarControllerSidebar {
    #[objc::msg_send(setPreferredPlacement:)]
    #[objc::available(ios = 27.0, visionos = 27.0)]
    pub fn set_preferred_placement(&mut self, val: Placement);

    #[objc::msg_send(setDelegate:)]
    #[objc::available(ios = 18.0, visionos = 2.0)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

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

#[objc::protocol(UITabBarControllerSidebarDelegate)]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(tabBarController:sidebar:contextMenuConfigurationForTab:)]
    fn context_menu_for_tab(
        &mut self,
        controller: &mut ui::TabBarController,
        sidebar: &mut TabBarControllerSidebar,
        tab: &ui::Tab,
    ) -> Option<arc::R<ui::ContextMenuConfiguration>>;
}
