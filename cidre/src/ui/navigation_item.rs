use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UINavigationItemLargeTitleDisplayMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum NavItemLargeTitleDisplayMode {
    /// Inherits the large title display from the previous item in the navigation stack.
    Automatic = 0,
    /// Always displays a large title, when the navigation bar prefers large titles.
    Always = 1,
    /// Never displays a large title.
    Never = 2,
    /// The title in the bar inline, with no room kept for a large one.
    Inline = 3,
}

/// Where a navigation item's search bar goes.
#[doc(alias = "UINavigationItemSearchBarPlacement")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum NavItemSearchBarPlacement {
    Automatic = 0,
    /// Inline with the other items, trailing; on iPhone in a navigation controller, maybe
    /// in the toolbar.
    Integrated = 1,
    /// Under the title.
    Stacked = 2,
    /// As integrated, centered in a regular-width iPad's bar.
    IntegratedCentered = 3,
    /// As integrated, a button while inactive even with room for the field.
    IntegratedButton = 4,
}

define_obj_type!(
    #[doc(alias = "UINavigationItem")]
    pub NavItem(ns::Id),
    sym UINavigationItem
);

impl NavItem {
    #[objc::msg_send(largeTitleDisplayMode)]
    #[objc::available(ios = 11.0)]
    pub fn large_title_display_mode(&self) -> NavItemLargeTitleDisplayMode;

    #[objc::msg_send(setLargeTitleDisplayMode:)]
    #[objc::available(ios = 11.0)]
    pub fn set_large_title_display_mode(&mut self, val: NavItemLargeTitleDisplayMode);

    #[objc::msg_send(setRightBarButtonItems:)]
    pub fn set_right_bar_button_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);

    #[objc::msg_send(setLeftBarButtonItems:)]
    pub fn set_left_bar_button_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);

    /// The line under the title in the navigation bar.
    #[objc::msg_send(subtitle)]
    #[objc::available(ios = 26.0)]
    pub fn subtitle(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setSubtitle:)]
    #[objc::available(ios = 26.0)]
    pub fn set_subtitle(&mut self, val: Option<&ns::String>);

    /// How the navigation bar minimizes in response to scrolling; an
    /// integrated top tab bar minimizes with it.
    #[objc::msg_send(navigationBarMinimization)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn nav_bar_minimization(&self) -> arc::R<ui::BarMinimization>;

    #[objc::msg_send(setNavigationBarMinimization:)]
    #[objc::available(ios = 27.0, tvos = 27.0, visionos = 27.0)]
    pub fn set_nav_bar_minimization(&mut self, val: &ui::BarMinimization);

    #[objc::msg_send(searchController)]
    #[objc::available(ios = 11.0)]
    pub fn search_controller(&self) -> Option<arc::R<ui::SearchController>>;

    #[objc::msg_send(setSearchController:)]
    #[objc::available(ios = 11.0)]
    pub fn set_search_controller(&mut self, val: Option<&ui::SearchController>);

    /// Ignored for the integrated placements.
    #[objc::msg_send(hidesSearchBarWhenScrolling)]
    #[objc::available(ios = 11.0)]
    pub fn hides_search_bar_when_scrolling(&self) -> bool;

    #[objc::msg_send(setHidesSearchBarWhenScrolling:)]
    #[objc::available(ios = 11.0)]
    pub fn set_hides_search_bar_when_scrolling(&mut self, val: bool);

    #[objc::msg_send(preferredSearchBarPlacement)]
    #[objc::available(ios = 16.0)]
    pub fn preferred_search_bar_placement(&self) -> NavItemSearchBarPlacement;

    #[objc::msg_send(setPreferredSearchBarPlacement:)]
    #[objc::available(ios = 16.0)]
    pub fn set_preferred_search_bar_placement(&mut self, val: NavItemSearchBarPlacement);

    /// Where the bar was put; valid once there is a search controller.
    #[objc::msg_send(searchBarPlacement)]
    #[objc::available(ios = 16.0)]
    pub fn search_bar_placement(&self) -> NavItemSearchBarPlacement;

    /// The search bar as an item, to place it among the toolbar's or the bar's items.
    #[objc::msg_send(searchBarPlacementBarButtonItem)]
    #[objc::available(ios = 26.0)]
    pub fn search_bar_placement_bar_button_item(&self) -> arc::R<ui::BarButtonItem>;

    /// On iPhone, whether the search bar may go in the toolbar (`true` by default).
    #[objc::msg_send(searchBarPlacementAllowsToolbarIntegration)]
    #[objc::available(ios = 26.0)]
    pub fn search_bar_placement_allows_toolbar_integration(&self) -> bool;

    #[objc::msg_send(setSearchBarPlacementAllowsToolbarIntegration:)]
    #[objc::available(ios = 26.0)]
    pub fn set_search_bar_placement_allows_toolbar_integration(&mut self, val: bool);
}
