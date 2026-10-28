use crate::{arc, blocks, define_obj_type, ns, objc, ui};

#[doc(alias = "UIContextualActionStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ContextualActionStyle {
    Normal,
    /// Red; the row goes once the handler calls back with `true`.
    Destructive,
}

/// Called with the action, the view it was shown in, and a block to call back with whether
/// it was done.
pub type ContextualActionHandler =
    blocks::EscBlock<fn(&ContextualAction, &ui::View, &mut blocks::EscBlock<fn(bool)>)>;

define_obj_type!(
    /// A button a swipe across a row shows.
    #[doc(alias = "UIContextualAction")]
    pub ContextualAction(ns::Id),
    sym UIContextualAction
);

impl ContextualAction {
    #[objc::msg_send(contextualActionWithStyle:title:handler:)]
    pub fn with_style_title_handler(
        style: ContextualActionStyle,
        title: Option<&ns::String>,
        handler: &mut ContextualActionHandler,
    ) -> arc::R<Self>;

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(backgroundColor)]
    pub fn bg_color(&self) -> arc::R<ui::Color>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_bg_color(&mut self, val: Option<&ui::Color>);
}

define_obj_type!(
    /// The buttons a swipe across a row shows, the first nearest the edge.
    #[doc(alias = "UISwipeActionsConfiguration")]
    pub SwipeActionsCfg(ns::Id),
    sym UISwipeActionsConfiguration
);

impl SwipeActionsCfg {
    #[objc::msg_send(configurationWithActions:)]
    pub fn with_actions(actions: &ns::Array<ContextualAction>) -> arc::R<Self>;

    /// A full swipe runs the first action (`true` by default).
    #[objc::msg_send(performsFirstActionWithFullSwipe)]
    pub fn performs_first_action_with_full_swipe(&self) -> bool;

    #[objc::msg_send(setPerformsFirstActionWithFullSwipe:)]
    pub fn set_performs_first_action_with_full_swipe(&mut self, val: bool);
}

/// The swipe actions of the row at an index path; `None` for none.
pub type SwipeActionsCfgProvider =
    blocks::EscBlock<fn(&ns::IndexPath) -> Option<arc::Rar<SwipeActionsCfg>>>;

impl ui::CollectionLayoutListCfg {
    #[objc::msg_send(setLeadingSwipeActionsConfigurationProvider:)]
    pub fn set_leading_swipe_actions_cfg_provider(
        &mut self,
        val: Option<&mut SwipeActionsCfgProvider>,
    );

    #[objc::msg_send(setTrailingSwipeActionsConfigurationProvider:)]
    pub fn set_trailing_swipe_actions_cfg_provider(
        &mut self,
        val: Option<&mut SwipeActionsCfgProvider>,
    );
}
