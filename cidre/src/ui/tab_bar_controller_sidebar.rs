use crate::{arc, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "UITabBarControllerSidebar")]
    pub TabBarControllerSidebar(ns::Id)
);

define_obj_type!(
    /// Takes animations and completions to run alongside the sidebar
    /// showing or hiding; they run at once when that is not animated.
    #[doc(alias = "UITabBarControllerSidebarAnimating")]
    pub Animating(ns::Id)
);

#[cfg(feature = "blocks")]
impl Animating {
    #[objc::msg_send(addAnimations:)]
    pub fn add_animations_block(&mut self, animations: &mut blocks::EscBlock<fn()>);

    pub fn add_animations(&mut self, animations: impl FnMut() + 'static) {
        let mut block = blocks::EscBlock::new0(animations);
        self.add_animations_block(&mut block);
    }

    #[objc::msg_send(addCompletion:)]
    pub fn add_completion_block(&mut self, completion: &mut blocks::EscBlock<fn()>);

    pub fn add_completion(&mut self, completion: impl FnMut() + 'static) {
        let mut block = blocks::EscBlock::new0(completion);
        self.add_completion_block(&mut block);
    }
}

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
    ) -> Option<arc::R<ui::ContextMenuCfg>>;

    /// The sidebar is about to show or hide: `sidebar.is_hidden()` is
    /// changing. `animator` takes animations and completions to run
    /// alongside.
    #[objc::optional]
    #[objc::msg_send(tabBarController:sidebarVisibilityWillChange:animator:)]
    fn sidebar_visibility_will_change(
        &mut self,
        controller: &mut ui::TabBarController,
        sidebar: &mut TabBarControllerSidebar,
        animator: &mut Animating,
    );
}
