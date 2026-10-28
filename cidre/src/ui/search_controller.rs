use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UISearchBar")]
    pub SearchBar(ui::View),
    sym UISearchBar
);

impl SearchBar {
    #[objc::msg_send(text)]
    pub fn text(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setText:)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(placeholder)]
    pub fn placeholder(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setPlaceholder:)]
    pub fn set_placeholder(&mut self, val: Option<&ns::String>);

    /// The bar's field: its return key, its tokens.
    #[objc::msg_send(searchTextField)]
    #[objc::available(ios = 13.0)]
    pub fn search_text_field(&self) -> arc::R<ui::SearchTextField>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: SearchBarDelegate>(&mut self, val: Option<&D>);
}

#[objc::protocol(UISearchBarDelegate)]
pub trait SearchBarDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(searchBar:textDidChange:)]
    fn search_bar_text_did_change(&mut self, search_bar: &mut SearchBar, text: &ns::String);

    /// The keyboard's Search was tapped.
    #[objc::optional]
    #[objc::msg_send(searchBarSearchButtonClicked:)]
    fn search_bar_search_button_clicked(&mut self, search_bar: &mut SearchBar);

    #[objc::optional]
    #[objc::msg_send(searchBarCancelButtonClicked:)]
    fn search_bar_cancel_button_clicked(&mut self, search_bar: &mut SearchBar);
}

define_obj_type!(
    pub AnySearchBarDelegate(ns::Id)
);

impl SearchBarDelegate for AnySearchBarDelegate {}

define_obj_type!(
    /// A search bar that shows its results, in a navigation item (its `search_controller`) or
    /// on its own.
    #[doc(alias = "UISearchController")]
    pub SearchController(ui::ViewController),
    sym UISearchController
);

impl SearchController {
    /// `results` shows the results while searching; `None` for the view controller under
    /// the search bar to show them itself.
    #[objc::init(initWithSearchResultsController:)]
    pub fn init_with_results_vc(self, results: Option<&ui::ViewController>)
    -> arc::R<SearchController>;

    pub fn with_results_vc(results: Option<&ui::ViewController>) -> arc::R<Self> {
        Self::alloc().init_with_results_vc(results)
    }

    #[objc::msg_send(searchBar)]
    pub fn search_bar(&self) -> arc::R<SearchBar>;

    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;

    #[objc::msg_send(setActive:)]
    pub fn set_active(&mut self, val: bool);

    /// Weak.
    #[objc::msg_send(setSearchResultsUpdater:)]
    pub fn set_search_results_updater<D: SearchResultsUpdating>(&mut self, val: Option<&D>);

    #[objc::msg_send(obscuresBackgroundDuringPresentation)]
    pub fn obscures_bg_during_presentation(&self) -> bool;

    #[objc::msg_send(setObscuresBackgroundDuringPresentation:)]
    pub fn set_obscures_bg_during_presentation(&mut self, val: bool);

    #[objc::msg_send(hidesNavigationBarDuringPresentation)]
    pub fn hides_nav_bar_during_presentation(&self) -> bool;

    #[objc::msg_send(setHidesNavigationBarDuringPresentation:)]
    pub fn set_hides_nav_bar_during_presentation(&mut self, val: bool);

    /// Where the bar was put; valid once in a navigation item.
    #[objc::msg_send(searchBarPlacement)]
    #[objc::available(ios = 16.0)]
    pub fn search_bar_placement(&self) -> ui::NavItemSearchBarPlacement;
}

#[objc::protocol(UISearchResultsUpdating)]
pub trait SearchResultsUpdating: objc::Obj {
    /// The search bar's text changed, or it became or stopped being active.
    #[objc::msg_send(updateSearchResultsForSearchController:)]
    fn update_search_results_for_search_controller(
        &mut self,
        search_controller: &mut SearchController,
    );
}

define_obj_type!(
    pub AnySearchResultsUpdating(ns::Id)
);

impl SearchResultsUpdating for AnySearchResultsUpdating {}
