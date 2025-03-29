use crate::{arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSSplitViewItemBehavior")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(isize)]
pub enum SplitViewItemBehavior {
    Default,
    Sidebar,
    ContentList,
    Inspector,
}

#[doc(alias = "NSSplitViewItemCollapseBehavior")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(isize)]
pub enum SplitViewItemCollapseBehavior {
    /// The item uses the default collapsing behavior for its set `behavior`. The default may change over time.
    Default,

    /// The item prefers to keep the other panes at their current size and position on screen, potentially growing or shrinking the window in the direction
    /// to best preserve that. But it will break that preference in order to keep the window fully on screen or when in full screen.
    PreferResizingSplitViewWithFixedSiblings,

    /// The item prefers to resize the other split panes. This will be broken when uncollapsing if the item can't fully uncollapse before hitting the minimum size
    /// of the other panes or the window.
    PreferResizingSiblingsWithFixedSplitView,

    /// The item will collapse/uncollapse purely from a constraint animation, with a constraint priority of the item’s `holding_priority`.
    /// This could result in a partial internal content resize and window resize, and has no implications for keeping the window on screen.
    /// External constraints can be used to tweak exactly how the animation affects item, sibling, and window size and positions.
    UseConstraints,
}

define_obj_type!(
    #[doc(alias = "NSSplitViewItem")]
    pub SplitViewItem(ns::Id),
    NS_SPLIT_VIEW_ITEM
);

impl SplitViewItem {
    #[objc::msg_send(splitViewItemWithViewController:)]
    pub fn with_vc(vc: &ns::ViewController) -> arc::R<Self>;

    #[objc::msg_send(sidebarWithViewController:)]
    pub fn sidebar_with_vc(vc: &ns::ViewController) -> arc::R<Self>;

    #[objc::msg_send(contentListWithViewController:)]
    pub fn content_list_with_vc(vc: &ns::ViewController) -> arc::R<Self>;

    #[objc::msg_send(inspectorWithViewController:)]
    pub fn inspector_with_vc(vc: &ns::ViewController) -> arc::R<Self>;

    #[objc::msg_send(behavior)]
    pub fn behavior(&self) -> SplitViewItemBehavior;

    #[objc::msg_send(viewController)]
    pub fn view_controller(&self) -> arc::R<ns::ViewController>;

    #[objc::msg_send(setViewController:)]
    pub fn set_view_controller(&mut self, val: &ns::ViewController);

    #[objc::msg_send(collapsed)]
    pub fn collapsed(&self) -> bool;

    #[objc::msg_send(setCollapsed:)]
    pub fn set_collapsed(&mut self, val: bool);

    #[objc::msg_send(canCollapse)]
    pub fn can_collapse(&self) -> bool;

    #[objc::msg_send(collapseBehavior)]
    pub fn collapse_behavior(&self) -> SplitViewItemCollapseBehavior;

    #[objc::msg_send(setCollapseBehavior:)]
    pub fn set_collapse_behavior(&mut self, val: SplitViewItemCollapseBehavior);

    #[objc::msg_send(minimumThickness)]
    pub fn min_thickness(&self) -> cg::Float;

    #[objc::msg_send(setMinimumThickness:)]
    pub fn set_min_thickness(&mut self, val: cg::Float);

    #[objc::msg_send(maximumThickness)]
    pub fn max_thickness(&self) -> cg::Float;

    #[objc::msg_send(setMaximumThickness:)]
    pub fn set_max_thickness(&mut self, val: cg::Float);

    #[objc::msg_send(preferredThicknessFraction)]
    pub fn preferred_thickness_fraction(&self) -> cg::Float;

    #[objc::msg_send(setPreferredThicknessFraction:)]
    pub fn set_preferred_thickness_fraction(&mut self, val: cg::Float);

    #[objc::msg_send(holdingPriority)]
    pub fn holding_priority(&self) -> ns::LayoutPriority;

    #[objc::msg_send(setHoldingPriority:)]
    pub fn set_holding_priority(&mut self, val: ns::LayoutPriority);

    #[objc::msg_send(automaticMaximumThickness)]
    pub fn automatic_max_thickness(&self) -> cg::Float;

    #[objc::msg_send(setAutomaticMaximumThickness:)]
    pub fn set_automatic_max_thickness(&mut self, val: cg::Float);

    #[objc::msg_send(isSpringLoaded)]
    pub fn is_spring_loaded(&self) -> bool;

    #[objc::msg_send(setSpringLoaded:)]
    pub fn set_spring_loaded(&mut self, val: bool);

    #[objc::msg_send(canCollapseFromWindowResize)]
    pub fn can_collapse_from_window_resize(&self) -> bool;

    #[objc::msg_send(setCanCollapseFromWindowResize:)]
    pub fn set_can_collapse_from_window_resize(&mut self, val: bool);

    #[objc::msg_send(allowsFullHeightLayout)]
    pub fn allows_full_height_layout(&self) -> bool;

    #[objc::msg_send(setAllowsFullHeightLayout:)]
    pub fn set_allows_full_height_layout(&mut self, val: bool);

    #[objc::msg_send(titlebarSeparatorStyle)]
    pub fn titlebar_separator_style(&self) -> ns::TitlebarSeparatorStyle;

    #[objc::msg_send(setTitlebarSeparatorStyle:)]
    pub fn set_titlebar_separator_style(&mut self, val: ns::TitlebarSeparatorStyle);
}

impl ns::AnimatablePropContainer for SplitViewItem {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_SPLIT_VIEW_ITEM: &'static objc::Class<SplitViewItem>;
}
