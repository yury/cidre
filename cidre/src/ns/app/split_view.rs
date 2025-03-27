use crate::{arc, cg, define_obj_type, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[doc(alias = "NSSplitViewDividerStyle")]
pub enum SplitViewDividerStyle {
    Thick = 1,
    Thin = 2,
    PaneSplitter = 3,
}

define_obj_type!(
    #[doc(alias = "NSSplitView")]
    pub SplitView(ns::View)
);

impl SplitView {
    /// Whether the long axes of a split view's dividers are oriented up-and-down (true) or left-and-right (false)
    #[objc::msg_send(isVertical)]
    pub fn is_vertical(&self) -> bool;

    #[objc::msg_send(setVertical:)]
    pub fn set_vertical(&mut self, val: bool);

    #[objc::msg_send(dividerStyle)]
    pub fn divider_style(&self) -> SplitViewDividerStyle;

    #[objc::msg_send(setDividerStyle:)]
    pub fn set_divider_style(&mut self, val: SplitViewDividerStyle);

    #[objc::msg_send(autosaveName)]
    pub fn autosave_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAutosaveName:)]
    pub fn set_autosave_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnySplitViewDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: SplitViewDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(drawDividerInRect:)]
    pub fn draw_divider_in_rect(&self, rect: ns::Rect);

    #[objc::msg_send(dividerColor)]
    pub fn divider_color(&self) -> arc::R<ns::Color>;

    #[objc::msg_send(dividerThickness)]
    pub fn divider_thickness(&self) -> cg::Float;

    #[objc::msg_send(adjustSubviews)]
    pub fn adjust_subviews(&mut self);

    #[objc::msg_send(isSubviewCollapsed:)]
    pub fn is_subview_collapsed(&self, subview: &ns::View) -> bool;

    #[objc::msg_send(minPossiblePositionOfDividerAtIndex:)]
    pub fn min_possible_pos_of_divider_at(&self, divider_index: isize) -> cg::Float;

    #[objc::msg_send(maxPossiblePositionOfDividerAtIndex:)]
    pub fn max_possible_pos_of_divider_at(&self, divider_index: isize) -> cg::Float;

    #[objc::msg_send(setPosition:ofDividerAtIndex:)]
    pub fn set_pos_of_divider_at(&mut self, val: cg::Float, divider_index: isize);

    #[objc::msg_send(holdingPriorityForSubviewAtIndex:)]
    pub fn holding_priority_for_subview_at(&self, subview_index: isize) -> ns::LayoutPriority;

    #[objc::msg_send(setHoldingPriority:forSubviewAtIndex:)]
    pub fn set_holding_priority_for_subview_at(
        &mut self,
        val: ns::LayoutPriority,
        subview_index: isize,
    );
}

/// NSSplitViewArrangedSubviews
impl SplitView {
    #[objc::msg_send(arrangesAllSubviews)]
    pub fn arranges_all_subviews(&self) -> bool;

    #[objc::msg_send(setArrangesAllSubviews:)]
    pub fn set_arranges_all_subviews(&mut self, val: bool);

    #[objc::msg_send(arrangedSubviews)]
    pub fn arranged_subviews(&self) -> arc::R<ns::Array<ns::View>>;

    #[objc::msg_send(addArrangedSubview:)]
    pub fn add_arranged_subview(&mut self, val: &ns::View);

    #[objc::msg_send(insertArrangedSubview:atIndex:)]
    pub fn insert_arranged_subview_at(&mut self, val: &ns::View, index: isize);

    #[objc::msg_send(removeArrangedSubview:)]
    pub fn removeArrangedSubview(&mut self, val: &ns::View);
}

#[objc::protocol(NSSplitViewDelegate)]
pub trait SplitViewDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(splitView:canCollapseSubview:)]
    fn split_view_can_collapse_subview(
        &mut self,
        split_view: &mut SplitView,
        subview: &ns::View,
    ) -> bool;
}

define_obj_type!(pub AnySplitViewDelegate(ns::Id));

impl SplitViewDelegate for AnySplitViewDelegate {}
