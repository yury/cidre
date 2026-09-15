use crate::{arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSUserInterfaceLayoutOrientation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum UiLayoutOrientation {
    Horizontal = 0,
    Vertical = 1,
}

#[doc(alias = "NSStackViewGravity")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum StackViewGravity {
    /// Top of a vertical stack, leading end of a horizontal one.
    Top = 1,
    Center = 2,
    /// Bottom of a vertical stack, trailing end of a horizontal one.
    Bottom = 3,
}

impl StackViewGravity {
    pub const LEADING: Self = Self::Top;
    pub const TRAILING: Self = Self::Bottom;
}

#[doc(alias = "NSStackViewDistribution")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum StackViewDistribution {
    GravityAreas = -1,
    Fill = 0,
    FillEqually = 1,
    FillProportionally = 2,
    EqualSpacing = 3,
    EqualCentering = 4,
}

define_obj_type!(
    #[doc(alias = "NSStackView")]
    pub StackView(ns::View),
    NS_STACK_VIEW
);

impl StackView {
    #[objc::msg_send(stackViewWithViews:)]
    pub fn with_views(views: &ns::Array<ns::View>) -> arc::R<Self>;

    #[objc::msg_send(orientation)]
    pub fn orientation(&self) -> UiLayoutOrientation;

    #[objc::msg_send(setOrientation:)]
    pub fn set_orientation(&mut self, val: UiLayoutOrientation);

    #[objc::msg_send(alignment)]
    pub fn alignment(&self) -> ns::LayoutAttr;

    #[objc::msg_send(setAlignment:)]
    pub fn set_alignment(&mut self, val: ns::LayoutAttr);

    #[objc::msg_send(distribution)]
    pub fn distribution(&self) -> StackViewDistribution;

    #[objc::msg_send(setDistribution:)]
    pub fn set_distribution(&mut self, val: StackViewDistribution);

    #[objc::msg_send(spacing)]
    pub fn spacing(&self) -> cg::Float;

    #[objc::msg_send(setSpacing:)]
    pub fn set_spacing(&mut self, val: cg::Float);

    #[objc::msg_send(edgeInsets)]
    pub fn edge_insets(&self) -> ns::EdgeInsets;

    #[objc::msg_send(setEdgeInsets:)]
    pub fn set_edge_insets(&mut self, val: ns::EdgeInsets);

    #[objc::msg_send(detachesHiddenViews)]
    pub fn detaches_hidden_views(&self) -> bool;

    #[objc::msg_send(setDetachesHiddenViews:)]
    pub fn set_detaches_hidden_views(&mut self, val: bool);

    #[objc::msg_send(arrangedSubviews)]
    pub fn arranged_subviews(&self) -> arc::R<ns::Array<ns::View>>;

    #[objc::msg_send(addArrangedSubview:)]
    pub fn add_arranged_subview(&mut self, view: &ns::View);

    #[objc::msg_send(insertArrangedSubview:atIndex:)]
    pub fn insert_arranged_subview_at(&mut self, view: &ns::View, index: isize);

    #[objc::msg_send(removeArrangedSubview:)]
    pub fn remove_arranged_subview(&mut self, view: &ns::View);

    #[objc::msg_send(addView:inGravity:)]
    pub fn add_view_in_gravity(&mut self, view: &ns::View, gravity: StackViewGravity);

    #[objc::msg_send(setCustomSpacing:afterView:)]
    pub fn set_custom_spacing_after_view(&mut self, spacing: cg::Float, view: &ns::View);

    #[objc::msg_send(customSpacingAfterView:)]
    pub fn custom_spacing_after_view(&self, view: &ns::View) -> cg::Float;
}

unsafe extern "C" {
    static NS_STACK_VIEW: &'static objc::Class<StackView>;
}
