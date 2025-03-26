use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UITabBarControllerMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum TabBarControllerMode {
    Automatic = 0,
    TabBar = 1,
    TabSideBar = 2,
}

define_obj_type!(
    #[doc(alias = "UITabBarController")]
    pub TabBarController(ui::ViewController),
    UI_TAB_BAR_CONTROLLER
);

impl arc::A<TabBarController> {
    #[objc::msg_send(initWithTabs:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn init_with_tabs(self, tabs: &ns::Array<ui::Tab>) -> arc::R<TabBarController>;
}

impl TabBarController {
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_tabs(tabs: &ns::Array<ui::Tab>) -> arc::R<Self> {
        Self::alloc().init_with_tabs(tabs)
    }

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyTabBarControllerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: TabBarControllerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(mode)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn mode(&self) -> TabBarControllerMode;

    #[objc::msg_send(setMode:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_mode(&mut self, val: TabBarControllerMode);

    #[objc::msg_send(sidebar)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn sidebar(&self) -> arc::R<ui::TabBarControllerSidebar>;

    #[objc::msg_send(customizationIdentifier)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn customization_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setCustomizationIdentifier:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_customization_id(&mut self, val: Option<&ns::String>);

    /// An optional filter to display only select root-level tabs when in a compact appearance. Default is None, which would make all tabs available.
    #[objc::msg_send(compactTabIdentifiers)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn compact_tab_ids(&self) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(setCompactTabIdentifiers:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_compact_tab_ids(&mut self, val: Option<&ns::Array<ns::String>>);

    #[objc::msg_send(selectedTab)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn selected_tab(&self) -> Option<arc::R<ui::Tab>>;

    #[objc::msg_send(setSelectedTab:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_selected_tab(&mut self, val: Option<&ui::Tab>);

    #[objc::msg_send(tabs)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn tabs(&self) -> arc::R<ns::Array<ui::Tab>>;

    #[objc::msg_send(setTabs:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_tabs(&mut self, val: &ns::Array<ui::Tab>);

    #[objc::msg_send(setTabs:animated:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_tabs_animated(&mut self, val: &ns::Array<ui::Tab>, animated: bool);

    #[objc::msg_send(tabForIdentifier:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn tab_for_id(&self, id: &ns::String) -> Option<arc::R<ui::Tab>>;

    #[objc::msg_send(isTabBarHidden)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn is_tab_bar_hidden(&self) -> bool;

    #[objc::msg_send(setTabBarHidden:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_tab_bar_hidden(&mut self, val: bool);

    #[objc::msg_send(setTabBarHidden:animated:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_tab_bar_hidden_animated(&mut self, val: bool, animated: bool);

    #[objc::msg_send(viewControllers)]
    pub fn view_controllers(&self) -> Option<arc::R<ns::Array<ui::ViewController>>>;

    #[objc::msg_send(setViewControllers:)]
    pub fn set_view_controllers(&mut self, val: Option<&ns::Array<ui::ViewController>>);

    #[objc::msg_send(setViewControllers:animated:)]
    pub fn set_view_controllers_animated(
        &mut self,
        val: Option<&ns::Array<ui::ViewController>>,
        animated: bool,
    );

    #[objc::msg_send(selectedIndex)]
    pub fn selected_index(&self) -> usize;

    #[objc::msg_send(setSelectedIndex:)]
    pub fn set_selected_index(&self, val: usize);

    #[cfg(not(all(target_os = "tvos", target_os = "visionos")))]
    #[objc::msg_send(moreNavigationController)]
    pub fn more_nav_controller(&self) -> arc::R<ui::NavController>;

    #[cfg(not(all(target_os = "tvos", target_os = "visionos")))]
    #[objc::msg_send(customizableViewControllers)]
    pub fn customizable_view_controllers(&self) -> Option<arc::R<ns::Array<ui::ViewController>>>;

    #[cfg(not(all(target_os = "tvos", target_os = "visionos")))]
    #[objc::msg_send(setCustomizableViewControllers:)]
    pub fn set_customizable_view_controllers(
        &mut self,
        val: Option<&ns::Array<ui::ViewController>>,
    );

    #[objc::msg_send(tabBar)]
    pub fn tab_bar(&self) -> arc::R<ui::TabBar>;
}

#[objc::protocol(UITabBarControllerDelegate)]
pub trait TabBarControllerDelegate: objc::Obj {}

define_obj_type!(
    pub AnyTabBarControllerDelegate(ns::Id)
);

impl TabBarControllerDelegate for AnyTabBarControllerDelegate {}

impl ui::ViewController {
    // #[objc::msg_send(tabBarItem)]
    // pub fn tab_bar_item(&self) -> arc::R<ui::TabBarItem>;

    #[objc::msg_send(tabBarController)]
    pub fn tab_bar_controller(&self) -> Option<arc::R<TabBarController>>;
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_TAB_BAR_CONTROLLER: &'static objc::Class<TabBarController>;
}
