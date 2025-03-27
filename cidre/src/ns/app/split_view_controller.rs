use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSplitViewController")]
    pub SplitViewController(ns::ViewController)
);

impl SplitViewController {
    #[objc::msg_send(splitView)]
    pub fn split_view(&self) -> arc::R<ns::SplitView>;

    #[objc::msg_send(setSplitView:)]
    pub fn set_split_view(&mut self, val: &ns::SplitView);

    #[objc::msg_send(splitViewItems)]
    pub fn split_view_items(&self) -> arc::R<ns::Array<ns::SplitViewItem>>;

    #[objc::msg_send(setSplitViewItems:)]
    pub fn set_split_view_items(&mut self, val: &ns::Array<ns::SplitViewItem>);

    #[objc::msg_send(insertSplitViewItem:atIndex:)]
    pub fn insert_split_view_item_at(&mut self, val: &ns::SplitViewItem, index: isize);

    #[objc::msg_send(removeSplitViewItem:)]
    pub fn remove_split_view_item(&mut self, val: &ns::SplitView);

    #[objc::msg_send(splitViewItemForViewController:)]
    pub fn split_view_item_for_vc(
        &self,
        vc: &ns::ViewController,
    ) -> Option<arc::R<ns::SplitViewItem>>;

    #[objc::msg_send(minimumThicknessForInlineSidebars)]
    pub fn min_thickness_for_inline_sidebars(&self) -> cg::Float;

    #[objc::msg_send(setMinimumThicknessForInlineSidebars:)]
    pub fn set_min_thickness_for_inline_sidebars(&mut self, val: cg::Float);

    // TODO: validation

    #[objc::msg_send(viewDidLoad)]
    pub fn view_did_load(&mut self);

    #[objc::msg_send(splitView:canCollapseSubview:)]
    pub fn split_view_can_collapse_subview(
        &self,
        split_view: &ns::SplitView,
        subview: &ns::View,
    ) -> bool;

    #[objc::msg_send(splitView:shouldHideDividerAtIndex:)]
    pub fn split_view_should_hide_divider_at(
        &self,
        split_view: &ns::SplitView,
        divider_index: isize,
    ) -> bool;

    #[objc::msg_send(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
    pub fn split_view_effective_rect_for_drawn_rect_of_divider_at(
        &self,
        split_view: &ns::SplitView,
        proposed_effective_rect: ns::Rect,
        draw_rect: ns::Rect,
        divider_index: isize,
    ) -> ns::Rect;

    #[objc::msg_send(splitView:additionalEffectiveRectOfDividerAtIndex:)]
    pub fn split_view_additional_effective_rect_of_divider_at(
        &self,
        split_view: &ns::SplitView,
        divider_index: isize,
    ) -> ns::Rect;
}

/// NSSplitViewControllerToggleSidebarAction
impl SplitViewController {
    #[objc::msg_send(toggleSidebar:)]
    #[objc::available(macos = 10.11)]
    pub fn toggle_sidebar(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(toggleInspector:)]
    #[objc::available(macos = 14.0)]
    pub fn toggle_inspector(&mut self, sender: Option<&ns::Id>);
}
