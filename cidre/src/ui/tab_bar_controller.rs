#[cfg(feature = "blocks")]
use crate::blocks;
use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UITabBarControllerMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum TabBarControllerMode {
    Automatic = 0,
    TabBar = 1,
    TabSideBar = 2,
}

#[doc(alias = "UITabBarMinimizeBehavior")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum TabBarMinimizeBehavior {
    /// Resolves to the platform's default.
    Automatic = 0,
    /// The tab bar never minimizes.
    Never = 1,
    /// The tab bar minimizes when scrolling down and expands when scrolling up.
    OnScrollDown = 2,
    /// The tab bar minimizes when scrolling up and expands when scrolling down.
    OnScrollUp = 3,
}

define_obj_type!(
    #[doc(alias = "UITabBarController")]
    pub TabBarController(ui::ViewController),
    UI_TAB_BAR_CONTROLLER
);

impl TabBarController {
    #[objc::msg_send(contentLayoutGuide)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn content_layout_guide(&self) -> arc::R<ui::LayoutGuide>;

    #[objc::init(initWithTabs:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn init_with_tabs(self, tabs: &ns::Array<ui::Tab>) -> arc::R<TabBarController>;

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

    /// Default is `TabBarMinimizeBehavior::Automatic`.
    #[objc::msg_send(tabBarMinimizeBehavior)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn tab_bar_minimize_behavior(&self) -> TabBarMinimizeBehavior;

    #[objc::msg_send(setTabBarMinimizeBehavior:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn set_tab_bar_minimize_behavior(&mut self, val: TabBarMinimizeBehavior);

    /// An accessory view shown above the tab bar, or inline with it when it is minimized.
    #[objc::msg_send(bottomAccessory)]
    #[objc::available(ios = 26.0)]
    pub fn bottom_accessory(&self) -> Option<arc::R<ui::TabAccessory>>;

    #[objc::msg_send(setBottomAccessory:)]
    #[objc::available(ios = 26.0)]
    pub fn set_bottom_accessory(&mut self, val: Option<&ui::TabAccessory>);

    #[objc::msg_send(setBottomAccessory:animated:)]
    #[objc::available(ios = 26.0)]
    pub fn set_bottom_accessory_animated(&mut self, val: Option<&ui::TabAccessory>, animated: bool);

    /// The identifier of the tab given a prominent placement in the tab bar.
    #[objc::msg_send(prominentTabIdentifier)]
    #[objc::available(ios = 27.0, visionos = 27.0)]
    pub fn prominent_tab_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setProminentTabIdentifier:)]
    #[objc::available(ios = 27.0, visionos = 27.0)]
    pub fn set_prominent_tab_id(&mut self, val: Option<&ns::String>);

    /// Batches changes made in `updates` into a single animated transaction.
    #[cfg(feature = "blocks")]
    #[objc::msg_send(performBatchUpdates:)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn perform_batch_updates_block(&mut self, updates: &mut blocks::NoEscBlock<fn()>);

    #[cfg(feature = "blocks")]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn perform_batch_updates(&mut self, mut updates: impl FnMut()) {
        let mut block = unsafe { blocks::NoEscBlock::stack0(&mut updates) };
        self.perform_batch_updates_block(&mut block);
    }
}

#[objc::protocol(UITabBarControllerDelegate)]
pub trait TabBarControllerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(tabBarController:shouldSelectViewController:)]
    fn tab_bar_controller_should_select_vc(
        &mut self,
        controller: &mut TabBarController,
        vc: &ui::ViewController,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(tabBarController:didSelectViewController:)]
    fn tab_bar_controller_did_select_vc(
        &mut self,
        controller: &mut TabBarController,
        vc: &ui::ViewController,
    );

    #[objc::optional]
    #[objc::msg_send(tabBarController:didSelectTab:previousTab:)]
    fn tab_bar_controller_did_select_tab(
        &mut self,
        controller: &mut TabBarController,
        selected: &ui::Tab,
        previous: Option<&ui::Tab>,
    );

    #[objc::optional]
    #[objc::msg_send(tabBarController:shouldSelectTab:)]
    fn tab_bar_controller_should_select_tab(
        &mut self,
        controller: &mut TabBarController,
        tab: &ui::Tab,
    ) -> bool;
}

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

unsafe extern "C" {
    static UI_TAB_BAR_CONTROLLER: &'static objc::Class<TabBarController>;
}
